use std::{collections::HashMap, io};
#[cfg(unix)]
use std::os::unix::io::AsRawFd;

use battlesnake_game_of_chicken_lib::{
    OriginalBattlesnake, OriginalCoord, OriginalDirection, OriginalGameState, read_game_state,
};
use battlesnake_game_of_chicken_lib::logic::get_move as ai_get_move;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction as LayoutDir, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use serde_json::json;

// ── Constants ────────────────────────────────────────────────────────────────

const BOARD_W: usize = 11;
const BOARD_H: usize = 11;
const MAX_SNAKES: usize = 4;

const BUF_H: usize = 2 + BOARD_H * 3; // 35
const BUF_W: usize = 3 + BOARD_W * 6; // 69

const SNAKE_COLORS: [Color; 4] = [Color::Red, Color::Green, Color::Cyan, Color::Yellow];
const SNAKE_NAMES: [&str; 4] = ["A", "B", "C", "D"];

// ── Setup-mode cell types ────────────────────────────────────────────────────

#[derive(Clone, PartialEq, Debug)]
enum Cell {
    Empty,
    Food,
    Snake(usize, usize),
}

#[derive(Clone, Debug)]
struct SnakeState {
    segments: Vec<(usize, usize)>,
    health: i32,
}

impl SnakeState {
    fn new() -> Self {
        SnakeState { segments: vec![], health: 100 }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum EditMode {
    Snake,
    Food,
    Erase,
}

// ── Controller ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
enum Controller {
    Manual,
    AI(String),
}

impl Controller {
    fn label(&self) -> &str {
        match self {
            Controller::Manual => "Manual",
            Controller::AI(v) => v.as_str(),
        }
    }

    fn cycle_next(&self) -> Self {
        const ALL: &[&str] = &[
            "Manual", "simple_hungry", "breadth_first",
            "depth_first", "single_gamestate_nodes",
        ];
        let cur = self.label();
        let idx = ALL.iter().position(|&s| s == cur).unwrap_or(0);
        let next = ALL[(idx + 1) % ALL.len()];
        if next == "Manual" { Controller::Manual } else { Controller::AI(next.to_string()) }
    }

    fn cycle_prev(&self) -> Self {
        const ALL: &[&str] = &[
            "Manual", "simple_hungry", "breadth_first",
            "depth_first", "single_gamestate_nodes",
        ];
        let cur = self.label();
        let idx = ALL.iter().position(|&s| s == cur).unwrap_or(0);
        let prev = ALL[(idx + ALL.len() - 1) % ALL.len()];
        if prev == "Manual" { Controller::Manual } else { Controller::AI(prev.to_string()) }
    }
}

// ── App mode ─────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone)]
enum AppMode {
    Setup,
    Config,
    MoveInput,
    GameOver,
}

// ── App state ─────────────────────────────────────────────────────────────────

struct AppState {
    // Setup fields
    board: [[Cell; BOARD_W]; BOARD_H],
    snakes: [SnakeState; MAX_SNAKES],
    cursor: (usize, usize),
    active_snake: usize,
    edit_mode: EditMode,
    you_snake: usize,
    setup_turn: i32,

    // Config fields
    config_idx: usize,
    controllers_pending: Vec<(String, Controller)>,

    // Play fields
    game_state: Option<OriginalGameState>,
    controllers: HashMap<String, Controller>,
    color_map: HashMap<String, usize>,
    pending_moves: HashMap<String, OriginalDirection>,
    manual_queue: Vec<String>,

    // Common
    app_mode: AppMode,
    status_msg: String,

    // Game over
    game_over_msg: String,

    // Saved initial state for reset
    initial_json: Option<String>,
    // State at last play-start for p-restart
    play_start_json: Option<String>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            board: std::array::from_fn(|_| std::array::from_fn(|_| Cell::Empty)),
            snakes: std::array::from_fn(|_| SnakeState::new()),
            cursor: (5, 5),
            active_snake: 0,
            edit_mode: EditMode::Snake,
            you_snake: 0,
            setup_turn: 0,

            config_idx: 0,
            controllers_pending: Vec::new(),

            game_state: None,
            controllers: HashMap::new(),
            color_map: HashMap::new(),
            pending_moves: HashMap::new(),
            manual_queue: Vec::new(),

            app_mode: AppMode::Setup,
            status_msg: String::from(
                "Board editor: arrows=cursor, space=place, 1-4=snake, f=food, e=erase, p=play, o=open, s=save, r=reset, q=quit",
            ),

            game_over_msg: String::new(),
            initial_json: None,
            play_start_json: None,
        }
    }

    fn place_at_cursor(&mut self) {
        let (x, y) = self.cursor;
        match self.edit_mode {
            EditMode::Food => {
                if matches!(self.board[y][x], Cell::Food) {
                    self.board[y][x] = Cell::Empty;
                } else {
                    self.erase_cell(x, y);
                    self.board[y][x] = Cell::Food;
                }
            }
            EditMode::Erase => {
                self.erase_cell(x, y);
            }
            EditMode::Snake => {
                if let Cell::Snake(sid, seg_idx) = self.board[y][x].clone() {
                    if sid == self.active_snake {
                        let tail_idx = self.snakes[sid].segments.len().saturating_sub(1);
                        if seg_idx == tail_idx && !self.snakes[sid].segments.is_empty() {
                            self.remove_snake_tail(sid);
                            return;
                        }
                    }
                }
                self.erase_cell(x, y);
                let seg_idx = self.snakes[self.active_snake].segments.len();
                self.snakes[self.active_snake].segments.push((x, y));
                self.board[y][x] = Cell::Snake(self.active_snake, seg_idx);
            }
        }
    }

    fn erase_cell(&mut self, x: usize, y: usize) {
        if let Cell::Snake(sid, seg_idx) = self.board[y][x].clone() {
            let to_remove: Vec<(usize, usize)> = self.snakes[sid].segments[seg_idx..].to_vec();
            self.snakes[sid].segments.truncate(seg_idx);
            for (rx, ry) in to_remove {
                self.board[ry][rx] = Cell::Empty;
            }
        } else {
            self.board[y][x] = Cell::Empty;
        }
    }

    fn remove_snake_tail(&mut self, sid: usize) {
        if let Some((tx, ty)) = self.snakes[sid].segments.pop() {
            self.board[ty][tx] = Cell::Empty;
        }
    }

    fn reset(&mut self) {
        let initial_json = self.initial_json.take();
        let play_start_json = self.play_start_json.take();
        *self = Self::new();
        let restore = play_start_json.as_deref().or(initial_json.as_deref());
        if let Some(json) = restore {
            if let Ok(gs) = serde_json::from_str::<OriginalGameState>(json) {
                let _ = self.load_from_gamestate(&gs);
                self.status_msg = "Back to editor.".to_string();
            }
        }
        self.initial_json = initial_json;
        self.play_start_json = play_start_json;
    }

    fn load_from_file(&mut self, path: &str) -> Result<(), String> {
        let gs = std::panic::catch_unwind(|| read_game_state(path))
            .map_err(|_| format!("Failed to parse '{}'", path))?;
        self.load_from_gamestate(&gs)?;
        if let Ok(content) = std::fs::read_to_string(path) {
            self.initial_json = Some(content);
        }
        Ok(())
    }

