ALTER TABLE game ADD COLUMN is_rated BOOLEAN DEFAULT FALSE NOT NULL;
ALTER TABLE game ADD COLUMN before_player0_rating INT;
ALTER TABLE game ADD COLUMN before_player1_rating INT;
ALTER TABLE game ADD COLUMN after_player0_rating INT;
ALTER TABLE game ADD COLUMN after_player1_rating INT;
ALTER TABLE game ADD COLUMN first_player_id INT;
ALTER TABLE game ADD COLUMN winner_player_id INT;
ALTER TABLE game ADD CONSTRAINT fk_first_player_id
  FOREIGN KEY(first_player_id)
  REFERENCES player(id);
ALTER TABLE game ADD CONSTRAINT fk_winner_player_id
  FOREIGN KEY(winner_player_id)
  REFERENCES player(id);

ALTER TABLE player ADD COLUMN rating INT;
