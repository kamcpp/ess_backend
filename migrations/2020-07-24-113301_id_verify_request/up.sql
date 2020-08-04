CREATE TABLE id_verify_request (
    id SERIAL PRIMARY KEY,
    reference VARCHAR(32) NOT NULL UNIQUE,
    secret VARCHAR(32) NOT NULL UNIQUE,
    active BOOLEAN NOT NULL,
    create_utc_dt TIMESTAMP NOT NULL,
    expire_utc_dt TIMESTAMP NOT NULL,
    verify_utc_dt TIMESTAMP,
    employee_id INTEGER REFERENCES employee(id) NOT NULL
)
