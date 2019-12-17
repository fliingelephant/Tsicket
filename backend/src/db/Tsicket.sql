SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for user account
-- ----------------------------
DROP TABLE IF EXISTS `user_account`;
CREATE TABLE `user_account`
(
    `account_id`  VARCHAR(32) NOT NULL,
    `tsinghua_id` VARCHAR(32),
    `nickname`    VARCHAR(255) CHARACTER SET utf8mb4 DEFAULT '',
    PRIMARY KEY (`account_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4;


-- ----------------------------
-- Records of user account
-- ----------------------------
INSERT INTO `user_account` (`account_id`) VALUES ('123');

-- ----------------------------
-- Table structure for sponsor account
-- ----------------------------

DROP TABLE IF EXISTS `sponsor_account`;
CREATE TABLE `sponsor_account`
(
    `account_id`    VARCHAR(32)                        NOT NULL UNIQUE,
    `sponsor_name`  VARCHAR(255) CHARACTER SET utf8mb4 NOT NULL,
    `password`      VARCHAR(255) CHARACTER SET utf8mb4 NOT NULL,
    `head_portrait` VARCHAR(255)                       NOT NULL DEFAULT 'img://',
    `email`         VARCHAR(255)                       NOT NULL,
    `phone_number`  VARCHAR(255)                       NOT NULL,
    PRIMARY KEY (`sponsor_name`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4;

-- ----------------------------
-- Records of sponsor account
-- ----------------------------+

INSERT INTO `sponsor_account` VALUES ('1', '2', '3', '9', '4', '5');
INSERT INTO `sponsor_account` VALUES ('6', '7', '8', '10', '8', '0');

-- ----------------------------
-- Table structure for administrator account
-- ----------------------------

DROP TABLE IF EXISTS `admin_account`;
CREATE TABLE `admin_account`
(
    `account_id` VARCHAR(32)                        NOT NULL UNIQUE,
    `admin_name` VARCHAR(255) CHARACTER SET utf8mb4 NOT NULL,
    `password`   VARCHAR(255) CHARACTER SET utf8mb4 NOT NULL,
    PRIMARY KEY (`admin_name`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4;

-- ----------------------------
-- Records of administrator account
-- ----------------------------

#INSERT INTO `admin_account` VALUES ('000', 'tsinghua', '123');

-- ----------------------------
-- Table structure for event
-- ----------------------------

DROP TABLE IF EXISTS `event`;
CREATE TABLE `event`
(
    `event_id`             VARCHAR(255)                       NOT NULL,
    `sponsor_name`         VARCHAR(255) CHARACTER SET utf8mb4 NOT NULL,
    `event_name`           VARCHAR(255) CHARACTER SET utf8mb4 NOT NULL,
    `start_time`           TIMESTAMP(0)                       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `end_time`             TIMESTAMP(0)                       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `event_type`           TINYINT                                     DEFAULT 0,
    `event_introduction`   VARCHAR(1024)                               DEFAULT NULL,
    `event_picture`        VARCHAR(255)                       NOT NULL DEFAULT 'img://',
    `event_capacity`       INT(32)                            NOT NULL,
    `current_participants` INT(32)                            NOT NULL DEFAULT 0,
    `left_tickets`         INT(32)                            NOT NULL,
    `event_status`         TINYINT                            NOT NULL,
    `event_location`       VARCHAR(255)                       NOT NULL,
    `event_time`           VARCHAR(255)                       NOT NULL DEFAULT '',
    PRIMARY KEY (`event_id`) USING BTREE,
    CONSTRAINT fk_event_spo FOREIGN KEY (`sponsor_name`) REFERENCES `sponsor_account` (`sponsor_name`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE = InnoDB CHARACTER SET = utf8mb4;

-- ----------------------------
-- Records of event
-- ----------------------------

INSERT INTO `event` VALUES ('1', '2', '3', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0, '简单的交流会', '11',  2, 0, 1, 1, '210中厅', '11');
INSERT INTO `event` VALUES ('0', '2', '3', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0, '简单的交流会', '22', 2, 0, 1, 1, '210中厅', '11');

-- ----------------------------
-- Table structure for ticket records
-- ----------------------------

DROP TABLE IF EXISTS `ticket_record`;
CREATE TABLE `ticket_record`
(
    `record_id`    VARCHAR(255) NOT NULL,
    `event_id`     VARCHAR(255) NOT NULL,
    `sponsor_name` VARCHAR(32)  NOT NULL,
    `user_id`      VARCHAR(32)  NOT NULL,
    `start_time`   TIMESTAMP(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `end_time`     TIMESTAMP(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`record_id`) USING BTREE,
    CONSTRAINT fk_record_event FOREIGN KEY (`event_id`) REFERENCES `event` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT fk_record_spo FOREIGN KEY (`sponsor_name`) REFERENCES `sponsor_account` (`sponsor_name`) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT fk_record_user FOREIGN KEY (`user_id`) REFERENCES `user_account` (`account_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE = InnoDB CHARACTER SET = utf8mb4;

-- ----------------------------
-- Records of ticket records
-- ----------------------------

#INSERT INTO `ticket_record` VALUES ('001', '000', 'zjr', '123', CURRENT_TIMESTAMP,CURRENT_TIMESTAMP);

-- ----------------------------
-- Table structure for like list
-- ----------------------------

DROP TABLE IF EXISTS `like`;
CREATE TABLE `like`
(
    `user_id`  VARCHAR(32)  NOT NULL,
    `event_id` VARCHAR(255) NOT NULL,
    CONSTRAINT fk_like_event FOREIGN KEY (`event_id`) REFERENCES `event` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT fk_like_user FOREIGN KEY (`user_id`) REFERENCES `user_account` (`account_id`) ON DELETE CASCADE ON UPDATE CASCADE,
    UNIQUE KEY `user_id` (`user_id`, `event_id`)
) ENGINE = InnoDB CHARACTER SET = utf8mb4;

-- ----------------------------
-- Records of like list
-- ----------------------------

INSERT INTO `like` VALUES ('123', '1');

-- ----------------------------
-- Table structure for follow list
-- ----------------------------

DROP TABLE IF EXISTS `follow`;
CREATE TABLE `follow`
(
    `user_id`      VARCHAR(32)  NOT NULL,
    `sponsor_name` VARCHAR(255) NOT NULL,
    CONSTRAINT fk_follow_sponsor FOREIGN KEY (`sponsor_name`) REFERENCES `sponsor_account` (`sponsor_name`) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT fk_follow_user FOREIGN KEY (`user_id`) REFERENCES `user_account` (`account_id`) ON DELETE CASCADE ON UPDATE CASCADE,
    UNIQUE KEY `user_id` (`user_id`, `sponsor_name`)
) ENGINE = InnoDB CHARACTER SET = utf8mb4;

-- ----------------------------
-- Records of follow list
-- ----------------------------

INSERT INTO `follow` VALUES ('123', '2');
INSERT INTO `follow` VALUES ('123', '7');

-- ----------------------------
-- Table structure for moment list
-- ----------------------------

DROP TABLE IF EXISTS `moment`;
CREATE TABLE `moment`
(
    `sponsor_name` VARCHAR(255) NOT NULL,
    `event_id`     VARCHAR(32)  NOT NULL,
    `moment_id`    VARCHAR(255) NOT NULL,
    `text`         TEXT,
    `pictures`     TEXT,
    `time`         TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_moment_sponsor FOREIGN KEY (`sponsor_name`) REFERENCES `sponsor_account` (`sponsor_name`) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT fk_moment_event FOREIGN KEY (`event_id`) REFERENCES `event` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE = InnoDB CHARACTER SET = utf8mb4;

-- ----------------------------
-- Records of follow list
-- ----------------------------

INSERT INTO moment (`sponsor_name`, `event_id`, `moment_id`) VALUES ('2', '0', '999');

-- ----------------------------
-- Table structure for push list
-- ----------------------------

DROP TABLE IF EXISTS `notification`;
CREATE TABLE `notification`
(
    `sponsor_name` VARCHAR(255) NOT NULL,
    `event_id`     VARCHAR(32)  NOT NULL,
    `user_id`      VARCHAR(32)  NOT NULL,
    `notice_id`    VARCHAR(255) NOT NULL,
    `text`         TEXT,
    `time`         TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `read`         TINYINT               DEFAULT 0,
    CONSTRAINT fk_push_sponsor FOREIGN KEY (`sponsor_name`) REFERENCES `sponsor_account` (`sponsor_name`) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT fk_push_event FOREIGN KEY (`event_id`) REFERENCES `event` (`event_id`) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT fk_push_user FOREIGN KEY (`user_id`) REFERENCES `user_account` (`account_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE = InnoDB CHARACTER SET = utf8mb4;

-- ----------------------------
-- Records of push list
-- ----------------------------


INSERT INTO notification (`sponsor_name`, `event_id`, `user_id`, `notice_id`, `text`) VALUES ('2', '0', '123', '090', '活动取消');


SET FOREIGN_KEY_CHECKS = 1;
