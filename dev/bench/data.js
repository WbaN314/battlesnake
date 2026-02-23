window.BENCHMARK_DATA = {
  "lastUpdate": 1771844657329,
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
      }
    ]
  }
}