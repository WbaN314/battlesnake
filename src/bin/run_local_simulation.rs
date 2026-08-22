use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use tabled::{builder::Builder as TableBuilder, settings::Style as TableStyle};

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

enum SnakeFate {
    Eliminated(usize),   // confirmed: disappeared from board at this turn
    Survived,            // log covers full game, alive to end
    UnknownAfter(usize), // alive when snake[0]'s log ended, game continued without us
}

enum DeathCondition {
    Unknown,
    NextToWall,
}

struct GameStats {
    avg_head_x: f64,
    avg_head_y: f64,
    avg_dist_from_center: f64,
    avg_health: f64,
    final_length: i64,
    fate: SnakeFate,
    death_condition: Option<DeathCondition>,
}

fn parse_all_game_stats(log_content: &str, actual_turns: Option<usize>, logging_snake: &str) -> HashMap<String, GameStats> {
    let arrow = " Request -> ";
    let mut samples: HashMap<String, usize> = HashMap::new();
    let mut sum_x: HashMap<String, f64> = HashMap::new();
    let mut sum_y: HashMap<String, f64> = HashMap::new();
    let mut sum_dist: HashMap<String, f64> = HashMap::new();
    let mut sum_health: HashMap<String, f64> = HashMap::new();
    let mut last_length: HashMap<String, i64> = HashMap::new();
    let mut last_turn_seen: HashMap<String, usize> = HashMap::new();
    let mut last_head_x: HashMap<String, f64> = HashMap::new();
    let mut last_head_y: HashMap<String, f64> = HashMap::new();
    let mut game_last_turn = 0usize;

    for line in log_content.lines() {
        if let Some(pos) = line.find(arrow) {
            let json_str = &line[pos + arrow.len()..];
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(json_str) {
                let turn = val["turn"].as_u64().unwrap_or(0) as usize;
                if turn > game_last_turn {
                    game_last_turn = turn;
                }
                if let Some(snakes) = val["board"]["snakes"].as_array() {
                    for snake in snakes {
                        let name = match snake["name"].as_str() {
                            Some(n) => n.to_string(),
                            None => continue,
                        };
                        let x = snake["head"]["x"].as_f64().unwrap_or(0.0);
                        let y = snake["head"]["y"].as_f64().unwrap_or(0.0);
                        let length = snake["length"].as_i64().unwrap_or(0);
                        let health = snake["health"].as_f64().unwrap_or(0.0);
                        let dx = x - 5.0;
                        let dy = y - 5.0;
                        *samples.entry(name.clone()).or_default() += 1;
                        *sum_x.entry(name.clone()).or_default() += x;
                        *sum_y.entry(name.clone()).or_default() += y;
                        *sum_dist.entry(name.clone()).or_default() += (dx * dx + dy * dy).sqrt();
                        *sum_health.entry(name.clone()).or_default() += health;
                        last_length.insert(name.clone(), length);
                        last_head_x.insert(name.clone(), x);
                        last_head_y.insert(name.clone(), y);
                        last_turn_seen.insert(name, turn);
                    }
                }
            }
        }
    }

    samples
        .iter()
        .map(|(name, &n)| {
            let nf = n as f64;
            let last_seen = last_turn_seen.get(name).copied().unwrap_or(0);
            let fate = if last_seen < game_last_turn {
                SnakeFate::Eliminated(last_seen + 1)
            } else {
                match actual_turns.map(|t| t.saturating_sub(1)) {
                    Some(actual_last) if game_last_turn >= actual_last => SnakeFate::Survived,
                    _ if name == logging_snake => SnakeFate::Eliminated(game_last_turn + 1),
                    _ => SnakeFate::UnknownAfter(game_last_turn),
                }
            };
            let lx = last_head_x.get(name).copied().unwrap_or(5.0);
            let ly = last_head_y.get(name).copied().unwrap_or(5.0);
            let death_condition = match &fate {
                SnakeFate::Survived => None,
                _ => Some(if lx <= 0.0 || lx >= 10.0 || ly <= 0.0 || ly >= 10.0 {
                    DeathCondition::NextToWall
                } else {
                    DeathCondition::Unknown
                }),
            };
            (
                name.clone(),
                GameStats {
                    avg_head_x: sum_x[name] / nf,
                    avg_head_y: sum_y[name] / nf,
                    avg_dist_from_center: sum_dist[name] / nf,
                    avg_health: sum_health[name] / nf,
                    final_length: *last_length.get(name).unwrap_or(&0),
                    fate,
                    death_condition,
                },
            )
        })
        .collect()
}

