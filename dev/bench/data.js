window.BENCHMARK_DATA = {
  "lastUpdate": 1772792207893,
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
      }
    ]
  }
}