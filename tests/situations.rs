#[cfg(test)]
use battlesnake_game_of_chicken::{get_move_from_json_file, Direction};

#[test]
fn test_move_request() {
    let chosen_move = get_move_from_json_file("test_move_request.json");
    assert_ne!(chosen_move, Direction::Down);
    assert_ne!(chosen_move, Direction::Left);
}

#[test]
fn example_move_request_2() {
    let chosen_move = get_move_from_json_file("example_move_request_2.json");
    assert_eq!(chosen_move, Direction::Up);
}

#[test]
fn example_move_request_3() {
    let chosen_move = get_move_from_json_file("example_move_request_3.json");
    assert_eq!(chosen_move, Direction::Down);
}
#[test]
fn failure_1() {
    let chosen_move = get_move_from_json_file("failure_1.json");
    assert_ne!(chosen_move, Direction::Up);
}

#[test]
fn failure_2() {
    let chosen_move = get_move_from_json_file("failure_2.json");
    assert_ne!(chosen_move, Direction::Right);
}

#[test]
fn failure_3() {
    let chosen_move = get_move_from_json_file("failure_3.json");
    assert_eq!(chosen_move, Direction::Down);
}

#[test]
fn failure_4() {
    let chosen_move = get_move_from_json_file("failure_4.json");
    assert_ne!(chosen_move, Direction::Right);
    assert_ne!(chosen_move, Direction::Down);
}

#[test]
fn failure_5() {
    let chosen_move = get_move_from_json_file("failure_5.json");
    assert_eq!(chosen_move, Direction::Up);
}

#[test]
fn failure_6() {
    let chosen_move = get_move_from_json_file("failure_6.json");
    assert_ne!(chosen_move, Direction::Up);
    assert_ne!(chosen_move, Direction::Down);
}

#[test]
fn failure_7() {
    let chosen_move = get_move_from_json_file("failure_7.json");
    assert_ne!(chosen_move, Direction::Up);
    assert_ne!(chosen_move, Direction::Down);
}

#[test]
fn failure_8() {
    let chosen_move = get_move_from_json_file("failure_8.json");
    assert_eq!(chosen_move, Direction::Down);
}

#[test]
fn failure_9() {
    let chosen_move = get_move_from_json_file("failure_9.json");
    // All other situations are barely survivable by using all available space
    assert_ne!(chosen_move, Direction::Left);
}

#[test]
fn failure_10() {
    let chosen_move = get_move_from_json_file("failure_10.json");
    assert_eq!(chosen_move, Direction::Left);
}

#[test]
fn failure_11() {
    let chosen_move = get_move_from_json_file("failure_11.json");
    assert_eq!(chosen_move, Direction::Left);
}

#[test]
fn failure_12() {
    let chosen_move = get_move_from_json_file("failure_12.json");
    assert_eq!(chosen_move, Direction::Left);
}

#[test]
fn failure_13() {
    let chosen_move = get_move_from_json_file("failure_13.json");
    assert_ne!(chosen_move, Direction::Up);
    assert_ne!(chosen_move, Direction::Down);
}

#[test]
fn failure_14() {
    let chosen_move = get_move_from_json_file("failure_14.json");
    assert_ne!(chosen_move, Direction::Down);
}

#[test]
fn failure_15() {
    let chosen_move = get_move_from_json_file("failure_15.json");
    assert_ne!(chosen_move, Direction::Down);
    assert_ne!(chosen_move, Direction::Up);
}

#[test]
fn failure_16() {
    let chosen_move = get_move_from_json_file("failure_16.json");
    assert_ne!(chosen_move, Direction::Down);
    assert_ne!(chosen_move, Direction::Up);
}

#[test]
fn failure_17() {
    let chosen_move = get_move_from_json_file("failure_17.json");
    assert_eq!(chosen_move, Direction::Down);
}

#[test]
fn failure_18() {
    let chosen_move = get_move_from_json_file("failure_18.json");
    assert!([Direction::Down, Direction::Left].contains(&chosen_move));
}

#[test]
fn failure_19() {
    let chosen_move = get_move_from_json_file("failure_19.json");
    assert_ne!(chosen_move, Direction::Left);
    assert_ne!(chosen_move, Direction::Right);
}

