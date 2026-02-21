window.BENCHMARK_DATA = {
  "lastUpdate": 1771672935959,
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
      }
    ]
  }
}