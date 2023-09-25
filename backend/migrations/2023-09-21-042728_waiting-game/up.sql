CREATE TABLE waiting_game (
  id SERIAL PRIMARY KEY,
  player_id INT NOT NULL,
  game_id INT,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
