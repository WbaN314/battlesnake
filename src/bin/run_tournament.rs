use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Instant;
use rand::Rng;
use serde::{Deserialize, Serialize};

// ── Tunable parameter definitions ────────────────────────────────────────────

const PARAM_NAMES: &[&str] = &[
    "WALL_SCORE",
    "CENTER_SCORE",
    "ENEMY_MIDPOINT_SCORE",
    "FOOD_D1_SCORE",
    "FOOD_D2_SCORE",
    "FOOD_D3_SCORE",
    "SQUEEZED_SNAKES_SCORE",
    "ENEMY_PUSHED_SCORE",
    "NOT_ENOUGH_AREA_SCORE",
];
const PARAM_DEFAULTS: &[f64] = &[-20.0, 10.0, 10.0, 60.0, 40.0, 30.0, 100.0, 100.0, -10.0];

const MUTATION_SCALES: &[f64] = &[0.5, 0.7, 0.85, 1.15, 1.5, 2.0];

// ── Minimal types to read simulation_results.json ───────────────────────────

#[derive(Deserialize)]
struct SimResults {
    snake_names: Vec<String>,
    games: Vec<GameRec>,
}

#[derive(Deserialize)]
struct GameRec {
    winner: Option<String>,
}

impl SimResults {
    fn win_rate(&self, name: &str) -> f64 {
        let total = self.games.len();
        if total == 0 { return 0.0; }
        let wins = self.games.iter().filter(|g| g.winner.as_deref() == Some(name)).count();
        wins as f64 * 100.0 / total as f64
    }
}

// ── Best-params file written after each improvement ──────────────────────────

