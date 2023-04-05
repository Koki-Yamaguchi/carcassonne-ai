curl -X POST "http://127.0.0.1:8000/tile-moves/create" \
    -H "Content-Type: application/json" \
    -d "{\"game_id\": 21, \"player_id\": 5, \"tile_id\": 0, \"rot\": 2, \"pos_y\": 50, \"pos_x\": 51 }"

