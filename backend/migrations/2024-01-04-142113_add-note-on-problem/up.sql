ALTER TABLE problem ADD COLUMN note TEXT NOT NULL DEFAULT '';
ALTER TABLE problem_proposal ADD COLUMN note TEXT NOT NULL DEFAULT '';

ALTER TABLE problem_proposal DROP COLUMN tile_id;
