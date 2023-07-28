CREATE TABLE color (
  id INT PRIMARY KEY,
  name TEXT NOT NULL
);


INSERT INTO color VALUES (0, 'red');
INSERT INTO color VALUES (1, 'yellow');
INSERT INTO color VALUES (2, 'green');
INSERT INTO color VALUES (3, 'black');
INSERT INTO color VALUES (4, 'blue');

ALTER TABLE player ADD COLUMN meeple_color INT DEFAULT 0 NOT NULL;
ALTER TABLE player
  ADD CONSTRAINT fk_meeple_color
  FOREIGN KEY(meeple_color)
  REFERENCES color(id);

