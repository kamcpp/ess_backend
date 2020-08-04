table! {
    employee (id) {
        id -> Int4,
        employee_nr -> Varchar,
        first_name -> Varchar,
        second_name -> Varchar,
        username -> Varchar,
        office_email -> Varchar,
        mobile -> Varchar,
    }
}

table! {
    id_verify_request (id) {
        id -> Int4,
        reference -> Varchar,
        secret -> Varchar,
        active -> Bool,
        create_utc_dt -> Timestamp,
        expire_utc_dt -> Timestamp,
        verify_utc_dt -> Nullable<Timestamp>,
        employee_id -> Int4,
    }
}

table! {
    notify_request (id) {
        id -> Int4,
        title -> Text,
        body -> Text,
        create_utc_dt -> Timestamp,
        expire_utc_dt -> Timestamp,
        send_utc_dt -> Nullable<Timestamp>,
        employee_id -> Int4,
    }
}

joinable!(id_verify_request -> employee (employee_id));
joinable!(notify_request -> employee (employee_id));

allow_tables_to_appear_in_same_query!(
    employee,
    id_verify_request,
    notify_request,
);
