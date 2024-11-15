window.BENCHMARK_DATA = {
  "lastUpdate": 1731680060494,
  "repoUrl": "https://github.com/WbaN314/battlesnake",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "email": "sascha.stoll@sap.com",
            "name": "sascha",
            "username": "SaschaAtWork"
          },
          "committer": {
            "email": "sascha.stoll@sap.com",
            "name": "sascha",
            "username": "SaschaAtWork"
          },
          "distinct": true,
          "id": "e508c91c7bd846593af70483727f5eddbeb64c68",
          "message": "benchmark test",
          "timestamp": "2024-11-15T14:56:48+01:00",
          "tree_id": "51b70df844cf15aeebb77f59934f06f257a6ce86",
          "url": "https://github.com/WbaN314/battlesnake/commit/e508c91c7bd846593af70483727f5eddbeb64c68"
        },
        "date": 1731680059879,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 1086.09,
            "range": "± 66.50",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 141.37,
            "range": "± 2.23",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 203.57,
            "range": "± 4.79",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 15.78,
            "range": "± 0.27",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 54.75,
            "range": "± 1.04",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_scope_node_1_up",
            "value": 1483.01,
            "range": "± 27.61",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_scope_node_3_up",
            "value": 4757.39,
            "range": "± 57.62",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_simulate_node_1_up",
            "value": 8850.92,
            "range": "± 2623.30",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_simulate_node_3_up",
            "value": 2808748,
            "range": "± 33883.68",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 222.95,
            "range": "± 4.25",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 142.93,
            "range": "± 5.06",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}