SET FOREIGN_KEY_CHECKS = 0;

ALTER TABLE moment ADD PRIMARY KEY (moment_id);
ALTER TABLE notification ADD PRIMARY KEY (notice_id);

SET FOREIGN_KEY_CHECKS = 1;