ALTER TABLE favorite DROP COLUMN problem_id;
ALTER TABLE favorite ADD COLUMN vote_id INT NOT NULL;
ALTER TABLE favorite ADD CONSTRAINT fk_vote_id
  FOREIGN KEY(vote_id)
    REFERENCES vote(id);

ALTER TABLE problem DROP COLUMN favorite_count;
ALTER TABLE vote ADD COLUMN favorite_count INT NOT NULL DEFAULT 0;