fn render_last_board(log_content: &str) -> Option<&str> {
    let board_marker = " Board\n";
    let last_pos = log_content.rfind(board_marker)?;
    let after = &log_content[last_pos + board_marker.len()..];
    // cut at the next log entry (line starting with '[')
    let end = after.find("\n[").map(|p| p + 1).unwrap_or(after.len());
    Some(after[..end].trim_end())
}

fn parse_avg_depths(log_content: &str) -> Option<f64> {
    let marker = "DEPTHS ";
    let mut total = 0.0f64;
    let mut count = 0usize;

    for line in log_content.lines() {
        if let Some(pos) = line.find(marker) {
            for part in line[pos + marker.len()..].split_whitespace() {
                if let Some((_, v)) = part.split_once('=') {
                    if let Ok(val) = v.parse::<f64>() {
                        total += val;
                        count += 1;
                    }
                }
            }
        }
    }

    if count == 0 { None } else { Some(total / count as f64) }
}

fn parse_end_turn(log_content: &str) -> Option<usize> {
    let arrow = " End -> ";
    for line in log_content.lines().rev() {
        if let Some(pos) = line.find(arrow) {
            let before = &line[..pos];
            if let Some(turn_pos) = before.rfind(" Turn ") {
                if let Ok(t) = before[turn_pos + 6..].trim().parse::<usize>() {
                    return Some(t);
                }
            }
        }
    }
    None
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
                .env("LOCAL_SIMULATION", "1")
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
    let mut game_stats_log: Vec<HashMap<String, GameStats>> = Vec::new();
    let mut game_winners: Vec<Option<String>> = Vec::new();
    let mut game_lengths: Vec<usize> = Vec::new();
    let mut game_depths: Vec<f64> = Vec::new();

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

        // Read new log content before printing summary so we can use End -> turn
        let new_log_content = {
            let mut content = String::new();
            if let Ok(mut file) = fs::File::open("game_logs/.server.log") {
                file.seek(SeekFrom::Start(log_position)).ok();
                file.read_to_string(&mut content).ok();
            }
            content
        };
        log_position += new_log_content.len() as u64;

        if let Some(t) = parse_end_turn(&new_log_content) {
            turns = t.to_string();
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
        game_winners.push(winner_name.clone());

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

        let actual_turns: Option<usize> = turns.parse().ok();
        game_lengths.push(actual_turns.unwrap_or(0));
        let all_game_stats = parse_all_game_stats(&new_log_content, actual_turns, &snake_names[0]);
        let avg_depths = parse_avg_depths(&new_log_content);

        if let Some(board) = render_last_board(&new_log_content) {
            for line in board.lines() {
                eprintln!("{}", line);
            }
        }

        if !all_game_stats.is_empty() {
            let mut ordered_names: Vec<&str> = snake_names
                .iter()
                .filter(|n| all_game_stats.contains_key(*n))
                .map(|n| n.as_str())
                .collect();
            for name in all_game_stats.keys() {
                if !ordered_names.contains(&name.as_str()) {
                    ordered_names.push(name.as_str());
                }
            }
            let mut builder = TableBuilder::default();
            builder.push_record(["Snake", "Avg Pos", "Avg Dist", "Avg Health", "Fin Len", "Avg Depth", "Fate", "Death"]);
            for name in &ordered_names {
                let s = &all_game_stats[*name];
                let fate = match s.fate {
                    SnakeFate::Eliminated(t) => format!("elim t{}", t),
                    SnakeFate::Survived => "survived".to_string(),
                    SnakeFate::UnknownAfter(t) => format!("t{}+", t),
                };
                let death = match &s.death_condition {
                    None => "-".to_string(),
                    Some(DeathCondition::NextToWall) => "wall".to_string(),
                    Some(DeathCondition::Unknown) => "unknown".to_string(),
                };
                let depth = if *name == snake_names[0] {
                    avg_depths.map_or("-".to_string(), |d| format!("{:.1}", d))
                } else {
                    "-".to_string()
                };
                builder.push_record([
                    name.to_string(),
                    format!("({:.1}, {:.1})", s.avg_head_x, s.avg_head_y),
                    format!("{:.1}", s.avg_dist_from_center),
                    format!("{:.0}", s.avg_health),
                    s.final_length.to_string(),
                    depth,
                    fate,
                    death,
                ]);
            }
            let mut table = builder.build();
            table.with(TableStyle::ascii());
            for line in table.to_string().lines() {
                eprintln!("  {}", line);
            }
        }
        game_stats_log.push(all_game_stats);

        if let Some(depth) = avg_depths {
            game_depths.push(depth);
        }

        eprintln!();
        {
            let mut builder = TableBuilder::default();
            builder.push_record(["Snake", "Win%", "Wins", "Longer", "Same", "Shorter"]);
            for name in &snake_names {
                let w = wins[name];
                let pct = if total > 0 { w as f64 * 100.0 / total as f64 } else { 0.0 };
                builder.push_record([
                    name.clone(),
                    format!("{:.1}%", pct),
                    w.to_string(),
                    wins_as_longer[name].to_string(),
                    wins_as_same[name].to_string(),
                    wins_as_shorter[name].to_string(),
                ]);
            }
            if draws > 0 {
                builder.push_record([
                    "(draws)".to_string(),
                    format!("{:.1}%", draws as f64 * 100.0 / total as f64),
                    draws.to_string(), "-".to_string(), "-".to_string(), "-".to_string(),
                ]);
            }
            let mut table = builder.build();
            table.with(TableStyle::ascii());
            for line in table.to_string().lines() { eprintln!("  {}", line); }
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

    // ── FINAL SUMMARY ─────────────────────────────────────────────────────────
    if total > 0 {
        let sep = "=".repeat(66);
        eprintln!("\n{}", sep);
        eprintln!("  FINAL SUMMARY  ({} games)", total);
        eprintln!("{}\n", sep);

        // Results table
        {
            let mut builder = TableBuilder::default();
            builder.push_record(["Snake", "Win%", "Wins", "Longer", "Same", "Shorter"]);
            for name in &snake_names {
                let w = wins[name];
                let pct = w as f64 * 100.0 / total as f64;
                builder.push_record([
                    name.clone(),
                    format!("{:.1}%", pct),
                    w.to_string(),
                    wins_as_longer[name].to_string(),
                    wins_as_same[name].to_string(),
                    wins_as_shorter[name].to_string(),
                ]);
            }
            if draws > 0 {
                builder.push_record([
                    "(draws)".to_string(),
                    format!("{:.1}%", draws as f64 * 100.0 / total as f64),
                    draws.to_string(), "-".to_string(), "-".to_string(), "-".to_string(),
                ]);
            }
            let mut table = builder.build();
            table.with(TableStyle::ascii());
            eprintln!("  Results");
            for line in table.to_string().lines() { eprintln!("  {}", line); }
            eprintln!();
        }

        // Aggregate game stats per snake
        let non_empty: Vec<&HashMap<String, GameStats>> =
            game_stats_log.iter().filter(|m| !m.is_empty()).collect();
        if !non_empty.is_empty() {
            let round1 = |v: f64| (v * 10.0).round() / 10.0;
            let mut per_snake: HashMap<String, (usize, f64, f64, f64, f64, f64)> = HashMap::new();
            for gs in &non_empty {
                for (name, s) in *gs {
                    let e = per_snake.entry(name.clone()).or_default();
                    e.0 += 1; e.1 += s.avg_head_x; e.2 += s.avg_head_y;
                    e.3 += s.avg_dist_from_center; e.4 += s.avg_health;
                    e.5 += s.final_length as f64;
                }
            }
            let overall_avg_depth = if game_depths.is_empty() {
                None
            } else {
                Some(game_depths.iter().sum::<f64>() / game_depths.len() as f64)
            };
            let mut builder = TableBuilder::default();
            builder.push_record(["Snake", "Avg Pos", "Avg Dist", "Avg Health", "Avg Fin Len", "Avg Depth"]);
            for name in &snake_names {
                if let Some(e) = per_snake.get(name) {
                    let n = e.0 as f64;
                    let depth = if name == &snake_names[0] {
                        overall_avg_depth.map_or("-".to_string(), |d| format!("{:.1}", d))
                    } else {
                        "-".to_string()
                    };
                    builder.push_record([
                        name.clone(),
                        format!("({:.1}, {:.1})", e.1/n, e.2/n),
                        format!("{:.1}", round1(e.3/n)),
                        format!("{:.0}", e.4/n),
                        format!("{:.1}", round1(e.5/n)),
                        depth,
                    ]);
                }
            }
            let mut table = builder.build();
            table.with(TableStyle::ascii());
            eprintln!("  Average game stats  (across all games)");
            for line in table.to_string().lines() { eprintln!("  {}", line); }
            eprintln!();

            // Outcome analysis for snake[0]: won vs lost
            let snake0 = &snake_names[0];
            let mut won = (0usize, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
            let mut lost = (0usize, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
            let mut winner_when_lost = (0usize, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
            let mut len_sum_won = 0usize;
            let mut len_sum_lost = 0usize;
            for (i, gs) in game_stats_log.iter().enumerate() {
                if let Some(s) = gs.get(snake0) {
                    let game_len = game_lengths.get(i).copied().unwrap_or(0);
                    let winner = game_winners.get(i).and_then(|w| w.as_deref());
                    let acc = if winner == Some(snake0.as_str()) {
                        &mut won
                    } else {
                        &mut lost
                    };
                    acc.0 += 1; acc.1 += s.avg_head_x; acc.2 += s.avg_head_y;
                    acc.3 += s.avg_dist_from_center; acc.4 += s.avg_health;
                    acc.5 += s.final_length as f64;
                    if winner == Some(snake0.as_str()) {
                        len_sum_won += game_len;
                    } else {
                        len_sum_lost += match s.fate {
                            SnakeFate::Eliminated(t) => t,
                            SnakeFate::Survived => game_len,
                            SnakeFate::UnknownAfter(t) => t,
                        };
                    }
                    // accumulate the winning opponent's stats when we lost
                    if winner != Some(snake0.as_str()) {
                        if let Some(w_name) = winner {
                            if let Some(ws) = gs.get(w_name) {
                                winner_when_lost.0 += 1;
                                winner_when_lost.1 += ws.avg_head_x;
                                winner_when_lost.2 += ws.avg_head_y;
                                winner_when_lost.3 += ws.avg_dist_from_center;
                                winner_when_lost.4 += ws.avg_health;
                                winner_when_lost.5 += ws.final_length as f64;
                            }
                        }
                    }
                }
            }
            let mut builder = TableBuilder::default();
            builder.push_record(["Outcome", "Avg Pos", "Avg Dist", "Avg Health", "Avg Fin Len", "Avg Duration"]);
            for (label, acc, len_sum) in [
                (format!("Won ({})", won.0), &won, len_sum_won),
                (format!("Lost/Draw ({})", lost.0), &lost, len_sum_lost),
                (format!("Winner vs us ({})", winner_when_lost.0), &winner_when_lost, 0usize),
            ] {
                if acc.0 > 0 {
                    let n = acc.0 as f64;
                    let avg_len = if len_sum > 0 {
                        format!("{:.1}", len_sum as f64 / acc.0 as f64)
                    } else {
                        "-".to_string()
                    };
                    builder.push_record([
                        label,
                        format!("({:.1}, {:.1})", acc.1/n, acc.2/n),
                        format!("{:.1}", round1(acc.3/n)),
                        format!("{:.0}", acc.4/n),
                        format!("{:.1}", round1(acc.5/n)),
                        avg_len,
                    ]);
                }
            }
            let mut table = builder.build();
            table.with(TableStyle::ascii());
            eprintln!("  Outcome analysis — {}  (center of board is 5.0, 5.0)", snake0);
            for line in table.to_string().lines() { eprintln!("  {}", line); }
            eprintln!();

            // Death condition breakdown for snake[0]
            let mut death_counts: HashMap<String, usize> = HashMap::new();
            let mut total_deaths = 0usize;
            for gs in &game_stats_log {
                if let Some(s) = gs.get(snake0) {
                    let label = match &s.death_condition {
                        None => continue,
                        Some(DeathCondition::NextToWall) => "wall",
                        Some(DeathCondition::Unknown) => "unknown",
                    };
                    *death_counts.entry(label.to_string()).or_default() += 1;
                    total_deaths += 1;
                }
            }
            if total_deaths > 0 {
                let mut builder = TableBuilder::default();
                builder.push_record(["Death condition", "Count", "%"]);
                for label in &["wall", "unknown"] {
                    let count = death_counts.get(*label).copied().unwrap_or(0);
                    if count > 0 {
                        builder.push_record([
                            label.to_string(),
                            count.to_string(),
                            format!("{:.1}%", count as f64 * 100.0 / total_deaths as f64),
                        ]);
                    }
                }
                let mut table = builder.build();
                table.with(TableStyle::ascii());
                eprintln!("  Death conditions — {}  ({} deaths total)", snake0, total_deaths);
                for line in table.to_string().lines() { eprintln!("  {}", line); }
                eprintln!();
            }
        }
    }
    // ── END FINAL SUMMARY ─────────────────────────────────────────────────────

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
