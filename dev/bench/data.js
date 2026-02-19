window.BENCHMARK_DATA = {
  "lastUpdate": 1771525983713,
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
          "id": "bc4bf5fbe2e349c0d38b879aa54bd677f9a7f54e",
          "message": "Added alive status handling to all node types",
          "timestamp": "2025-01-02T23:29:58+01:00",
          "tree_id": "df20a40e85337936c52312a96e9ca4a59396f48f",
          "url": "https://github.com/WbaN314/battlesnake/commit/bc4bf5fbe2e349c0d38b879aa54bd677f9a7f54e"
        },
        "date": 1735857108915,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 854.63,
            "range": "± 19.08",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 145.32,
            "range": "± 9.49",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 126.87,
            "range": "± 0.87",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 19.6,
            "range": "± 0.36",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 77.1,
            "range": "± 1.16",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 82.52,
            "range": "± 0.46",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 220.13,
            "range": "± 48.05",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 232.94,
            "range": "± 4.68",
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
          "id": "0856e77961c34f6baa909a5ccc25c72cd7cab52c",
          "message": "Added relevant_snakes to DNodeStatistics\n\nIntended to be used to slim full calculation",
          "timestamp": "2025-01-05T13:59:22+01:00",
          "tree_id": "19d59a3332d62d4300424cb89092114cc39b7a53",
          "url": "https://github.com/WbaN314/battlesnake/commit/0856e77961c34f6baa909a5ccc25c72cd7cab52c"
        },
        "date": 1736082067416,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 687.97,
            "range": "± 6.09",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 143.59,
            "range": "± 3.08",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 127.53,
            "range": "± 1.63",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 19.62,
            "range": "± 0.29",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 76.6,
            "range": "± 3.79",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 82,
            "range": "± 1.57",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 223.7,
            "range": "± 4.87",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 209.25,
            "range": "± 13.41",
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
          "id": "df9d9b8c616d50c426591b8771f21dc454bc9893",
          "message": "added tree simulate bench",
          "timestamp": "2025-01-10T15:46:11+01:00",
          "tree_id": "b8e0fe332040292ae72d7231135c505c37d8e663",
          "url": "https://github.com/WbaN314/battlesnake/commit/df9d9b8c616d50c426591b8771f21dc454bc9893"
        },
        "date": 1736521428907,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 683.06,
            "range": "± 5.44",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 154.34,
            "range": "± 2.33",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 130.52,
            "range": "± 1.05",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 104.72,
            "range": "± 1.50",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 121.81,
            "range": "± 0.99",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 141.71,
            "range": "± 2.07",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 133019274.9,
            "range": "± 2661721.03",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 222,
            "range": "± 2.53",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 223.43,
            "range": "± 6.09",
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
          "id": "07cb8958a995f4be4308a7f89a3241272e00e97c",
          "message": "Added direction durations",
          "timestamp": "2025-01-17T10:07:27+01:00",
          "tree_id": "7cf0f3c51d04e6e4f852512e1d0a0abe970f26d0",
          "url": "https://github.com/WbaN314/battlesnake/commit/07cb8958a995f4be4308a7f89a3241272e00e97c"
        },
        "date": 1737104995055,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 698.38,
            "range": "± 7.74",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 144.57,
            "range": "± 2.26",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 131.14,
            "range": "± 1.37",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 95.78,
            "range": "± 1.53",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 111.8,
            "range": "± 1.20",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 138.19,
            "range": "± 3.89",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 136790822.1,
            "range": "± 2514344.06",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 220.26,
            "range": "± 5.06",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 231.81,
            "range": "± 3.57",
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
          "id": "64efd59a48d82e46bcc1ee2588522c64eb482321",
          "message": "Distribute simulation time equally over nodes now.\n\nNode timer gives node break, simulation timer gives simulation break.",
          "timestamp": "2025-01-18T14:59:19+01:00",
          "tree_id": "c71128a5ffde35b733e9ba9f3640f002fe04d702",
          "url": "https://github.com/WbaN314/battlesnake/commit/64efd59a48d82e46bcc1ee2588522c64eb482321"
        },
        "date": 1737208990138,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 706.02,
            "range": "± 13.35",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 143.64,
            "range": "± 1.63",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 132.02,
            "range": "± 1.94",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 94.23,
            "range": "± 2.37",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 112.09,
            "range": "± 1.64",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 137.59,
            "range": "± 2.90",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 169686546.8,
            "range": "± 2342378.03",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 221.39,
            "range": "± 2.70",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 230.49,
            "range": "± 1.64",
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
          "id": "f7207002bfb36f2b4daa81b250b794c54b886f91",
          "message": "Changed test as right is better due to increase in size",
          "timestamp": "2025-01-28T17:52:35+01:00",
          "tree_id": "6b9b0d34a223d27fbb437f012d7bd197741b3304",
          "url": "https://github.com/WbaN314/battlesnake/commit/f7207002bfb36f2b4daa81b250b794c54b886f91"
        },
        "date": 1738083310118,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 720.3,
            "range": "± 18.20",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 145.51,
            "range": "± 12.07",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 129.16,
            "range": "± 1.57",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 97.07,
            "range": "± 5.61",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 121.06,
            "range": "± 4.61",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 136.39,
            "range": "± 3.23",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 152092834.5,
            "range": "± 1669708.91",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 220.37,
            "range": "± 1.96",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 235.82,
            "range": "± 3.06",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "4fd7f757f6e2230c697702df0ed708ee38ce62ee",
          "message": "Improved approved directions selection",
          "timestamp": "2025-03-28T21:26:55+01:00",
          "tree_id": "90560b0873f89477c4802f0eb426e4c43fe316b7",
          "url": "https://github.com/WbaN314/battlesnake/commit/4fd7f757f6e2230c697702df0ed708ee38ce62ee"
        },
        "date": 1743193765119,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 662.94,
            "range": "± 29.41",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 150.06,
            "range": "± 2.92",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 130.14,
            "range": "± 1.36",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 91.19,
            "range": "± 2.36",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 127.93,
            "range": "± 1.46",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 144.38,
            "range": "± 3.35",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 158060535.4,
            "range": "± 822029.05",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 222.8,
            "range": "± 4.23",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 222.47,
            "range": "± 2.36",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "3c2d968fe6ba96ce740ebadfce6bf66b18ee773d",
          "message": "Improved resolution",
          "timestamp": "2025-03-29T21:19:29+01:00",
          "tree_id": "5516b6366dd2b051d6398932d6b203438d4bfdc0",
          "url": "https://github.com/WbaN314/battlesnake/commit/3c2d968fe6ba96ce740ebadfce6bf66b18ee773d"
        },
        "date": 1743279733428,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 659.68,
            "range": "± 33.92",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 149.01,
            "range": "± 4.92",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 130.98,
            "range": "± 1.01",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 97.67,
            "range": "± 1.65",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 129.43,
            "range": "± 1.47",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 147.36,
            "range": "± 5.13",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 172392757.2,
            "range": "± 3410446.19",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 218.83,
            "range": "± 2.37",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 219.14,
            "range": "± 2.37",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "5a8026894660b4eda202947c9c26e866f21de67f",
          "message": "Adapted situation",
          "timestamp": "2025-03-29T21:26:25+01:00",
          "tree_id": "6102b1fa5e6f963622c6f80c17a4e5b98d48dee4",
          "url": "https://github.com/WbaN314/battlesnake/commit/5a8026894660b4eda202947c9c26e866f21de67f"
        },
        "date": 1743280146799,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 643.51,
            "range": "± 3.28",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 149.21,
            "range": "± 59.41",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 131.05,
            "range": "± 0.72",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 105.88,
            "range": "± 1.75",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 130.17,
            "range": "± 1.78",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 146.01,
            "range": "± 1.34",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 170130799.7,
            "range": "± 1625986.65",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 218.74,
            "range": "± 2.75",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 218.04,
            "range": "± 3.83",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "d7da35fa5c6eefa16df465115765636cac38c82d",
          "message": "Adapted test",
          "timestamp": "2025-03-29T21:35:55+01:00",
          "tree_id": "c4d3a8d0c7a7af9457c55669602a45be6996c1ce",
          "url": "https://github.com/WbaN314/battlesnake/commit/d7da35fa5c6eefa16df465115765636cac38c82d"
        },
        "date": 1743280705367,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 665.32,
            "range": "± 15.20",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 149.08,
            "range": "± 2.04",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 131.14,
            "range": "± 3.05",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 97.59,
            "range": "± 1.76",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 130.87,
            "range": "± 2.68",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 146.07,
            "range": "± 2.96",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 151788967.6,
            "range": "± 1875042.93",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 218.98,
            "range": "± 3.32",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 218.8,
            "range": "± 4.35",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "82edb1d4b30afecd3d21ee75319f0f9f6963b42f",
          "message": "dead code",
          "timestamp": "2025-03-29T21:36:32+01:00",
          "tree_id": "89c8c20da1e7f22560b480f2c470dc30344ead78",
          "url": "https://github.com/WbaN314/battlesnake/commit/82edb1d4b30afecd3d21ee75319f0f9f6963b42f"
        },
        "date": 1743280749335,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 643.75,
            "range": "± 3.65",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 148.79,
            "range": "± 3.74",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 131.02,
            "range": "± 1.53",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 97.54,
            "range": "± 1.68",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 153.79,
            "range": "± 3.00",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 146.04,
            "range": "± 4.54",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 137205013.2,
            "range": "± 3047101.16",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 219.44,
            "range": "± 3.76",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 219.66,
            "range": "± 1.79",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "c9bbe42f069177d2d106f97e9ac4d931614fdcf6",
          "message": "better scripts",
          "timestamp": "2025-03-31T19:57:17+02:00",
          "tree_id": "7a1909abdbe9952408c456430b795aed75f5f35c",
          "url": "https://github.com/WbaN314/battlesnake/commit/c9bbe42f069177d2d106f97e9ac4d931614fdcf6"
        },
        "date": 1743443998825,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 644.37,
            "range": "± 36.03",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 150.72,
            "range": "± 3.72",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 130.01,
            "range": "± 1.55",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 97.27,
            "range": "± 2.37",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 129.48,
            "range": "± 2.82",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 145.69,
            "range": "± 3.04",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 169409412.3,
            "range": "± 2590525.77",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 217.81,
            "range": "± 2.23",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 219.45,
            "range": "± 3.17",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "dac32e9794eb627a51ed837d68a6a587aa00abba",
          "message": "Merge branch 'main' into depth_first",
          "timestamp": "2025-04-01T12:21:27+02:00",
          "tree_id": "f74ea1a2bbd8cd12c4a91c6183a55a5626405186",
          "url": "https://github.com/WbaN314/battlesnake/commit/dac32e9794eb627a51ed837d68a6a587aa00abba"
        },
        "date": 1743503043439,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 652.11,
            "range": "± 10.98",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 150.08,
            "range": "± 1.45",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 130.33,
            "range": "± 2.27",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 97.87,
            "range": "± 2.84",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 116.59,
            "range": "± 1.98",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 152.37,
            "range": "± 3.84",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 152871331.4,
            "range": "± 1637974.36",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 218.98,
            "range": "± 3.34",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 243.27,
            "range": "± 5.97",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "5d103a8244715b5323744993f307da8572eae5c7",
          "message": "fix test",
          "timestamp": "2025-04-01T12:22:40+02:00",
          "tree_id": "9e56a26e59b670b591d6536c9a9669362a40400d",
          "url": "https://github.com/WbaN314/battlesnake/commit/5d103a8244715b5323744993f307da8572eae5c7"
        },
        "date": 1743503113325,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 643.9,
            "range": "± 4.82",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 150.84,
            "range": "± 1.67",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 130.48,
            "range": "± 1.13",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 98.7,
            "range": "± 2.90",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 125.07,
            "range": "± 1.91",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 154.46,
            "range": "± 2.32",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 155840573.6,
            "range": "± 2436481.80",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 219.2,
            "range": "± 3.15",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 239.38,
            "range": "± 4.21",
            "unit": "ns/iter"
          }
        ]
      },
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
          "distinct": false,
          "id": "5d103a8244715b5323744993f307da8572eae5c7",
          "message": "fix test",
          "timestamp": "2025-04-01T12:22:40+02:00",
          "tree_id": "9e56a26e59b670b591d6536c9a9669362a40400d",
          "url": "https://github.com/WbaN314/battlesnake/commit/5d103a8244715b5323744993f307da8572eae5c7"
        },
        "date": 1743503173062,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 673.7,
            "range": "± 9.88",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 150.69,
            "range": "± 2.57",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 130.31,
            "range": "± 1.42",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 97.74,
            "range": "± 2.40",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 116.36,
            "range": "± 2.40",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 151.76,
            "range": "± 1.30",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 155129628.1,
            "range": "± 3515050.11",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 218.55,
            "range": "± 3.81",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 243.43,
            "range": "± 9.45",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "833548b3acbed7f60a5b272a6da40c78e317849f",
          "message": "changed docker image",
          "timestamp": "2025-04-01T13:17:58+02:00",
          "tree_id": "56bc77dfe212e0bb4345623f1d23cb91a19a1ade",
          "url": "https://github.com/WbaN314/battlesnake/commit/833548b3acbed7f60a5b272a6da40c78e317849f"
        },
        "date": 1743506439553,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 644.28,
            "range": "± 17.68",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 150.53,
            "range": "± 7.48",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 130.23,
            "range": "± 1.45",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 97.65,
            "range": "± 2.25",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 116.37,
            "range": "± 2.32",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 147.42,
            "range": "± 3.62",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 169499040.3,
            "range": "± 2551987.42",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 218.56,
            "range": "± 2.21",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 239.65,
            "range": "± 12.37",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "1ddff96893b8516b463db517db9cce18ce56c6cd",
          "message": "Added d_scores",
          "timestamp": "2025-04-02T17:31:47+02:00",
          "tree_id": "2b1c6fcfb858ca7d699f95fffc56f505bd152548",
          "url": "https://github.com/WbaN314/battlesnake/commit/1ddff96893b8516b463db517db9cce18ce56c6cd"
        },
        "date": 1743608064451,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 649.83,
            "range": "± 9.46",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 149.61,
            "range": "± 1.70",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 130.6,
            "range": "± 0.80",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 97.38,
            "range": "± 2.53",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 115.56,
            "range": "± 1.50",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 152.95,
            "range": "± 3.23",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 154768010.3,
            "range": "± 1488643.99",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 222.99,
            "range": "± 2.89",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 217.3,
            "range": "± 3.93",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "4bdf4cb353de60983217edc015dd7a3395b28072",
          "message": "improved benchmark script",
          "timestamp": "2025-04-04T12:10:02+02:00",
          "tree_id": "3e6656890a6de51c4173f03f4f4d0818eb057f01",
          "url": "https://github.com/WbaN314/battlesnake/commit/4bdf4cb353de60983217edc015dd7a3395b28072"
        },
        "date": 1743761553781,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 644.93,
            "range": "± 5.31",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 145.15,
            "range": "± 2.63",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 137.13,
            "range": "± 1.28",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 97.23,
            "range": "± 1.80",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 123.19,
            "range": "± 1.62",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 147.15,
            "range": "± 2.33",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 153729177.3,
            "range": "± 2109860.54",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 221.22,
            "range": "± 3.51",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 245.24,
            "range": "± 6.04",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "ed8c6aa9d3f07b6d66111fbc00a45d1cc8af761b",
          "message": "removed prints",
          "timestamp": "2025-04-04T17:08:35+02:00",
          "tree_id": "5627397ef8e6e5a8701fb50d6cba0955e0b044a2",
          "url": "https://github.com/WbaN314/battlesnake/commit/ed8c6aa9d3f07b6d66111fbc00a45d1cc8af761b"
        },
        "date": 1743779465565,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 645.08,
            "range": "± 5.44",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 145.55,
            "range": "± 1.56",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 138.28,
            "range": "± 0.79",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 97.36,
            "range": "± 2.75",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 123.8,
            "range": "± 1.81",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 149.06,
            "range": "± 2.90",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 152989494.3,
            "range": "± 1070959.94",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 221.2,
            "range": "± 2.26",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 220.9,
            "range": "± 10.94",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "e6b3aedde0ebeb1d9714d216250895d5945c4bc1",
          "message": "Additional capture test",
          "timestamp": "2025-04-06T11:39:49+02:00",
          "tree_id": "7e3942e5f8ed401abbc142ff7bd506edb3a82d4c",
          "url": "https://github.com/WbaN314/battlesnake/commit/e6b3aedde0ebeb1d9714d216250895d5945c4bc1"
        },
        "date": 1743932534040,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 645.23,
            "range": "± 9.56",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 144.27,
            "range": "± 1.33",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 137.62,
            "range": "± 1.62",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 97.3,
            "range": "± 6.31",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 124.11,
            "range": "± 16.92",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 145.81,
            "range": "± 1.74",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 150541068.7,
            "range": "± 1115076.12",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 219.71,
            "range": "± 2.95",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 230.03,
            "range": "± 4.23",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "ae88e81f3620acdf9ebbc867386d22608e7a8cc9",
          "message": "Added some removal of unnecessary states",
          "timestamp": "2025-04-06T17:38:53+02:00",
          "tree_id": "f7cf61b0ab23e9927f856954b33aec8c78186b5a",
          "url": "https://github.com/WbaN314/battlesnake/commit/ae88e81f3620acdf9ebbc867386d22608e7a8cc9"
        },
        "date": 1743954088961,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 644.7,
            "range": "± 9.15",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 144.92,
            "range": "± 2.39",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 137.38,
            "range": "± 1.53",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 96.96,
            "range": "± 4.55",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 123.43,
            "range": "± 1.25",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 146.39,
            "range": "± 2.14",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 176407827.5,
            "range": "± 3369381.80",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 216.69,
            "range": "± 1.66",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 220.88,
            "range": "± 3.69",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "71c4951ace65d153ee5d221a0af208886b6764c3",
          "message": "maybe working sparse simulation",
          "timestamp": "2025-04-07T14:58:56+02:00",
          "tree_id": "a766325ea0c6d1dacd7857b1c6621b818bcfbba7",
          "url": "https://github.com/WbaN314/battlesnake/commit/71c4951ace65d153ee5d221a0af208886b6764c3"
        },
        "date": 1744030891984,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 669.82,
            "range": "± 31.40",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 143.48,
            "range": "± 15.87",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 138.26,
            "range": "± 1.16",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 96.81,
            "range": "± 3.39",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 124.11,
            "range": "± 2.37",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 145.2,
            "range": "± 1.75",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 168421430.5,
            "range": "± 2285153.51",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 218.92,
            "range": "± 2.68",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 226.27,
            "range": "± 5.57",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "a1e23275c0875187bade7e8a9609090b2e66faa3",
          "message": "sparse implemented",
          "timestamp": "2025-04-07T20:39:26+02:00",
          "tree_id": "1df642935ee14d2de12ae388e6146d10739bbbd0",
          "url": "https://github.com/WbaN314/battlesnake/commit/a1e23275c0875187bade7e8a9609090b2e66faa3"
        },
        "date": 1744051317560,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 645.86,
            "range": "± 27.39",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 143.62,
            "range": "± 1.59",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 137.59,
            "range": "± 1.55",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 97.07,
            "range": "± 2.77",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 133.67,
            "range": "± 24.43",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 148.54,
            "range": "± 3.19",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 172370889.1,
            "range": "± 4525971.36",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 219.72,
            "range": "± 3.06",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 226.94,
            "range": "± 4.13",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "779211b3f1a01f0953e2165beae71fce3bd14e64",
          "message": "naming",
          "timestamp": "2025-04-10T14:23:53+02:00",
          "tree_id": "50e3d74e41790d050c648ecea47ccd4556eaf8f9",
          "url": "https://github.com/WbaN314/battlesnake/commit/779211b3f1a01f0953e2165beae71fce3bd14e64"
        },
        "date": 1744287981528,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 683.09,
            "range": "± 22.78",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 136.34,
            "range": "± 1.78",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 124.99,
            "range": "± 1.77",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 92.74,
            "range": "± 1.99",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 123.63,
            "range": "± 2.96",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 142.68,
            "range": "± 6.52",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 150198064.8,
            "range": "± 10912639.29",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 216.25,
            "range": "± 2.95",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 230.28,
            "range": "± 3.32",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "ac35d67a992211de9148bdb9e6f65e6f43db53ae",
          "message": "Finetuning simulation parameters",
          "timestamp": "2025-04-10T14:44:57+02:00",
          "tree_id": "842892401e82f6ea9c439320392cb33e5bd9c736",
          "url": "https://github.com/WbaN314/battlesnake/commit/ac35d67a992211de9148bdb9e6f65e6f43db53ae"
        },
        "date": 1744289244253,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 645.29,
            "range": "± 10.03",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 137.15,
            "range": "± 1.21",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 124.76,
            "range": "± 1.78",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 93.86,
            "range": "± 2.26",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 122.39,
            "range": "± 2.38",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 149.17,
            "range": "± 4.22",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 150908901.4,
            "range": "± 2235870.53",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 217.04,
            "range": "± 3.72",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 231.27,
            "range": "± 4.27",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "1a849923b05b35c1d9b792d6d9096258d8ca4d83",
          "message": "working on sparse node quick exploaration",
          "timestamp": "2025-04-19T21:31:21+02:00",
          "tree_id": "5b64340853fa258927ba29faa8498a76a5af44b2",
          "url": "https://github.com/WbaN314/battlesnake/commit/1a849923b05b35c1d9b792d6d9096258d8ca4d83"
        },
        "date": 1745091237340,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 677.12,
            "range": "± 12.62",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 136.08,
            "range": "± 1.79",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 124.21,
            "range": "± 2.82",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 86.78,
            "range": "± 1.90",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 107.31,
            "range": "± 1.69",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 134.87,
            "range": "± 5.76",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 172347754.3,
            "range": "± 1958254.84",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 216.2,
            "range": "± 3.75",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 229.87,
            "range": "± 5.45",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "dd364f94a7067697089687024eb9c9c21a96d7c2",
          "message": "Switched to env logger",
          "timestamp": "2025-04-19T22:38:23+02:00",
          "tree_id": "f959f7f0153de8d33706d357c71f9e2f86ea48dc",
          "url": "https://github.com/WbaN314/battlesnake/commit/dd364f94a7067697089687024eb9c9c21a96d7c2"
        },
        "date": 1745095243304,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 721.38,
            "range": "± 21.81",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 135.55,
            "range": "± 1.43",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 124.21,
            "range": "± 5.35",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 88.45,
            "range": "± 2.08",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 107.29,
            "range": "± 1.30",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 138.39,
            "range": "± 6.33",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 151864700.3,
            "range": "± 11158281.60",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 210.27,
            "range": "± 13.39",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 218.7,
            "range": "± 11.54",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "1c89905ba0643f55421a0f6583009c9913d77342",
          "message": "Added fastend",
          "timestamp": "2025-04-20T16:50:18+02:00",
          "tree_id": "0364827e3dbe5b3c6e49e87ccbd7a1f3cf6e1b92",
          "url": "https://github.com/WbaN314/battlesnake/commit/1c89905ba0643f55421a0f6583009c9913d77342"
        },
        "date": 1745160761498,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 680.36,
            "range": "± 9.39",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 136.27,
            "range": "± 0.98",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 125.3,
            "range": "± 2.79",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 88.16,
            "range": "± 1.31",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 117,
            "range": "± 1.15",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 135.42,
            "range": "± 2.62",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 148601838.9,
            "range": "± 1364502.55",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 217.09,
            "range": "± 2.07",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 242.05,
            "range": "± 5.56",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "72305cbf4c4b1a2c394f64cd2bce8aee64314bde",
          "message": "Small renaming",
          "timestamp": "2025-04-20T21:14:04+02:00",
          "tree_id": "8dccb7ecd00c05cb4e4bad53369c2cc5115bd058",
          "url": "https://github.com/WbaN314/battlesnake/commit/72305cbf4c4b1a2c394f64cd2bce8aee64314bde"
        },
        "date": 1745176593913,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 713.51,
            "range": "± 28.79",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 134.58,
            "range": "± 1.75",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 131.34,
            "range": "± 1.72",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 86.5,
            "range": "± 1.31",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 116.52,
            "range": "± 2.21",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 134.34,
            "range": "± 22.54",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 148890707.2,
            "range": "± 7423676.90",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 213.78,
            "range": "± 1.36",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 229.97,
            "range": "± 3.83",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "6d3ee27f17e9c1da10c7e294083f8b212420e5b1",
          "message": "Removed NodeStatus TimedOut",
          "timestamp": "2025-04-20T22:47:47+02:00",
          "tree_id": "e4cfe717b438a7c2871a5d0aad3db814cbe96df3",
          "url": "https://github.com/WbaN314/battlesnake/commit/6d3ee27f17e9c1da10c7e294083f8b212420e5b1"
        },
        "date": 1745182413736,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 722.52,
            "range": "± 17.99",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 136.39,
            "range": "± 1.14",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 124.48,
            "range": "± 0.89",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 87.43,
            "range": "± 2.84",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 118.27,
            "range": "± 1.07",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 138.09,
            "range": "± 2.25",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 146610049.2,
            "range": "± 626027.53",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 213.61,
            "range": "± 2.06",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 218.78,
            "range": "± 5.83",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "fd6685082f4752bedef9651359a6dec3ff7f3584",
          "message": "Only Alive Child States",
          "timestamp": "2025-04-20T23:23:20+02:00",
          "tree_id": "cc6d6372660f659077939ed12d7e04567569de14",
          "url": "https://github.com/WbaN314/battlesnake/commit/fd6685082f4752bedef9651359a6dec3ff7f3584"
        },
        "date": 1745184351370,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 679.21,
            "range": "± 9.25",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 134.12,
            "range": "± 1.52",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 124.36,
            "range": "± 3.06",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 87.08,
            "range": "± 3.94",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 128.47,
            "range": "± 5.04",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 137.96,
            "range": "± 2.72",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 138372787.4,
            "range": "± 1847744.06",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 209.02,
            "range": "± 3.17",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 223.58,
            "range": "± 9.94",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "791f821ac60e1723f5e0746a1dbf307b00c3b0c2",
          "message": "Improved edge case handling when simulation finishes or is immediately dead ended",
          "timestamp": "2025-04-28T16:58:48+02:00",
          "tree_id": "9d8cb611e8d17e75cfbf03c7e38c1640e95b159f",
          "url": "https://github.com/WbaN314/battlesnake/commit/791f821ac60e1723f5e0746a1dbf307b00c3b0c2"
        },
        "date": 1745853569377,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 679.2,
            "range": "± 13.21",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 134.71,
            "range": "± 1.09",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 124.75,
            "range": "± 1.75",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 86.62,
            "range": "± 2.53",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 120.73,
            "range": "± 1.31",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 136.57,
            "range": "± 1.97",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 172268372.2,
            "range": "± 2481971.71",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 223.42,
            "range": "± 2.08",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 241.48,
            "range": "± 3.88",
            "unit": "ns/iter"
          }
        ]
      },
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
          "distinct": false,
          "id": "791f821ac60e1723f5e0746a1dbf307b00c3b0c2",
          "message": "Improved edge case handling when simulation finishes or is immediately dead ended",
          "timestamp": "2025-04-28T16:58:48+02:00",
          "tree_id": "9d8cb611e8d17e75cfbf03c7e38c1640e95b159f",
          "url": "https://github.com/WbaN314/battlesnake/commit/791f821ac60e1723f5e0746a1dbf307b00c3b0c2"
        },
        "date": 1745853683901,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 677.79,
            "range": "± 5.67",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 134.79,
            "range": "± 1.04",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 124.38,
            "range": "± 3.42",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 86.63,
            "range": "± 1.50",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 118.71,
            "range": "± 1.35",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 136.53,
            "range": "± 3.19",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 170935130.2,
            "range": "± 2758805.51",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 221.42,
            "range": "± 2.23",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 241.76,
            "range": "± 3.55",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "e43c1acabc803466e86ae0abfa166ca907a2c43a",
          "message": "Improved logging",
          "timestamp": "2025-04-28T17:54:32+02:00",
          "tree_id": "787d03f2cbf79d0e80e0d6113d865c1e1c29e461",
          "url": "https://github.com/WbaN314/battlesnake/commit/e43c1acabc803466e86ae0abfa166ca907a2c43a"
        },
        "date": 1745855820749,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 678.97,
            "range": "± 7.65",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 134.76,
            "range": "± 1.98",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 124.3,
            "range": "± 1.37",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 86.78,
            "range": "± 2.82",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 118.66,
            "range": "± 1.06",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 136.68,
            "range": "± 22.05",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 140241262.9,
            "range": "± 2344667.63",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 223,
            "range": "± 1.95",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 242.57,
            "range": "± 8.98",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "8b4532226b5ea280a0351847191d31b0d9ea77c6",
          "message": "changed log levels",
          "timestamp": "2025-05-06T17:09:13+02:00",
          "tree_id": "06bfc728c546002ff39aee69c652c889b2aef92c",
          "url": "https://github.com/WbaN314/battlesnake/commit/8b4532226b5ea280a0351847191d31b0d9ea77c6"
        },
        "date": 1746544307843,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 677.96,
            "range": "± 4.00",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 136.6,
            "range": "± 0.87",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 125.51,
            "range": "± 3.30",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 87.67,
            "range": "± 2.21",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 127.13,
            "range": "± 1.08",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 138,
            "range": "± 2.32",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 135955781.2,
            "range": "± 3838937.54",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 219.38,
            "range": "± 3.26",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 230.72,
            "range": "± 4.00",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "f272a60e5ddbd8a358acbca6abfadfc2f66262a6",
          "message": "bugfix with simulations partly reaching max depth and other directions being discarded",
          "timestamp": "2025-05-07T09:35:13+02:00",
          "tree_id": "697e036e29f3a2f29fa0611d905e8d4f782de706",
          "url": "https://github.com/WbaN314/battlesnake/commit/f272a60e5ddbd8a358acbca6abfadfc2f66262a6"
        },
        "date": 1746603453198,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 720.03,
            "range": "± 10.10",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 136.65,
            "range": "± 2.12",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 124.51,
            "range": "± 1.53",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 93.69,
            "range": "± 1.42",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 124.71,
            "range": "± 1.44",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 137.12,
            "range": "± 2.06",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 135380894.7,
            "range": "± 2848943.97",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 218.77,
            "range": "± 2.05",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 241.75,
            "range": "± 6.32",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "1b54edd26a7f381384cc15dbc6f85a5413cf924d",
          "message": "Fixing tests",
          "timestamp": "2025-06-17T16:11:07+02:00",
          "tree_id": "8ea47595f38948846f7a88850663080a866baa00",
          "url": "https://github.com/WbaN314/battlesnake/commit/1b54edd26a7f381384cc15dbc6f85a5413cf924d"
        },
        "date": 1750170160857,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 679.6,
            "range": "± 8.70",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 141.35,
            "range": "± 2.28",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 127.99,
            "range": "± 2.64",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 91.28,
            "range": "± 5.31",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 117.05,
            "range": "± 1.64",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 137.95,
            "range": "± 3.74",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 152100738.2,
            "range": "± 8376754.57",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 218.59,
            "range": "± 9.83",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 218.23,
            "range": "± 13.08",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "1967114a3ff79ed3ff1716a97279a632807253ac",
          "message": "improved logging",
          "timestamp": "2025-06-17T16:34:27+02:00",
          "tree_id": "2515801cf0b592d71b5b183aac191b95681f0ca9",
          "url": "https://github.com/WbaN314/battlesnake/commit/1967114a3ff79ed3ff1716a97279a632807253ac"
        },
        "date": 1750171017812,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 682.16,
            "range": "± 16.70",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 140.77,
            "range": "± 32.14",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 126.23,
            "range": "± 2.07",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 86.62,
            "range": "± 1.89",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 116.02,
            "range": "± 1.40",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 137.83,
            "range": "± 25.09",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 156280298.4,
            "range": "± 4443371.51",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 216.63,
            "range": "± 4.15",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 226.85,
            "range": "± 2.60",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "f98c89fad84a759e1e3e093b67c07d48cd18f24a",
          "message": "Adapted quick hash to avoid wrong alive snake counts when it activates",
          "timestamp": "2025-06-17T20:27:15+02:00",
          "tree_id": "8ac6e702b4fb58ed8cc180a3951ab92f5154601b",
          "url": "https://github.com/WbaN314/battlesnake/commit/f98c89fad84a759e1e3e093b67c07d48cd18f24a"
        },
        "date": 1750184989483,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 680.75,
            "range": "± 15.88",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 136.34,
            "range": "± 1.38",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 125.14,
            "range": "± 1.64",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 87.23,
            "range": "± 2.16",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 125.44,
            "range": "± 0.92",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 141.92,
            "range": "± 2.37",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 173165861.6,
            "range": "± 2479323.12",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 215.15,
            "range": "± 2.19",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 224.22,
            "range": "± 7.64",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "617c3282a94e00205307ee79a892963a40d5d94d",
          "message": "Fixing linter errors",
          "timestamp": "2026-02-19T11:28:19+01:00",
          "tree_id": "bb71df9892e8f849cd6b3e9d5a03a79da092515c",
          "url": "https://github.com/WbaN314/battlesnake/commit/617c3282a94e00205307ee79a892963a40d5d94d"
        },
        "date": 1771497050447,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 686.34,
            "range": "± 20.93",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 138.39,
            "range": "± 2.86",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 123.19,
            "range": "± 1.18",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 84.13,
            "range": "± 2.17",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 116.3,
            "range": "± 1.56",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 129.73,
            "range": "± 3.7",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 168903500.2,
            "range": "± 2034754.16",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 212.84,
            "range": "± 3.41",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 206.61,
            "range": "± 4.74",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "9f0121cb35b3d27dfa484e3e865163d12474d580",
          "message": "moving to 2024 edition rust",
          "timestamp": "2026-02-19T11:39:48+01:00",
          "tree_id": "d7b059a1e2cbf7c311a6118bf97aac698d3d830a",
          "url": "https://github.com/WbaN314/battlesnake/commit/9f0121cb35b3d27dfa484e3e865163d12474d580"
        },
        "date": 1771497739557,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 698.68,
            "range": "± 22.01",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 137.21,
            "range": "± 2.2",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 123.61,
            "range": "± 1.52",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 84.81,
            "range": "± 1.76",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 116.63,
            "range": "± 1.49",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 133.89,
            "range": "± 4.12",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 173724289.5,
            "range": "± 3458701.68",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 209.67,
            "range": "± 2.7",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 201.62,
            "range": "± 5.57",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "d6680d231f3d86a7af35b6ea131827afac30ba68",
          "message": "correct lib imports",
          "timestamp": "2026-02-19T19:30:00+01:00",
          "tree_id": "40000355c3c529d7668f75ccee545d72be3bfc4d",
          "url": "https://github.com/WbaN314/battlesnake/commit/d6680d231f3d86a7af35b6ea131827afac30ba68"
        },
        "date": 1771525982638,
        "tool": "cargo",
        "benches": [
          {
            "name": "game::field::benchmarks::bench_next_state_with_basic_field",
            "value": 138.32,
            "range": "± 2.22",
            "unit": "ns/iter"
          },
          {
            "name": "game::field::benchmarks::bench_next_state_with_bit_field",
            "value": 117.08,
            "range": "± 13.60",
            "unit": "ns/iter"
          },
          {
            "name": "game::game_state::benchmarks::bench_next_state",
            "value": 133.48,
            "range": "± 0.71",
            "unit": "ns/iter"
          },
          {
            "name": "game::game_state::benchmarks::bench_possible_moves",
            "value": 88.65,
            "range": "± 1.50",
            "unit": "ns/iter"
          },
          {
            "name": "game::moves::benchmarks::bench_generate",
            "value": 150.79,
            "range": "± 1.70",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_move_reachable",
            "value": 679.43,
            "range": "± 4.85",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_fast",
            "value": 137.56,
            "range": "± 0.92",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_next_state_slow",
            "value": 123.19,
            "range": "± 1.45",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_possible_moves",
            "value": 84.57,
            "range": "± 1.56",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_game_state::tests::bench_scope_moves",
            "value": 133.94,
            "range": "± 2.82",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::game::d_moves_set::tests::bench_generate",
            "value": 131.68,
            "range": "± 2.72",
            "unit": "ns/iter"
          },
          {
            "name": "logic::depth_first::simulation::d_tree::tests::bench_tree_simulate",
            "value": 168691979.1,
            "range": "± 1162397.02",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_next_state",
            "value": 215.62,
            "range": "± 2.63",
            "unit": "ns/iter"
          },
          {
            "name": "logic::legacy::shared::e_game_state::tests::bench_possible_moves",
            "value": 202.45,
            "range": "± 3.78",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}