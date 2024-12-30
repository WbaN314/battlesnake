window.BENCHMARK_DATA = {
  "lastUpdate": 1735578287225,
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
      },
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
          "id": "a624dc02ed3f2eb6761d87ffdcd801a2253f9c90",
          "message": "Changed readme",
          "timestamp": "2024-11-15T15:26:45+01:00",
          "tree_id": "be2ef9600235dc27f7641110a253436a368b21de",
          "url": "https://github.com/WbaN314/battlesnake/commit/a624dc02ed3f2eb6761d87ffdcd801a2253f9c90"
        },
        "date": 1731680900601,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 1138.91,
            "range": "± 673.56",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 141.24,
            "range": "± 1.52",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 203.04,
            "range": "± 2.66",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 15.77,
            "range": "± 0.20",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 54.46,
            "range": "± 1.10",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_scope_node_1_up",
            "value": 1479.7,
            "range": "± 25.46",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_scope_node_3_up",
            "value": 4770.49,
            "range": "± 75.88",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_simulate_node_1_up",
            "value": 8694.71,
            "range": "± 267.52",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_simulate_node_3_up",
            "value": 2794231.1,
            "range": "± 34619.15",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 223.14,
            "range": "± 3.41",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 142.97,
            "range": "± 4.42",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "21e39814368b2d93815dbfc45cc1395bed57c478",
          "message": "added check_dead_end",
          "timestamp": "2024-11-29T23:30:47+01:00",
          "tree_id": "30c61f2c79ff722cce60cac875d3f762c16c8ac1",
          "url": "https://github.com/WbaN314/battlesnake/commit/21e39814368b2d93815dbfc45cc1395bed57c478"
        },
        "date": 1732919551469,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_check_dead_end",
            "value": 300.64,
            "range": "± 3.60",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 980.36,
            "range": "± 80.19",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 139.06,
            "range": "± 1.50",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 135.22,
            "range": "± 2.00",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 15.8,
            "range": "± 0.33",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 55.89,
            "range": "± 0.62",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_scope_node_1_up",
            "value": 1435.89,
            "range": "± 16.95",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_scope_node_3_up",
            "value": 4481.87,
            "range": "± 95.37",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_simulate_node_1_up",
            "value": 8586.3,
            "range": "± 69.05",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_simulate_node_3_up",
            "value": 2761586.8,
            "range": "± 63112.56",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 220.92,
            "range": "± 4.21",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 154.23,
            "range": "± 3.24",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "45f8b5d47ed96a71c383cdde9ba5d3b39b52d225",
          "message": "Working on pessimistic node",
          "timestamp": "2024-12-25T19:29:13+01:00",
          "tree_id": "b97206e1fdf2588a10f78514627870f2c31bbd82",
          "url": "https://github.com/WbaN314/battlesnake/commit/45f8b5d47ed96a71c383cdde9ba5d3b39b52d225"
        },
        "date": 1735151458693,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 845.52,
            "range": "± 24.34",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 143.1,
            "range": "± 16.75",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 125.94,
            "range": "± 0.68",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 19.74,
            "range": "± 0.15",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 76.5,
            "range": "± 0.88",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 73.36,
            "range": "± 0.56",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 223.35,
            "range": "± 10.96",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 233.21,
            "range": "± 39.33",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "b7ddbee7b2d47dd0cbe93a12edddb154f9a21cb6",
          "message": "Some statistics",
          "timestamp": "2024-12-27T21:18:10+01:00",
          "tree_id": "318499511d770b03f95fe37643ca43ffd11c903a",
          "url": "https://github.com/WbaN314/battlesnake/commit/b7ddbee7b2d47dd0cbe93a12edddb154f9a21cb6"
        },
        "date": 1735330792907,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 844.36,
            "range": "± 43.14",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 142.91,
            "range": "± 2.53",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 127.04,
            "range": "± 0.90",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 19.62,
            "range": "± 0.20",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 80.99,
            "range": "± 1.49",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 81.05,
            "range": "± 0.95",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 224.23,
            "range": "± 3.64",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 221.36,
            "range": "± 13.00",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "23e8287c56224c694ad47daa692a226e4afaa309",
          "message": "Bugfix",
          "timestamp": "2024-12-30T18:03:01+01:00",
          "tree_id": "33e370250e04dfedb1c781d0f9628dee29f8840c",
          "url": "https://github.com/WbaN314/battlesnake/commit/23e8287c56224c694ad47daa692a226e4afaa309"
        },
        "date": 1735578286931,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 840.32,
            "range": "± 17.93",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 142.24,
            "range": "± 2.77",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 126.15,
            "range": "± 0.92",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 19.62,
            "range": "± 0.30",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 77.03,
            "range": "± 0.59",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 82.55,
            "range": "± 1.48",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 219.58,
            "range": "± 2.02",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 207.96,
            "range": "± 5.54",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}