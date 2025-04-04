#!/bin/bash

# Ensure servers are running
./run_servers.sh &
sleep 2

# Initialize a file to keep track of wins
tally_file="winners_tally.txt"
> "$tally_file" # Clear the file at the start

# Run the game in an endless loop and keep a tally of the winners
while true; do
    winner=$(../battlesnake_local/battlesnake play -W 11 -H 11 \
        --name depth_first --url http://localhost:8001 \
        --name breadth_first --url http://localhost:8002 \
        --name simple_tree_search --url http://localhost:8003 \
        --name simple_hungry --url http://localhost:8004 2>&1 | \
        grep "Game completed" | \
        awk -F 'after | turns. | was the winner.' '{print $3}')

    if [ -n "$winner" ]; then
        # Increment the winner's tally in the file
        if grep -q "^$winner:" "$tally_file"; then
            awk -F: -v winner="$winner" '
                BEGIN { OFS = ":" }
                $1 == winner { $2 = $2 + 1 }
                { print }
            ' "$tally_file" > "${tally_file}.tmp" && mv "${tally_file}.tmp" "$tally_file"
        else
            echo "$winner:1" >> "$tally_file"
        fi

        # Print the current tally
        cat "$tally_file"
        echo "----------------------"
    fi
done