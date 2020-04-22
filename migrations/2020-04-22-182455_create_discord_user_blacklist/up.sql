CREATE TABLE discord_users_blacklist (
  id INT AUTO_INCREMENT PRIMARY KEY,
  discord_userid VARCHAR(64) NOT NULL,
  discord_id BIGINT NOT NULL,
  added_by_id BIGINT NOT NULL,
  guild_id BIGINT,
  dt_created DATETIME DEFAULT CURRENT_TIMESTAMP,
  dt_modified DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
)
COLLATE='utf8mb4_unicode_ci'
;