curl -X POST "http://127.0.0.1:8000/games/create" \
    -H "Content-Type: application/json" \
    -d "{\"note\": \"this is test\", \"player0_id\": 12, \"player1_id\": 11}"

