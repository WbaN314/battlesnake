# Ensure servers are running
./run_servers.sh &
sleep 2

# Run the game in an endless loop and only print the last line of each iteration
while true; do
    ../battlesnake_local/battlesnake play -W 11 -H 11 \
        --name depth_first --url http://localhost:8001 \
        --name breadth_first --url http://localhost:8002 \
        --name simple_tree_search --url http://localhost:8003 \
        --name simple_hungry --url http://localhost:8004 2>&1 | \
        awk '{line=$0} END {print line}' | \
        awk -F 'after | turns. | was the winner.' '{print $3, $2}'

done