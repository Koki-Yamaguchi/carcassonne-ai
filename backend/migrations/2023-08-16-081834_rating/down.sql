ALTER TABLE game DROP COLUMN is_rated;
ALTER TABLE game DROP COLUMN before_player0_rating;
ALTER TABLE game DROP COLUMN before_player1_rating;
ALTER TABLE game DROP COLUMN after_player0_rating;
ALTER TABLE game DROP COLUMN after_player1_rating;
ALTER TABLE game DROP COLUMN first_player_id;
ALTER TABLE game DROP COLUMN winner_player_id;

ALTER TABLE player DROP COLUMN rating;
