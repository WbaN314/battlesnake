window.BENCHMARK_DATA = {
  "lastUpdate": 1775384548218,
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
      }
    ]
  }
}