    fn load_from_gamestate(&mut self, gs: &OriginalGameState) -> Result<(), String> {
        let mut new = AppState::new();
        new.setup_turn = gs.turn;
        let you_id = &gs.you.id;
        let mut ordered: Vec<_> = gs.board.snakes.iter().collect();
        if let Some(pos) = ordered.iter().position(|s| &s.id == you_id) {
            ordered.swap(0, pos);
        }
        if ordered.len() > MAX_SNAKES {
            return Err(format!("Too many snakes ({} > {})", ordered.len(), MAX_SNAKES));
        }
        for (sid, snake) in ordered.iter().enumerate() {
            if snake.body.is_empty() { continue; }
            new.snakes[sid].health = snake.health;
            for seg in &snake.body {
                let x = seg.x as usize;
                let y = seg.y as usize;
                if x >= BOARD_W || y >= BOARD_H {
                    return Err(format!("Coord ({},{}) out of bounds", x, y));
                }
                let seg_idx = new.snakes[sid].segments.len();
                if seg_idx > 0 && new.snakes[sid].segments[seg_idx - 1] == (x, y) {
                    continue;
                }
                new.snakes[sid].segments.push((x, y));
                new.board[y][x] = Cell::Snake(sid, new.snakes[sid].segments.len() - 1);
            }
            if &snake.id == you_id { new.you_snake = sid; }
        }
        for food in &gs.board.food {
            let x = food.x as usize;
            let y = food.y as usize;
            if x < BOARD_W && y < BOARD_H && matches!(new.board[y][x], Cell::Empty) {
                new.board[y][x] = Cell::Food;
            }
        }
        *self = new;
        Ok(())
    }

