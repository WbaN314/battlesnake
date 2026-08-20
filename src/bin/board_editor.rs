use std::{env, io};

use battlesnake_game_of_chicken_lib::{OriginalGameState, read_game_state};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use serde_json::json;

const BOARD_W: usize = 11;
const BOARD_H: usize = 11;
const MAX_SNAKES: usize = 4;
const CELL_W: u16 = 4; // chars per cell
const CELL_H: u16 = 2; // rows per cell

// Colors for snakes A-D
const SNAKE_COLORS: [Color; 4] = [Color::Red, Color::Green, Color::Cyan, Color::Yellow];
const SNAKE_NAMES: [&str; 4] = ["A", "B", "C", "D"];

#[derive(Clone, PartialEq, Debug)]
enum Cell {
    Empty,
    Food,
    // snake_id (0-3), segment_index (0 = head)
    Snake(usize, usize),
}

#[derive(Clone, Debug)]
struct SnakeState {
    segments: Vec<(usize, usize)>, // (x, y) ordered head to tail
    health: i32,
}

impl SnakeState {
    fn new() -> Self {
        SnakeState { segments: vec![], health: 100 }
    }
}

#[derive(Debug)]
struct AppState {
    board: [[Cell; BOARD_W]; BOARD_H],
    snakes: [SnakeState; MAX_SNAKES],
    cursor: (usize, usize), // (x, y) board coords
    active_snake: usize,
    mode: PlacementMode,
    status_msg: String,
    turn: i32,
    you_snake: usize,
    // Tracks which snake id owns which segment at each cell for editing
}

#[derive(Debug, Clone, PartialEq)]
enum PlacementMode {
    Snake,
    Food,
    Erase,
}

impl AppState {
    fn new() -> Self {
        AppState {
            board: std::array::from_fn(|_| std::array::from_fn(|_| Cell::Empty)),
            snakes: std::array::from_fn(|_| SnakeState::new()),
            cursor: (5, 5),
            active_snake: 0,
            mode: PlacementMode::Snake,
            status_msg: String::from("Ready. Space/Enter=place, f=food, e=erase, 1-4=snake, s=save, r=reset, q=quit"),
            turn: 0,
            you_snake: 0,
        }
    }

    fn place_at_cursor(&mut self) {
        let (x, y) = self.cursor;
        match self.mode {
            PlacementMode::Food => {
                self.erase_cell(x, y);
                self.board[y][x] = Cell::Food;
            }
            PlacementMode::Erase => {
                self.erase_cell(x, y);
            }
            PlacementMode::Snake => {
                // If cell already has this snake's tail segment, remove it (toggle off)
                if let Cell::Snake(sid, seg_idx) = self.board[y][x].clone() {
                    if sid == self.active_snake {
                        // Remove last segment of this snake if it's the tail
                        let snake = &self.snakes[sid];
                        if seg_idx == snake.segments.len().saturating_sub(1) && snake.segments.len() > 0 {
                            self.remove_snake_tail(sid);
                            return;
                        }
                    }
                    // Otherwise erase first
                    self.erase_cell(x, y);
                } else {
                    self.erase_cell(x, y);
                }
                // Add new segment to active snake
                let seg_idx = self.snakes[self.active_snake].segments.len();
                self.snakes[self.active_snake].segments.push((x, y));
                self.board[y][x] = Cell::Snake(self.active_snake, seg_idx);
            }
        }
    }

    fn erase_cell(&mut self, x: usize, y: usize) {
        if let Cell::Snake(sid, seg_idx) = self.board[y][x].clone() {
            // Remove this and all subsequent segments from the snake
            let snake = &mut self.snakes[sid];
            let to_remove: Vec<(usize, usize)> = snake.segments[seg_idx..].to_vec();
            snake.segments.truncate(seg_idx);
            for (rx, ry) in to_remove {
                self.board[ry][rx] = Cell::Empty;
            }
        } else {
            self.board[y][x] = Cell::Empty;
        }
    }

