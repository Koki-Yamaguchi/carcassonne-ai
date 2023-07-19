CREATE TABLE player (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL
);

CREATE TABLE game (
  id SERIAL PRIMARY KEY,
  player0_id INT NOT NULL,
  player1_id INT NOT NULL,
  player0_point INT NOT NULL,
  player1_point INT NOT NULL,
  next_tile_id INT,
  next_player_id INT,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  ended_at TIMESTAMP DEFAULT NULL,
  CONSTRAINT fk_player0_id
    FOREIGN KEY(player0_id)
      REFERENCES player(id),
  CONSTRAINT fk_player1_id
    FOREIGN KEY(player1_id)
      REFERENCES player(id)
);

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
);

