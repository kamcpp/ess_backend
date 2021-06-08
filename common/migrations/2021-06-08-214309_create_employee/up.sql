CREATE TABLE employee (
    first_name VARCHAR(64) NOT NULL,
    second_name VARCHAR(64) NOT NULL,
    username VARCHAR(16) PRIMARY KEY,
    totp_secret VARCHAR(64) NOT NULL
)
