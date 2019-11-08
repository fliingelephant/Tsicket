SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for user account
-- ----------------------------
DROP TABLE IF EXISTS `user_account`;
CREATE TABLE `user_account`  (
  `account_id` VARCHAR(32) NOT NULL,
  `nickname` VARCHAR(255) CHARACTER SET utf8mb4 DEFAULT '',
  PRIMARY KEY (`account_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4;

-- ----------------------------
-- Records of user account
-- ----------------------------
INSERT INTO `user_account` (`account_id`) VALUES ('123');

-- ----------------------------
-- Table structure for publisher account
-- ----------------------------

DROP TABLE IF EXISTS `publisher_account`;
CREATE TABLE `publisher_account`  (
    `account_id` VARCHAR(32) NOT NULL,
    `username` VARCHAR(255) CHARACTER SET utf8mb4 NOT NULL,
    `password` VARCHAR(255) CHARACTER SET utf8mb4 NOT NULL,
    PRIMARY KEY(`account_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4;

-- ----------------------------
-- Records of publisher account
-- ----------------------------

INSERT INTO `publisher_account` VALUES ('123', 'zjr', '123');

-- ----------------------------
-- Table structure for administrator account
-- ----------------------------

DROP TABLE IF EXISTS `admin_account`;
CREATE TABLE `admin_account`  (
    `account_id` VARCHAR(32) NOT NULL,
    `username` VARCHAR(255) CHARACTER SET utf8mb4 NOT NULL,
    `password` VARCHAR(255) CHARACTER SET utf8mb4 NOT NULL,
    PRIMARY KEY(`account_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4;

-- ----------------------------
-- Records of administrator account
-- ----------------------------

INSERT INTO `admin_account` VALUES ('000', 'tsinghua', '123');

-- ----------------------------
-- Table structure for event
-- ----------------------------

DROP TABLE IF EXISTS `event`;
CREATE TABLE `event`  (
    `event_id` VARCHAR(255) NOT NULL,
    `publisher_id` VARCHAR(32) NOT NULL,
    `publisher_name` VARCHAR(255) CHARACTER SET utf8mb4 NOT NULL ,
    `event_name` VARCHAR(255) CHARACTER SET utf8mb4 NOT NULL ,
    `start_time` TIMESTAMP(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `end_time` TIMESTAMP(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `event_type` TINYINT DEFAULT 0 ,
    `event_introduction` VARCHAR(1024) DEFAULT '',
    `event_capacity` INT(32) NOT NULL ,
    `current_participants` INT(32) NOT NULL DEFAULT 0,
    `left_tickets` INT(32) NOT NULL ,
    `event_status` TINYINT NOT NULL ,
    `event_location` VARCHAR(255) NOT NULL ,
    PRIMARY KEY(`event_id`) USING BTREE,
    CONSTRAINT fk_event_pub FOREIGN KEY (`publisher_id`) REFERENCES `publisher_account`(`account_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE = InnoDB CHARACTER SET = utf8mb4;

-- ----------------------------
-- Records of event
-- ----------------------------

INSERT INTO `event` VALUES ('000', '123', '210', '交流', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0, '简单的交流会', 20, 1, 19, 0, '210中厅');

-- ----------------------------
-- Table structure for ticket records
-- ----------------------------

DROP TABLE IF EXISTS `ticket_record`;
CREATE TABLE `ticket_record`  (
    `record_id` VARCHAR(255) NOT NULL,
    `event_id` VARCHAR(255) NOT NULL,
    `publisher_id` VARCHAR(32) NOT NULL,
    `user_id` VARCHAR(32) NOT NULL,
    `start_time` TIMESTAMP(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `end_time` TIMESTAMP(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY(`record_id`) USING BTREE,
    CONSTRAINT fk_record_event FOREIGN KEY (`event_id`) REFERENCES `event`(`event_id`) ON DELETE CASCADE ON UPDATE CASCADE ,
    CONSTRAINT fk_record_pub FOREIGN KEY (`publisher_id`) REFERENCES `publisher_account`(`account_id`) ON DELETE CASCADE ON UPDATE CASCADE ,
    CONSTRAINT fk_record_user FOREIGN KEY (`user_id`) REFERENCES `user_account`(`account_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE = InnoDB CHARACTER SET = utf8mb4;

-- ----------------------------
-- Records of ticket records
-- ----------------------------

INSERT INTO `ticket_record` VALUES ('001', '000', '123', '123', CURRENT_TIMESTAMP,CURRENT_TIMESTAMP);


SET FOREIGN_KEY_CHECKS = 1;
