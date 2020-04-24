SELECT id, display_name, rws, rounds_total, FIND_IN_SET( rws, (
SELECT GROUP_CONCAT( rws
ORDER BY rws DESC ) 
FROM users WHERE rounds_total > 50 )
) AS rank
FROM users
WHERE rounds_total > 50
ORDER BY rank
