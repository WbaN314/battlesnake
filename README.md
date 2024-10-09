# Battlesnake Rust Project

Competing at [play.battlesnake.com](https://play.battlesnake.com). Each participant provides the brain of his own snake on a self hosted server to survive the longest on the playing field. Games run daily and a leaderboard of the smartest snakes is created. [Here](https://play.battlesnake.com/profile/wban314) is the current ranking of this snake.

## Technologies Used

This project uses [Rust](https://www.rust-lang.org/) and [Rocket](https://rocket.rs) for performance reasons (simulating future gamestates scales exponentially so optimization is key). It also comes with an [Dockerfile](https://docs.docker.com/engine/reference/builder/) that is used for easy Google Could Run deployment.

## Commands

```
cargo run
cargo test
cargo bench
```

To test snakes against stored states

```
VARIANT=simple_hungry cargo test
VARIANT=simple_tree_search cargo test
VARIANT=depth_first cargo test
VARIANT=breadth_first cargo test
```

