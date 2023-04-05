curl -X POST "http://127.0.0.1:8000/tile-moves/create" \
    -H "Content-Type: application/json" \
    -d "{\"game_id\": 23, \"player_id\": 10, \"tile_id\": 3, \"rot\": 2, \"pos_y\": 49, \"pos_x\": 49 }"

