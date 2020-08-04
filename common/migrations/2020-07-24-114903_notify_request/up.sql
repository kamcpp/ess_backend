CREATE TABLE notify_request (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    create_utc_dt TIMESTAMP NOT NULL,
    expire_utc_dt TIMESTAMP NOT NULL,
    send_utc_dt TIMESTAMP,
    employee_id INTEGER REFERENCES employee(id) NOT NULL
)
