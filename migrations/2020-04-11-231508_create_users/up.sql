CREATE TABLE users (
  id INT AUTO_INCREMENT PRIMARY KEY,
  display_name VARCHAR(64) NOT NULL,
  steamid2 VARCHAR(32) NOT NULL,
  discord VARCHAR(18),
  elo FLOAT NOT NULL DEFAULT 0.0,
  rws FLOAT NOT NULL DEFAULT 0.0,
  rounds_total INT NOT NULL DEFAULT 0
)
COLLATE='utf8mb4_unicode_ci'
;