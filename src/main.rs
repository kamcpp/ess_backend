#[macro_use]
extern crate diesel;

mod schema;

use std::ops::Deref;

use diesel::insert_into;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

use schema::employee;

#[derive(Insertable)]
#[table_name = "employee"]
struct Employee {
    id: i32,
    employee_nr: String,
    first_name: String,
    second_name: String,
    username: String,
    office_email: String,
    mobile: String,
}

fn main() {
    println!("1");
    let database_url = "postgres://simurgh_da:3ME8MCrbsSsxfneJ8Bg4KH7wu@localhost/simurgh_db";
    println!("2");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    println!("3");
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create PostgreSQL connection pool.");
    println!("4");
    let conn = pool.get().unwrap();

    println!("5");
    use schema::employee::dsl::*;

    let e = Employee {
        id: 100,
        employee_nr: "500321".to_string(),
        first_name: "Steve".to_string(),
        second_name: "Anderson".to_string(),
        username: "steve_a".to_string(),
        office_email: "steve@labcrypto.org".to_string(),
        mobile: "+1 (123) 456–7890".to_string(),
    };

    insert_into(employee).values(&e).execute(conn.deref()).expect("Insert failed!");

    println!("Ok!");
}
