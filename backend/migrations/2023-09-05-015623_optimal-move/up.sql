CREATE TABLE optimal_move (
  id SERIAL PRIMARY KEY,
  game_id INT NOT NULL,
  last_n INT NOT NULL,
  tile_move_id INT NOT NULL,
  meeple_move_id INT NOT NULL,
  CONSTRAINT fk_game_id
    FOREIGN KEY(game_id)
      REFERENCES game(id),
  CONSTRAINT fk_tile_move_id
    FOREIGN KEY(tile_move_id)
      REFERENCES move(id),
  CONSTRAINT fk_meeple_move_id
    FOREIGN KEY(meeple_move_id)
      REFERENCES move(id)
);
