-- Add up migration script here
CREATE TABLE IF NOT EXISTS `external_links`
(
    `id`         VARCHAR(36) NOT NULL,
    `name`       VARCHAR(63) NOT NULL,
    `created_at` DATETIME(3) NOT NULL,
    `updated_at` DATETIME(3) NOT NULL,
    `deleted_at` DATETIME(3) DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `idx_external_links_deleted_at` (`deleted_at`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  DEFAULT COLLATE = utf8mb4_general_ci;