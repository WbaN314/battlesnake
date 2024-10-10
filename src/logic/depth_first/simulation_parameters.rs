use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct SimulationParameters {
    pub move_snake_heads_radius: Option<u8>,
    pub prune_hash_radius: Option<u8>,
    pub simulation_duration: Option<Duration>,
    start: Instant,
}

impl SimulationParameters {
    pub fn new() -> Self {
        SimulationParameters {
            move_snake_heads_radius: None,
            prune_hash_radius: None,
            simulation_duration: None,
            start: Instant::now(),
        }
    }

    pub fn move_snake_heads_radius(self, distance: u8) -> Self {
        SimulationParameters {
            move_snake_heads_radius: Some(distance),
            ..self
        }
    }

    pub fn prune_hash_radius(self, distance: u8) -> Self {
        SimulationParameters {
            prune_hash_radius: Some(distance),
            ..self
        }
    }

    pub fn simulation_duration(self, duration: Duration) -> Self {
        SimulationParameters {
            simulation_duration: Some(duration),
            ..self
        }
    }

    pub fn is_time_up(&self) -> bool {
        if let Some(time) = self.simulation_duration {
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
        parameters.simulation_duration = Some(Duration::from_millis(100));
        assert!(!parameters.is_time_up());
        std::thread::sleep(Duration::from_millis(200));
        assert!(parameters.is_time_up());
    }

    #[test]
    fn test_parameters_builder_pattern() {
        let parameters = SimulationParameters::new()
            .move_snake_heads_radius(5)
            .prune_hash_radius(3)
            .simulation_duration(Duration::from_millis(100));
        assert_eq!(parameters.move_snake_heads_radius, Some(5));
        assert_eq!(parameters.prune_hash_radius, Some(3));
        assert_eq!(
            parameters.simulation_duration,
            Some(Duration::from_millis(100))
        );
    }
}
