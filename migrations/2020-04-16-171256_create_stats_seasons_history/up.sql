CREATE TABLE `stats_seasons_history` (
	`id` INT NOT NULL AUTO_INCREMENT,
	`user_id` INT NOT NULL,
	`season` INT NOT NULL,
	`season_rws` FLOAT NOT NULL DEFAULT '0',
	`season_elo` FLOAT NOT NULL DEFAULT '0',
	`season_rank` VARCHAR(2) NOT NULL DEFAULT '0',
	`season_roundsplayed` INT NOT NULL DEFAULT '0',
	PRIMARY KEY (`id`) USING BTREE,
	INDEX `fk_user_id` (`user_id`) USING BTREE
)
COLLATE='utf8mb4_unicode_ci'
;
