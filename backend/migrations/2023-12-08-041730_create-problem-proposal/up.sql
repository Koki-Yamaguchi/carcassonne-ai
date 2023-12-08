CREATE TABLE problem_proposal (
  id SERIAL PRIMARY KEY,
  table_id VARCHAR(20) NOT NULL,
  remaining_tile_count INT NOT NULL,
  tile_id INT NOT NULL,
  creator_id INT,
  used_at TIMESTAMP DEFAULT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

ALTER TABLE problem ADD COLUMN is_draft BOOLEAN NOT NULL DEFAULT FALSE;