#[derive(Serialize, Deserialize, Clone)]
struct BestParams {
    params: HashMap<String, f64>,
    win_rate: f64,
    games_played: usize,
    round: usize,
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn default_params() -> HashMap<String, f64> {
    PARAM_NAMES.iter().zip(PARAM_DEFAULTS.iter())
        .map(|(&n, &v)| (n.to_string(), v))
        .collect()
}

fn mutate(base: &HashMap<String, f64>, rng: &mut impl Rng) -> HashMap<String, f64> {
    let mut params = base.clone();
    let n_changes = rng.gen_range(1..=3_usize).min(PARAM_NAMES.len());
    // Fisher-Yates partial shuffle to pick n_changes distinct indices
    let mut indices: Vec<usize> = (0..PARAM_NAMES.len()).collect();
    for i in 0..n_changes {
        let j = rng.gen_range(i..PARAM_NAMES.len());
        indices.swap(i, j);
    }
    for &idx in &indices[..n_changes] {
        let scale = MUTATION_SCALES[rng.gen_range(0..MUTATION_SCALES.len())];
        let name = PARAM_NAMES[idx];
        let default = PARAM_DEFAULTS[idx];
        let v = params.get(name).copied().unwrap_or(default);
        // preserve sign when the default is negative
        let new_v = if default < 0.0 { -(v.abs() * scale) } else { v * scale };
        params.insert(name.to_string(), new_v);
    }
    params
}

fn passes_sanity_tests(params: &HashMap<String, f64>) -> bool {
    Command::new("cargo")
        .args(["test", "--release", "failure_", "--", "--test-threads=4"])
        .envs(params.iter().map(|(k, v)| (k.as_str(), format!("{:.6}", v))))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn write_snake_config(
    config_path: &str,
    best_env: &HashMap<String, String>,
    cand_a_env: HashMap<String, String>,
    cand_b_env: HashMap<String, String>,
) {
    let config = serde_json::json!({
        "snakes": [
            { "env": best_env   },   // snake 0: current champion (always best params)
            { "env": cand_a_env },   // snake 1: candidate A
            { "env": cand_b_env },   // snake 2: candidate B
            { "env": {} }            // snake 3: breadth_first
        ]
    });
    fs::write(config_path, serde_json::to_string_pretty(&config).unwrap())
        .expect("Could not write tournament config");
}

/// Returns (snake0_rate, snake1_rate, snake2_rate, snake3_rate) for all 4 snakes.
fn run_evaluation(
    sim_binary: &PathBuf,
    config_path: &str,
    results_path: &str,
    snake_variant: &str,
    n_games: usize,
) -> Option<(f64, f64, f64, f64)> {
    let _ = fs::remove_file(results_path);

    let status = Command::new(sim_binary)
        .args([
            "--no-build",
            "-n", &n_games.to_string(),
            "-c", config_path,
            "-o", results_path,
            snake_variant,   // slot 0: best
            snake_variant,   // slot 1: candidate A
            snake_variant,   // slot 2: candidate B
            "breadth_first", // slot 3: fixed opponent
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .ok()?;

    if !status.success() { return None; }

    let json = fs::read_to_string(results_path).ok()?;
    let r: SimResults = serde_json::from_str(&json).ok()?;
    if r.snake_names.len() < 4 { return None; }
    Some((
        r.win_rate(&r.snake_names[0]),
        r.win_rate(&r.snake_names[1]),
        r.win_rate(&r.snake_names[2]),
        r.win_rate(&r.snake_names[3]),
    ))
}

fn print_params(params: &HashMap<String, f64>) {
    for &name in PARAM_NAMES {
        let v = params.get(name).copied()
            .unwrap_or(PARAM_DEFAULTS[PARAM_NAMES.iter().position(|&n| n == name).unwrap()]);
        let default = PARAM_DEFAULTS[PARAM_NAMES.iter().position(|&n| n == name).unwrap()];
        let marker = if (v - default).abs() > 0.01 { " *" } else { "" };
        eprintln!("    {:<32} {:>9.3}  (default {:>8.3}){}", name, v, default, marker);
    }
}

fn to_env(params: &HashMap<String, f64>) -> HashMap<String, String> {
    params.iter().map(|(k, v)| (k.clone(), format!("{:.6}", v))).collect()
}

fn pick_survivor(
    r_champ: f64,
    r_a: f64, cand_a: &HashMap<String, f64>,
    r_b: f64, cand_b: &HashMap<String, f64>,
    r_bf: f64,
) -> Option<HashMap<String, f64>> {
    let (best_rate, best_cand) = if r_a >= r_b { (r_a, cand_a) } else { (r_b, cand_b) };
    if best_rate > r_champ && best_rate > r_bf { Some(best_cand.clone()) } else { None }
}

fn elapsed_str(start: Instant) -> String {
    let s = start.elapsed().as_secs();
    format!("[+{:02}:{:02}:{:02}]", s / 3600, (s % 3600) / 60, s % 60)
}

fn log(log_path: &str, start: Instant, msg: &str) {
    let line = format!("{} {}\n", elapsed_str(start), msg);
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(log_path) {
        let _ = f.write_all(line.as_bytes());
    }
}

// ── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut n_games: usize = 3;
    let mut rounds: usize = 5;
    let mut initial_params_path: Option<String> = None;
    let mut output_path = "tournament_best.json".to_string();
    let mut snake_variant = "single_gamestate_nodes".to_string();
    let mut no_build = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-n" => { i += 1; n_games = args[i].parse().expect("-n needs a number"); }
            "-r" => { i += 1; rounds = args[i].parse().expect("-r needs a number"); }
            "-i" => { i += 1; initial_params_path = Some(args[i].clone()); }
            "-o" => { i += 1; output_path = args[i].clone(); }
            "-v" => { i += 1; snake_variant = args[i].clone(); }
            "--no-build" => no_build = true,
            _ => {}
        }
        i += 1;
    }

    // No positional args needed: opponents are fixed (variant default + depth_first + simple_hungry)

    let sim_binary = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("run_local_simulation")))
        .unwrap_or_else(|| PathBuf::from("./target/release/run_local_simulation"));

    if !sim_binary.exists() {
        eprintln!("Error: run_local_simulation binary not found at {}", sim_binary.display());
        eprintln!("Run `cargo build --release` first, or use --no-build if already built.");
        std::process::exit(1);
    }

    if !no_build {
        eprintln!("Building release binaries...");
        let status = Command::new("cargo")
            .args(["build", "--release"])
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .status()
            .expect("Failed to run cargo build");
        if !status.success() { std::process::exit(1); }
        eprintln!();
    }

    // Load or initialise params
    let mut best = initial_params_path
        .as_deref()
        .and_then(|p| fs::read_to_string(p).ok())
        .and_then(|s| serde_json::from_str::<BestParams>(&s).ok())
        .unwrap_or_else(|| BestParams {
            params: default_params(),
            win_rate: 0.0,
            games_played: 0,
            round: 0,
        });

    let log_path = if output_path.ends_with(".json") {
        output_path.replace(".json", ".log")
    } else {
        format!("{}.log", output_path)
    };

    let sep = "=".repeat(68);
    eprintln!("{}", sep);
    eprintln!("  TOURNAMENT  {} rounds  x  {} games/batch  x  up to 3 batches/round",
        rounds, n_games);
    eprintln!("  Tuning: {}  (4 candidates/round, 2 elimination + 1 final)", snake_variant);
    eprintln!("{}", sep);
    eprintln!("  Starting params:");
    print_params(&best.params);
    eprintln!();

    let pid = std::process::id();
    let config_path = format!("/tmp/tournament_{}_config.json", pid);
    let results_path_tmp = format!("/tmp/tournament_{}_results.json", pid);

    let mut rng = rand::thread_rng();
    let mut total_games = 0usize;
    let start = Instant::now();
    log(&log_path, start, &format!("Tournament started — {} rounds x {} games/batch x up to 3 batches/round", rounds, n_games));

    for round in 0..rounds {
        eprintln!("── Round {}/{} ─────────────────────────────────────────────────────", round + 1, rounds);

        // Generate 4 valid candidates — regenerate any that fail sanity tests
        let cands: Vec<HashMap<String, f64>> = (0..4).map(|i| {
            loop {
                let c = mutate(&best.params, &mut rng);
                if passes_sanity_tests(&c) {
                    break c;
                }
                eprintln!("  cand_{} failed sanity tests — regenerating", i);
            }
        }).collect();
        let best_env = to_env(&best.params);

        // Batch 1: champion | cand[0] | cand[1]
        eprint!("  Batch 1/2: {} games (champion | cand_0 | cand_1)... ", n_games);
        write_snake_config(&config_path, &best_env, to_env(&cands[0]), to_env(&cands[1]));
        let survivor_a = match run_evaluation(&sim_binary, &config_path, &results_path_tmp, &snake_variant, n_games) {
            Some((r0, r1, r2, r3)) => {
                total_games += n_games;
                eprintln!("{:.1}% | {:.1}% | {:.1}% | {:.1}%", r0, r1, r2, r3);
                pick_survivor(r0, r1, &cands[0], r2, &cands[1], r3)
            }
            None => { eprintln!("failed"); None }
        };

        // Batch 2: champion | cand[2] | cand[3]
        eprint!("  Batch 2/2: {} games (champion | cand_2 | cand_3)... ", n_games);
        write_snake_config(&config_path, &best_env, to_env(&cands[2]), to_env(&cands[3]));
        let survivor_b = match run_evaluation(&sim_binary, &config_path, &results_path_tmp, &snake_variant, n_games) {
            Some((r0, r1, r2, r3)) => {
                total_games += n_games;
                eprintln!("{:.1}% | {:.1}% | {:.1}% | {:.1}%", r0, r1, r2, r3);
                pick_survivor(r0, r1, &cands[2], r2, &cands[3], r3)
            }
            None => { eprintln!("failed"); None }
        };

        // Final: 0 survivors → skip; 1 → fill with champion copy; 2 → both challengers
        match (survivor_a, survivor_b) {
            (None, None) => {
                eprintln!("  Champion holds both batches — new candidates next round.");
            }
            (sa, sb) => {
                let a_is_challenger = sa.is_some();
                let b_is_challenger = sb.is_some();
                let final_a = sa.unwrap_or_else(|| best.params.clone());
                let final_b = sb.unwrap_or_else(|| best.params.clone());
                let label_a = if a_is_challenger { "survivor_A" } else { "champion_copy" };
                let label_b = if b_is_challenger { "survivor_B" } else { "champion_copy" };
                eprint!("  Final:      {} games (champion | {} | {})... ", n_games, label_a, label_b);
                write_snake_config(&config_path, &best_env, to_env(&final_a), to_env(&final_b));
                match run_evaluation(&sim_binary, &config_path, &results_path_tmp, &snake_variant, n_games) {
                    Some((r_champ, r_a, r_b, r_bf)) => {
                        total_games += n_games;
                        eprintln!("{:.1}% | {:.1}% | {:.1}% | {:.1}%", r_champ, r_a, r_b, r_bf);
                        let (new_rate, new_params) = if r_a >= r_b { (r_a, final_a) } else { (r_b, final_b) };
                        if new_rate > r_champ && new_rate > r_bf {
                            let old_params = best.params.clone();
                            best.params = new_params;
                            best.win_rate = new_rate;
                            best.games_played += total_games;
                            best.round = round + 1;
                            let json = serde_json::to_string_pretty(&best).unwrap();
                            fs::write(&output_path, &json).unwrap();
                            eprintln!("  New champion! ({:.1}% > {:.1}%)  Saved -> {}", new_rate, r_champ, output_path);
                            let diffs: Vec<String> = PARAM_NAMES.iter().filter_map(|&name| {
                                let old = old_params.get(name).copied().unwrap_or(0.0);
                                let new = best.params.get(name).copied().unwrap_or(0.0);
                                if (old - new).abs() > 0.001 {
                                    Some(format!("{}:{:.1}->{:.1}", name, old, new))
                                } else {
                                    None
                                }
                            }).collect();
                            log(&log_path, start, &format!(
                                "Round {:>3}: NEW CHAMPION  {:.1}%  (games: {})  {}",
                                round + 1, new_rate, total_games,
                                if diffs.is_empty() { "no param change".to_string() } else { diffs.join("  ") }
                            ));
                        } else {
                            eprintln!("  Champion holds final ({:.1}% vs {:.1}%).", r_champ, new_rate);
                        }
                    }
                    None => eprintln!("failed"),
                }
            }
        }
        eprintln!();
    }

    // Clean up temp files
    let _ = fs::remove_file(&config_path);
    let _ = fs::remove_file(&results_path_tmp);

    eprintln!("{}", sep);
    eprintln!("  TOURNAMENT COMPLETE");
    eprintln!("  Best win rate : {:.1}%  (reached in round {})", best.win_rate, best.round);
    eprintln!("  Games played  : {}", total_games);
    eprintln!("  Best params   : {}", output_path);
    eprintln!();
    print_params(&best.params);
    eprintln!("{}", sep);
    eprintln!();
    eprintln!("  To use best params, pass to run_local_simulation:");
    eprintln!("    run_local_simulation -c {} single_gamestate_nodes <ref>", output_path);
    eprintln!();
    eprintln!("  Or continue tuning from this baseline:");
    eprintln!("    run_tournament -n 100 -r 20 -i {}", output_path);
    log(&log_path, start, &format!(
        "Tournament complete — best {:.1}% (round {})  games: {}  log: {}",
        best.win_rate, best.round, total_games, output_path
    ));
}
