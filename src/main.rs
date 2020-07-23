#[macro_use]
extern crate diesel;

mod schema;

use diesel::prelude::*;
use diesel::insert_into;

use schema::employee;

#[derive(Insertable)]
#[table_name = "employee"]
struct Employee {
    id: u64,
    employee_nr: String,
    first_name: String,
    second_name: String,
    username: String,
    office_email: String,
    mobile: String,
}

fn main() {
    let database_url = "postgres://simurgh_da:3ME8MCrbsSsxfneJ8Bg4KH7wui1@localhost/simurgh_db";
    let conn = PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    use schema::employee::dsl::*;

    let e = Employee {
        id: 100,
        employee_nr: "500321".to_string(),
        first_name: "Steve".to_string(),
        second_name: "Anderson".to_string(),
        username: "steve_a".to_string(),
        office_email: "steve@labcrypto.org".to_string(),
        mobile: "+132132145".to_string(),
    };

    insert_into(employee).values(&e).execute(conn);
}
