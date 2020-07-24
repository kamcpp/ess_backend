CREATE TABLE employee (
    id SERIAL PRIMARY KEY,
    employee_nr VARCHAR(16) NOT NULL UNIQUE,
    first_name VARCHAR(64) NOT NULL,
    second_name VARCHAR(64) NOT NULL,
    username VARCHAR(16) NOT NULL UNIQUE,
    office_email VARCHAR(64) NOT NULL UNIQUE,
    mobile VARCHAR(16) NOT NULL
)
