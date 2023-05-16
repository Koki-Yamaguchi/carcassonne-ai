CREATE TABLE move (
  id SERIAL PRIMARY KEY,
  ord INT NOT NULL,
  game_id INT NOT NULL,
  player_id INT NOT NULL,
  tile_id INT NOT NULL,
  meeple_id INT NOT NULL,
  rot INT NOT NULL,
  tile_pos_y INT NOT NULL,
  tile_pos_x INT NOT NULL,
  meeple_pos INT NOT NULL,
  CONSTRAINT fk_game_id
    FOREIGN KEY(game_id)
      REFERENCES game(id),
  CONSTRAINT fk_player_id
    FOREIGN KEY(player_id)
      REFERENCES player(id)
)
