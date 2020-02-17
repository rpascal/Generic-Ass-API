CREATE TABLE posts (
  `id` INT NOT NULL,
  `title` VARCHAR(45) NOT NULL,
  `body` VARCHAR(45) NOT NULL,
  `published` TINYINT NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`));
