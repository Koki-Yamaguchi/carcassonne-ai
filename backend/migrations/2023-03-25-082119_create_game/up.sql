CREATE TABLE game (
  id SERIAL PRIMARY KEY,
  note TEXT NOT NULL,
  player0_id INT,
  player1_id INT,
  player0_point INT,
  player1_point INT,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  ended_at TIMESTAMP DEFAULT NULL,
  CONSTRAINT fk_player0_id
    FOREIGN KEY(player0_id)
      REFERENCES player(id),
  CONSTRAINT fk_player1_id
    FOREIGN KEY(player1_id)
      REFERENCES player(id)
)
