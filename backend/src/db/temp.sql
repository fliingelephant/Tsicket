SET FOREIGN_KEY_CHECKS = 0;

UPDATE notification SET read=1 WHERE notice_id='090';

SET FOREIGN_KEY_CHECKS = 1;