#[test]
fn failure_20_for_improved_area_evaluation() {
    // Going left leads to death when B goes up and then right, as there is not enough space
    let chosen_move = get_move_from_json_file("failure_20_for_improved_area_evaluation.json");
    assert_eq!(chosen_move, Direction::Down);
}

#[test]
fn failure_21_bait_into_trap_with_top_wall() {
    let chosen_move = get_move_from_json_file("failure_21_bait_into_trap_with_top_wall.json");
    assert_eq!(chosen_move, Direction::Right);
}

#[test]
fn failure_22_bait_into_trap_with_top_wall_modified() {
    let chosen_move =
        get_move_from_json_file("failure_22_bait_into_trap_with_top_wall_modified.json");
    assert_eq!(chosen_move, Direction::Right);
}

#[test]
fn failure_23_go_for_kill_here() {
    // Down leads to guaranteed kill in 2, right leads to guaranteed kill in 4
    let chosen_move = get_move_from_json_file("failure_23_go_for_kill_here.json");
    assert_ne!(chosen_move, Direction::Up);
    assert_ne!(chosen_move, Direction::Left);
}

#[test]
fn failure_24_debug_space() {
    let chosen_move = get_move_from_json_file("failure_24_debug_space.json");
    assert_ne!(chosen_move, Direction::Up);
}

#[test]
fn failure_25_continue_down_for_kill() {
    let chosen_move = get_move_from_json_file("failure_25_continue_down_for_kill.json");
    assert_eq!(chosen_move, Direction::Down);
}

#[test]
fn failure_26_continue_down_for_kill() {
    let chosen_move = get_move_from_json_file("failure_26_continue_down_for_kill.json");
    assert_eq!(chosen_move, Direction::Down);
}

#[test]
fn failure_27_grab_food() {
    let chosen_move = get_move_from_json_file("failure_27_grab_food.json");
    assert_eq!(chosen_move, Direction::Left);
}

#[test]
fn failure_28_grab_food() {
    let chosen_move = get_move_from_json_file("failure_28_grab_food.json");
    assert_eq!(chosen_move, Direction::Down);
}

#[test]
fn failure_29_move_down_towards_food() {
    let chosen_move = get_move_from_json_file("failure_29_move_down_towards_food.json");
    assert_eq!(chosen_move, Direction::Down);
}

#[test]
fn failure_30_grab_food_leads_to_death() {
    let chosen_move = get_move_from_json_file("failure_30_grab_food_leads_to_death.json");
    assert_ne!(chosen_move, Direction::Right);
    assert_ne!(chosen_move, Direction::Up);
}

#[test]
fn failure_31_going_right_leads_to_death() {
    let chosen_move = get_move_from_json_file("failure_31_going_right_leads_to_death.json");
    assert_ne!(chosen_move, Direction::Right);
    assert_ne!(chosen_move, Direction::Up);
}

#[test]
fn failure_32_right_certain_death_down_maybe_death() {
    let chosen_move =
        get_move_from_json_file("failure_32_right_certain_death_down_maybe_death.json");
    // Assuming perfect opponent play, right is the best move as it leads to certain death in 3 turns whereas down is in 2 turns
    // If opponents do not play perfectly, down is better as it might not lead to death
    assert_ne!(chosen_move, Direction::Up);
    assert_ne!(chosen_move, Direction::Left);
}

#[test]
fn failure_33_do_not_move_left_as_you_can_get_killed() {
    let chosen_move =
        get_move_from_json_file("failure_33_do_not_move_left_as_you_can_get_killed.json");
    assert_ne!(chosen_move, Direction::Left);
    assert_ne!(chosen_move, Direction::Down);
}

#[test]
fn failure_34_follow_own_tail() {
    // Too long to really evaluate with current visualisation tools, therefore left and right accepted
    let chosen_move = get_move_from_json_file("failure_34_follow_own_tail.json");
    assert_ne!(chosen_move, Direction::Up);
    assert_ne!(chosen_move, Direction::Down);
}

#[test]
fn failure_35_up_2() {
    // Up is dead in 3
    // Down is dead in 8
    let chosen_move = get_move_from_json_file("failure_35_up_2.json");
    assert_ne!(chosen_move, Direction::Left);
    assert_ne!(chosen_move, Direction::Right);
}

