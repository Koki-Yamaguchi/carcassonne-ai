CREATE TABLE waiting_game_history (
  id SERIAL PRIMARY KEY,
  player_id INT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_player_id
    FOREIGN KEY (player_id) REFERENCES player(id)
);
