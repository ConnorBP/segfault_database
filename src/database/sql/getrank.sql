SELECT id, display_name, rws, FIND_IN_SET( rws, (
SELECT GROUP_CONCAT( rws
ORDER BY rws DESC ) 
FROM users )
) AS rank
FROM users
WHERE id = 1
ORDER BY rank
