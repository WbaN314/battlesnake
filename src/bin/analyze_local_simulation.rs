use battlesnake_game_of_chicken_lib::{OriginalDirection, OriginalGameState, logic};
use battlesnake_game_of_chicken_lib::logic::general::field::BasicField;
use battlesnake_game_of_chicken_lib::logic::general::game_state::GameState;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct TurnRecord {
    turn: i32,
    json: String,
    picked: Option<OriginalDirection>,
}

struct Game {
    id: String,
    turns: Vec<TurnRecord>,
}

fn parse_result_line(line: &str) -> Option<(String, i32, OriginalDirection)> {
    // "... ID <game_id> Turn <turn_num> Result -> <direction>"
    let id_pos = line.find("] ID ")?;
    let after_id = &line[id_pos + 5..];
    let space = after_id.find(' ')?;
    let game_id = after_id[..space].to_string();

    let turn_pos = after_id.find("Turn ")?;
    let after_turn = &after_id[turn_pos + 5..];
    let arrow = after_turn.find(" Result -> ")?;
    let turn: i32 = after_turn[..arrow].parse().ok()?;
    let dir_str = after_turn[arrow + 11..].trim();

    let direction = match dir_str {
        "up" => OriginalDirection::Up,
        "down" => OriginalDirection::Down,
        "left" => OriginalDirection::Left,
        "right" => OriginalDirection::Right,
        _ => return None,
    };

    Some((game_id, turn, direction))
}

fn parse_log(path: &str) -> Vec<Game> {
    let file = fs::File::open(path).expect("Cannot open log file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    // Group turns by game ID
    let mut game_turns: HashMap<String, Vec<TurnRecord>> = HashMap::new();
    let mut game_order: Vec<String> = Vec::new();

    for line in &lines {
        if let Some((game_id, turn, json)) = parse_turn_line(line) {
            if !game_turns.contains_key(&game_id) {
                game_order.push(game_id.clone());
            }
            game_turns.entry(game_id).or_default().push(TurnRecord {
                turn,
                json,
                picked: None,
            });
        } else if let Some((game_id, turn, direction)) = parse_result_line(line) {
            if let Some(turns) = game_turns.get_mut(&game_id) {
                if let Some(record) = turns.iter_mut().find(|r| r.turn == turn) {
                    record.picked = Some(direction);
                }
            }
        }
    }

    game_order
        .into_iter()
        .map(|id| {
            let mut turns = game_turns.remove(&id).unwrap();
            turns.sort_by_key(|t| t.turn);
            Game { id, turns }
        })
        .collect()
}

fn parse_turn_line(line: &str) -> Option<(String, i32, String)> {
    // "... ID <game_id> Turn <turn_num> Request -> <json>"
    let id_pos = line.find("] ID ")?;
    let after_id = &line[id_pos + 5..];
    let space = after_id.find(' ')?;
    let game_id = after_id[..space].to_string();

    let turn_pos = after_id.find("Turn ")?;
    let after_turn = &after_id[turn_pos + 5..];
    let arrow = after_turn.find(" Request -> ")?;
    let turn: i32 = after_turn[..arrow].parse().ok()?;
    let json = after_turn[arrow + 12..].to_string();

    Some((game_id, turn, json))
}

fn direction_name(d: &OriginalDirection) -> &'static str {
    match d {
        OriginalDirection::Up => "up",
        OriginalDirection::Down => "down",
        OriginalDirection::Left => "left",
        OriginalDirection::Right => "right",
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let log_dir = args.get(1).map(|s| s.as_str()).unwrap_or("game_logs");
    let timeout_ms: u64 = args
        .get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10_000);
    let max_turns_back: usize = args
        .get(3)
        .and_then(|s| s.parse().ok())
        .unwrap_or(20);

    eprintln!("Re-evaluation timeout: {}ms", timeout_ms);
    eprintln!("Max turns back per game: {}", max_turns_back);

    let mut log_files: Vec<String> = fs::read_dir(log_dir)
        .unwrap_or_else(|_| panic!("Cannot read directory {}", log_dir))
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            let name = path.file_name()?.to_str()?.to_string();
            if name.ends_with("_lost.log") {
                Some(path.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect();
    log_files.sort();
    eprintln!("Found {} lost game logs in {}", log_files.len(), log_dir);

    let games: Vec<Game> = log_files
        .iter()
        .flat_map(|path| {
            eprintln!("  Parsing {}", path);
            parse_log(path)
        })
        .collect();
    let lost_games: Vec<&Game> = games.iter().collect();

    eprintln!("Lost games to analyze: {}", lost_games.len());

    if lost_games.is_empty() {
        eprintln!("No lost games to analyze.");
        return;
    }

    // Ensure output directory exists
    let out_dir = Path::new("requests/automated");
    fs::create_dir_all(out_dir).unwrap();

    unsafe {
        env::set_var("MODE", "test");
        env::set_var("SIMULATION_TIME_MS", timeout_ms.to_string());
        env::set_var("RUST_LOG", "info");
    }
    env_logger::builder()
        .format_timestamp(None)
        .format_target(false)
        .init();

    let mut saved_count = 0;

    for game in &lost_games {
        eprintln!(
            "\nGame {} ({} turns):",
            &game.id[..8.min(game.id.len())],
            game.turns.len()
        );

        // Iterate from last turn backwards
        let start = if game.turns.len() > max_turns_back {
            game.turns.len() - max_turns_back
        } else {
            0
        };

        for turn_record in game.turns[start..].iter().rev() {
            let Some(original_pick) = &turn_record.picked else {
                continue;
            };

            // Parse and validate gamestate
            let gs: Result<OriginalGameState, _> = serde_json::from_str(&turn_record.json);
            let Ok(gs) = gs else {
                continue;
            };

            // Skip states where our snake has only 2 opponents left and is about to die
            // (not much we can learn from those)
            if gs.board.snakes.len() < 2 {
                continue;
            }

            // Print the game state
            let board_state = GameState::<BasicField>::from(&gs);
            eprintln!("\n  Turn {}: original={}", turn_record.turn, direction_name(original_pick));
            eprintln!("{}", board_state);
            eprintln!("  Re-evaluating...");

            let re_eval = std::panic::catch_unwind(|| {
                logic::get_move_with_evaluation(&gs)
            });

            match re_eval {
                Ok((new_pick, eval_string)) => {
                    if new_pick != *original_pick {
                        let filename = format!(
                            "game_{}_turn_{}_{}.json",
                            &game.id[..8.min(game.id.len())],
                            turn_record.turn,
                            direction_name(&new_pick)
                        );
                        let out_path = out_dir.join(&filename);
                        fs::write(&out_path, &turn_record.json).unwrap();

                        let txt_filename = filename.replace(".json", ".txt");
                        let txt_path = out_dir.join(&txt_filename);
                        let txt_content = format!("Picked: {}  Should have picked: {}\n\n{}\n{}", direction_name(original_pick), direction_name(&new_pick), board_state, eval_string);
                        fs::write(&txt_path, txt_content).unwrap();

                        saved_count += 1;

                        eprintln!(
                            "    DIFFERS: {} -> {} (saved: {})",
                            direction_name(original_pick),
                            direction_name(&new_pick),
                            filename
                        );
                        break;
                    } else {
                        eprintln!("    same: {}", direction_name(original_pick));
                    }
                }
                Err(_) => {
                    eprintln!("    PANICKED (skipping)");
                }
            }
        }
    }

    unsafe {
        env::remove_var("SIMULATION_TIME_MS");
    }

    eprintln!("\nSaved {} differing game states to requests/automated/", saved_count);
}