    fn remove_snake_tail(&mut self, sid: usize) {
        let snake = &mut self.snakes[sid];
        if let Some((tx, ty)) = snake.segments.pop() {
            self.board[ty][tx] = Cell::Empty;
        }
    }

    fn reset(&mut self) {
        *self = AppState::new();
    }

    fn load_from_file(&mut self, path: &str) -> Result<(), String> {
        let gs = std::panic::catch_unwind(|| read_game_state(path))
            .map_err(|_| format!("Failed to parse '{}'", path))?;
        self.load_from_gamestate(&gs)
    }

    fn load_from_gamestate(&mut self, gs: &OriginalGameState) -> Result<(), String> {
        let mut new = AppState::new();
        new.turn = gs.turn;

        // Map snake id strings to our 0-3 slots; "you" gets slot 0
        let you_id = &gs.you.id;
        let mut ordered: Vec<_> = gs.board.snakes.iter().collect();
        // Put "you" first so it maps to snake 0
        if let Some(pos) = ordered.iter().position(|s| &s.id == you_id) {
            ordered.swap(0, pos);
        }
        if ordered.len() > MAX_SNAKES {
            return Err(format!("Too many snakes ({} > {})", ordered.len(), MAX_SNAKES));
        }

        for (sid, snake) in ordered.iter().enumerate() {
            if snake.body.is_empty() {
                continue;
            }
            new.snakes[sid].health = snake.health;
            for seg in &snake.body {
                let x = seg.x as usize;
                let y = seg.y as usize;
                if x >= BOARD_W || y >= BOARD_H {
                    return Err(format!("Coord ({},{}) out of bounds", x, y));
                }
                // Skip stacked duplicate tail segments (eating artifact)
                let seg_idx = new.snakes[sid].segments.len();
                if seg_idx > 0 {
                    let prev = new.snakes[sid].segments[seg_idx - 1];
                    if prev == (x, y) {
                        continue;
                    }
                }
                new.snakes[sid].segments.push((x, y));
                new.board[y][x] = Cell::Snake(sid, new.snakes[sid].segments.len() - 1);
            }
            if &snake.id == you_id {
                new.you_snake = sid;
            }
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
            if snake.segments.is_empty() {
                continue;
            }
            if snake.segments.len() < 3 {
                return Err(format!(
                    "Snake {} has only {} segments (minimum 3)",
                    SNAKE_NAMES[sid],
                    snake.segments.len()
                ));
            }
            // Check adjacency
            for w in snake.segments.windows(2) {
                let (x1, y1) = w[0];
                let (x2, y2) = w[1];
                let dx = (x1 as i32 - x2 as i32).abs();
                let dy = (y1 as i32 - y2 as i32).abs();
                if !((dx == 1 && dy == 0) || (dx == 0 && dy == 1) || (dx == 0 && dy == 0)) {
                    return Err(format!(
                        "Snake {} has non-adjacent segments between ({},{}) and ({},{})",
                        SNAKE_NAMES[sid], x1, y1, x2, y2
                    ));
                }
            }
        }
        // At least one snake
        if self.snakes.iter().all(|s| s.segments.is_empty()) {
            return Err("No snakes on board".to_string());
        }
        Ok(())
    }

