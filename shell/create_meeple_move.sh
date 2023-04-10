curl -X POST "http://127.0.0.1:8000/meeple-moves/create" \
    -H "Content-Type: application/json" \
    -d "{\"game_id\": 23, \"player_id\": 10, \"meeple_id\": 4, \"pos\": 0, \"tile_pos_y\": -1, \"tile_pos_x\": -1 }"

