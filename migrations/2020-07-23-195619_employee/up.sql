CREATE TABLE employee (
    id SERIAL PRIMARY KEY,
    employee_nr VARCHAR NOT NULL UNIQUE,
    first_name VARCHAR NOT NULL,
    second_name VARCHAR NOT NULL,
    username VARCHAR NOT NULL UNIQUE,
    office_email VARCHAR NOT NULL UNIQUE,
    mobile VARCHAR NOT NULL
)
