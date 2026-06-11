use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::process::{Child, Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut n_games: usize = 0;
    let mut watch = false;
    let mut log = false;
    let mut snakes: Vec<String> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-n" => {
                i += 1;
                n_games = args[i].parse().expect("Invalid number for -n");
            }
            "-w" => watch = true,
            "-l" => log = true,
            arg if arg.starts_with('-') && arg[1..].chars().all(|c| c.is_ascii_digit()) => {
                n_games = arg[1..].parse().unwrap();
            }
            other => snakes.push(other.to_string()),
        }
        i += 1;
    }

    if snakes.len() < 2 {
        eprintln!("Usage: run_local_simulation [-n NUM_GAMES|-NUM_GAMES] [-w] [-l] snake1 snake2 [snake3 snake4]");
        eprintln!("Variants: depth_first breadth_first simple_tree_search simple_hungry single_gamestate_nodes");
        std::process::exit(1);
    }

    // Build release binary
    eprintln!("Building...");
    let build_status = Command::new("cargo")
        .args(["build", "--release"])
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("Failed to run cargo build");
    if !build_status.success() {
        std::process::exit(1);
    }

    let base_port: u16 = 8001;
    let mut server_pids: Vec<Child> = Vec::new();
    let mut battlesnake_args: Vec<String> = Vec::new();
    let mut snake_names: Vec<String> = Vec::new();

    // Start servers
    if log {
        fs::create_dir_all("game_logs").unwrap();
    }
    for (idx, variant) in snakes.iter().enumerate() {
        let port = base_port + idx as u16;
        let name = format!("{}_{}", variant, idx + 1);

        // Kill anything on that port
        kill_port(port);

        let child = if log && idx == 0 {
            let log_file =
                std::fs::File::create("game_logs/.server.log").expect("Cannot create log file");
            Command::new("./target/release/battlesnake_game_of_chicken")
                .env("PORT", port.to_string())
                .env("VARIANT", variant)
                .env("LOG_BOARD", "1")
                .env("LOG_EVAL", "1")
                .stdout(log_file.try_clone().unwrap())
                .stderr(log_file)
                .spawn()
                .expect("Failed to start server")
        } else {
            Command::new("./target/release/battlesnake_game_of_chicken")
                .env("PORT", port.to_string())
                .env("VARIANT", variant)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("Failed to start server")
        };

        eprintln!(
            "Started {} on :{} (PID {}){}",
            name,
            port,
            child.id(),
            if log && idx == 0 {
                " (logging)"
            } else {
                ""
            }
        );

        server_pids.push(child);
        battlesnake_args.extend_from_slice(&[
            "--name".to_string(),
            name.clone(),
            "--url".to_string(),
            format!("http://localhost:{}", port),
        ]);
        snake_names.push(name);
    }

    // Wait for servers to start
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Tally
    let mut wins: HashMap<String, usize> = HashMap::new();
    for name in &snake_names {
        wins.insert(name.clone(), 0);
    }
    let mut draws: usize = 0;
    let mut total: usize = 0;

    // Per-game log tracking
    let mut log_position: u64 = 0;

    let mut play_flags: Vec<String> = Vec::new();
    if watch {
        play_flags.extend_from_slice(&["-v".to_string(), "-c".to_string()]);
    }

    loop {
        let mut cmd = Command::new("../battlesnake_local/battlesnake");
        cmd.args(["play", "-W", "11", "-H", "11"]);
        cmd.args(&play_flags);
        cmd.args(&battlesnake_args);
        cmd.stderr(Stdio::piped());
        cmd.stdout(Stdio::piped());

        let child = cmd.spawn().expect("Failed to start battlesnake play");
        let output = child.wait_with_output().expect("Failed to wait for game");

        let combined = String::from_utf8_lossy(&output.stderr);
        if watch {
            eprint!("{}", combined);
        }

        // Parse game info from stderr
        let mut winner_name: Option<String> = None;
        let mut turns = String::from("?");
        let mut seed = String::from("?");

        for line in combined.lines() {
            if let Some(pos) = line.find(" was the winner") {
                let before = &line[..pos];
                if let Some(w) = before.split_whitespace().last() {
                    winner_name = Some(w.to_string());
                }
                // "Game completed after NN turns."
                if let Some(after_pos) = line.find("after ") {
                    let rest = &line[after_pos + 6..];
                    if let Some(sp) = rest.find(' ') {
                        turns = rest[..sp].to_string();
                    }
                }
            } else if line.contains("Game completed") && !line.contains("was the winner") {
                // Draw - "Game completed after NN turns."
                if let Some(after_pos) = line.find("after ") {
                    let rest = &line[after_pos + 6..];
                    if let Some(sp) = rest.find(' ') {
                        turns = rest[..sp].to_string();
                    }
                }
            }
            if let Some(seed_pos) = line.find("Seed: ") {
                seed = line[seed_pos + 6..].trim().to_string();
            }
        }

        // Print per-game result
        match &winner_name {
            Some(w) => {
                if let Some(count) = wins.get_mut(w.as_str()) {
                    *count += 1;
                }
                total += 1;
                eprintln!(
                    "Game {:>3}: {} won in {} turns (seed {})",
                    total, w, turns, seed
                );
            }
            None => {
                draws += 1;
                total += 1;
                eprintln!(
                    "Game {:>3}: draw after {} turns (seed {})",
                    total, turns, seed
                );
            }
        }

        // Write per-game log file
        if log {
            let first_snake_won = winner_name.as_ref() == Some(&snake_names[0]);
            if let Ok(mut file) = fs::File::open("game_logs/.server.log") {
                file.seek(SeekFrom::Start(log_position)).ok();
                let mut new_content = String::new();
                file.read_to_string(&mut new_content).ok();
                let new_position = log_position + new_content.len() as u64;
                if !new_content.is_empty() {
                    let suffix = if first_snake_won { "" } else { "_lost" };
                    let game_log_path = format!("game_logs/game_{}{}.log", total, suffix);
                    fs::write(&game_log_path, &new_content).unwrap();
                    eprintln!("  Game log: {}", game_log_path);
                }
                log_position = new_position;
            }
        }

        eprintln!();
        eprintln!(
            "  {:<28} {:>6}  {:>6}",
            "Snake", "Wins", "Win%"
        );
        eprintln!(
            "  {:<28} {:>6}  {:>6}",
            "----------------------------", "------", "------"
        );
        for name in &snake_names {
            let w = wins[name];
            let pct = if total > 0 {
                w as f64 * 100.0 / total as f64
            } else {
                0.0
            };
            eprintln!("  {:<28} {:>6}  {:>5.1}%", name, w, pct);
        }
        if draws > 0 {
            let pct = draws as f64 * 100.0 / total as f64;
            eprintln!("  {:<28} {:>6}  {:>5.1}%", "draws", draws, pct);
        }
        eprintln!("  Games played: {}", total);
        if n_games > 0 {
            eprintln!("  Target: {}", n_games);
        }

        if n_games > 0 && total >= n_games {
            break;
        }
    }

    // Cleanup
    eprintln!("\nStopping servers...");
    for mut child in server_pids {
        let _ = child.kill();
        let _ = child.wait();
    }
    // Sweep ports
    for (idx, _) in snakes.iter().enumerate() {
        kill_port(base_port + idx as u16);
    }
    // Clean up temp server log
    let _ = fs::remove_file("game_logs/.server.log");
    eprintln!("Done.");
}

fn kill_port(port: u16) {
    if let Ok(output) = Command::new("lsof")
        .args(["-t", &format!("-i:{}", port)])
        .output()
    {
        let pids = String::from_utf8_lossy(&output.stdout);
        for pid_str in pids.split_whitespace() {
            if let Ok(pid) = pid_str.parse::<i32>() {
                let _ = Command::new("kill").args(["-9", &pid.to_string()]).status();
            }
        }
    }
}
