-- CreateTable
CREATE TABLE `base_something` (
    `id` VARCHAR(191) NOT NULL,
    `name` VARCHAR(191) NOT NULL,

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `open_something` (
    `id` VARCHAR(191) NOT NULL,
    `open_reason` VARCHAR(191) NOT NULL,
    `base_id` VARCHAR(191) NOT NULL,

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- AddForeignKey
ALTER TABLE `open_something` ADD CONSTRAINT `open_something_base_id_fkey` FOREIGN KEY (`base_id`) REFERENCES `base_something`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;
