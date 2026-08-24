macro_rules! situation_test {
    ($name:ident, $( $dir:ident ),+ => $msg:literal) => {
        #[test]
        fn $name() {
            let chosen_move = get_move_from_json_file(concat!(stringify!($name), ".json"));
            assert!([$(OriginalDirection::$dir),+].contains(&chosen_move), $msg);
        }
    };
    ($name:ident, $( $dir:ident ),+) => {
        #[test]
        fn $name() {
            let chosen_move = get_move_from_json_file(concat!(stringify!($name), ".json"));
            assert!([$(OriginalDirection::$dir),+].contains(&chosen_move));
        }
    };
}