#[test]
fn failure_36_tail_2_food_4() {
    let chosen_move = get_move_from_json_file("failure_36_tail_2_food_4.json");
    assert_eq!(chosen_move, Direction::Right);
}

#[test]
fn failure_37_unclear_best_move() {
    let chosen_move = get_move_from_json_file("failure_37_unclear_best_move.json");
    assert_ne!(chosen_move, Direction::Up);
}

#[test]
fn failure_38_left_possible_wall_squeeze() {
    // Moving right would equally allow for squeeze
    let chosen_move = get_move_from_json_file("failure_38_left_possible_wall_squeeze.json");
    assert_ne!(chosen_move, Direction::Up);
    assert_ne!(chosen_move, Direction::Down);
}

#[test]
fn failure_39_grab_food_in_middle() {
    // This might be fixed with chicken logic
    // or adding length to simulate timed evaluation
    let chosen_move = get_move_from_json_file("failure_39_grab_food_in_middle.json");
    assert_eq!(chosen_move, Direction::Left);
}

#[test]
fn failure_40_should_go_up_to_food() {
    // Food can be secured in both ways either up or right
    let chosen_move = get_move_from_json_file("failure_40_should_go_up_to_food.json");
    assert_ne!(chosen_move, Direction::Down);
    assert_ne!(chosen_move, Direction::Left);
}

#[test]
fn failure_41_area_suggests_right_but_left_might_be_better() {
    let chosen_move =
        get_move_from_json_file("failure_41_area_suggests_right_but_left_might_be_better.json");
    assert_eq!(chosen_move, Direction::Left);
}

#[test]
fn failure_42_going_right_enables_getting_killed() {
    let chosen_move = get_move_from_json_file("failure_42_going_right_enables_getting_killed.json");
    assert_ne!(chosen_move, Direction::Left);
    assert_ne!(chosen_move, Direction::Up);
}

#[test]
fn failure_43_going_down_guarantees_getting_killed() {
    let chosen_move =
        get_move_from_json_file("failure_43_going_down_guarantees_getting_killed.json");
    assert_eq!(chosen_move, Direction::Up);
}

#[test]
fn failure_44_panic() {
    let chosen_move = get_move_from_json_file("failure_44_panic.json");
    assert_ne!(chosen_move, Direction::Down);
}

#[test]
fn failure_45_panic_again() {
    let chosen_move = get_move_from_json_file("failure_45_panic_again.json");
    assert_eq!(chosen_move, Direction::Right);
}

#[test]
fn failure_46_go_for_kill() {
    let chosen_move = get_move_from_json_file("failure_46_go_for_kill.json");
    assert_eq!(chosen_move, Direction::Right);
}

#[test]
fn failure_47_grab_food() {
    let chosen_move = get_move_from_json_file("failure_47_grab_food.json");
    assert_eq!(chosen_move, Direction::Down);
}

#[test]
fn failure_48_grab_food() {
    // Depending on what B does in the next move it is always possible to escape
    // But after moving left, decision where to move must depend on which move B took and go in opposite direction
    let chosen_move = get_move_from_json_file("failure_48_grab_food.json");
    assert_eq!(chosen_move, Direction::Left);
}

#[test]
fn failure_49() {
    // Failed when L,D,R reached depth 10 by only selecting L as viable from simulation
    let chosen_move = get_move_from_json_file("failure_49.json");
    assert_eq!(chosen_move, Direction::Down);
}

#[test]
fn failure_50() {
    // Failed when L,D,R reached depth 10 by only selecting L as viable from simulation
    let chosen_move = get_move_from_json_file("failure_50.json");
    assert_eq!(chosen_move, Direction::Left);
}

#[test]
fn failure_51_grab_food_after_other_moved_down_in_48() {
    let chosen_move =
        get_move_from_json_file("failure_51_grab_food_after_other_moved_down_in_48.json");
    assert_eq!(chosen_move, Direction::Up);
}

#[test]
fn failure_52_grab_food_after_other_moved_up_in_48() {
    let chosen_move =
        get_move_from_json_file("failure_52_grab_food_after_other_moved_up_in_48.json");
    assert_eq!(chosen_move, Direction::Down);
}

#[test]
fn failure_53_go_for_kill() {
    let chosen_move = get_move_from_json_file("failure_53_go_for_kill.json");
    assert_eq!(chosen_move, Direction::Left);
}