    fn validate(&self) -> Result<(), String> {
        for sid in 0..MAX_SNAKES {
            let snake = &self.snakes[sid];
            if snake.segments.is_empty() { continue; }
            if snake.segments.len() < 3 {
                return Err(format!(
                    "Snake {} has only {} segments (min 3)",
                    SNAKE_NAMES[sid], snake.segments.len()
                ));
            }
            if snake.health <= 0 {
                return Err(format!(
                    "Snake {} has health {} (must be > 0)",
                    SNAKE_NAMES[sid], snake.health
                ));
            }
            let (hx, hy) = snake.segments[0];
            if (hx + hy) % 2 != (self.setup_turn.unsigned_abs() as usize) % 2 {
                return Err(format!(
                    "Snake {} head ({},{}) parity mismatch for turn {} — (x+y) % 2 must equal turn % 2",
                    SNAKE_NAMES[sid], hx, hy, self.setup_turn
                ));
            }
            for w in snake.segments.windows(2) {
                let (x1, y1) = w[0];
                let (x2, y2) = w[1];
                let dx = (x1 as i32 - x2 as i32).abs();
                let dy = (y1 as i32 - y2 as i32).abs();
                if !((dx == 1 && dy == 0) || (dx == 0 && dy == 1) || (dx == 0 && dy == 0)) {
                    return Err(format!(
                        "Snake {} non-adjacent segments ({},{})→({},{})",
                        SNAKE_NAMES[sid], x1, y1, x2, y2
                    ));
                }
            }
        }
        if self.snakes.iter().all(|s| s.segments.is_empty()) {
            return Err("No snakes on board".to_string());
        }
        // Check no two different snakes share a cell
        for sid in 0..MAX_SNAKES {
            for seg in &self.snakes[sid].segments {
                for other_sid in (sid + 1)..MAX_SNAKES {
                    if self.snakes[other_sid].segments.contains(seg) {
                        return Err(format!(
                            "Snakes {} and {} overlap at ({},{})",
                            SNAKE_NAMES[sid], SNAKE_NAMES[other_sid], seg.0, seg.1
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn to_json(&self) -> Result<String, String> {
        self.validate()?;
        let mut snakes_json = vec![];
        for sid in 0..MAX_SNAKES {
            let snake = &self.snakes[sid];
            if snake.segments.is_empty() { continue; }
            let body_json: Vec<_> = snake.segments.iter()
                .map(|(x, y)| json!({"x": x, "y": y}))
                .collect();
            let head = &snake.segments[0];
            snakes_json.push(json!({
                "id": format!("snake-{}", sid),
                "name": format!("Snake {}", SNAKE_NAMES[sid]),
                "health": snake.health,
                "body": body_json,
                "head": {"x": head.0, "y": head.1},
                "length": snake.segments.len(),
                "latency": "100",
                "shout": null,
                "customizations": {
                    "color": match sid { 0 => "#FF0000", 1 => "#00FF00", 2 => "#00FFFF", _ => "#FFFF00" },
                    "head": "default",
                    "tail": "default"
                }
            }));
        }
        let food_json: Vec<_> = (0..BOARD_H).flat_map(|y| {
            (0..BOARD_W).filter_map(move |x| {
                if matches!(self.board[y][x], Cell::Food) {
                    Some(json!({"x": x, "y": y}))
                } else { None }
            })
        }).collect();
        let you_idx = snakes_json.iter()
            .position(|s| s["id"].as_str() == Some(&format!("snake-{}", self.you_snake)))
            .unwrap_or(0);
        let you = snakes_json[you_idx].clone();
        let state = json!({
            "game": {
                "id": "play-game",
                "ruleset": {
                    "name": "standard",
                    "version": "v1.1.15",
                    "settings": {"foodSpawnChance": 0, "minimumFood": 0, "hazardDamagePerTurn": 0}
                },
                "map": "standard",
                "source": "play_game",
                "timeout": 500
            },
            "turn": self.setup_turn,
            "board": {
                "height": BOARD_H, "width": BOARD_W,
                "food": food_json, "hazards": [], "snakes": snakes_json
            },
            "you": you
        });
        Ok(serde_json::to_string_pretty(&state).unwrap())
    }

    fn save_to_path(&mut self, path: &str) {
        match self.to_json() {
            Ok(json) => {
                if let Some(parent) = std::path::Path::new(path).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                match std::fs::write(path, &json) {
                    Ok(()) => self.status_msg = format!("Saved to {}", path),
                    Err(e) => self.status_msg = format!("Save error: {}", e),
                }
            }
            Err(e) => self.status_msg = format!("Error: {}", e),
        }
    }

    fn start_config(&mut self) {
        if let Err(e) = self.validate() {
            self.status_msg = format!("Error: {}", e);
            return;
        }
        let json = match self.to_json() {
            Err(e) => { self.status_msg = format!("Error: {}", e); return; }
            Ok(j) => j,
        };
        let gs: OriginalGameState = match serde_json::from_str(&json) {
            Ok(gs) => gs,
            Err(e) => { self.status_msg = format!("Parse error: {}", e); return; }
        };

        let mut color_map: HashMap<String, usize> = HashMap::new();
        for snake in &gs.board.snakes {
            let color_idx = snake.id.trim_start_matches("snake-")
                .parse::<usize>().unwrap_or(0);
            color_map.insert(snake.id.clone(), color_idx);
        }

        let controllers_pending: Vec<(String, Controller)> = gs.board.snakes.iter()
            .map(|s| {
                let ctrl = if s.id == gs.you.id {
                    Controller::Manual
                } else {
                    Controller::AI("single_gamestate_nodes".to_string())
                };
                (s.id.clone(), ctrl)
            })
            .collect();

        self.color_map = color_map;
        self.controllers_pending = controllers_pending;
        self.config_idx = 0;
        self.game_state = Some(gs);
        self.app_mode = AppMode::Config;
        self.status_msg = "Configure controllers: ←/→ or Tab=cycle, Enter=confirm & next snake, Esc=back".to_string();
    }

    // ── Restart play from initial board with the same controller config ──────

    fn restart_play_with_last_config(&mut self) {
        let saved_controllers = self.controllers.clone();
        let play_start_json = self.play_start_json.clone();

        let gs: OriginalGameState = match play_start_json.as_deref()
            .and_then(|j| serde_json::from_str(j).ok())
        {
            Some(gs) => gs,
            None => { self.status_msg = "No play state saved — start a game first.".to_string(); return; }
        };

        let mut color_map = HashMap::new();
        for snake in &gs.board.snakes {
            let color_idx = snake.id.trim_start_matches("snake-").parse::<usize>().unwrap_or(0);
            color_map.insert(snake.id.clone(), color_idx);
        }
        self.controllers = gs.board.snakes.iter()
            .map(|s| {
                let ctrl = saved_controllers.get(&s.id).cloned()
                    .unwrap_or(Controller::AI("single_gamestate_nodes".to_string()));
                (s.id.clone(), ctrl)
            })
            .collect();
        self.color_map = color_map;
        self.game_state = Some(gs);
        self.play_start_json = play_start_json;
        self.pending_moves.clear();
        self.manual_queue.clear();
        self.enter_move_input();
    }

    // ── Auto-start: skip Setup+Config, go directly to MoveInput ────────────

    fn auto_start_play(&mut self) {
        if let Err(e) = self.validate() {
            self.status_msg = format!("Error: {}", e);
            return;
        }
        let json = match self.to_json() {
            Err(e) => { self.status_msg = format!("Error: {}", e); return; }
            Ok(j) => j,
        };
        let gs: OriginalGameState = match serde_json::from_str(&json) {
            Ok(gs) => gs,
            Err(e) => { self.status_msg = format!("Parse error: {}", e); return; }
        };
        let mut color_map = HashMap::new();
        for snake in &gs.board.snakes {
            let color_idx = snake.id.trim_start_matches("snake-").parse::<usize>().unwrap_or(0);
            color_map.insert(snake.id.clone(), color_idx);
        }
        let controllers: HashMap<String, Controller> = gs.board.snakes.iter()
            .map(|s| {
                let ctrl = if s.id == gs.you.id {
                    Controller::Manual
                } else {
                    Controller::AI("single_gamestate_nodes".to_string())
                };
                (s.id.clone(), ctrl)
            })
            .collect();
        self.color_map = color_map;
        self.game_state = Some(gs);
        self.play_start_json = self.game_state.as_ref().and_then(|g| serde_json::to_string(g).ok());
        self.controllers = controllers;
        self.pending_moves.clear();
        self.manual_queue.clear();
        self.enter_move_input();
    }

    // ── Transition: Config → MoveInput ──────────────────────────────────────

    fn start_play(&mut self) {
        self.play_start_json = self.game_state.as_ref().and_then(|gs| serde_json::to_string(gs).ok());
        self.controllers = self.controllers_pending.iter().cloned().collect();
        self.pending_moves.clear();
        self.manual_queue.clear();
        self.enter_move_input();
    }

    // ── Transition: (re)enter MoveInput ─────────────────────────────────────

    fn enter_move_input(&mut self) {
        let snake_ctrl: Vec<(String, Controller)> = {
            let gs = match self.game_state.as_ref() { Some(g) => g, None => return };
            gs.board.snakes.iter().map(|s| {
                let ctrl = self.controllers.get(&s.id).cloned().unwrap_or(Controller::Manual);
                (s.id.clone(), ctrl)
            }).collect()
        };

        let mut ai_moves: HashMap<String, OriginalDirection> = HashMap::new();
        let mut manual_queue: Vec<String> = Vec::new();

        for (snake_id, ctrl) in &snake_ctrl {
            match ctrl {
                Controller::AI(variant) => {
                    let gs_for_ai = make_gs_for_ai(self.game_state.as_ref().unwrap(), snake_id);
                    let variant = variant.clone();
                    let dir = with_stdout_suppressed(|| ai_get_move(&gs_for_ai, variant));
                    ai_moves.insert(snake_id.clone(), dir);
                }
                Controller::Manual => {
                    manual_queue.push(snake_id.clone());
                }
            }
        }

        self.pending_moves = ai_moves;
        self.manual_queue = manual_queue;
        self.app_mode = AppMode::MoveInput;
        self.update_move_status();
    }

    // Re-run AI moves only (called after food changes mid-turn).
    // Preserves manual snake pending moves and the manual queue.
    fn recompute_ai_moves(&mut self) {
        let snake_ctrl: Vec<(String, Controller)> = {
            let gs = match self.game_state.as_ref() { Some(g) => g, None => return };
            gs.board.snakes.iter().map(|s| {
                let ctrl = self.controllers.get(&s.id).cloned().unwrap_or(Controller::Manual);
                (s.id.clone(), ctrl)
            }).collect()
        };
        for (snake_id, ctrl) in &snake_ctrl {
            if let Controller::AI(variant) = ctrl {
                let gs_for_ai = make_gs_for_ai(self.game_state.as_ref().unwrap(), snake_id);
                let variant = variant.clone();
                let dir = with_stdout_suppressed(|| ai_get_move(&gs_for_ai, variant));
                self.pending_moves.insert(snake_id.clone(), dir);
            }
        }
    }

    fn update_move_status(&mut self) {
        let turn = self.game_state.as_ref().map(|g| g.turn).unwrap_or(0);
        if self.manual_queue.is_empty() {
            self.status_msg = format!(
                "Turn {} — all moves set. Enter=advance  WASD=cursor  F=food  E=erase food",
                turn
            );
        } else {
            let sid = &self.manual_queue[0].clone();
            let cidx = self.color_map.get(sid.as_str()).copied().unwrap_or(0);
            self.status_msg = format!(
                "Turn {} — Snake {} move: ↑↓←→  |  WASD=cursor  F=food  E=erase food",
                turn, SNAKE_NAMES[cidx]
            );
        }
    }

    fn apply_manual_move(&mut self, dir: OriginalDirection) {
        if let Some(snake_id) = self.manual_queue.first().cloned() {
            self.pending_moves.insert(snake_id, dir);
            self.manual_queue.remove(0);
        }
        if self.manual_queue.is_empty() {
            self.advance_turn();
        } else {
            self.update_move_status();
        }
    }

    // Toggle food at cursor (MoveInput); recomputes AI if food changed.
    fn toggle_food_in_move_input(&mut self) {
        let (x, y) = self.cursor;
        let coord = OriginalCoord { x: x as i32, y: y as i32 };
        if let Some(gs) = &mut self.game_state {
            if gs.board.food.contains(&coord) {
                gs.board.food.retain(|f| f != &coord);
            } else {
                let on_snake = gs.board.snakes.iter().any(|s| s.body.contains(&coord));
                if !on_snake { gs.board.food.push(coord); }
            }
        }
        let has_ai = self.controllers.values().any(|c| matches!(c, Controller::AI(_)));
        if has_ai { self.recompute_ai_moves(); }
        self.update_move_status();
    }

    // ── Game step ─────────────────────────────────────────────────────────────

    fn advance_turn(&mut self) {
        let moves = self.pending_moves.clone();
        if let Some(gs) = &mut self.game_state {
            step_game(gs, &moves);
        }
        self.pending_moves.clear();
        self.manual_queue.clear();

        let alive = self.game_state.as_ref().map(|g| g.board.snakes.len()).unwrap_or(0);
        if alive <= 1 {
            self.game_over_msg = if alive == 0 {
                "Draw — all snakes died simultaneously.".to_string()
            } else {
                let gs = self.game_state.as_ref().unwrap();
                let winner = &gs.board.snakes[0];
                let cidx = self.color_map.get(&winner.id).copied().unwrap_or(0);
                format!("Snake {} wins!", SNAKE_NAMES[cidx])
            };
            self.app_mode = AppMode::GameOver;
            self.status_msg = format!("GAME OVER: {}  Press 'r' to reset or 'q' to quit.", self.game_over_msg);
            return;
        }

        self.enter_move_input();
    }
}

// ── Stdout suppression (silences debug println! from AI during TUI) ───────────

#[cfg(unix)]
fn with_stdout_suppressed<F: FnOnce() -> R, R>(f: F) -> R {
    let devnull = std::fs::OpenOptions::new().write(true).open("/dev/null").unwrap();
    let saved = unsafe { libc::dup(1) };
    unsafe { libc::dup2(devnull.as_raw_fd(), 1); }
    drop(devnull);
    let result = f();
    unsafe { libc::dup2(saved, 1); libc::close(saved); }
    result
}

#[cfg(not(unix))]
fn with_stdout_suppressed<F: FnOnce() -> R, R>(f: F) -> R { f() }

// ── Game logic helpers ────────────────────────────────────────────────────────

fn dir_delta(dir: OriginalDirection) -> (i32, i32) {
    match dir {
        OriginalDirection::Up    => (0, 1),
        OriginalDirection::Down  => (0, -1),
        OriginalDirection::Left  => (-1, 0),
        OriginalDirection::Right => (1, 0),
    }
}

fn make_gs_for_ai(gs: &OriginalGameState, snake_id: &str) -> OriginalGameState {
    let v = serde_json::to_value(gs).unwrap();
    let mut cloned: OriginalGameState = serde_json::from_value(v).unwrap();
    if let Some(snake) = cloned.board.snakes.iter().find(|s| s.id == snake_id).cloned() {
        cloned.you = snake;
    }
    cloned
}

fn step_game(gs: &mut OriginalGameState, moves: &HashMap<String, OriginalDirection>) {
    gs.turn += 1;
    let width = gs.board.width;
    let height = gs.board.height as i32;

    for snake in gs.board.snakes.iter_mut() {
        let dir = moves.get(&snake.id).copied().unwrap_or(OriginalDirection::Up);
        let (dx, dy) = dir_delta(dir);
        let new_head = OriginalCoord { x: snake.head.x + dx, y: snake.head.y + dy };
        snake.body.insert(0, new_head);
        snake.head = new_head;
        snake.health -= 1;
    }

    let mut eat_set: Vec<usize> = Vec::new();
    let mut food_remove: Vec<usize> = Vec::new();
    for (si, snake) in gs.board.snakes.iter().enumerate() {
        for (fi, food) in gs.board.food.iter().enumerate() {
            if *food == snake.head {
                eat_set.push(si);
                if !food_remove.contains(&fi) { food_remove.push(fi); }
                break;
            }
        }
    }

    for (si, snake) in gs.board.snakes.iter_mut().enumerate() {
        if eat_set.contains(&si) {
            snake.health = 100;
        } else {
            snake.body.pop();
        }
        snake.length = snake.body.len() as i32;
    }

    food_remove.sort_unstable_by(|a, b| b.cmp(a));
    food_remove.dedup();
    for fi in food_remove { gs.board.food.remove(fi); }

    let n = gs.board.snakes.len();
    let mut dead = vec![false; n];

    for (i, snake) in gs.board.snakes.iter().enumerate() {
        if snake.head.x < 0 || snake.head.x >= width
            || snake.head.y < 0 || snake.head.y >= height
            || snake.health <= 0
        {
            dead[i] = true;
        }
    }

    for i in 0..n {
        if dead[i] { continue; }
        let head = gs.board.snakes[i].head;
        'outer: for j in 0..n {
            for k in 1..gs.board.snakes[j].body.len() {
                if gs.board.snakes[j].body[k] == head {
                    dead[i] = true;
                    break 'outer;
                }
            }
        }
    }

    for i in 0..n {
        if dead[i] { continue; }
        for j in (i + 1)..n {
            if dead[j] { continue; }
            if gs.board.snakes[i].head == gs.board.snakes[j].head {
                let li = gs.board.snakes[i].length;
                let lj = gs.board.snakes[j].length;
                if li <= lj { dead[i] = true; }
                if lj <= li { dead[j] = true; }
            }
        }
    }

    let mut living: Vec<OriginalBattlesnake> = Vec::new();
    for (i, snake) in gs.board.snakes.drain(..).enumerate() {
        if !dead[i] { living.push(snake); }
    }
    gs.board.snakes = living;

    let you_id = gs.you.id.clone();
    if let Some(you) = gs.board.snakes.iter().find(|s| s.id == you_id).cloned() {
        gs.you = you;
    }
}

// ── Tile rendering ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
enum BDir { Up, Down, Left, Right }

#[derive(Clone, Copy, Debug)]
enum TileCell {
    Empty,
    Food,
    Head(u8),
    Body(u8, BDir),
}

fn compute_tile(
    cell: TileCell,
    up: Option<TileCell>,
    down: Option<TileCell>,
    left: Option<TileCell>,
    right: Option<TileCell>,
) -> ([[char; 5]; 3], Option<Color>) {
    let mut t = [[' '; 5]; 3];
    match cell {
        TileCell::Empty => { t[1][2] = '.'; return (t, None); }
        TileCell::Food  => { t[1][2] = 'X'; return (t, None); }
        TileCell::Head(id) => {
            let uc = (b'A' + id) as char;
            t[1][0] = uc; t[1][2] = uc; t[1][4] = uc;
            t[0][2] = uc; t[2][2] = uc;
        }
        TileCell::Body(id, dir) => {
            let lc = (b'a' + id) as char;
            t[1][2] = '+';
            match dir {
                BDir::Up    => t[0][2] = lc,
                BDir::Down  => t[2][2] = lc,
                BDir::Left  => t[1][0] = lc,
                BDir::Right => t[1][4] = lc,
            }
        }
    }
    let id = match cell {
        TileCell::Head(id) | TileCell::Body(id, _) => id,
        _ => unreachable!(),
    };
    let lc = (b'a' + id) as char;
    let color = SNAKE_COLORS[id as usize];
    if let Some(TileCell::Body(_, BDir::Down))  = up    { t[0][2] = lc; }
    if let Some(TileCell::Body(_, BDir::Up))    = down  { t[2][2] = lc; }
    if let Some(TileCell::Body(_, BDir::Right)) = left  { t[1][0] = lc; }
    if let Some(TileCell::Body(_, BDir::Left))  = right { t[1][4] = lc; }
    (t, Some(color))
}

fn char_color(c: char, owner: Option<Color>) -> Color {
    match c {
        ' '       => Color::Reset,
        '.'       => Color::DarkGray,
        'X'       => Color::Magenta,
        'A' | 'a' => SNAKE_COLORS[0],
        'B' | 'b' => SNAKE_COLORS[1],
        'C' | 'c' => SNAKE_COLORS[2],
        'D' | 'd' => SNAKE_COLORS[3],
        '+'       => owner.unwrap_or(Color::Gray),
        _         => Color::Gray,
    }
}

fn build_tile_grid_setup(
    board: &[[Cell; BOARD_W]; BOARD_H],
    snakes: &[SnakeState; MAX_SNAKES],
) -> [[TileCell; BOARD_W]; BOARD_H] {
    let mut grid = [[TileCell::Empty; BOARD_W]; BOARD_H];
    for y in 0..BOARD_H {
        for x in 0..BOARD_W {
            if matches!(board[y][x], Cell::Food) {
                grid[y][x] = TileCell::Food;
            }
        }
    }
    for (sid, snake) in snakes.iter().enumerate() {
        for (i, &(x, y)) in snake.segments.iter().enumerate() {
            if x >= BOARD_W || y >= BOARD_H { continue; }
            if i == 0 {
                grid[y][x] = TileCell::Head(sid as u8);
            } else {
                let &(px, py) = &snake.segments[i - 1];
                if (px, py) == (x, y) { continue; }
                let dir = match (px as i32 - x as i32, py as i32 - y as i32) {
                    (0, 1)  => BDir::Up,
                    (0, -1) => BDir::Down,
                    (-1, 0) => BDir::Left,
                    (1, 0)  => BDir::Right,
                    _       => continue,
                };
                grid[y][x] = TileCell::Body(sid as u8, dir);
            }
        }
    }
    grid
}

fn build_tile_grid_play(
    gs: &OriginalGameState,
    color_map: &HashMap<String, usize>,
) -> [[TileCell; BOARD_W]; BOARD_H] {
    let mut grid = [[TileCell::Empty; BOARD_W]; BOARD_H];
    for food in &gs.board.food {
        let (x, y) = (food.x, food.y);
        if x >= 0 && x < BOARD_W as i32 && y >= 0 && y < BOARD_H as i32 {
            grid[y as usize][x as usize] = TileCell::Food;
        }
    }
    for snake in &gs.board.snakes {
        let cidx = color_map.get(&snake.id).copied().unwrap_or(0) as u8;
        for (i, coord) in snake.body.iter().enumerate() {
            let (x, y) = (coord.x, coord.y);
            if x < 0 || x >= BOARD_W as i32 || y < 0 || y >= BOARD_H as i32 { continue; }
            if i == 0 {
                grid[y as usize][x as usize] = TileCell::Head(cidx);
            } else {
                let prev = snake.body[i - 1];
                if coord == &prev { continue; }
                let dir = match (prev.x - coord.x, prev.y - coord.y) {
                    (0, 1)  => BDir::Up,
                    (0, -1) => BDir::Down,
                    (-1, 0) => BDir::Left,
                    (1, 0)  => BDir::Right,
                    _       => continue,
                };
                grid[y as usize][x as usize] = TileCell::Body(cidx, dir);
            }
        }
    }
    grid
}

fn build_display_buffer(
    grid: &[[TileCell; BOARD_W]; BOARD_H],
    cursor: Option<(usize, usize)>,
    highlight_head: Option<(usize, usize)>,
) -> ([[char; BUF_W]; BUF_H], [[Color; BUF_W]; BUF_H], [[bool; BUF_W]; BUF_H]) {
    let mut chars: [[char; BUF_W]; BUF_H] = [[' '; BUF_W]; BUF_H];
    let mut owner: [[Option<Color>; BUF_W]; BUF_H] = [[None; BUF_W]; BUF_H];
    let mut is_rev = [[false; BUF_W]; BUF_H];

    for y in 0..BOARD_H {
        for x in 0..BOARD_W {
            let cell = grid[y][x];
            let up    = if y + 1 < BOARD_H { Some(grid[y+1][x]) } else { None };
            let down  = if y > 0           { Some(grid[y-1][x]) } else { None };
            let left  = if x > 0           { Some(grid[y][x-1]) } else { None };
            let right = if x + 1 < BOARD_W { Some(grid[y][x+1]) } else { None };
            let (tile, sc) = compute_tile(cell, up, down, left, right);
            let br0 = (BOARD_H - 1 - y) * 3 + 1;
            let bc0 = x * 6 + 2;
            for tr in 0..3 {
                for tc in 0..5 {
                    chars[br0 + tr][bc0 + tc] = tile[tr][tc];
                    owner[br0 + tr][bc0 + tc] = sc;
                }
            }
        }
    }

    const BOTTOM: &str = "+---0-----1-----2-----3-----4-----5-----6-----7-----8-----9----10---+";
    const LEFT:   &str = "+|0||1||2||3||4||5||6||7||8||9|01|+";
    let bot: Vec<char> = BOTTOM.chars().collect();
    let lft: Vec<char> = LEFT.chars().collect();
    for col in 0..BUF_W {
        if col < bot.len() {
            chars[0][col] = bot[col];
            chars[BUF_H - 1][col] = bot[col];
        }
    }
    for row in 0..BUF_H {
        let li = BUF_H - 1 - row;
        if li < lft.len() {
            chars[row][0] = lft[li];
            chars[row][BUF_W - 1] = lft[li];
        }
    }

    let mut colors: [[Color; BUF_W]; BUF_H] = [[Color::Reset; BUF_W]; BUF_H];
    for row in 0..BUF_H {
        for col in 0..BUF_W {
            colors[row][col] = char_color(chars[row][col], owner[row][col]);
        }
    }

    if let Some((cx, cy)) = cursor {
        if cx < BOARD_W && cy < BOARD_H {
            let br0 = (BOARD_H - 1 - cy) * 3 + 1;
            let bc0 = cx * 6 + 2;
            for tr in 0..3 { for tc in 0..5 { is_rev[br0+tr][bc0+tc] = true; } }
        }
    }

    if let Some((hx, hy)) = highlight_head {
        if hx < BOARD_W && hy < BOARD_H {
            let br0 = (BOARD_H - 1 - hy) * 3 + 1;
            let bc0 = hx * 6 + 2;
            for tr in 0..3 { for tc in 0..5 { is_rev[br0+tr][bc0+tc] = true; } }
        }
    }

    (chars, colors, is_rev)
}

fn render_tile_board(
    f: &mut Frame,
    grid: &[[TileCell; BOARD_W]; BOARD_H],
    cursor: Option<(usize, usize)>,
    highlight_head: Option<(usize, usize)>,
    area: Rect,
) {
    let (chars, colors, is_rev) = build_display_buffer(grid, cursor, highlight_head);
    let mut lines: Vec<Line> = Vec::with_capacity(BUF_H);
    for row in 0..BUF_H {
        let mut spans: Vec<Span> = Vec::new();
        let mut cur_text = String::new();
        let mut cur_color = colors[row][0];
        let mut cur_rev = is_rev[row][0];
        for col in 0..BUF_W {
            let c = chars[row][col];
            let cc = colors[row][col];
            let rv = is_rev[row][col];
            if cc == cur_color && rv == cur_rev {
                cur_text.push(c);
            } else {
                if !cur_text.is_empty() {
                    let mut sty = Style::default().fg(cur_color);
                    if cur_rev { sty = sty.add_modifier(Modifier::REVERSED); }
                    spans.push(Span::styled(cur_text.clone(), sty));
                }
                cur_text = c.to_string();
                cur_color = cc;
                cur_rev = rv;
            }
        }
        if !cur_text.is_empty() {
            let mut sty = Style::default().fg(cur_color);
            if cur_rev { sty = sty.add_modifier(Modifier::REVERSED); }
            spans.push(Span::styled(cur_text, sty));
        }
        lines.push(Line::from(spans));
    }
    f.render_widget(Paragraph::new(lines), area);
}

// ── Sidebar rendering ─────────────────────────────────────────────────────────

fn render_setup_sidebar(f: &mut Frame, app: &AppState, area: Rect) {
    let mut lines: Vec<Line> = vec![];
    lines.push(Line::from(Span::styled(" BOARD EDITOR", Style::default().fg(Color::White).add_modifier(Modifier::BOLD))));
    lines.push(Line::from(""));

    let mode_label = match app.edit_mode {
        EditMode::Snake => format!("Mode: Snake {}", SNAKE_NAMES[app.active_snake]),
        EditMode::Food  => "Mode: Food".to_string(),
        EditMode::Erase => "Mode: Erase".to_string(),
    };
    let mode_color = match app.edit_mode {
        EditMode::Snake => SNAKE_COLORS[app.active_snake],
        EditMode::Food  => Color::Magenta,
        EditMode::Erase => Color::Gray,
    };
    lines.push(Line::from(Span::styled(mode_label, Style::default().fg(mode_color).add_modifier(Modifier::BOLD))));
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(" Snakes:", Style::default().fg(Color::White))));
    for sid in 0..MAX_SNAKES {
        let snake = &app.snakes[sid];
        let marker = if sid == app.active_snake { ">" } else { " " };
        let you_m = if sid == app.you_snake { " (you)" } else { "" };
        let label = if snake.segments.is_empty() {
            format!("{} {} [{}]: empty{}", marker, sid + 1, SNAKE_NAMES[sid], you_m)
        } else {
            format!("{} {} [{}]: len={} hp={}{}", marker, sid + 1, SNAKE_NAMES[sid], snake.segments.len(), snake.health, you_m)
        };
        lines.push(Line::from(Span::styled(label, Style::default().fg(
            if sid == app.active_snake { SNAKE_COLORS[sid] } else { Color::Gray }
        ))));
    }
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(" Controls:", Style::default().fg(Color::White))));
    for (key, desc) in &[
        ("Arrows/HJKL", "Move cursor"),
        ("Space/Enter", "Place"),
        ("1-4",         "Select snake"),
        ("f",           "Food mode"),
        ("e",           "Erase mode"),
        ("n",           "Snake mode"),
        ("y",           "Cycle 'you'"),
        ("t",           "Set turn"),
        ("+/-",         "Change health"),
        ("o",           "Open file"),
        ("s",           "Save JSON"),
        ("p",           "▶ Play (A=manual, rest=AI)"),
        ("c",           "▶ Configure controllers & play"),
        ("r",           "Reset"),
        ("q",           "Quit"),
    ] {
        lines.push(Line::from(vec![
            Span::styled(format!("  {:12}", key), Style::default().fg(Color::Yellow)),
            Span::styled(*desc, Style::default().fg(Color::Gray)),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(format!(" Turn: {}", app.setup_turn), Style::default().fg(Color::White))));
    lines.push(Line::from(Span::styled(format!(" 'You': Snake {}", SNAKE_NAMES[app.you_snake]), Style::default().fg(SNAKE_COLORS[app.you_snake]))));

    f.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::LEFT).border_style(Style::default().fg(Color::DarkGray))),
        area,
    );
}

fn render_config_sidebar(f: &mut Frame, app: &AppState, area: Rect) {
    let mut lines: Vec<Line> = vec![];
    lines.push(Line::from(Span::styled(" CONFIGURE CONTROLLERS", Style::default().fg(Color::White).add_modifier(Modifier::BOLD))));
    lines.push(Line::from(""));

    for (i, (snake_id, ctrl)) in app.controllers_pending.iter().enumerate() {
        let cidx = app.color_map.get(snake_id.as_str()).copied().unwrap_or(i % MAX_SNAKES);
        let is_cur = i == app.config_idx;
        let marker = if is_cur { "> " } else { "  " };
        let you_m = app.game_state.as_ref()
            .map(|gs| if gs.you.id == *snake_id { " (you)" } else { "" })
            .unwrap_or("");
        let style = if is_cur {
            Style::default().fg(SNAKE_COLORS[cidx]).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        lines.push(Line::from(Span::styled(
            format!("{}[{}]{} {}", marker, SNAKE_NAMES[cidx], you_m, ctrl.label()),
            style,
        )));
        if is_cur {
            lines.push(Line::from(Span::styled("  ← Tab/→ to cycle →", Style::default().fg(Color::Yellow))));
        }
        lines.push(Line::from(""));
    }

    lines.push(Line::from(Span::styled(" Controls:", Style::default().fg(Color::White))));
    for (key, desc) in &[
        ("Tab / →", "Next option"),
        ("←",       "Prev option"),
        ("Enter",   "Confirm & next"),
        ("Esc",     "Back to setup"),
    ] {
        lines.push(Line::from(vec![
            Span::styled(format!("  {:12}", key), Style::default().fg(Color::Yellow)),
            Span::styled(*desc, Style::default().fg(Color::Gray)),
        ]));
    }

    f.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::LEFT).border_style(Style::default().fg(Color::DarkGray))),
        area,
    );
}

