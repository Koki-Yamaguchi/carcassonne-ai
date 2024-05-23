ALTER TABLE favorite DROP COLUMN vote_id;
ALTER TABLE favorite ADD COLUMN problem_id INT NOT NULL;
ALTER TABLE favorite ADD CONSTRAINT fk_problem_id
  FOREIGN KEY(problem_id)
    REFERENCES problem(id);

ALTER TABLE vote DROP COLUMN favorite_count;
ALTER TABLE problem ADD COLUMN favorite_count INT NOT NULL DEFAULT 0;
