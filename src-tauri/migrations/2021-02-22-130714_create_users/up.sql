CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(60) NOT NULL,
    password VARCHAR(140) NOT NULL,
    full_name VARCHAR(30) NOT NULL,
    active BOOLEAN NOT NULL,
    last_login TIMESTAMP NOT NULL DEFAULT current_timestamp
);

INSERT INTO users(username, password, full_name, active) VALUES('admin', '$2y$05$mw56Wls35HoufQH7QipJnOzqzVmZuwcVUojcqQxKZ5hcG8aBdZRo.', 'SuperAdmin', true);