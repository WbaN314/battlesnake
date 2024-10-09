use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct SimulationParameters {
    pub move_snake_head_distance: Option<u8>,
    pub board_state_prune_distance: Option<u8>,
    pub duration: Option<Duration>,
    pub start: Instant,
}

impl SimulationParameters {
    pub fn new() -> Self {
        SimulationParameters {
            move_snake_head_distance: None,
            board_state_prune_distance: None,
            duration: None,
            start: Instant::now(),
        }
    }

    pub fn move_snake_head_distance(self, distance: u8) -> Self {
        SimulationParameters {
            move_snake_head_distance: Some(distance),
            ..self
        }
    }

    pub fn board_state_prune_distance(self, distance: u8) -> Self {
        SimulationParameters {
            board_state_prune_distance: Some(distance),
            ..self
        }
    }

    pub fn duration(self, duration: Duration) -> Self {
        SimulationParameters {
            duration: Some(duration),
            ..self
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_time_up() {
        let mut parameters = SimulationParameters::new();
        parameters.duration = Some(Duration::from_millis(100));
        assert!(!parameters.is_time_up());
        std::thread::sleep(Duration::from_millis(200));
        assert!(parameters.is_time_up());
    }

    #[test]
    fn test_parameters_builder_pattern() {
        let parameters = SimulationParameters::new()
            .move_snake_head_distance(5)
            .board_state_prune_distance(3)
            .duration(Duration::from_millis(100));
        assert_eq!(parameters.move_snake_head_distance, Some(5));
        assert_eq!(parameters.board_state_prune_distance, Some(3));
        assert_eq!(parameters.duration, Some(Duration::from_millis(100)));
    }
}
