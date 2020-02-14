CREATE TABLE `comments` (
                            id INTEGER NOT NULL AUTO_INCREMENT PRIMARY KEY,
                            `author` VARCHAR(255) NOT NULL DEFAULT '',
                            `content` TEXT NOT NULL DEFAULT '',
                            `createdat` DATETIME NOT NULL DEFAULT '0000-00-00 00:00:00',
                            `post_id` INTEGER NOT NULL
)
    COLLATE='utf8mb4_general_ci'
    ENGINE=InnoDB
;
