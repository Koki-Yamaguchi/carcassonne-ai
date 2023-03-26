curl -X POST "http://127.0.0.1:8000/tile-moves/create" \
    -H "Content-Type: application/json" \
    -d "{\"game_id\": 12, \"player_id\": 4, \"tile_id\": 1, \"rot\": 2, \"pos_y\": 1, \"pos_x\": 2 }"

