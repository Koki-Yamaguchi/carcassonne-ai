ALTER TABLE problem ADD COLUMN is_solved BOOLEAN DEFAULT false NOT NULL;
ALTER TABLE problem ADD COLUMN optimal_move_count INT;
ALTER TABLE problem ADD COLUMN tester_id INT;
ALTER TABLE problem ADD COLUMN tester_name VARCHAR(100);
