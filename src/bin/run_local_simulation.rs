use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

struct SnakeSpec {
    variant: String,
    git_ref: Option<String>,
    /// Directory of the binary to use (None = current ./target/release)
    binary_dir: Option<String>,
}

impl SnakeSpec {
    fn parse(s: &str) -> Self {
        if let Some((variant, git_ref)) = s.split_once(':') {
            let safe: String = git_ref
                .chars()
                .map(|c| {
                    if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' {
                        c
                    } else {
                        '_'
                    }
                })
                .collect();
            SnakeSpec {
                variant: variant.to_string(),
                git_ref: Some(git_ref.to_string()),
                binary_dir: Some(format!("/tmp/battlesnake_ref_{}", safe)),
            }
        } else {
            SnakeSpec {
                variant: s.to_string(),
                git_ref: None,
                binary_dir: None,
            }
        }
    }

    fn binary_path(&self) -> PathBuf {
        match &self.binary_dir {
            Some(dir) => PathBuf::from(format!("{}/release/battlesnake_game_of_chicken", dir)),
            None => PathBuf::from("./target/release/battlesnake_game_of_chicken"),
        }
    }

    /// Display name shown in stats (e.g. "single_gamestate_nodes:v1" or "depth_first")
    fn display_name(&self, idx: usize) -> String {
        let base = match &self.git_ref {
            Some(r) => format!("{}:{}", self.variant, r),
            None => self.variant.clone(),
        };
        format!("{}_{}", base, idx + 1)
    }
}

