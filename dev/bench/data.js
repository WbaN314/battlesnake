window.BENCHMARK_DATA = {
  "lastUpdate": 1771619110358,
  "repoUrl": "https://github.com/WbaN314/battlesnake",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "email": "sascha.stoll@sap.com",
            "name": "I530412",
            "username": "SaschaAtWork"
          },
          "committer": {
            "email": "sascha.stoll@sap.com",
            "name": "I530412",
            "username": "SaschaAtWork"
          },
          "distinct": true,
          "id": "25965fa348f2d1597044f862e2b7e554debff13f",
          "message": "Cleaning up benchmarks",
          "timestamp": "2026-02-20T21:23:26+01:00",
          "tree_id": "46143bcc5f1a05c8fdd0394039b008ab0653a717",
          "url": "https://github.com/WbaN314/battlesnake/commit/25965fa348f2d1597044f862e2b7e554debff13f"
        },
        "date": 1771619109333,
        "tool": "cargo",
        "benches": [
          {
            "name": "game::field::benchmarks::bench_next_state_with_basic_field",
            "value": 138.38,
            "range": "± 2.88",
            "unit": "ns/iter"
          },
          {
            "name": "game::field::benchmarks::bench_next_state_with_bit_field",
            "value": 116.66,
            "range": "± 15.13",
            "unit": "ns/iter"
          },
          {
            "name": "game::game_state::benchmarks::bench_next_state",
            "value": 138.6,
            "range": "± 1.93",
            "unit": "ns/iter"
          },
          {
            "name": "game::game_state::benchmarks::bench_possible_moves",
            "value": 85.05,
            "range": "± 1.07",
            "unit": "ns/iter"
          },
          {
            "name": "game::moves::benchmarks::bench_possible_moves_generate",
            "value": 150.39,
            "range": "± 1.73",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 674.98,
            "range": "± 6.17",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 137.11,
            "range": "± 1.23",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 124,
            "range": "± 1.19",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 84.84,
            "range": "± 1.76",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 117.67,
            "range": "± 1.20",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_possible_moves_generate",
            "value": 137.14,
            "range": "± 6.81",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 214.62,
            "range": "± 2.89",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 222.74,
            "range": "± 4.63",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}