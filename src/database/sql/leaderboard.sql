SELECT `id`, FIND_IN_SET( rws, (
SELECT GROUP_CONCAT( rws
ORDER BY rws DESC ) 
FROM users WHERE `rounds_total` > {} )
) AS `rank`, `rws`, `rounds_total`, `display_name`
FROM users
WHERE `rounds_total` > {}
ORDER BY `rank`
LIMIT {}