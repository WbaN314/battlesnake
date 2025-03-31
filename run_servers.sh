cargo build --release

for port in {8001..8004}; do
    pid=$(lsof -t -i:${port})
    if [ -n "$pid" ]; then
        kill -9 $pid
        echo "Killed process on port ${port}"
    else
        echo "No process running on port ${port}"
    fi
done

export PORT=8001
export VARIANT=depth_first
./target/release/battlesnake_game_of_chicken > /dev/null 2>&1 &

export PORT=8002
export VARIANT=breadth_first
./target/release/battlesnake_game_of_chicken > /dev/null 2>&1 &

export PORT=8003
export VARIANT=simple_tree_search
./target/release/battlesnake_game_of_chicken > /dev/null 2>&1 &

export PORT=8004
export VARIANT=simple_hungry
./target/release/battlesnake_game_of_chicken > /dev/null 2>&1 &