fn render_play_sidebar(f: &mut Frame, app: &AppState, area: Rect) {
    let gs = match &app.game_state { Some(g) => g, None => return };
    let mut lines: Vec<Line> = vec![];

    let header = match app.app_mode {
        AppMode::MoveInput => " MOVE INPUT",
        AppMode::GameOver  => " GAME OVER",
        _ => " PLAY GAME",
    };
    lines.push(Line::from(Span::styled(header, Style::default().fg(Color::White).add_modifier(Modifier::BOLD))));
    lines.push(Line::from(Span::styled(format!(" Turn: {}", gs.turn), Style::default().fg(Color::White))));
    lines.push(Line::from(""));

    let alive_ids: Vec<&str> = gs.board.snakes.iter().map(|s| s.id.as_str()).collect();

    let mut ordered: Vec<(usize, String)> = app.color_map.iter()
        .map(|(id, &ci)| (ci, id.clone()))
        .collect();
    ordered.sort_by_key(|(ci, _)| *ci);

    let waiting_id = if app.app_mode == AppMode::MoveInput {
        app.manual_queue.first().cloned()
    } else { None };

    for (cidx, snake_id) in &ordered {
        let is_alive = alive_ids.contains(&snake_id.as_str());
        let ctrl = app.controllers.get(snake_id.as_str()).cloned().unwrap_or(Controller::Manual);
        let pending = app.pending_moves.get(snake_id.as_str()).map(|d| match d {
            OriginalDirection::Up    => " [↑]",
            OriginalDirection::Down  => " [↓]",
            OriginalDirection::Left  => " [←]",
            OriginalDirection::Right => " [→]",
        }).unwrap_or("");
        let is_waiting = waiting_id.as_deref() == Some(snake_id.as_str());
        let you_m = if gs.you.id == *snake_id { "(you)" } else { "" };

        if is_alive {
            let snake = gs.board.snakes.iter().find(|s| s.id == *snake_id).unwrap();
            let wait_mark = if is_waiting { " ← INPUT" } else { "" };
            let s_style = if is_waiting {
                Style::default().fg(SNAKE_COLORS[*cidx]).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(SNAKE_COLORS[*cidx])
            };
            lines.push(Line::from(Span::styled(
                format!(" [{}] {} {}{}{}", SNAKE_NAMES[*cidx], you_m, ctrl.label(), pending, wait_mark),
                s_style,
            )));
            lines.push(Line::from(Span::styled(
                format!("  hp:{} len:{}", snake.health, snake.length),
                Style::default().fg(Color::Gray),
            )));
        } else {
            lines.push(Line::from(Span::styled(
                format!(" [{}] (dead)", SNAKE_NAMES[*cidx]),
                Style::default().fg(Color::DarkGray),
            )));
        }
        lines.push(Line::from(""));
    }

    lines.push(Line::from(Span::styled(" Controls:", Style::default().fg(Color::White))));
    let controls: &[(&str, &str)] = match app.app_mode {
        AppMode::MoveInput => &[
            ("↑↓←→",         "Choose move"),
            ("WASD",          "Move cursor"),
            ("F / click",     "Place food"),
            ("E / rclick",    "Erase food"),
            ("Enter",         "Advance turn"),
            ("p",             "Restart (same config)"),
            ("r",             "Back to editor"),
            ("q",             "Quit"),
        ],
        AppMode::GameOver => &[
            ("p",  "Play again (same config)"),
            ("r",  "Back to editor"),
            ("q",  "Quit"),
        ],
        _ => &[],
    };
    for (key, desc) in controls {
        lines.push(Line::from(vec![
            Span::styled(format!("  {:14}", key), Style::default().fg(Color::Yellow)),
            Span::styled(*desc, Style::default().fg(Color::Gray)),
        ]));
    }

    if app.app_mode == AppMode::GameOver {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!(" {}", app.game_over_msg),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        )));
    }

    f.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::LEFT).border_style(Style::default().fg(Color::DarkGray))),
        area,
    );
}

