#!/bin/bash
# Usage: ./bench.sh [-n NUM_GAMES|-NUM_GAMES] [-w] snake1 snake2 [snake3 snake4]
# Example: ./bench.sh -n 100 gamestate_nodes depth_first breadth_first simple_tree_search
# Example: ./bench.sh -w depth_first

N_GAMES=0
WATCH=0
SNAKES=()

while [[ $# -gt 0 ]]; do
    case $1 in
        -[0-9]*) N_GAMES=${1#-}; shift ;;
        -n) N_GAMES=$2; shift 2 ;;
        -w) WATCH=1; shift ;;
        *) SNAKES+=("$1"); shift ;;
    esac
done

if [ ${#SNAKES[@]} -lt 2 ]; then
    echo "Usage: $0 [-n NUM_GAMES|-NUM_GAMES] [-w] snake1 snake2 [snake3 snake4]"
    echo "Variants: depth_first breadth_first simple_tree_search simple_hungry gamestate_nodes"
    exit 1
fi

BASE_PORT=8001
SERVER_PIDS=()
_cleaned=0

cleanup() {
    [ "$_cleaned" -eq 1 ] && return
    _cleaned=1
    echo ""
    echo "Stopping servers..."
    for pid in "${SERVER_PIDS[@]}"; do
        kill "$pid" 2>/dev/null || true
    done
    # Belt-and-suspenders: sweep the ports we used
    for ((i=0; i<${#SNAKES[@]}; i++)); do
        port=$((BASE_PORT + i))
        pids=$(lsof -t -i:"${port}" 2>/dev/null || true)
        [ -n "$pids" ] && kill -9 $pids 2>/dev/null || true
    done
    echo "Done."
}

trap cleanup EXIT
trap 'exit 130' INT TERM

# Build
echo "Building..."
cargo build --release 2>&1 | grep -E "^(error|warning: unused|Compiling|Finished)" || true

# Start servers
BATTLESNAKE_ARGS=()
SNAKE_NAMES=()
for ((i=0; i<${#SNAKES[@]}; i++)); do
    port=$((BASE_PORT + i))
    variant="${SNAKES[$i]}"
    name="${variant}_$((i+1))"

    existing=$(lsof -t -i:"${port}" 2>/dev/null || true)
    if [ -n "$existing" ]; then
        kill -9 $existing 2>/dev/null || true
        echo "Cleared stale process on :$port"
    fi

    PORT=$port VARIANT=$variant ./target/release/battlesnake_game_of_chicken >/dev/null 2>&1 &
    SERVER_PIDS+=("$!")
    echo "Started $name on :$port (PID $!)"

    BATTLESNAKE_ARGS+=(--name "$name" --url "http://localhost:$port")
    SNAKE_NAMES+=("$name")
done

sleep 1

# Tally
declare -A wins
for name in "${SNAKE_NAMES[@]}"; do
    wins["$name"]=0
done
total=0
tally_lines=0

print_tally() {
    if [ "$tally_lines" -gt 0 ]; then
        printf "\033[%dA\033[J" "$tally_lines"
    fi

    local lines=0
    printf "  %-28s %6s  %6s\n" "Snake" "Wins" "Win%"; ((lines++))
    printf "  %-28s %6s  %6s\n" "----------------------------" "------" "------"; ((lines++))
    for name in "${SNAKE_NAMES[@]}"; do
        w=${wins["$name"]}
        pct=$(awk -v w="$w" -v t="$total" 'BEGIN { printf "%.1f", (t > 0) ? w * 100 / t : 0 }')
        printf "  %-28s %6d  %5s%%\n" "$name" "$w" "$pct"; ((lines++))
    done
    printf "  %s\n" "Games played: $total"; ((lines++))
    if [ "$N_GAMES" -gt 0 ]; then
        printf "  %s\n" "Target: $N_GAMES"; ((lines++))
    fi

    tally_lines=$lines
}

PLAY_FLAGS=()
[ "$WATCH" -eq 1 ] && PLAY_FLAGS+=(-v -c)

echo ""
while true; do
    if [ "$WATCH" -eq 1 ]; then
        # tee /dev/tty streams to terminal live while keeping output for winner parsing
        output=$(../battlesnake_local/battlesnake play -W 11 -H 11 "${PLAY_FLAGS[@]}" "${BATTLESNAKE_ARGS[@]}" 2>&1 | tee /dev/tty)
        winner=$(echo "$output" | grep -oE '[A-Za-z0-9_-]+ was the winner' | awk '{print $1}')
        # Tally prints below the scrolled game output (no in-place overwrite in watch mode)
        tally_lines=0
    else
        winner=$(../battlesnake_local/battlesnake play -W 11 -H 11 "${PLAY_FLAGS[@]}" "${BATTLESNAKE_ARGS[@]}" 2>&1 | \
            grep -oE '[A-Za-z0-9_-]+ was the winner' | \
            awk '{print $1}')
    fi

    if [ -n "$winner" ]; then
        if [[ -v wins["$winner"] ]]; then
            wins["$winner"]=$((wins["$winner"] + 1))
        fi
        total=$((total + 1))
        print_tally
    fi

    if [ "$N_GAMES" -gt 0 ] && [ "$total" -ge "$N_GAMES" ]; then
        break
    fi
done
