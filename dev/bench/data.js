window.BENCHMARK_DATA = {
  "lastUpdate": 1781102917856,
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
          "id": "a470e2f769d531bfe387912f7e2bcdbf1bf984dc",
          "message": "Starting with clean benchmarks",
          "timestamp": "2026-02-21T12:20:40+01:00",
          "tree_id": "fa991a2fa3e539fbe9ee712fe69264bc96bb7603",
          "url": "https://github.com/WbaN314/battlesnake/commit/a470e2f769d531bfe387912f7e2bcdbf1bf984dc"
        },
        "date": 1771672935068,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 324.96,
            "range": "± 16.78",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 10.25,
            "range": "± 0.03",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 23.59,
            "range": "± 0.08",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.06,
            "range": "± 0.06",
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
          "id": "68cb283f5beb4ffa4639a513a4fcc6996b98b794",
          "message": "Benchmarks for moves",
          "timestamp": "2026-02-21T16:16:58+01:00",
          "tree_id": "98193e3e8698192940bed4b5f674423cc3f431b9",
          "url": "https://github.com/WbaN314/battlesnake/commit/68cb283f5beb4ffa4639a513a4fcc6996b98b794"
        },
        "date": 1771687112213,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 303.41,
            "range": "± 5.01",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 10.25,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 23.59,
            "range": "± 0.2",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.1,
            "range": "± 0.08",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_generate_and_iterate",
            "value": 430.22,
            "range": "± 9.42",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 215.94,
            "range": "± 3.75",
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
          "id": "2456dbf5c3e6d2b1969e38c63896570c0150143b",
          "message": "Changed moves logic to be simpler and removed edge case handling",
          "timestamp": "2026-02-23T12:02:36+01:00",
          "tree_id": "b93c9f1eb01c74794150266dd29e1750fc0fbcf7",
          "url": "https://github.com/WbaN314/battlesnake/commit/2456dbf5c3e6d2b1969e38c63896570c0150143b"
        },
        "date": 1771844656812,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 391.5,
            "range": "± 48.89",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 10.25,
            "range": "± 0.06",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 23.6,
            "range": "± 0.08",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.07,
            "range": "± 0.1",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_generate_and_iterate",
            "value": 437.61,
            "range": "± 11.25",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 789.57,
            "range": "± 18.37",
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
          "id": "52dff8bad3015cc1f25bc8bf9fe5b04ebb273b54",
          "message": "Adding 2026 branch to benchmark",
          "timestamp": "2026-02-23T12:05:08+01:00",
          "tree_id": "523b73d6d8b89827ea67f4b9684f267914bf9b2b",
          "url": "https://github.com/WbaN314/battlesnake/commit/52dff8bad3015cc1f25bc8bf9fe5b04ebb273b54"
        },
        "date": 1771844807440,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 237.38,
            "range": "± 0.63",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 7.48,
            "range": "± 0.04",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 23.02,
            "range": "± 0.1",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.61,
            "range": "± 0.05",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_generate_and_iterate",
            "value": 401.48,
            "range": "± 5.31",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 626.1,
            "range": "± 6.48",
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
          "id": "2093f48d7a4331a2e4e659f6acc44747cc8918b4",
          "message": "Optimised pregenerate performance",
          "timestamp": "2026-02-23T13:58:49+01:00",
          "tree_id": "74020cc92cc63ff99bf44d71de2a3f30849eb613",
          "url": "https://github.com/WbaN314/battlesnake/commit/2093f48d7a4331a2e4e659f6acc44747cc8918b4"
        },
        "date": 1771851626428,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 399.27,
            "range": "± 2.72",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 10.25,
            "range": "± 0.03",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 23.6,
            "range": "± 0.11",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.1,
            "range": "± 0.09",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_generate_and_iterate",
            "value": 444.17,
            "range": "± 8.38",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 235.27,
            "range": "± 2.59",
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
          "id": "b42abdeacca8a1fc287280b0cdaa35245ebb3ca1",
          "message": "Fixed bugs, cleaned up gamestate",
          "timestamp": "2026-03-06T11:14:59+01:00",
          "tree_id": "10ec610ba7b3fbd986af182ba06665fdc2ce47a9",
          "url": "https://github.com/WbaN314/battlesnake/commit/b42abdeacca8a1fc287280b0cdaa35245ebb3ca1"
        },
        "date": 1772792206923,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 393.73,
            "range": "± 94.63",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 10.26,
            "range": "± 0.04",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 23.63,
            "range": "± 0.07",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.1,
            "range": "± 0.23",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_local_environment_hash",
            "value": 300.71,
            "range": "± 8.20",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_heads",
            "value": 177.48,
            "range": "± 4.76",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_tails",
            "value": 130.11,
            "range": "± 0.48",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_next_state",
            "value": 203.12,
            "range": "± 2.04",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_generate_and_iterate",
            "value": 432.8,
            "range": "± 8.74",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 230.21,
            "range": "± 3.16",
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
          "id": "c24cad9c28c98334a7e7484abeefa023cb070628",
          "message": "Using a faster hash algorithm",
          "timestamp": "2026-03-09T09:52:54+01:00",
          "tree_id": "8ad6d7456d54d29f0f5f91739c6c5222f5375f45",
          "url": "https://github.com/WbaN314/battlesnake/commit/c24cad9c28c98334a7e7484abeefa023cb070628"
        },
        "date": 1773046486045,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 392.41,
            "range": "± 4.12",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 10.27,
            "range": "± 0.08",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 23.61,
            "range": "± 0.31",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.07,
            "range": "± 0.16",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_local_environment_hash",
            "value": 45.32,
            "range": "± 0.83",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_heads",
            "value": 184.47,
            "range": "± 1.71",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_tails",
            "value": 130.44,
            "range": "± 0.55",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_next_state",
            "value": 211.79,
            "range": "± 2.15",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_generate_and_iterate",
            "value": 424.32,
            "range": "± 10.09",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 231.09,
            "range": "± 4.55",
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
          "id": "3c22b4241c55bd4ea94e63735c291889d625c7cc",
          "message": "qol changes to game",
          "timestamp": "2026-03-10T10:31:00+01:00",
          "tree_id": "d3db2a6020fd4b5111725e68bfd43c3dc4c5e1f3",
          "url": "https://github.com/WbaN314/battlesnake/commit/3c22b4241c55bd4ea94e63735c291889d625c7cc"
        },
        "date": 1773135175347,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 402.15,
            "range": "± 1.85",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 10.25,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 23.62,
            "range": "± 0.35",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.05,
            "range": "± 0.14",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_local_environment_hash",
            "value": 48.97,
            "range": "± 1.84",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_heads",
            "value": 173.77,
            "range": "± 1.43",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_tails",
            "value": 127.57,
            "range": "± 0.44",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_next_state",
            "value": 198.7,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_generate_and_iterate",
            "value": 437.39,
            "range": "± 78.68",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 236.11,
            "range": "± 2.23",
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
          "id": "a4f0d3e486e903e080918f2c6337fc9a9b9a6d6c",
          "message": "added benchmark to node_id",
          "timestamp": "2026-03-24T13:23:55+01:00",
          "tree_id": "34b28705643e2f0f683abb48984fca77e50fd92d",
          "url": "https://github.com/WbaN314/battlesnake/commit/a4f0d3e486e903e080918f2c6337fc9a9b9a6d6c"
        },
        "date": 1774355145428,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 405.02,
            "range": "± 3.1",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 10.25,
            "range": "± 0.04",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 23.61,
            "range": "± 0.14",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.07,
            "range": "± 0.13",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_local_environment_hash",
            "value": 46.57,
            "range": "± 0.53",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_heads",
            "value": 282.81,
            "range": "± 1.18",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_tails",
            "value": 128.26,
            "range": "± 1.09",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_next_state",
            "value": 311.72,
            "range": "± 4.09",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_generate_and_iterate",
            "value": 419.43,
            "range": "± 4.57",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 248.28,
            "range": "± 14.85",
            "unit": "ns/iter"
          },
          {
            "name": "logic::new_year_new_snake::node_id::benchmarks::bench_node_id_tree_walk",
            "value": 32.36,
            "range": "± 0.39",
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
          "id": "e2fc4fbc2da9adcb9185dbc4dcb6ac483888eef2",
          "message": "Changed node id display pattern to group by depth",
          "timestamp": "2026-03-24T20:02:47+01:00",
          "tree_id": "6002bfdd088251ac6cccd86ab1ecd361e13b800d",
          "url": "https://github.com/WbaN314/battlesnake/commit/e2fc4fbc2da9adcb9185dbc4dcb6ac483888eef2"
        },
        "date": 1774379080876,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 403.47,
            "range": "± 3.4",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 10.25,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 23.6,
            "range": "± 0.16",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.06,
            "range": "± 0.15",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_local_environment_hash",
            "value": 45.14,
            "range": "± 0.93",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_heads",
            "value": 175.73,
            "range": "± 4.41",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_tails",
            "value": 126.9,
            "range": "± 0.54",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_next_state",
            "value": 201.3,
            "range": "± 2.38",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_generate_and_iterate",
            "value": 428.37,
            "range": "± 13.59",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 271.16,
            "range": "± 5.63",
            "unit": "ns/iter"
          },
          {
            "name": "logic::new_year_new_snake::node_id::benchmarks::bench_node_id_tree_walk",
            "value": 33.36,
            "range": "± 0.29",
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
          "id": "8fbb1fb5ad8e90cde780d35f3fedcbf19488b4fa",
          "message": "Some more tracing",
          "timestamp": "2026-03-24T20:07:54+01:00",
          "tree_id": "339ee5ea0888e9c2eb30f82bdce7f4fb4fd4e60a",
          "url": "https://github.com/WbaN314/battlesnake/commit/8fbb1fb5ad8e90cde780d35f3fedcbf19488b4fa"
        },
        "date": 1774379389571,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 399.77,
            "range": "± 2.55",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 10.26,
            "range": "± 0.07",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 23.62,
            "range": "± 0.12",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.08,
            "range": "± 0.13",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_local_environment_hash",
            "value": 44.98,
            "range": "± 0.64",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_heads",
            "value": 171.78,
            "range": "± 3.41",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_tails",
            "value": 126.81,
            "range": "± 0.48",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_next_state",
            "value": 198.17,
            "range": "± 2.51",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_generate_and_iterate",
            "value": 425.21,
            "range": "± 12.18",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 235.71,
            "range": "± 3.61",
            "unit": "ns/iter"
          },
          {
            "name": "logic::new_year_new_snake::node_id::benchmarks::bench_node_id_tree_walk",
            "value": 33.41,
            "range": "± 0.34",
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
          "id": "94661e1b5282604e997d05fc48af391e62a069bf",
          "message": "Added dead ancestor pruning",
          "timestamp": "2026-03-29T10:51:27+02:00",
          "tree_id": "b36955dd408e218420b65b169d64bc606a66c107",
          "url": "https://github.com/WbaN314/battlesnake/commit/94661e1b5282604e997d05fc48af391e62a069bf"
        },
        "date": 1774774405284,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 332.73,
            "range": "± 4.5",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 10.26,
            "range": "± 0.03",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 23.63,
            "range": "± 0.15",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.08,
            "range": "± 0.09",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_local_environment_hash",
            "value": 44.74,
            "range": "± 0.94",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_heads",
            "value": 95.74,
            "range": "± 2.3",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_tails",
            "value": 48.07,
            "range": "± 0.52",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_next_state",
            "value": 118.38,
            "range": "± 1.26",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_generate_and_iterate",
            "value": 427.69,
            "range": "± 5.73",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 235.8,
            "range": "± 9.23",
            "unit": "ns/iter"
          },
          {
            "name": "logic::new_year_new_snake::node::node_id::benchmarks::bench_node_id_tree_walk",
            "value": 33.43,
            "range": "± 0.33",
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
          "id": "c4428ff08fdd375f4dcf6dceba8f151c41c67e7f",
          "message": "Small bugfix",
          "timestamp": "2026-04-04T16:32:08+02:00",
          "tree_id": "7ddca8fdc1cfb8c68856b3168918ff8d19dca4f1",
          "url": "https://github.com/WbaN314/battlesnake/commit/c4428ff08fdd375f4dcf6dceba8f151c41c67e7f"
        },
        "date": 1775374425519,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 379.24,
            "range": "± 84.24",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 10.26,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 23.63,
            "range": "± 0.1",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.09,
            "range": "± 0.08",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_local_environment_hash",
            "value": 49.14,
            "range": "± 1.28",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_heads",
            "value": 172.6,
            "range": "± 2.36",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_tails",
            "value": 126.79,
            "range": "± 0.32",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_next_state",
            "value": 198.5,
            "range": "± 1.48",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_generate_and_iterate",
            "value": 430.53,
            "range": "± 12.15",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 234.28,
            "range": "± 9.04",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::node_id::benchmarks::bench_node_id_tree_walk",
            "value": 33.44,
            "range": "± 0.44",
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
          "id": "5b1330a9ea378baae3c1faa3d4089b00515da2c0",
          "message": "added watch flag",
          "timestamp": "2026-04-05T10:19:59+02:00",
          "tree_id": "c9f9055ff9dd9d28642507a35e6a0452e7f54bda",
          "url": "https://github.com/WbaN314/battlesnake/commit/5b1330a9ea378baae3c1faa3d4089b00515da2c0"
        },
        "date": 1775378256215,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 390.08,
            "range": "± 19.56",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 10.26,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 23.6,
            "range": "± 0.14",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.06,
            "range": "± 0.1",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_local_environment_hash",
            "value": 48.75,
            "range": "± 1.5",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_heads",
            "value": 172.76,
            "range": "± 1.97",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_tails",
            "value": 126.76,
            "range": "± 0.87",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_next_state",
            "value": 198.64,
            "range": "± 1.23",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_generate_and_iterate",
            "value": 428.45,
            "range": "± 9.67",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 233.18,
            "range": "± 11.28",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::node_id::benchmarks::bench_node_id_tree_walk",
            "value": 33.47,
            "range": "± 0.37",
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
          "id": "af3528b6c37b4cfd6d252c63043e9f59bf39cb92",
          "message": "Added SituationSet benchmarks",
          "timestamp": "2026-04-05T12:20:36+02:00",
          "tree_id": "ebd4577586c16d8751373364117ad89ac25581a7",
          "url": "https://github.com/WbaN314/battlesnake/commit/af3528b6c37b4cfd6d252c63043e9f59bf39cb92"
        },
        "date": 1775384547859,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 330.36,
            "range": "± 2.72",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 10.26,
            "range": "± 0.05",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 23.63,
            "range": "± 0.13",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.07,
            "range": "± 0.11",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_local_environment_hash",
            "value": 46.57,
            "range": "± 0.97",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_heads",
            "value": 93.84,
            "range": "± 2.02",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_tails",
            "value": 48.43,
            "range": "± 3.6",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_next_state",
            "value": 118.06,
            "range": "± 1.69",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_generate_and_iterate",
            "value": 433.06,
            "range": "± 14.24",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 237.7,
            "range": "± 6.11",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::node_id::benchmarks::bench_node_id_tree_walk",
            "value": 33.41,
            "range": "± 0.69",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_evaluate",
            "value": 45.6,
            "range": "± 0.77",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_with_condition_evaluate",
            "value": 42.54,
            "range": "± 0.32",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_set_evaluate",
            "value": 121.53,
            "range": "± 1.67",
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
          "id": "1b81c871bda4dc41c90a4b9e26644a06875a0c22",
          "message": "Some benchmarks",
          "timestamp": "2026-04-07T19:55:23+02:00",
          "tree_id": "63aa09d46ada1dbf5b02c444f570399f5618e4a8",
          "url": "https://github.com/WbaN314/battlesnake/commit/1b81c871bda4dc41c90a4b9e26644a06875a0c22"
        },
        "date": 1775584645016,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 379.43,
            "range": "± 68.21",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 10.26,
            "range": "± 0.16",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 23.63,
            "range": "± 0.12",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.06,
            "range": "± 0.09",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_local_environment_hash",
            "value": 44.96,
            "range": "± 0.95",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_heads",
            "value": 177.84,
            "range": "± 2.34",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_tails",
            "value": 127.9,
            "range": "± 1.22",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_next_state",
            "value": 201,
            "range": "± 2.05",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_generate_and_iterate",
            "value": 431.68,
            "range": "± 20.22",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 231.81,
            "range": "± 9.49",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_propagate_update_from_child",
            "value": 186.21,
            "range": "± 2.13",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_simulate",
            "value": 2408.75,
            "range": "± 37.01",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_status",
            "value": 32.33,
            "range": "± 0.39",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::node_id::benchmarks::bench_node_id_tree_walk",
            "value": 128.5,
            "range": "± 1.13",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_evaluate",
            "value": 42.85,
            "range": "± 0.63",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_with_condition_evaluate",
            "value": 40.35,
            "range": "± 0.61",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_set_evaluate",
            "value": 118.42,
            "range": "± 1.68",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::tree::benchmarks::bench_depth_queue_push_pop",
            "value": 58.89,
            "range": "± 0.76",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::tree::benchmarks::bench_tree_simulate_max_nodes",
            "value": 7704829.9,
            "range": "± 816670.48",
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
          "id": "7cc7fede60c152f9e7741482522ae2989ac6fd43",
          "message": "Improved internal childid storage",
          "timestamp": "2026-04-07T20:06:37+02:00",
          "tree_id": "d73df8161936369d46753b1a1be79de5092c1851",
          "url": "https://github.com/WbaN314/battlesnake/commit/7cc7fede60c152f9e7741482522ae2989ac6fd43"
        },
        "date": 1775585502588,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 373.59,
            "range": "± 90.19",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 10.26,
            "range": "± 0.03",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 23.62,
            "range": "± 0.18",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.1,
            "range": "± 0.15",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_local_environment_hash",
            "value": 45.36,
            "range": "± 0.88",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_heads",
            "value": 174.18,
            "range": "± 2.07",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_tails",
            "value": 127.55,
            "range": "± 3.51",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_next_state",
            "value": 198.79,
            "range": "± 1.15",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_generate_and_iterate",
            "value": 445.11,
            "range": "± 7.01",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 231.78,
            "range": "± 13.62",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_propagate_update_from_child",
            "value": 172.22,
            "range": "± 2.03",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_simulate",
            "value": 2435.57,
            "range": "± 23.59",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_status",
            "value": 32.28,
            "range": "± 0.38",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::node_id::benchmarks::bench_node_id_tree_walk",
            "value": 126.27,
            "range": "± 2.23",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_evaluate",
            "value": 45.83,
            "range": "± 0.69",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_with_condition_evaluate",
            "value": 44.19,
            "range": "± 0.43",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_set_evaluate",
            "value": 125.73,
            "range": "± 0.65",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::tree::benchmarks::bench_depth_queue_push_pop",
            "value": 60.91,
            "range": "± 1.77",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::tree::benchmarks::bench_tree_simulate_max_nodes",
            "value": 7215902.8,
            "range": "± 1688199.25",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "sascha.stoll@sap.com",
            "name": "i530412",
            "username": "SaschaAtWork"
          },
          "committer": {
            "email": "sascha.stoll@sap.com",
            "name": "i530412",
            "username": "SaschaAtWork"
          },
          "distinct": true,
          "id": "120841b5895f211c497e76bfff1b8b907fd9a25b",
          "message": "Refactored display method of gamestate to be more flexible with fields",
          "timestamp": "2026-06-02T15:13:11+02:00",
          "tree_id": "9af526e586f55a47bf27b027fb3956ee9b39471d",
          "url": "https://github.com/WbaN314/battlesnake/commit/120841b5895f211c497e76bfff1b8b907fd9a25b"
        },
        "date": 1780406499773,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 404.73,
            "range": "± 111.06",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 9.95,
            "range": "± 0.04",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 27.98,
            "range": "± 0.12",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.22,
            "range": "± 0.09",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_local_environment_hash",
            "value": 46.59,
            "range": "± 0.35",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_heads",
            "value": 103.92,
            "range": "± 2.9",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_tails",
            "value": 63.7,
            "range": "± 0.74",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_next_state",
            "value": 135.67,
            "range": "± 3.66",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_generate_and_iterate",
            "value": 356.85,
            "range": "± 9.35",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 197.32,
            "range": "± 9.29",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_propagate_update_from_child",
            "value": 110.75,
            "range": "± 0.59",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_simulate",
            "value": 1703.52,
            "range": "± 19.18",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_status",
            "value": 32.4,
            "range": "± 0.22",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::node_id::benchmarks::bench_node_id_tree_walk",
            "value": 128.83,
            "range": "± 1.28",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_evaluate",
            "value": 44.71,
            "range": "± 0.42",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_with_condition_evaluate",
            "value": 41.39,
            "range": "± 0.41",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_set_evaluate",
            "value": 122.2,
            "range": "± 4.72",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::tree::benchmarks::bench_depth_queue_push_pop",
            "value": 59.35,
            "range": "± 0.41",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::tree::benchmarks::bench_tree_simulate_max_nodes",
            "value": 4967965.9,
            "range": "± 614834.46",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "limejuicestudio@gmail.com",
            "name": "WbaN314",
            "username": "WbaN314"
          },
          "committer": {
            "email": "limejuicestudio@gmail.com",
            "name": "WbaN314",
            "username": "WbaN314"
          },
          "distinct": true,
          "id": "3cde13c63de74f85e1c1be1bef077b9022d888a1",
          "message": "Added flood fill to snake logic",
          "timestamp": "2026-06-03T16:32:29+02:00",
          "tree_id": "2ba981a1f3fc1b9d033c013c3eea1b3df78ee1fb",
          "url": "https://github.com/WbaN314/battlesnake/commit/3cde13c63de74f85e1c1be1bef077b9022d888a1"
        },
        "date": 1780497285330,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::game::board::benchmarks::bench_remove_snake",
            "value": 395.52,
            "range": "± 46.17",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 9.95,
            "range": "± 0.12",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_basic_field",
            "value": 27.95,
            "range": "± 0.18",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::field::benchmarks::bench_bit_field",
            "value": 6.08,
            "range": "± 0.09",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_flood_fill",
            "value": 3009.39,
            "range": "± 25.25",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_local_environment_hash",
            "value": 49.01,
            "range": "± 1.57",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_heads",
            "value": 103.27,
            "range": "± 0.75",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_move_tails",
            "value": 57.7,
            "range": "± 0.98",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::game_state::benchmarks::bench_next_state",
            "value": 127.74,
            "range": "± 0.78",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_generate_and_iterate",
            "value": 370.39,
            "range": "± 6.79",
            "unit": "ns/iter"
          },
          {
            "name": "logic::game::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 170.89,
            "range": "± 12.94",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_propagate_update_from_child",
            "value": 107.91,
            "range": "± 0.86",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_simulate",
            "value": 1703.65,
            "range": "± 18.83",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_status",
            "value": 17.61,
            "range": "± 0.35",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::node_id::benchmarks::bench_node_id_tree_walk",
            "value": 127.76,
            "range": "± 1.47",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_evaluate",
            "value": 46.16,
            "range": "± 0.6",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_with_condition_evaluate",
            "value": 43.01,
            "range": "± 0.47",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_set_evaluate",
            "value": 125.41,
            "range": "± 5.63",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::tree::benchmarks::bench_depth_queue_push_pop",
            "value": 60.5,
            "range": "± 0.86",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::tree::benchmarks::bench_tree_simulate_max_nodes",
            "value": 8359962,
            "range": "± 392760.67",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "limejuicestudio@gmail.com",
            "name": "WbaN314",
            "username": "WbaN314"
          },
          "committer": {
            "email": "limejuicestudio@gmail.com",
            "name": "WbaN314",
            "username": "WbaN314"
          },
          "distinct": true,
          "id": "4e22654d715ab582c37f9d9472f99d62015dd1e3",
          "message": "Early stop fill when only tail chasing",
          "timestamp": "2026-06-08T17:06:44+02:00",
          "tree_id": "c7917deb61503c35861aad7287b6a5855ab7c6c7",
          "url": "https://github.com/WbaN314/battlesnake/commit/4e22654d715ab582c37f9d9472f99d62015dd1e3"
        },
        "date": 1780931329292,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::general::board::benchmarks::bench_remove_snake",
            "value": 396.23,
            "range": "± 30.34",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 9.96,
            "range": "± 0.08",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::field::benchmarks::bench_basic_field",
            "value": 28,
            "range": "± 0.21",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::field::benchmarks::bench_bit_field",
            "value": 6.22,
            "range": "± 0.08",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_flood_fill",
            "value": 24130.67,
            "range": "± 659.64",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_local_environment_hash",
            "value": 48.71,
            "range": "± 2.24",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_move_heads",
            "value": 102.72,
            "range": "± 1.94",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_move_tails",
            "value": 52.16,
            "range": "± 1.68",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_next_state",
            "value": 127.12,
            "range": "± 2.42",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::moves::benchmarks::bench_generate_and_iterate",
            "value": 359.53,
            "range": "± 4.60",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 168.49,
            "range": "± 3.93",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_propagate_update_from_child",
            "value": 113.54,
            "range": "± 2.17",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_simulate",
            "value": 1748.49,
            "range": "± 24.98",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_status",
            "value": 31.06,
            "range": "± 2.19",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::node_id::benchmarks::bench_node_id_tree_walk",
            "value": 128.42,
            "range": "± 2.35",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_evaluate",
            "value": 45.99,
            "range": "± 3.75",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_with_condition_evaluate",
            "value": 43.17,
            "range": "± 1.05",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_set_evaluate",
            "value": 127.58,
            "range": "± 3.42",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::tree::benchmarks::bench_depth_queue_push_pop",
            "value": 60.05,
            "range": "± 1.24",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::tree::benchmarks::bench_tree_simulate_max_nodes",
            "value": 4961072.65,
            "range": "± 534474.26",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "limejuicestudio@gmail.com",
            "name": "WbaN314",
            "username": "WbaN314"
          },
          "committer": {
            "email": "limejuicestudio@gmail.com",
            "name": "WbaN314",
            "username": "WbaN314"
          },
          "distinct": true,
          "id": "af213151e3d718a814d6ed6cfddd9e22d0e3073d",
          "message": "Changed memory size by storing turn",
          "timestamp": "2026-06-08T17:08:02+02:00",
          "tree_id": "d834206c5f150a8704b3bdbf6b6870ce4aee6254",
          "url": "https://github.com/WbaN314/battlesnake/commit/af213151e3d718a814d6ed6cfddd9e22d0e3073d"
        },
        "date": 1780931416921,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::general::board::benchmarks::bench_remove_snake",
            "value": 391.02,
            "range": "± 9.75",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 9.96,
            "range": "± 0.29",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::field::benchmarks::bench_basic_field",
            "value": 28.02,
            "range": "± 0.21",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::field::benchmarks::bench_bit_field",
            "value": 6.02,
            "range": "± 0.09",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_flood_fill",
            "value": 24696.58,
            "range": "± 369.7",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_local_environment_hash",
            "value": 48.7,
            "range": "± 1.79",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_move_heads",
            "value": 102.23,
            "range": "± 0.82",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_move_tails",
            "value": 52.19,
            "range": "± 10.52",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_next_state",
            "value": 125.96,
            "range": "± 2.16",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::moves::benchmarks::bench_generate_and_iterate",
            "value": 359.02,
            "range": "± 6.53",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 168.34,
            "range": "± 4.95",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_propagate_update_from_child",
            "value": 114.92,
            "range": "± 1.9",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_simulate",
            "value": 1771.24,
            "range": "± 14.54",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_status",
            "value": 31.38,
            "range": "± 0.27",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::node_id::benchmarks::bench_node_id_tree_walk",
            "value": 128.59,
            "range": "± 1.32",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_evaluate",
            "value": 42.59,
            "range": "± 0.46",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_with_condition_evaluate",
            "value": 40.08,
            "range": "± 0.63",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_set_evaluate",
            "value": 119.83,
            "range": "± 1.59",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::tree::benchmarks::bench_depth_queue_push_pop",
            "value": 59.81,
            "range": "± 0.46",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::tree::benchmarks::bench_tree_simulate_max_nodes",
            "value": 4935964.35,
            "range": "± 534385.63",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "limejuicestudio@gmail.com",
            "name": "WbaN314",
            "username": "WbaN314"
          },
          "committer": {
            "email": "limejuicestudio@gmail.com",
            "name": "WbaN314",
            "username": "WbaN314"
          },
          "distinct": true,
          "id": "88def08a6b9f58ba96440ec8c00c86cebd4b9b3f",
          "message": "Bugfix ignite",
          "timestamp": "2026-06-08T17:32:13+02:00",
          "tree_id": "236108c496e6adcf9c2f56298dbd6dd73b04f9a1",
          "url": "https://github.com/WbaN314/battlesnake/commit/88def08a6b9f58ba96440ec8c00c86cebd4b9b3f"
        },
        "date": 1780932865488,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::general::board::benchmarks::bench_remove_snake",
            "value": 258.19,
            "range": "± 1.84",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 9.13,
            "range": "± 0.07",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::field::benchmarks::bench_basic_field",
            "value": 28.28,
            "range": "± 0.27",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::field::benchmarks::bench_bit_field",
            "value": 6.14,
            "range": "± 0.07",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_flood_fill",
            "value": 27229.79,
            "range": "± 281.52",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_local_environment_hash",
            "value": 49.81,
            "range": "± 0.51",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_move_heads",
            "value": 104.02,
            "range": "± 2.32",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_move_tails",
            "value": 51.47,
            "range": "± 0.38",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_next_state",
            "value": 130.94,
            "range": "± 1.86",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::moves::benchmarks::bench_generate_and_iterate",
            "value": 370.71,
            "range": "± 2.92",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 193.8,
            "range": "± 1.6",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_propagate_update_from_child",
            "value": 114.73,
            "range": "± 3.14",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_simulate",
            "value": 1834.19,
            "range": "± 19.14",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_status",
            "value": 34.54,
            "range": "± 0.22",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::node_id::benchmarks::bench_node_id_tree_walk",
            "value": 148.15,
            "range": "± 6.58",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_evaluate",
            "value": 46.73,
            "range": "± 0.88",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_with_condition_evaluate",
            "value": 42.7,
            "range": "± 1.12",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_set_evaluate",
            "value": 124.19,
            "range": "± 2.24",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::tree::benchmarks::bench_depth_queue_push_pop",
            "value": 65.24,
            "range": "± 0.55",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::tree::benchmarks::bench_tree_simulate_max_nodes",
            "value": 5767701.95,
            "range": "± 705713.78",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "limejuicestudio@gmail.com",
            "name": "WbaN314",
            "username": "WbaN314"
          },
          "committer": {
            "email": "limejuicestudio@gmail.com",
            "name": "WbaN314",
            "username": "WbaN314"
          },
          "distinct": true,
          "id": "ab60b4e1aa4a14b1ee13c3808e2bd433fb5e0aaf",
          "message": "Added scores to situations, all situations pass",
          "timestamp": "2026-06-10T15:17:16+02:00",
          "tree_id": "8939aada91ccd89481eb2a9b1b4ecec515208022",
          "url": "https://github.com/WbaN314/battlesnake/commit/ab60b4e1aa4a14b1ee13c3808e2bd433fb5e0aaf"
        },
        "date": 1781097579277,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::general::board::benchmarks::bench_remove_snake",
            "value": 257.99,
            "range": "± 9.72",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 9.13,
            "range": "± 0.06",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::field::benchmarks::bench_basic_field",
            "value": 28.25,
            "range": "± 0.2",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::field::benchmarks::bench_bit_field",
            "value": 6.14,
            "range": "± 0.1",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_flood_fill",
            "value": 26937.09,
            "range": "± 164.99",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_local_environment_hash",
            "value": 49.94,
            "range": "± 3.02",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_move_heads",
            "value": 101.88,
            "range": "± 3.63",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_move_tails",
            "value": 55.23,
            "range": "± 1.08",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_next_state",
            "value": 139.9,
            "range": "± 3.17",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::moves::benchmarks::bench_generate_and_iterate",
            "value": 371.48,
            "range": "± 1.93",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 200.8,
            "range": "± 8.25",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_propagate_update_from_child",
            "value": 101.08,
            "range": "± 1.26",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_simulate",
            "value": 1844.61,
            "range": "± 34.42",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_status",
            "value": 18.12,
            "range": "± 0.53",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::node_id::benchmarks::bench_node_id_tree_walk",
            "value": 148.5,
            "range": "± 1.06",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_evaluate",
            "value": 46.97,
            "range": "± 1.17",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_with_condition_evaluate",
            "value": 42.9,
            "range": "± 0.29",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_set_evaluate",
            "value": 184.99,
            "range": "± 2.09",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::tree::benchmarks::bench_depth_queue_push_pop",
            "value": 67.08,
            "range": "± 1.9",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::tree::benchmarks::bench_tree_simulate_max_nodes",
            "value": 9723601.05,
            "range": "± 701067.44",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "limejuicestudio@gmail.com",
            "name": "WbaN314",
            "username": "WbaN314"
          },
          "committer": {
            "email": "limejuicestudio@gmail.com",
            "name": "WbaN314",
            "username": "WbaN314"
          },
          "distinct": true,
          "id": "4155bf97ee1342ec3c64d47ae752360778385efc",
          "message": "Bugfix with section not beeing created",
          "timestamp": "2026-06-10T16:46:10+02:00",
          "tree_id": "a3c7f76cf992446e0d2e81f78eca9f0e33a7d30d",
          "url": "https://github.com/WbaN314/battlesnake/commit/4155bf97ee1342ec3c64d47ae752360778385efc"
        },
        "date": 1781102916661,
        "tool": "cargo",
        "benches": [
          {
            "name": "logic::general::board::benchmarks::bench_remove_snake",
            "value": 383.41,
            "range": "± 3.93",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::board::benchmarks::bench_set_field_get_field_via_cell",
            "value": 9.97,
            "range": "± 0.18",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::field::benchmarks::bench_basic_field",
            "value": 27.97,
            "range": "± 0.09",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::field::benchmarks::bench_bit_field",
            "value": 6.03,
            "range": "± 0.12",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_flood_fill",
            "value": 26037.28,
            "range": "± 185.54",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_local_environment_hash",
            "value": 45.18,
            "range": "± 0.76",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_move_heads",
            "value": 102.39,
            "range": "± 1.22",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_move_tails",
            "value": 58.16,
            "range": "± 1.37",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::game_state::benchmarks::bench_next_state",
            "value": 128.44,
            "range": "± 3.34",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::moves::benchmarks::bench_generate_and_iterate",
            "value": 361.79,
            "range": "± 5.46",
            "unit": "ns/iter"
          },
          {
            "name": "logic::general::moves::benchmarks::bench_pregenerate_and_iterate",
            "value": 168.91,
            "range": "± 10.34",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_propagate_update_from_child",
            "value": 106.88,
            "range": "± 1.76",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_simulate",
            "value": 1756.37,
            "range": "± 33.05",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::benchmarks::bench_node_status",
            "value": 16.68,
            "range": "± 0.19",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::node::node_id::benchmarks::bench_node_id_tree_walk",
            "value": 128,
            "range": "± 1.49",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_evaluate",
            "value": 46.26,
            "range": "± 0.94",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_full_symmetry_with_condition_evaluate",
            "value": 42.72,
            "range": "± 0.82",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::situation::benchmarks::bench_situation_set_evaluate",
            "value": 187.16,
            "range": "± 3.65",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::tree::benchmarks::bench_depth_queue_push_pop",
            "value": 59.98,
            "range": "± 37.73",
            "unit": "ns/iter"
          },
          {
            "name": "logic::single_gamestate_nodes::tree::benchmarks::bench_tree_simulate_max_nodes",
            "value": 8754881.3,
            "range": "± 1059602.49",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}