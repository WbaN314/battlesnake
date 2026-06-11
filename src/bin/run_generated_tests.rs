use battlesnake_game_of_chicken_lib::{OriginalDirection, read_game_state, logic};
use std::env;
use std::fs;
use std::process;

fn parse_expected_direction(filename: &str) -> Option<OriginalDirection> {
    let stem = filename.strip_suffix(".json")?;
    let dir_str = stem.rsplit('_').next()?;
    match dir_str {
        "up" => Some(OriginalDirection::Up),
        "down" => Some(OriginalDirection::Down),
        "left" => Some(OriginalDirection::Left),
        "right" => Some(OriginalDirection::Right),
        _ => None,
    }
}

fn main() {
    let dir = env::args()
        .nth(1)
        .unwrap_or_else(|| "requests/automated".to_string());

    let Ok(entries) = fs::read_dir(&dir) else {
        eprintln!("No test directory found at {}", dir);
        process::exit(1);
    };

    let mut files: Vec<String> = entries
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "json"))
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    files.sort();

    if files.is_empty() {
        eprintln!("No test files found in {}", dir);
        process::exit(0);
    }

    unsafe {
        env::set_var("MODE", "test");
        env::set_var("RUST_LOG", "info");
    }
    env_logger::builder()
        .format_timestamp(None)
        .format_target(false)
        .init();

    eprintln!("Running {} generated tests from {}:", files.len(), dir);

    let mut pass = 0;
    let mut fail = 0;
    let mut panicked = 0;
    let mut failures: Vec<String> = Vec::new();

    for file in &files {
        let json_path = format!("{}/{}", dir, file);
        let expected = parse_expected_direction(file);
        eprint!("  {} ", file);

        let result = std::panic::catch_unwind(|| {
            let gs = read_game_state(&json_path);
            logic::get_move(&gs, "single_gamestate_nodes".to_string())
        });

        match (&result, expected) {
            (Ok(actual), Some(expected_dir)) => {
                if *actual == expected_dir {
                    eprintln!("... ok ({})", actual);
                    pass += 1;
                } else {
                    eprintln!("... FAIL (expected: {}, got: {})", expected_dir, actual);
                    failures.push(format!("{}: expected {} got {}", file, expected_dir, actual));
                    fail += 1;
                }
            }
            (Ok(actual), None) => {
                eprintln!("... ok (no expected direction, picked: {})", actual);
                pass += 1;
            }
            (Err(_), _) => {
                eprintln!("... PANICKED");
                failures.push(format!("{}: panicked", file));
                panicked += 1;
            }
        }
    }

    eprintln!();
    eprintln!(
        "Results: {} passed, {} failed, {} panicked out of {} total",
        pass,
        fail,
        panicked,
        files.len()
    );

    if !failures.is_empty() {
        eprintln!();
        eprintln!("Failures:");
        for f in &failures {
            eprintln!("  {}", f);
        }
        process::exit(1);
    }
}
