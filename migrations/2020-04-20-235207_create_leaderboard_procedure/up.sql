CREATE DEFINER=`segfault_users`@`%` PROCEDURE `leaderboard`(
	IN `maximum` INT
)
LANGUAGE SQL
NOT DETERMINISTIC
CONTAINS SQL
SQL SECURITY DEFINER
COMMENT ''
BEGIN    

	SELECT id, display_name, rws, FIND_IN_SET( rws, (
	SELECT GROUP_CONCAT( rws
	ORDER BY rws DESC ) 
	FROM users )
	) AS `rank`
	FROM users
	ORDER BY `rank`
	LIMIT maximum;


END