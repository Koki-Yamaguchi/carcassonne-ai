curl -X POST "http://127.0.0.1:8000/meeple-moves/create" \
    -H "Content-Type: application/json" \
    -d "{\"game_id\": 15, \"player_id\": 5, \"meeple_id\": 0, \"pos\": 3, \"tile_pos_y\": 50, \"tile_pos_x\": 51 }"

