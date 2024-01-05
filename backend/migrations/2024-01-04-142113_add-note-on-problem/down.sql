ALTER TABLE problem DROP COLUMN note;
ALTER TABLE problem_proposal DROP COLUMN note;

ALTER TABLE problem_proposal ADD COLUMN tile_id INT;
