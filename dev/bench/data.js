window.BENCHMARK_DATA = {
  "lastUpdate": 1743279734401,
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
      }
    ]
  }
}