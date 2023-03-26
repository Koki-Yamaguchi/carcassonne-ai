curl -X POST "http://127.0.0.1:8000/meeple-moves/create" \
    -H "Content-Type: application/json" \
    -d "{\"game_id\": 12, \"player_id\": 4, \"meeple_id\": 1, \"pos\": 2 }"

