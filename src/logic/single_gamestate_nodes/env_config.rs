use std::{env, sync::LazyLock, time::Duration};

pub static ENV_CONFIG: LazyLock<EnvironmentConfig> = LazyLock::new(EnvironmentConfig::read);

macro_rules! env_config {
    ( $( $name:ident = $default:expr ),+ $(,)? ) => {
        #[allow(non_snake_case)]
        pub struct EnvironmentConfig {
            pub SIMULATION_TIME_MS: Duration,
            pub LOCAL_SIMULATION: bool,
            $( pub $name: f32, )+
        }
        impl EnvironmentConfig {
            pub fn read() -> Self {
                let ef = |name: &str, default: f32| -> f32 {
                    env::var(name).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
                };
                Self {
                    SIMULATION_TIME_MS: Duration::from_millis(
                        env::var("SIMULATION_TIME_MS").ok().and_then(|v| v.parse().ok()).unwrap_or(200),
                    ),
                    LOCAL_SIMULATION: env::var("LOCAL_SIMULATION").is_ok_and(|v| !v.is_empty()),
                    $( $name: ef(stringify!($name), $default), )+
                }
            }
        }
    }
}

env_config! {
    // First Moves
    SCORE_FIRST_MOVES_TOWARD_CENTER = 200.0,

    // Simulation Back Propagation Scores
    SCORE_SIMULATION_WINNER = 1000.0,
    SCORE_SIMULATION_KILL = 200.0,
    SCORE_SIMULATION_FOOD = 10.0,

    // Situation Matches
    SCORE_AVOID_MOVING_NEXT_TO_WALL = -20.0,
    SCORE_GRAB_FOOD = 60.0,
    SCORE_KILL_SITUATION = 100.0,
    SCORE_RESTRICT_SITUATION = 50.0,

    // Capture
    SCORE_NOT_ENOUGH_AREA = -10.0,
    SCORE_SQUEEZED_SNAKES = 100.0,
    SCORE_FOOD = 70.0,
    SCORE_FOOD_DECAY_COEFFICIENT = 0.2,
    SCORE_ENEMY_PUSHED = 20.0,

    // Wall Avoidance
    SCORE_NEXT_TO_WALL = -20.0,

    // Positioning
    SCORE_ENEMY_MIDPOINT = 10.0,
    SCORE_TOWARDS_CENTER = 5.0,
}