fn render_status(f: &mut Frame, app: &AppState, area: Rect) {
    f.render_widget(
        Paragraph::new(app.status_msg.as_str())
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::TOP).border_style(Style::default().fg(Color::DarkGray))),
        area,
    );
}

fn ui(f: &mut Frame, app: &AppState, board_area_out: &mut Rect) {
    let area = f.area();

    let vertical = Layout::default()
        .direction(LayoutDir::Vertical)
        .constraints([Constraint::Min(BUF_H as u16), Constraint::Length(3)])
        .split(area);
    let main_area = vertical[0];
    let status_area = vertical[1];

    let horizontal = Layout::default()
        .direction(LayoutDir::Horizontal)
        .constraints([Constraint::Length(BUF_W as u16), Constraint::Min(30)])
        .split(main_area);
    let board_area = horizontal[0];
    let sidebar_area = horizontal[1];

    *board_area_out = board_area;

    let grid: [[TileCell; BOARD_W]; BOARD_H] = match app.app_mode {
        AppMode::Setup => build_tile_grid_setup(&app.board, &app.snakes),
        _ => app.game_state.as_ref()
            .map(|gs| build_tile_grid_play(gs, &app.color_map))
            .unwrap_or([[TileCell::Empty; BOARD_W]; BOARD_H]),
    };

    // Cursor visible in Setup and MoveInput (for food placement)
    let cursor = match app.app_mode {
        AppMode::Setup | AppMode::MoveInput => Some(app.cursor),
        _ => None,
    };

    let highlight_head: Option<(usize, usize)> = if app.app_mode == AppMode::MoveInput {
        app.manual_queue.first().and_then(|sid| {
            app.game_state.as_ref().and_then(|gs| {
                gs.board.snakes.iter().find(|s| s.id == *sid)
                    .map(|s| (s.head.x as usize, s.head.y as usize))
            })
        })
    } else {
        None
    };

    render_tile_board(f, &grid, cursor, highlight_head, board_area);

    match app.app_mode {
        AppMode::Setup  => render_setup_sidebar(f, app, sidebar_area),
        AppMode::Config => render_config_sidebar(f, app, sidebar_area),
        _               => render_play_sidebar(f, app, sidebar_area),
    }

    render_status(f, app, status_area);
}

