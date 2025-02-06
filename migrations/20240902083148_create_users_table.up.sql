-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS `users`
(
    `id`         VARCHAR(36)  NOT NULL,
    `email`      VARCHAR(127) NOT NULL,
    `password`   VARCHAR(191) NOT NULL,
    `lastname`   VARCHAR(63)  NOT NULL,
    `firstname`  VARCHAR(63)  NOT NULL,
    `created_at` DATETIME(3)  NOT NULL,
    `updated_at` DATETIME(3)  NOT NULL,
    `deleted_at` DATETIME(3) DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `email` (`email`),
    KEY `idx_users_password` (`password`),
    KEY `idx_users_deleted_at` (`deleted_at`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  DEFAULT COLLATE = utf8mb4_general_ci;