use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct SimulationParameters {
    pub move_snake_head_distance: u8,
    pub board_state_prune_distance: Option<u8>,
    pub duration: Option<Duration>,
    pub start: Instant,
}

impl SimulationParameters {
    pub fn new() -> Self {
        SimulationParameters {
            move_snake_head_distance: u8::MAX,
            board_state_prune_distance: None,
            duration: None,
            start: Instant::now(),
        }
    }

    pub fn is_time_up(&self) -> bool {
        if let Some(time) = self.duration {
            self.start.elapsed() >= time
        } else {
            false
        }
    }
}