    fn to_json(&self) -> Result<String, String> {
        self.validate()?;

        let mut snakes_json = vec![];
        for sid in 0..MAX_SNAKES {
            let snake = &self.snakes[sid];
            if snake.segments.is_empty() {
                continue;
            }
            let body_json: Vec<_> = snake.segments.iter().map(|(x, y)| json!({"x": x, "y": y})).collect();
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
                } else {
                    None
                }
            })
        }).collect();

        let you_idx = snakes_json.iter().position(|s| {
            s["id"].as_str().map(|id| id == format!("snake-{}", self.you_snake)).unwrap_or(false)
        }).unwrap_or(0);
        let you = snakes_json[you_idx].clone();

        let state = json!({
            "game": {
                "id": "editor-game",
                "ruleset": {
                    "name": "standard",
                    "version": "v1.1.15",
                    "settings": {
                        "foodSpawnChance": 15,
                        "minimumFood": 1,
                        "hazardDamagePerTurn": 14
                    }
                },
                "map": "standard",
                "source": "editor",
                "timeout": 500
            },
            "turn": self.turn,
            "board": {
                "height": BOARD_H,
                "width": BOARD_W,
                "food": food_json,
                "hazards": [],
                "snakes": snakes_json
            },
            "you": you
        });

        Ok(serde_json::to_string_pretty(&state).unwrap())
    }

    fn save(&mut self) {
        match self.to_json() {
            Ok(json) => {
                // Find next free filename
                let mut i = 1;
                loop {
                    let path = format!("requests/editor_{}.json", i);
                    if !std::path::Path::new(&path).exists() {
                        std::fs::write(&path, &json).unwrap();
                        self.status_msg = format!("Saved to {}", path);
                        break;
                    }
                    i += 1;
                }
            }
            Err(e) => {
                self.status_msg = format!("Error: {}", e);
            }
        }
    }
}

fn cell_color(cell: &Cell, _active_snake: usize) -> (Color, bool) {
    match cell {
        Cell::Empty => (Color::DarkGray, false),
        Cell::Food => (Color::Magenta, false),
        Cell::Snake(sid, seg_idx) => {
            let color = SNAKE_COLORS[*sid];
            let is_head = *seg_idx == 0;
            (color, is_head)
        }
    }
}

fn render_board(f: &mut Frame, app: &AppState, board_area: Rect) {
    // board_area is where we draw the 11x11 grid
    // Each cell is CELL_W wide, CELL_H tall
    let _total_w = BOARD_W as u16 * CELL_W;
    let _total_h = BOARD_H as u16 * CELL_H;

    // Draw column labels (x axis) along the top
    for x in 0..BOARD_W {
        let label_area = Rect {
            x: board_area.x + x as u16 * CELL_W,
            y: board_area.y,
            width: CELL_W,
            height: 1,
        };
        let label = Paragraph::new(format!("{:^4}", x))
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(label, label_area);
    }

    // Draw row labels (y axis) on left, note y=0 is bottom so we flip
    for y in 0..BOARD_H {
        let display_y = BOARD_H - 1 - y; // flip so y=0 at bottom
        let row_y = board_area.y + 1 + display_y as u16 * CELL_H;
        let label_area = Rect {
            x: board_area.x.saturating_sub(3),
            y: row_y,
            width: 3,
            height: 1,
        };
        let label = Paragraph::new(format!("{:>2} ", y))
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(label, label_area);
    }

    // Draw cells
    for y in 0..BOARD_H {
        let display_y = BOARD_H - 1 - y;
        for x in 0..BOARD_W {
            let cell = &app.board[y][x];
            let is_cursor = app.cursor == (x, y);

            let cell_rect = Rect {
                x: board_area.x + x as u16 * CELL_W,
                y: board_area.y + 1 + display_y as u16 * CELL_H,
                width: CELL_W,
                height: CELL_H,
            };

            if cell_rect.y + cell_rect.height > f.area().height {
                continue;
            }

            let (color, _is_head) = cell_color(cell, app.active_snake);

            let mut style = Style::default().fg(color);
            if is_cursor {
                style = style.bg(Color::White).fg(Color::Black).add_modifier(Modifier::BOLD);
            } else if matches!(cell, Cell::Empty) {
                style = style.fg(Color::DarkGray);
            }

            let content = match cell {
                Cell::Empty => {
                    if is_cursor { "  ##".to_string() } else { "  · ".to_string() }
                }
                Cell::Food => " ✦  ".to_string(),
                Cell::Snake(sid, seg_idx) => {
                    let snake = &app.snakes[*sid];
                    let label = if *seg_idx == 0 {
                        format!(" {} H", SNAKE_NAMES[*sid])
                    } else if *seg_idx == snake.segments.len() - 1 {
                        format!(" {} T", SNAKE_NAMES[*sid])
                    } else {
                        format!(" {}  ", SNAKE_NAMES[*sid])
                    };
                    label
                }
            };

            let top_line = Line::from(Span::styled(content, style));
            // Second row: show segment index for snake cells
            let bottom_content = match cell {
                Cell::Snake(_sid, seg_idx) => {
                    format!("{:>4}", seg_idx)
                }
                _ => "    ".to_string(),
            };
            let bottom_line = Line::from(Span::styled(
                bottom_content,
                Style::default().fg(Color::DarkGray),
            ));

            let para = Paragraph::new(vec![top_line, bottom_line]);
            f.render_widget(para, cell_rect);
        }
    }
}

