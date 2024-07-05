-- Your SQL goes here
CREATE TABLE `messages`(
	`id` INTEGER PRIMARY KEY AUTOINCREMENT,
	`text` TEXT NOT NULL,
	`added_at` TIMESTAMP NOT NULL,
	`notify_at` TIMESTAMP
);

