SET FOREIGN_KEY_CHECKS = 0;

DROP TABLE IF EXISTS `sponsor_account`;
CREATE TABLE `sponsor_account`  (
                                    `account_id` VARCHAR(32) NOT NULL UNIQUE ,
                                    `sponsor_name` VARCHAR(255) CHARACTER SET utf8mb4 NOT NULL,
                                    `password` VARCHAR(255) CHARACTER SET utf8mb4 NOT NULL,
                                    `email` VARCHAR(255) NOT NULL,
                                    `phone_number` VARCHAR(255) NOT NULL,
                                    PRIMARY KEY(`sponsor_name`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4;

SET FOREIGN_KEY_CHECKS = 1;