fn render_sidebar(f: &mut Frame, app: &AppState, area: Rect) {
    let mut lines: Vec<Line> = vec![];

    lines.push(Line::from(Span::styled(
        " BOARD EDITOR ",
        Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    // Mode
    let mode_label = match app.mode {
        PlacementMode::Snake => format!("Mode: Snake {}", SNAKE_NAMES[app.active_snake]),
        PlacementMode::Food => "Mode: Food".to_string(),
        PlacementMode::Erase => "Mode: Erase".to_string(),
    };
    let mode_color = match app.mode {
        PlacementMode::Snake => SNAKE_COLORS[app.active_snake],
        PlacementMode::Food => Color::Magenta,
        PlacementMode::Erase => Color::Gray,
    };
    lines.push(Line::from(Span::styled(mode_label, Style::default().fg(mode_color).add_modifier(Modifier::BOLD))));
    lines.push(Line::from(""));

    // Snake info
    lines.push(Line::from(Span::styled(" Snakes:", Style::default().fg(Color::White))));
    for sid in 0..MAX_SNAKES {
        let snake = &app.snakes[sid];
        let active_marker = if sid == app.active_snake { ">" } else { " " };
        let you_marker = if sid == app.you_snake { " (you)" } else { "" };
        let len = snake.segments.len();
        let label = if len == 0 {
            format!("{} {} [{}]: empty{}", active_marker, sid + 1, SNAKE_NAMES[sid], you_marker)
        } else {
            format!("{} {} [{}]: len={} hp={}{}", active_marker, sid + 1, SNAKE_NAMES[sid], len, snake.health, you_marker)
        };
        lines.push(Line::from(Span::styled(
            label,
            Style::default().fg(if sid == app.active_snake { SNAKE_COLORS[sid] } else { Color::Gray }),
        )));
    }
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(" Controls:", Style::default().fg(Color::White))));
    let controls = vec![
        ("Arrows/HJKL", "Move cursor"),
        ("Space/Enter", "Place"),
        ("1-4", "Select snake"),
        ("f", "Food mode"),
        ("e", "Erase mode"),
        ("n", "Snake mode"),
        ("y", "Cycle 'you' snake"),
        ("t", "Set turn number"),
        ("+/-", "Change health"),
        ("o", "Open file"),
        ("s", "Save JSON"),
        ("r", "Reset board"),
        ("q", "Quit"),
    ];
    for (key, desc) in controls {
        lines.push(Line::from(vec![
            Span::styled(format!("  {:12}", key), Style::default().fg(Color::Yellow)),
            Span::styled(desc, Style::default().fg(Color::Gray)),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!(" Turn: {}", app.turn),
        Style::default().fg(Color::White),
    )));
    lines.push(Line::from(Span::styled(
        format!(" 'You': Snake {}", SNAKE_NAMES[app.you_snake]),
        Style::default().fg(SNAKE_COLORS[app.you_snake]),
    )));

    let para = Paragraph::new(lines)
        .block(Block::default().borders(Borders::LEFT).border_style(Style::default().fg(Color::DarkGray)));
    f.render_widget(para, area);
}

fn render_status(f: &mut Frame, app: &AppState, area: Rect) {
    let para = Paragraph::new(app.status_msg.as_str())
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::TOP).border_style(Style::default().fg(Color::DarkGray)));
    f.render_widget(para, area);
}

