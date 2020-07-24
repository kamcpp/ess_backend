CREATE TABLE id_verify_request (
    id SERIAL PRIMARY KEY,
    reference VARCHAR(32) NOT NULL UNIQUE,
    secret VARCHAR(32) NOT NULL UNIQUE,
    create_utc_dt TIMESTAMP NOT NULL,
    expire_utc_dt TIMESTAMP NOT NULL,
    employee_id INTEGER REFERENCES employee(id) NOT NULL
)