fn board_pixel_to_cell(mx: u16, my: u16, board_area: Rect) -> Option<(usize, usize)> {
    if mx < board_area.x || my < board_area.y { return None; }
    let rel_col = (mx - board_area.x) as usize;
    let rel_row = (my - board_area.y) as usize;
    if rel_col < 2 || rel_row < 1 { return None; }
    let gc = rel_col - 2;
    let gr = rel_row - 1;
    let x = gc / 6;
    let board_row = gr / 3;
    if x >= BOARD_W || board_row >= BOARD_H { return None; }
    Some((x, BOARD_H - 1 - board_row))
}

// Drain crossterm events buffered during synchronous AI computation.
fn drain_events() {
    while event::poll(std::time::Duration::from_millis(0)).unwrap_or(false) {
        let _ = event::read();
    }
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = AppState::new();

    let args: Vec<String> = std::env::args().collect();
    let auto_play = args.iter().any(|a| a == "-p" || a == "--play");
    let file_arg = args.iter().skip(1).find(|a| !a.starts_with('-')).cloned();

    if let Some(path) = &file_arg {
        match app.load_from_file(path) {
            Ok(()) => app.status_msg = format!("Loaded {}", path),
            Err(e) => app.status_msg = format!("Load error: {}", e),
        }
    }

    if auto_play {
        app.auto_start_play();
        drain_events();
    }

    let mut board_area: Rect = Rect::default();
    let mut turn_input: Option<String> = None;
    let mut file_input: Option<String> = None;
    let mut save_input: Option<String> = None;

    loop {
        terminal.draw(|f| ui(f, &app, &mut board_area))?;

        // Turn-number input mode
        if let Some(ref mut buf) = turn_input {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter | KeyCode::Esc => {
                        if let Ok(n) = buf.parse::<i32>() {
                            app.setup_turn = n;
                            app.status_msg = format!("Turn set to {}", n);
                        }
                        turn_input = None;
                    }
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        buf.push(c);
                        app.status_msg = format!("Turn: {} (Enter to confirm)", buf);
                    }
                    KeyCode::Backspace => { buf.pop(); app.status_msg = format!("Turn: {} (Enter to confirm)", buf); }
                    _ => {}
                }
            }
            continue;
        }

        // File-open input mode
        if let Some(ref mut buf) = file_input {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => {
                        let path = buf.clone();
                        file_input = None;
                        match app.load_from_file(&path) {
                            Ok(()) => app.status_msg = format!("Loaded {}", path),
                            Err(e) => app.status_msg = format!("Load error: {}", e),
                        }
                    }
                    KeyCode::Esc => { file_input = None; app.status_msg = "Cancelled".to_string(); }
                    KeyCode::Char(c) => {
                        buf.push(c);
                        app.status_msg = format!("Open: {}_ (Enter/Esc)", buf);
                    }
                    KeyCode::Backspace => { buf.pop(); app.status_msg = format!("Open: {}_ (Enter/Esc)", buf); }
                    KeyCode::Tab => {
                        if let Ok(entries) = std::fs::read_dir("requests") {
                            let mut matches: Vec<String> = entries
                                .flatten()
                                .filter_map(|e| e.file_name().into_string().ok())
                                .filter(|name| name.ends_with(".json") && {
                                    let full = format!("requests/{}", name);
                                    full.starts_with(buf.as_str()) || name.starts_with(buf.as_str())
                                })
                                .collect();
                            matches.sort();
                            if matches.len() == 1 {
                                *buf = format!("requests/{}", matches[0]);
                                app.status_msg = format!("Open: {}_ (Enter/Esc)", buf);
                            } else if !matches.is_empty() {
                                app.status_msg = format!("Matches: {}", matches.join(", "));
                            }
                        }
                    }
                    _ => {}
                }
            }
            continue;
        }

        // Save filename input mode
        if let Some(ref mut buf) = save_input {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => {
                        let path = buf.clone();
                        save_input = None;
                        app.save_to_path(&path);
                    }
                    KeyCode::Esc => {
                        save_input = None;
                        app.status_msg = "Save cancelled.".to_string();
                    }
                    KeyCode::Char(c) => {
                        buf.push(c);
                        app.status_msg = format!("Save as: {}_ (Enter=save, Esc=cancel)", buf);
                    }
                    KeyCode::Backspace => {
                        buf.pop();
                        app.status_msg = format!("Save as: {}_ (Enter=save, Esc=cancel)", buf);
                    }
                    _ => {}
                }
            }
            continue;
        }

        match event::read()? {
            Event::Key(key) => {
                match app.app_mode {
                    // ── Setup mode ────────────────────────────────────────────
                    AppMode::Setup => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('r') => app.reset(),
                        KeyCode::Char('s') => {
                            let mut i = 1;
                            let proposed = loop {
                                let p = format!("requests/editor_{}.json", i);
                                if !std::path::Path::new(&p).exists() { break p; }
                                i += 1;
                            };
                            app.status_msg = format!("Save as: {}_ (Enter=save, Esc=cancel)", proposed);
                            save_input = Some(proposed);
                        }
                        KeyCode::Char('o') => {
                            file_input = Some("requests/".to_string());
                            app.status_msg = "Open: requests/_ (Tab=complete, Enter/Esc)".to_string();
                        }
                        KeyCode::Char('f') => { app.edit_mode = EditMode::Food; app.status_msg = "Food mode".to_string(); }
                        KeyCode::Char('e') => { app.edit_mode = EditMode::Erase; app.status_msg = "Erase mode".to_string(); }
                        KeyCode::Char('n') => { app.edit_mode = EditMode::Snake; app.status_msg = format!("Snake {} mode", SNAKE_NAMES[app.active_snake]); }
                        KeyCode::Char('1') => { app.active_snake = 0; app.edit_mode = EditMode::Snake; app.status_msg = format!("Active: Snake {}", SNAKE_NAMES[0]); }
                        KeyCode::Char('2') => { app.active_snake = 1; app.edit_mode = EditMode::Snake; app.status_msg = format!("Active: Snake {}", SNAKE_NAMES[1]); }
                        KeyCode::Char('3') => { app.active_snake = 2; app.edit_mode = EditMode::Snake; app.status_msg = format!("Active: Snake {}", SNAKE_NAMES[2]); }
                        KeyCode::Char('4') => { app.active_snake = 3; app.edit_mode = EditMode::Snake; app.status_msg = format!("Active: Snake {}", SNAKE_NAMES[3]); }
                        KeyCode::Char('y') => {
                            let mut next = (app.you_snake + 1) % MAX_SNAKES;
                            for _ in 0..MAX_SNAKES {
                                if !app.snakes[next].segments.is_empty() { break; }
                                next = (next + 1) % MAX_SNAKES;
                            }
                            app.you_snake = next;
                            app.status_msg = format!("'You' = Snake {}", SNAKE_NAMES[next]);
                        }
                        KeyCode::Char('t') => { turn_input = Some(String::new()); app.status_msg = "Turn: (Enter to confirm)".to_string(); }
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            app.snakes[app.active_snake].health = (app.snakes[app.active_snake].health + 10).min(100);
                            app.status_msg = format!("Snake {} health: {}", SNAKE_NAMES[app.active_snake], app.snakes[app.active_snake].health);
                        }
                        KeyCode::Char('-') => {
                            app.snakes[app.active_snake].health = (app.snakes[app.active_snake].health - 10).max(1);
                            app.status_msg = format!("Snake {} health: {}", SNAKE_NAMES[app.active_snake], app.snakes[app.active_snake].health);
                        }
                        KeyCode::Up    | KeyCode::Char('k') => { if app.cursor.1 < BOARD_H - 1 { app.cursor.1 += 1; } }
                        KeyCode::Down  | KeyCode::Char('j') => { if app.cursor.1 > 0 { app.cursor.1 -= 1; } }
                        KeyCode::Left  | KeyCode::Char('h') => { if app.cursor.0 > 0 { app.cursor.0 -= 1; } }
                        KeyCode::Right | KeyCode::Char('l') => { if app.cursor.0 < BOARD_W - 1 { app.cursor.0 += 1; } }
                        KeyCode::Char(' ') | KeyCode::Enter => app.place_at_cursor(),
                        KeyCode::Char('p') => {
                            app.auto_start_play();
                            drain_events();
                        }
                        KeyCode::Char('c') => app.start_config(),
                        _ => {}
                    },

                    // ── Config mode ───────────────────────────────────────────
                    AppMode::Config => match key.code {
                        KeyCode::Esc => {
                            app.app_mode = AppMode::Setup;
                            app.game_state = None;
                            app.status_msg = "Back to setup.".to_string();
                        }
                        KeyCode::Tab | KeyCode::Right => {
                            if let Some((_, ctrl)) = app.controllers_pending.get_mut(app.config_idx) {
                                *ctrl = ctrl.cycle_next();
                            }
                        }
                        KeyCode::Left => {
                            if let Some((_, ctrl)) = app.controllers_pending.get_mut(app.config_idx) {
                                *ctrl = ctrl.cycle_prev();
                            }
                        }
                        KeyCode::Enter => {
                            if app.config_idx + 1 < app.controllers_pending.len() {
                                app.config_idx += 1;
                                app.status_msg = "Configure next snake: ←/→=cycle, Enter=confirm".to_string();
                            } else {
                                app.start_play();  // includes AI computation
                                drain_events();
                            }
                        }
                        KeyCode::Char('q') => break,
                        _ => {}
                    },

                    // ── Move input mode ───────────────────────────────────────
                    AppMode::MoveInput => {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('r') => { app.reset(); }
                            KeyCode::Char('p') => { app.restart_play_with_last_config(); drain_events(); }
                            // Direction: arrow keys only
                            KeyCode::Up    if !app.manual_queue.is_empty() => app.apply_manual_move(OriginalDirection::Up),
                            KeyCode::Down  if !app.manual_queue.is_empty() => app.apply_manual_move(OriginalDirection::Down),
                            KeyCode::Left  if !app.manual_queue.is_empty() => app.apply_manual_move(OriginalDirection::Left),
                            KeyCode::Right if !app.manual_queue.is_empty() => app.apply_manual_move(OriginalDirection::Right),
                            // Cursor movement: WASD
                            KeyCode::Char('w') => { if app.cursor.1 < BOARD_H - 1 { app.cursor.1 += 1; } }
                            KeyCode::Char('s') => { if app.cursor.1 > 0 { app.cursor.1 -= 1; } }
                            KeyCode::Char('a') => { if app.cursor.0 > 0 { app.cursor.0 -= 1; } }
                            KeyCode::Char('d') => { if app.cursor.0 < BOARD_W - 1 { app.cursor.0 += 1; } }
                            // Food placement
                            KeyCode::Char('f') | KeyCode::Char(' ') => app.toggle_food_in_move_input(),
                            KeyCode::Char('e') => app.toggle_food_in_move_input(),
                            // Advance turn (when all moves set)
                            KeyCode::Enter => {
                                if app.manual_queue.is_empty() { app.advance_turn(); }
                            }
                            _ => {}
                        }
                        // Drain events buffered during any AI computation above
                        drain_events();
                    }

                    // ── Game over mode ────────────────────────────────────────
                    AppMode::GameOver => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('r') => { app.reset(); }
                        KeyCode::Char('p') => { app.restart_play_with_last_config(); drain_events(); }
                        _ => {}
                    },
                }
            }

            Event::Mouse(mouse) => {
                if let Some((bx, by)) = board_pixel_to_cell(mouse.column, mouse.row, board_area) {
                    match app.app_mode {
                        AppMode::Setup => {
                            app.cursor = (bx, by);
                            match mouse.kind {
                                MouseEventKind::Down(MouseButton::Left) => app.place_at_cursor(),
                                MouseEventKind::Down(MouseButton::Right) => {
                                    let prev = app.edit_mode.clone();
                                    app.edit_mode = EditMode::Erase;
                                    app.place_at_cursor();
                                    app.edit_mode = prev;
                                }
                                _ => {}
                            }
                        }
                        AppMode::MoveInput => {
                            app.cursor = (bx, by);
                            match mouse.kind {
                                MouseEventKind::Down(MouseButton::Left) | MouseEventKind::Down(MouseButton::Right) => {
                                    app.toggle_food_in_move_input();
                                    drain_events();
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }

            _ => {}
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