fn board_pixel_to_cell(
    mouse_x: u16,
    mouse_y: u16,
    board_origin: (u16, u16), // (col, row) of top-left of cell (0,10) in terminal
) -> Option<(usize, usize)> {
    let (ox, oy) = board_origin;
    if mouse_x < ox || mouse_y < oy {
        return None;
    }
    let rel_x = mouse_x - ox;
    let rel_y = mouse_y - oy;
    let grid_x = rel_x / CELL_W;
    let grid_display_y = rel_y / CELL_H;
    if grid_x as usize >= BOARD_W || grid_display_y as usize >= BOARD_H {
        return None;
    }
    // display_y 0 = board row y=10 (top), display_y 10 = board row y=0 (bottom)
    let board_y = BOARD_H - 1 - grid_display_y as usize;
    Some((grid_x as usize, board_y))
}

fn ui(f: &mut Frame, app: &AppState, board_origin: &mut (u16, u16)) {
    let area = f.area();

    // Layout: main row | sidebar, status at bottom
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(3)])
        .split(area);

    let main_area = vertical[0];
    let status_area = vertical[1];

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Length(32)])
        .split(main_area);

    let board_area = horizontal[0];
    let sidebar_area = horizontal[1];

    // board cells start at board_area.x + some left padding for y-labels, board_area.y + 1 for x-labels
    let cell_origin_x = board_area.x + 3; // 3 chars for y-labels
    let cell_origin_y = board_area.y + 1; // 1 row for x-labels
    *board_origin = (cell_origin_x, cell_origin_y);

    let board_render_area = Rect {
        x: cell_origin_x,
        y: board_area.y,
        width: board_area.width.saturating_sub(3),
        height: board_area.height,
    };

    render_board(f, app, board_render_area);
    render_sidebar(f, app, sidebar_area);
    render_status(f, app, status_area);
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = AppState::new();

    // Accept optional path argument to preload a file
    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        match app.load_from_file(path) {
            Ok(()) => app.status_msg = format!("Loaded {}", path),
            Err(e) => app.status_msg = format!("Load error: {}", e),
        }
    }

    let mut board_origin: (u16, u16) = (3, 1);
    let mut turn_input: Option<String> = None;
    let mut file_input: Option<String> = None;

    loop {
        terminal.draw(|f| ui(f, &app, &mut board_origin))?;

        if let Some(ref mut buf) = turn_input {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter | KeyCode::Esc => {
                        if let Ok(n) = buf.parse::<i32>() {
                            app.turn = n;
                            app.status_msg = format!("Turn set to {}", n);
                        }
                        turn_input = None;
                    }
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        buf.push(c);
                        app.status_msg = format!("Turn: {} (Enter to confirm)", buf);
                    }
                    KeyCode::Backspace => {
                        buf.pop();
                        app.status_msg = format!("Turn: {} (Enter to confirm)", buf);
                    }
                    _ => {}
                }
            }
            continue;
        }

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
                    KeyCode::Esc => {
                        file_input = None;
                        app.status_msg = "Cancelled".to_string();
                    }
                    KeyCode::Char(c) => {
                        buf.push(c);
                        app.status_msg = format!("Open: {}_ (Enter/Esc)", buf);
                    }
                    KeyCode::Backspace => {
                        buf.pop();
                        app.status_msg = format!("Open: {}_ (Enter/Esc)", buf);
                    }
                    KeyCode::Tab => {
                        // Tab-complete: list matching files in requests/
                        if let Ok(entries) = std::fs::read_dir("requests") {
                            let matches: Vec<String> = entries
                                .flatten()
                                .filter_map(|e| e.file_name().into_string().ok())
                                .filter(|name| {
                                    name.ends_with(".json") && {
                                        let full = format!("requests/{}", name);
                                        full.starts_with(buf.as_str()) || name.starts_with(buf.as_str())
                                    }
                                })
                                .collect();
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

        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('r') => {
                    app.reset();
                }
                KeyCode::Char('s') => {
                    app.save();
                }
                KeyCode::Char('o') => {
                    file_input = Some(String::from("requests/"));
                    app.status_msg = "Open: requests/_ (Tab=complete, Enter/Esc)".to_string();
                }
                KeyCode::Char('f') => {
                    app.mode = PlacementMode::Food;
                    app.status_msg = "Food mode".to_string();
                }
                KeyCode::Char('e') => {
                    app.mode = PlacementMode::Erase;
                    app.status_msg = "Erase mode".to_string();
                }
                KeyCode::Char('n') => {
                    app.mode = PlacementMode::Snake;
                    app.status_msg = format!("Snake {} mode", SNAKE_NAMES[app.active_snake]);
                }
                KeyCode::Char('1') => {
                    app.active_snake = 0;
                    app.mode = PlacementMode::Snake;
                    app.status_msg = format!("Active: Snake {}", SNAKE_NAMES[0]);
                }
                KeyCode::Char('2') => {
                    app.active_snake = 1;
                    app.mode = PlacementMode::Snake;
                    app.status_msg = format!("Active: Snake {}", SNAKE_NAMES[1]);
                }
                KeyCode::Char('3') => {
                    app.active_snake = 2;
                    app.mode = PlacementMode::Snake;
                    app.status_msg = format!("Active: Snake {}", SNAKE_NAMES[2]);
                }
                KeyCode::Char('4') => {
                    app.active_snake = 3;
                    app.mode = PlacementMode::Snake;
                    app.status_msg = format!("Active: Snake {}", SNAKE_NAMES[3]);
                }
                KeyCode::Char('y') => {
                    let mut next = (app.you_snake + 1) % MAX_SNAKES;
                    for _ in 0..MAX_SNAKES {
                        if !app.snakes[next].segments.is_empty() {
                            break;
                        }
                        next = (next + 1) % MAX_SNAKES;
                    }
                    app.you_snake = next;
                    app.status_msg = format!("'You' = Snake {}", SNAKE_NAMES[next]);
                }
                KeyCode::Char('t') => {
                    turn_input = Some(String::new());
                    app.status_msg = "Turn: (Enter to confirm)".to_string();
                }
                KeyCode::Char('+') | KeyCode::Char('=') => {
                    let snake = &mut app.snakes[app.active_snake];
                    snake.health = (snake.health + 10).min(100);
                    app.status_msg = format!("Snake {} health: {}", SNAKE_NAMES[app.active_snake], snake.health);
                }
                KeyCode::Char('-') => {
                    let snake = &mut app.snakes[app.active_snake];
                    snake.health = (snake.health - 10).max(1);
                    app.status_msg = format!("Snake {} health: {}", SNAKE_NAMES[app.active_snake], snake.health);
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if app.cursor.1 < BOARD_H - 1 { app.cursor.1 += 1; }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.cursor.1 > 0 { app.cursor.1 -= 1; }
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    if app.cursor.0 > 0 { app.cursor.0 -= 1; }
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    if app.cursor.0 < BOARD_W - 1 { app.cursor.0 += 1; }
                }
                KeyCode::Char(' ') | KeyCode::Enter => {
                    app.place_at_cursor();
                }
                _ => {}
            },
            Event::Mouse(mouse) => {
                if let Some((bx, by)) = board_pixel_to_cell(mouse.column, mouse.row, board_origin) {
                    app.cursor = (bx, by);
                    match mouse.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            app.place_at_cursor();
                        }
                        MouseEventKind::Down(MouseButton::Right) => {
                            let prev_mode = app.mode.clone();
                            app.mode = PlacementMode::Erase;
                            app.place_at_cursor();
                            app.mode = prev_mode;
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
