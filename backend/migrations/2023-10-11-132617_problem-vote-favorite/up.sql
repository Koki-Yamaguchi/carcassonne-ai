CREATE TABLE problem (
  id SERIAL PRIMARY KEY,
  game_id INT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_game_id
    FOREIGN KEY(game_id)
      REFERENCES game(id)
);

CREATE TABLE vote (
  id SERIAL PRIMARY KEY,
  problem_id INT NOT NULL,
  player_id INT NOT NULL,
  player_name VARCHAR(100) NOT NULL,
  note TEXT NOT NULL DEFAULT '',
  favorite_count INT NOT NULL DEFAULT 0,
  tile_move_id INT NOT NULL,
  meeple_move_id INT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_problem_id
    FOREIGN KEY(problem_id)
      REFERENCES problem(id),
  CONSTRAINT fk_player_id
    FOREIGN KEY(player_id)
      REFERENCES player(id),
  CONSTRAINT fk_tile_move_id
    FOREIGN KEY(tile_move_id)
      REFERENCES move(id),
  CONSTRAINT fk_meeple_move_id
    FOREIGN KEY(meeple_move_id)
      REFERENCES move(id)
);

CREATE TABLE favorite (
  id SERIAL PRIMARY KEY,
  vote_id INT NOT NULL,
  player_id INT NOT NULL,
  player_name VARCHAR(100) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_vote_id
    FOREIGN KEY(vote_id)
      REFERENCES vote(id),
  CONSTRAINT fk_player_id
    FOREIGN KEY(player_id)
      REFERENCES player(id)
);