/// From the last request turn where 2+ snakes were alive, return (winner_length, max_other_length).
fn parse_last_decisive_lengths(log_content: &str, winner_name: &str) -> Option<(i64, i64)> {
    let mut last_state: Option<(i64, i64)> = None;
    let arrow = " Request -> ";
    for line in log_content.lines() {
        if let Some(pos) = line.find(arrow) {
            let json_str = &line[pos + arrow.len()..];
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(json_str) {
                if let Some(snakes) = val["board"]["snakes"].as_array() {
                    if snakes.len() >= 2 {
                        let winner_len = snakes
                            .iter()
                            .find(|s| s["name"].as_str() == Some(winner_name))
                            .and_then(|s| s["length"].as_i64());
                        let max_other = snakes
                            .iter()
                            .filter(|s| s["name"].as_str() != Some(winner_name))
                            .filter_map(|s| s["length"].as_i64())
                            .max();
                        if let (Some(wl), Some(mo)) = (winner_len, max_other) {
                            last_state = Some((wl, mo));
                        }
                    }
                }
            }
        }
    }
    last_state
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut n_games: usize = 0;
    let mut watch = false;
    let mut log = false;
    let mut raw_snakes: Vec<String> = Vec::new();

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
            other => raw_snakes.push(other.to_string()),
        }
        i += 1;
    }

    if raw_snakes.len() < 2 {
        eprintln!("Usage: run_local_simulation [-n NUM_GAMES|-NUM_GAMES] [-w] [-l] snake1 snake2 [snake3 snake4]");
        eprintln!("Variants: depth_first breadth_first simple_tree_search simple_hungry single_gamestate_nodes");
        eprintln!("Append :<git-ref> to build that snake from a specific tag/commit:");
        eprintln!("  single_gamestate_nodes:v2025-06-01  depth_first:abc1234");
        std::process::exit(1);
    }

    let snakes: Vec<SnakeSpec> = raw_snakes.iter().map(|s| SnakeSpec::parse(s)).collect();

    // Build current version (covers all snakes without a git_ref)
    let needs_current = snakes.iter().any(|s| s.git_ref.is_none());
    if needs_current {
        eprintln!("Building current version...");
        let status = Command::new("cargo")
            .args(["build", "--release"])
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .status()
            .expect("Failed to run cargo build");
        if !status.success() {
            std::process::exit(1);
        }
    }

    // Build each unique git ref
    let mut worktrees: Vec<String> = Vec::new();
    let mut built_refs: HashMap<String, bool> = HashMap::new();
    for snake in &snakes {
        if let (Some(git_ref), Some(binary_dir)) = (&snake.git_ref, &snake.binary_dir) {
            if built_refs.contains_key(git_ref) {
                continue;
            }
            built_refs.insert(git_ref.clone(), true);

            let safe_ref = binary_dir
                .trim_start_matches("/tmp/battlesnake_ref_");
            let worktree_path = format!(".wt_{}", safe_ref);

            eprintln!("Setting up worktree for '{}'...", git_ref);
            let _ = Command::new("git")
                .args(["worktree", "remove", "--force", &worktree_path])
                .output();
            let wt_status = Command::new("git")
                .args(["worktree", "add", "--detach", &worktree_path, git_ref])
                .status()
                .expect("Failed to run git worktree add");
            if !wt_status.success() {
                eprintln!("Failed to create worktree for '{}'. Available refs:", git_ref);
                eprintln!("  git tag -l          (tags)");
                eprintln!("  git log --oneline -10  (recent commits)");
                cleanup_all_worktrees(&worktrees);
                std::process::exit(1);
            }
            worktrees.push(worktree_path.clone());

            eprintln!("Building '{}' in {}...", git_ref, worktree_path);
            fs::create_dir_all(binary_dir).unwrap();
            let build_status = Command::new("cargo")
                .args(["build", "--release", "--target-dir", binary_dir])
                .current_dir(&worktree_path)
                .stderr(Stdio::inherit())
                .stdout(Stdio::inherit())
                .status()
                .expect("Failed to build git ref");
            if !build_status.success() {
                cleanup_all_worktrees(&worktrees);
                std::process::exit(1);
            }
        }
    }

    let base_port: u16 = 8001;
    let mut server_pids: Vec<Child> = Vec::new();
    let mut battlesnake_args: Vec<String> = Vec::new();
    let mut snake_names: Vec<String> = Vec::new();

    if log {
        let _ = fs::remove_dir_all("game_logs");
    }
    fs::create_dir_all("game_logs").unwrap();
    for (idx, snake) in snakes.iter().enumerate() {
        let port = base_port + idx as u16;
        let name = snake.display_name(idx);

        kill_port(port);

        let binary = snake.binary_path();
        let child = if idx == 0 {
            let log_file =
                fs::File::create("game_logs/.server.log").expect("Cannot create log file");
            Command::new(&binary)
                .env("PORT", port.to_string())
                .env("VARIANT", &snake.variant)
                .env("LOG_BOARD", "1")
                .env("LOG_EVAL", if log { "1" } else { "" })
                .stdout(log_file.try_clone().unwrap())
                .stderr(log_file)
                .spawn()
                .expect("Failed to start server")
        } else {
            Command::new(&binary)
                .env("PORT", port.to_string())
                .env("VARIANT", &snake.variant)
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
            if idx == 0 { " (logging)" } else { "" }
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

    std::thread::sleep(std::time::Duration::from_secs(1));

    let mut wins: HashMap<String, usize> = HashMap::new();
    let mut wins_as_longer: HashMap<String, usize> = HashMap::new();
    let mut wins_as_same: HashMap<String, usize> = HashMap::new();
    let mut wins_as_shorter: HashMap<String, usize> = HashMap::new();
    for name in &snake_names {
        wins.insert(name.clone(), 0);
        wins_as_longer.insert(name.clone(), 0);
        wins_as_same.insert(name.clone(), 0);
        wins_as_shorter.insert(name.clone(), 0);
    }
    let mut draws: usize = 0;
    let mut total: usize = 0;
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

        let mut winner_name: Option<String> = None;
        let mut turns = String::from("?");
        let mut seed = String::from("?");

        for line in combined.lines() {
            if let Some(pos) = line.find(" was the winner") {
                let before = &line[..pos];
                if let Some(w) = before.split_whitespace().last() {
                    winner_name = Some(w.to_string());
                }
                if let Some(after_pos) = line.find("after ") {
                    let rest = &line[after_pos + 6..];
                    if let Some(sp) = rest.find(' ') {
                        turns = rest[..sp].to_string();
                    }
                }
            } else if line.contains("Game completed") && !line.contains("was the winner") {
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

        // Read new log content for length analysis and optional file saving
        let new_log_content = {
            let mut content = String::new();
            if let Ok(mut file) = fs::File::open("game_logs/.server.log") {
                file.seek(SeekFrom::Start(log_position)).ok();
                file.read_to_string(&mut content).ok();
            }
            content
        };
        log_position += new_log_content.len() as u64;

        if let Some(winner) = &winner_name {
            if let Some((wl, mo)) = parse_last_decisive_lengths(&new_log_content, winner) {
                if wl > mo {
                    *wins_as_longer.entry(winner.clone()).or_default() += 1;
                } else if wl == mo {
                    *wins_as_same.entry(winner.clone()).or_default() += 1;
                } else {
                    *wins_as_shorter.entry(winner.clone()).or_default() += 1;
                }
            }
        }

        if log && !new_log_content.is_empty() {
            let first_snake_won = winner_name.as_ref() == Some(&snake_names[0]);
            let suffix = if first_snake_won { "" } else { "_lost" };
            let game_log_path = format!("game_logs/game_{}{}.log", total, suffix);
            fs::write(&game_log_path, &new_log_content).unwrap();
            eprintln!("  Game log: {}", game_log_path);
        }

        eprintln!();
        eprintln!(
            "  {:<34} {:>6}  {:>6}  {:>14}  {:>12}  {:>14}",
            "Snake", "Win%", "Wins", "Wins as Longer", "Wins as Same", "Wins as Shorter"
        );
        eprintln!(
            "  {:<34} {:>6}  {:>6}  {:>14}  {:>12}  {:>14}",
            "----------------------------------", "------", "------", "--------------", "------------", "--------------"
        );
        for name in &snake_names {
            let w = wins[name];
            let pct = if total > 0 {
                w as f64 * 100.0 / total as f64
            } else {
                0.0
            };
            let wl = wins_as_longer[name];
            let ws = wins_as_same[name];
            let wsh = wins_as_shorter[name];
            eprintln!(
                "  {:<34} {:>5.1}%  {:>6}  {:>14}  {:>12}  {:>14}",
                name, pct, w, wl, ws, wsh
            );
        }
        if draws > 0 {
            let pct = draws as f64 * 100.0 / total as f64;
            eprintln!(
                "  {:<34} {:>5.1}%",
                "draws", pct
            );
        }
        eprintln!("  Games played: {}", total);
        if n_games > 0 {
            eprintln!("  Target: {}", n_games);
        }

        if n_games > 0 && total >= n_games {
            break;
        }
    }

    eprintln!("\nStopping servers...");
    for mut child in server_pids {
        let _ = child.kill();
        let _ = child.wait();
    }
    for (idx, _) in snakes.iter().enumerate() {
        kill_port(base_port + idx as u16);
    }
    let _ = fs::remove_file("game_logs/.server.log");

    cleanup_all_worktrees(&worktrees);
    eprintln!("Done.");
}

fn cleanup_all_worktrees(worktrees: &[String]) {
    for wt in worktrees {
        eprintln!("Removing worktree {}...", wt);
        let status = Command::new("git")
            .args(["worktree", "remove", "--force", wt])
            .status();
        if status.map(|s| !s.success()).unwrap_or(true) {
            eprintln!(
                "Warning: could not remove worktree. Clean up manually:\n  git worktree remove --force {}",
                wt
            );
        }
    }
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
