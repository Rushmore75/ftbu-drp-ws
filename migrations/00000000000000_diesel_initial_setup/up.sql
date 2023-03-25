CREATE TABLE teams (
	id int PRIMARY KEY,
	name VARCHAR NOT NULL
);

CREATE TABLE users (
	username CHAR(32) PRIMARY KEY,
	team int REFERENCES teams(id)
);
