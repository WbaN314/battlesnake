pub mod e_board;
pub mod e_coord;
pub mod e_direction;
pub mod e_game_state;
pub mod e_snakes;
pub mod e_state_node;
pub mod e_state_tree;

#[cfg(test)]
mod tests {
    use crate::logic::efficient_game_objects::{
        e_board::EField, e_coord::ECoord, e_direction::EDirection, e_game_state::EGameState,
        e_state_tree::EStateTree,
    };

    fn read_game_state(path: &str) -> crate::GameState {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let game_state: crate::GameState = serde_json::from_reader(reader).unwrap();
        game_state
    }

    #[test]
    fn print_board_1() {
        let game_state = read_game_state("requests/example_move_request.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        println!("{board}")
    }

    #[test]
    fn print_board_1_up() {
        let game_state = read_game_state("requests/example_move_request.json");
        let mut board = EGameState::from(&game_state.board, &game_state.you);
        board
            .move_snakes([Some(EDirection::Up), Some(EDirection::Up), None, None])
            .unwrap();
        println!("{board}")
    }

    #[test]
    fn print_board_1_up_up() {
        let game_state = read_game_state("requests/example_move_request.json");
        let mut board = EGameState::from(&game_state.board, &game_state.you);
        board
            .move_snakes([Some(EDirection::Up), Some(EDirection::Up), None, None])
            .unwrap();
        board
            .move_snakes([Some(EDirection::Up), Some(EDirection::Up), None, None])
            .unwrap();
        println!("{board}")
    }

    #[test]
    fn print_board_1_up_up_up() {
        let game_state = read_game_state("requests/example_move_request.json");
        let mut board = EGameState::from(&game_state.board, &game_state.you);
        board
            .move_snakes([Some(EDirection::Up), Some(EDirection::Up), None, None])
            .unwrap();
        board
            .move_snakes([Some(EDirection::Up), Some(EDirection::Up), None, None])
            .unwrap();
        board
            .move_snakes([Some(EDirection::Up), Some(EDirection::Up), None, None])
            .unwrap();
        println!("{board}")
    }

    #[test]
    fn print_board_2() {
        let game_state = read_game_state("requests/example_move_request_2.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        println!("{board}")
    }

    #[test]
    fn snakes_to_board() {
        let game_state = read_game_state("requests/example_move_request.json");
        let gamestate = EGameState::from(&game_state.board, &game_state.you);
        assert_eq!(gamestate.snakes.get(0).as_ref().unwrap().health, 54);
        assert_eq!(gamestate.snakes.get(1).as_ref().unwrap().health, 16);
        assert!(gamestate.snakes.get(2).is_none());
        assert!(gamestate.snakes.get(3).is_none());
    }

    #[test]
    fn snakeparts_on_board() {
        let game_state = read_game_state("requests/example_move_request.json");
        let gamestate = EGameState::from(&game_state.board, &game_state.you);
        assert_eq!(
            gamestate.board.get(0, 0).unwrap(),
            EField::SnakePart {
                snake_number: 0,
                next: None,
                stacked: 1
            }
        );
        assert_eq!(
            gamestate.board.get(1, 0).unwrap(),
            EField::SnakePart {
                snake_number: 0,
                next: Some(ECoord { x: 0, y: 0 }),
                stacked: 1
            }
        );
        assert_eq!(
            gamestate.board.get(2, 0).unwrap(),
            EField::SnakePart {
                snake_number: 0,
                next: Some(ECoord { x: 1, y: 0 }),
                stacked: 1
            }
        );
    }

    #[test]
    fn fill_board() {
        let game_state = read_game_state("requests/example_move_request.json");
        let mut state = EGameState::from(&game_state.board, &game_state.you);
        assert!(state.board.clone().fill(&ECoord::from(0, 0)).is_none());
        assert!(state.board.clone().fill(&ECoord::from(-1, 0)).is_none());
        assert_eq!(state.board.fill(&ECoord::from(0, 1)).unwrap().area, 114);
        println!("{state}");
    }

    #[test]
    fn fill_board_2() {
        let game_state = read_game_state("requests/example_move_request_2.json");
        let mut state = EGameState::from(&game_state.board, &game_state.you);
        assert_eq!(state.board.fill(&ECoord::from(0, 1)).unwrap().area, 20);
        println!("{state}");
    }

    #[test]
    fn relevant_moves() {
        let game_state = read_game_state("requests/example_move_request.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        let movesets = board.relevant_moves(u8::MAX);
        for m in movesets {
            println!("{:?}", m);
        }
    }

    #[test]
    fn relevant_moves_2() {
        let game_state = read_game_state("requests/example_move_request_2.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        let movesets = board.relevant_moves(u8::MAX);
        for m in movesets {
            println!("{:?}", m);
        }
    }

    #[test]
    fn relevant_moves_3() {
        let game_state = read_game_state("requests/example_move_request_3.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        let movesets = board.relevant_moves(u8::MAX);
        for m in movesets {
            println!("{:?}", m);
        }
    }

    #[test]
    fn move_other_snakes_up() {
        let game_state = read_game_state("requests/example_move_request.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        let mut moved_up = board.clone();
        match moved_up.move_snakes([None, Some(EDirection::Up), None, None]) {
            Ok(_) => println!("{}", moved_up),
            Err(_) => println!("Death"),
        }
    }

    #[test]
    fn move_other_snakes_left() {
        let game_state = read_game_state("requests/example_move_request.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        let mut moved_up = board.clone();
        match moved_up.move_snakes([Some(EDirection::Left), Some(EDirection::Left), None, None]) {
            Ok(_) => println!("{}", moved_up),
            Err(_) => println!("Death"),
        }
    }

    #[test]
    fn move_other_snakes_down() {
        let game_state = read_game_state("requests/example_move_request.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        let mut moved_up = board.clone();
        match moved_up.move_snakes([Some(EDirection::Up), Some(EDirection::Down), None, None]) {
            Ok(_) => println!("{}", moved_up),
            Err(_) => println!("Death"),
        }
    }

    #[test]
    fn direction_tree_simulate() {
        let game_state = read_game_state("requests/example_move_request.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        let mut d_tree = EStateTree::from(board);
        d_tree.simulate_timed(u8::MAX, 200);
        println!("{}", d_tree)
    }

    #[test]
    fn print_board_3() {
        let game_state = read_game_state("requests/failure_1.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        println!("{board}");
    }

    #[test]
    fn print_board_3_after_move() {
        let game_state = read_game_state("requests/failure_1.json");
        let mut board = EGameState::from(&game_state.board, &game_state.you);
        println!("{board}");
        board
            .move_snakes([
                Some(EDirection::Down),
                Some(EDirection::Up),
                Some(EDirection::Down),
                None,
            ])
            .unwrap();
        println!("{board}")
    }

    #[test]
    fn failure_1() {
        let game_state = read_game_state("requests/failure_1.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        let mut d_tree = EStateTree::from(board);
        d_tree.simulate_timed(u8::MAX, 200);
    }

    #[test]
    fn failure_2() {
        let game_state = read_game_state("requests/failure_2.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        println!("{}", &board);
        let mut d_tree = EStateTree::from(board);
        let result = d_tree.simulate_timed(u8::MAX, 200);
        println!("{:?}", result);
    }

    #[test]
    fn failure_4() {
        let game_state = read_game_state("requests/failure_4.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        println!("{}", &board);
        let mut d_tree = EStateTree::from(board);
        let result = d_tree.simulate_timed(u8::MAX, 200);
        println!("{:?}", result);
    }

    #[test]
    fn failure_5() {
        let game_state = read_game_state("requests/failure_5.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        println!("{}", &board);
        let mut d_tree = EStateTree::from(board);
        let result = d_tree.simulate_timed(u8::MAX, 200);
        println!("{:?}", result);
    }

    #[test]
    fn limit_distance() {
        let game_state = read_game_state("requests/failure_1.json");
        let board = EGameState::from(&game_state.board, &game_state.you);
        let mut d_tree = EStateTree::from(board);
        let mut d_tree_2 = d_tree.clone();
        let result = d_tree.simulate_timed(u8::MAX, 200);
        println!("{:?}", result);
        let result_2 = d_tree_2.simulate_timed(4, 200);
        println!("{:?}", result_2);
    }
}
