DELIMITER $$
/* This is a complete statement, not part of the procedure, so use the custom delimiter $$ */
DROP PROCEDURE dank$$

/* Now start the procedure code */
CREATE PROCEDURE dank (IN user_id INT)
BEGIN    
  /* Inside the procedure, individual statements terminate with ; */
	SELECT id, display_name, rws, FIND_IN_SET( rws, (
	SELECT GROUP_CONCAT( rws
	ORDER BY rws DESC ) 
	FROM users )
	) AS rank
	FROM users
	WHERE id = user_id
	ORDER BY rank;

/* whole procedure ends with the custom delimiter */
END$$

/* Finally, reset the delimiter to the default ; */
DELIMITER ;