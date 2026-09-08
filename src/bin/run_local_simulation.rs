use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use serde::{Deserialize, Serialize};
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

// ── Per-snake env config read from -c <file> ──────────────────────────────

#[derive(Default, Deserialize)]
struct SnakeConfig {
    #[serde(default)]
    env: HashMap<String, String>,
}

#[derive(Default, Deserialize)]
struct SimulationConfig {
    #[serde(default)]
    snakes: Vec<SnakeConfig>,
}

// ── JSON results written after each game, read for final summary ──────────

#[derive(Serialize, Deserialize, Clone)]
struct SnakeGameRecord {
    avg_head_x: f64,
    avg_head_y: f64,
    avg_dist: f64,
    avg_health: f64,
    final_length: i64,
    fate: String,
    death_condition: Option<String>,
    win_length_relation: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct GameRecord {
    game_num: usize,
    winner: Option<String>,
    turns: usize,
    seed: String,
    per_snake: HashMap<String, SnakeGameRecord>,
    depth: Option<f64>,
    nodes: Option<f64>,
    depth_ctrl: Option<f64>,
    nodes_ctrl: Option<f64>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SimulationResults {
    pub snake_names: Vec<String>,
    pub games: Vec<GameRecord>,
}

impl SimulationResults {
    pub fn win_rate(&self, snake_name: &str) -> f64 {
        let total = self.games.len();
        if total == 0 { return 0.0; }
        let wins = self.games.iter().filter(|g| g.winner.as_deref() == Some(snake_name)).count();
        wins as f64 * 100.0 / total as f64
    }
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

struct DepthStats {
    avg_depth: f64,
    avg_nodes: f64,
}

fn parse_depth_stats(log_content: &str) -> Option<DepthStats> {
    let marker = "DEPTHS ";
    let mut total_depth = 0.0f64;
    let mut total_nodes = 0.0f64;
    let mut depth_count = 0usize;
    let mut nodes_count = 0usize;

    for line in log_content.lines() {
        if let Some(pos) = line.find(marker) {
            let json_str = line[pos + marker.len()..].trim();
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(json_str) {
                if let Some(obj) = val.as_object() {
                    for (_, dir_val) in obj {
                        if let Some(d) = dir_val["depth"].as_f64() {
                            total_depth += d;
                            depth_count += 1;
                        }
                        if let Some(n) = dir_val["nodes"].as_f64() {
                            total_nodes += n;
                            nodes_count += 1;
                        }
                    }
                }
            }
        }
    }

    if depth_count == 0 && nodes_count == 0 {
        None
    } else {
        Some(DepthStats {
            avg_depth: if depth_count > 0 { total_depth / depth_count as f64 } else { 0.0 },
            avg_nodes: if nodes_count > 0 { total_nodes / nodes_count as f64 } else { 0.0 },
        })
    }
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

fn write_results_json(results: &SimulationResults, path: &str) {
    match serde_json::to_string_pretty(results) {
        Ok(json) => { let _ = fs::write(path, json); }
        Err(e) => eprintln!("Warning: could not serialize results: {}", e),
    }
}

fn print_final_summary_from_json(results: &SimulationResults) {
    let total = results.games.len();
    if total == 0 { return; }

    let snake_names = &results.snake_names;
    let sep = "=".repeat(66);
    eprintln!("\n{}", sep);
    eprintln!("  FINAL SUMMARY  ({} games)", total);
    eprintln!("{}\n", sep);

    let mut wins: HashMap<String, usize> = snake_names.iter().map(|n| (n.clone(), 0)).collect();
    let mut wins_as_longer: HashMap<String, usize> = wins.clone();
    let mut wins_as_same: HashMap<String, usize> = wins.clone();
    let mut wins_as_shorter: HashMap<String, usize> = wins.clone();
    let mut draws = 0usize;

    for game in &results.games {
        match &game.winner {
            Some(w) => {
                *wins.entry(w.clone()).or_default() += 1;
                if let Some(sr) = game.per_snake.get(w) {
                    match sr.win_length_relation.as_deref() {
                        Some("longer")  => *wins_as_longer.entry(w.clone()).or_default() += 1,
                        Some("same")    => *wins_as_same.entry(w.clone()).or_default() += 1,
                        Some("shorter") => *wins_as_shorter.entry(w.clone()).or_default() += 1,
                        _ => {}
                    }
                }
            }
            None => draws += 1,
        }
    }

    {
        let mut builder = TableBuilder::default();
        builder.push_record(["Snake", "Win%", "Wins", "Longer", "Same", "Shorter"]);
        for name in snake_names {
            let w = wins.get(name).copied().unwrap_or(0);
            let pct = w as f64 * 100.0 / total as f64;
            builder.push_record([
                name.clone(),
                format!("{:.1}%", pct),
                w.to_string(),
                wins_as_longer.get(name).copied().unwrap_or(0).to_string(),
                wins_as_same.get(name).copied().unwrap_or(0).to_string(),
                wins_as_shorter.get(name).copied().unwrap_or(0).to_string(),
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

    let round1 = |v: f64| (v * 10.0).round() / 10.0;
    let mut per_snake: HashMap<String, (usize, f64, f64, f64, f64, f64)> = HashMap::new();
    let mut depth_sum = 0.0f64;
    let mut depth_count = 0usize;
    let mut nodes_sum = 0.0f64;
    let mut nodes_count = 0usize;
    let mut depth_ctrl_sum = 0.0f64;
    let mut depth_ctrl_count = 0usize;
    let mut nodes_ctrl_sum = 0.0f64;
    let mut nodes_ctrl_count = 0usize;

    for game in &results.games {
        for (name, sr) in &game.per_snake {
            let e = per_snake.entry(name.clone()).or_default();
            e.0 += 1; e.1 += sr.avg_head_x; e.2 += sr.avg_head_y;
            e.3 += sr.avg_dist; e.4 += sr.avg_health; e.5 += sr.final_length as f64;
        }
        if let Some(d) = game.depth { depth_sum += d; depth_count += 1; }
        if let Some(n) = game.nodes { nodes_sum += n; nodes_count += 1; }
        if let Some(d) = game.depth_ctrl { depth_ctrl_sum += d; depth_ctrl_count += 1; }
        if let Some(n) = game.nodes_ctrl { nodes_ctrl_sum += n; nodes_ctrl_count += 1; }
    }

    let overall_avg_depth = if depth_count > 0 { Some(depth_sum / depth_count as f64) } else { None };
    let overall_avg_nodes = if nodes_count > 0 { Some(nodes_sum / nodes_count as f64) } else { None };
    let overall_avg_depth_ctrl = if depth_ctrl_count > 0 { Some(depth_ctrl_sum / depth_ctrl_count as f64) } else { None };
    let overall_avg_nodes_ctrl = if nodes_ctrl_count > 0 { Some(nodes_ctrl_sum / nodes_ctrl_count as f64) } else { None };

    {
        let mut builder = TableBuilder::default();
        builder.push_record(["Snake", "Avg Pos", "Avg Dist", "Avg Health", "Avg Fin Len", "Avg Depth", "Avg Nodes"]);
        for name in snake_names {
            if let Some(e) = per_snake.get(name) {
                let n = e.0 as f64;
                let depth_s = if name == &snake_names[0] {
                    overall_avg_depth.map_or("-".to_string(), |d| format!("{:.1}", d))
                } else if snake_names.len() > 1 && name == &snake_names[1] {
                    overall_avg_depth_ctrl.map_or("-".to_string(), |d| format!("{:.1}", d))
                } else { "-".to_string() };
                let nodes_s = if name == &snake_names[0] {
                    overall_avg_nodes.map_or("-".to_string(), |n| format!("{:.0}", n))
                } else if snake_names.len() > 1 && name == &snake_names[1] {
                    overall_avg_nodes_ctrl.map_or("-".to_string(), |n| format!("{:.0}", n))
                } else { "-".to_string() };
                builder.push_record([
                    name.clone(),
                    format!("({:.1}, {:.1})", e.1/n, e.2/n),
                    format!("{:.1}", round1(e.3/n)),
                    format!("{:.0}", e.4/n),
                    format!("{:.1}", round1(e.5/n)),
                    depth_s, nodes_s,
                ]);
            }
        }
        let mut table = builder.build();
        table.with(TableStyle::ascii());
        eprintln!("  Average game stats  (across all games)");
        for line in table.to_string().lines() { eprintln!("  {}", line); }
        eprintln!();
    }

    let snake0 = &snake_names[0];
    let mut won  = (0usize, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let mut lost = (0usize, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let mut wvl  = (0usize, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let mut len_sum_won = 0usize;
    let mut len_sum_lost = 0usize;

    for game in &results.games {
        if let Some(sr) = game.per_snake.get(snake0) {
            let is_winner = game.winner.as_deref() == Some(snake0.as_str());
            let acc = if is_winner { &mut won } else { &mut lost };
            acc.0 += 1; acc.1 += sr.avg_head_x; acc.2 += sr.avg_head_y;
            acc.3 += sr.avg_dist; acc.4 += sr.avg_health; acc.5 += sr.final_length as f64;
            if is_winner {
                len_sum_won += game.turns;
            } else {
                let elim = if let Some(t) = sr.fate.strip_prefix("eliminated_") {
                    t.parse::<usize>().unwrap_or(game.turns)
                } else { game.turns };
                len_sum_lost += elim;
            }
            if !is_winner {
                if let Some(w_name) = &game.winner {
                    if let Some(ws) = game.per_snake.get(w_name) {
                        wvl.0 += 1; wvl.1 += ws.avg_head_x; wvl.2 += ws.avg_head_y;
                        wvl.3 += ws.avg_dist; wvl.4 += ws.avg_health; wvl.5 += ws.final_length as f64;
                    }
                }
            }
        }
    }

    {
        let mut builder = TableBuilder::default();
        builder.push_record(["Outcome", "Avg Pos", "Avg Dist", "Avg Health", "Avg Fin Len", "Avg Duration"]);
        for (label, acc, len_sum) in [
            (format!("Won ({})", won.0), &won, len_sum_won),
            (format!("Lost/Draw ({})", lost.0), &lost, len_sum_lost),
            (format!("Winner vs us ({})", wvl.0), &wvl, 0usize),
        ] {
            if acc.0 > 0 {
                let n = acc.0 as f64;
                let avg_len = if len_sum > 0 {
                    format!("{:.1}", len_sum as f64 / acc.0 as f64)
                } else { "-".to_string() };
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
    }

    let mut death_counts: HashMap<String, usize> = HashMap::new();
    let mut total_deaths = 0usize;
    for game in &results.games {
        if let Some(sr) = game.per_snake.get(snake0) {
            if let Some(dc) = &sr.death_condition {
                *death_counts.entry(dc.clone()).or_default() += 1;
                total_deaths += 1;
            }
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

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut n_games: usize = 0;
    let mut watch = false;
    let mut log = false;
    let mut raw_snakes: Vec<String> = Vec::new();
    let mut sim_config_path: Option<String> = None;
    let mut results_path = "simulation_results.json".to_string();
    let mut no_build = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-n" => {
                i += 1;
                n_games = args[i].parse().expect("Invalid number for -n");
            }
            "-w" => watch = true,
            "-l" => log = true,
            "-c" => {
                i += 1;
                sim_config_path = Some(args[i].clone());
            }
            "-o" => {
                i += 1;
                results_path = args[i].clone();
            }
            "--no-build" => no_build = true,
            arg if arg.starts_with('-') && arg[1..].chars().all(|c| c.is_ascii_digit()) => {
                n_games = arg[1..].parse().unwrap();
            }
            other => raw_snakes.push(other.to_string()),
        }
        i += 1;
    }

    if raw_snakes.len() < 2 {
        eprintln!("Usage: run_local_simulation [-n NUM] [-w] [-l] [-c config.json] [-o results.json] [--no-build] snake1 snake2 [snake3 snake4]");
        eprintln!("Variants: depth_first breadth_first simple_tree_search simple_hungry single_gamestate_nodes");
        eprintln!("Append :<git-ref> to build that snake from a specific tag/commit:");
        eprintln!("  single_gamestate_nodes:v2025-06-01  depth_first:abc1234");
        std::process::exit(1);
    }

    let sim_config: SimulationConfig = sim_config_path
        .as_deref()
        .and_then(|p| fs::read_to_string(p).ok())
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();

    let snakes: Vec<SnakeSpec> = raw_snakes.iter().map(|s| SnakeSpec::parse(s)).collect();

    // Build current version (covers all snakes without a git_ref)
    let needs_current = snakes.iter().any(|s| s.git_ref.is_none());
    if needs_current && !no_build {
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
            let mut cmd = Command::new(&binary);
            cmd.env("PORT", port.to_string())
                .env("VARIANT", &snake.variant)
                .env("LOG_BOARD", "1")
                .env("LOCAL_SIMULATION", "1");
            if let Some(sc) = sim_config.snakes.get(idx) {
                cmd.envs(&sc.env);
            }
            cmd.stdout(log_file.try_clone().unwrap())
                .stderr(log_file)
                .spawn()
                .expect("Failed to start server")
        } else if idx == 1 {
            let log_file =
                fs::File::create("game_logs/.control.log").expect("Cannot create control log file");
            let mut cmd = Command::new(&binary);
            cmd.env("PORT", port.to_string())
                .env("VARIANT", &snake.variant)
                .env("LOCAL_SIMULATION", "1");
            if let Some(sc) = sim_config.snakes.get(idx) {
                cmd.envs(&sc.env);
            }
            cmd.stdout(log_file.try_clone().unwrap())
                .stderr(log_file)
                .spawn()
                .expect("Failed to start server")
        } else {
            let mut cmd = Command::new(&binary);
            cmd.env("PORT", port.to_string())
                .env("VARIANT", &snake.variant);
            if let Some(sc) = sim_config.snakes.get(idx) {
                cmd.envs(&sc.env);
            }
            cmd.stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("Failed to start server")
        };

        eprintln!(
            "Started {} on :{} (PID {}){}",
            name,
            port,
            child.id(),
            if idx == 0 { " (logging)" } else if idx == 1 { " (logging control)" } else { "" }
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
    let mut control_log_position: u64 = 0;
    let mut game_stats_log: Vec<HashMap<String, GameStats>> = Vec::new();
    let mut results = SimulationResults { snake_names: snake_names.clone(), games: Vec::new() };

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

        let new_control_log_content = {
            let mut content = String::new();
            if let Ok(mut file) = fs::File::open("game_logs/.control.log") {
                file.seek(SeekFrom::Start(control_log_position)).ok();
                file.read_to_string(&mut content).ok();
            }
            content
        };
        control_log_position += new_control_log_content.len() as u64;

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
        let all_game_stats = parse_all_game_stats(&new_log_content, actual_turns, &snake_names[0]);
        let depth_stats = parse_depth_stats(&new_log_content);
        let control_depth_stats = if snake_names.len() > 1 {
            parse_depth_stats(&new_control_log_content)
        } else {
            None
        };

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
            builder.push_record(["Snake", "Avg Pos", "Avg Dist", "Avg Health", "Fin Len", "Avg Depth", "Avg Nodes", "Fate", "Death"]);
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
                    depth_stats.as_ref().map_or("-".to_string(), |ds| format!("{:.1}", ds.avg_depth))
                } else if snake_names.len() > 1 && *name == snake_names[1] {
                    control_depth_stats.as_ref().map_or("-".to_string(), |ds| format!("{:.1}", ds.avg_depth))
                } else {
                    "-".to_string()
                };
                let nodes = if *name == snake_names[0] {
                    depth_stats.as_ref().map_or("-".to_string(), |ds| format!("{:.0}", ds.avg_nodes))
                } else if snake_names.len() > 1 && *name == snake_names[1] {
                    control_depth_stats.as_ref().map_or("-".to_string(), |ds| format!("{:.0}", ds.avg_nodes))
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
                    nodes,
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

        // Build and persist GameRecord to JSON
        {
            let win_rel: Option<String> = if let Some(winner) = &winner_name {
                parse_last_decisive_lengths(&new_log_content, winner).map(|(wl, mo)| {
                    if wl > mo { "longer".to_string() }
                    else if wl == mo { "same".to_string() }
                    else { "shorter".to_string() }
                })
            } else { None };
            let gs = game_stats_log.last().unwrap();
            let mut per_snake: HashMap<String, SnakeGameRecord> = HashMap::new();
            for (name, stats) in gs {
                let fate_str = match &stats.fate {
                    SnakeFate::Survived => "survived".to_string(),
                    SnakeFate::Eliminated(t) => format!("eliminated_{}", t),
                    SnakeFate::UnknownAfter(t) => format!("unknown_after_{}", t),
                };
                let dc_str = match &stats.death_condition {
                    None => None,
                    Some(DeathCondition::NextToWall) => Some("wall".to_string()),
                    Some(DeathCondition::Unknown) => Some("unknown".to_string()),
                };
                let is_winner = winner_name.as_deref() == Some(name.as_str());
                per_snake.insert(name.clone(), SnakeGameRecord {
                    avg_head_x: stats.avg_head_x,
                    avg_head_y: stats.avg_head_y,
                    avg_dist: stats.avg_dist_from_center,
                    avg_health: stats.avg_health,
                    final_length: stats.final_length,
                    fate: fate_str,
                    death_condition: dc_str,
                    win_length_relation: if is_winner { win_rel.clone() } else { None },
                });
            }
            results.games.push(GameRecord {
                game_num: total,
                winner: winner_name.clone(),
                turns: actual_turns.unwrap_or(0),
                seed: seed.clone(),
                per_snake,
                depth: depth_stats.as_ref().map(|ds| ds.avg_depth),
                nodes: depth_stats.as_ref().map(|ds| ds.avg_nodes),
                depth_ctrl: control_depth_stats.as_ref().map(|ds| ds.avg_depth),
                nodes_ctrl: control_depth_stats.as_ref().map(|ds| ds.avg_nodes),
            });
            write_results_json(&results, &results_path);
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
    let _ = fs::remove_file("game_logs/.control.log");

    cleanup_all_worktrees(&worktrees);

    // ── FINAL SUMMARY ─────────────────────────────────────────────────────────
    if let Ok(json) = fs::read_to_string(&results_path) {
        if let Ok(r) = serde_json::from_str::<SimulationResults>(&json) {
            print_final_summary_from_json(&r);
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
