-- Your SQL goes here
CREATE TABLE heroes (
  id INT(11) PRIMARY KEY AUTO_INCREMENT,
  `name` VARCHAR(60) NOT NULL,
  identity VARCHAR(60) NOT NULL,
  hometown VARCHAR(60) NOT NULL,
  age INT(11) NOT NULL
)