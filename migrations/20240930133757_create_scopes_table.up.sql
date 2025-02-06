-- Add up migration script here
CREATE TABLE IF NOT EXISTS `scopes`
(
    `id`             VARCHAR(36) NOT NULL,
    `application_id` VARCHAR(36) NOT NULL,
    `created_at`     DATETIME(3) NOT NULL,
    `updated_at`     DATETIME(3) NOT NULL,
    `deleted_at`     DATETIME(3) DEFAULT NULL,
    PRIMARY KEY (`id`),
    INDEX `idx_scopes_deleted_at` (`deleted_at`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  DEFAULT COLLATE = utf8mb4_general_ci;

ALTER TABLE `scopes`
    ADD CONSTRAINT `fk_scopes_application_id`
        FOREIGN KEY (`application_id`)
            REFERENCES `applications` (`id`) ON DELETE CASCADE;