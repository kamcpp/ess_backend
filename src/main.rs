#[macro_use]
extern crate diesel;

mod domain;
mod models;
mod schema;

use std::sync::{Arc, Mutex};
use std::ops::Deref;

use tide::{Request, Response, Result};

use domain::Employee;

use models::{HelloRequest, HelloResponse, EmployeeModel};

use diesel::insert_into;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use r2d2_diesel::ConnectionManager;

struct ServiceState {
    conn_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl ServiceState {
    fn new() -> Self {
        let database_url = "postgres://simurgh_da:3ME8MCrbsSsxfneJ8Bg4KH7wu@localhost/simurgh_db";
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Self {
            conn_pool: r2d2::Pool::builder().build(manager).expect("Failed to create PostgreSQL connection pool."),
        }
    }
}

type SharedSyncState = Arc<Mutex<ServiceState>>;

async fn handle_add_employee(mut req: Request<SharedSyncState>) -> Result<Response> {

    // Try to parse the request body containing the employee model
    let employee_model: EmployeeModel;
    match req.body_json().await {
        Ok(parsed_employee_model) => {
            employee_model = parsed_employee_model;
        },
        Err(err) => {
                    println!("{:?}", err);
            return Ok(Response::builder(500).body(format!("{}", err)).build());
        }
    }

    // This locks the 'state' and unlocks it when returning from function
    let state = req.state().lock().unwrap();

    // Get a conneciton from the pool
    let conn = state.conn_pool.get().expect("Cannot create connection!");

    use schema::employee::dsl::*;

    // Create the values for the new employee
    let values = (
        employee_nr.eq(employee_model.employee_nr),
        first_name.eq(employee_model.first_name),
        second_name.eq(employee_model.second_name),
        username.eq(employee_model.username),
        office_email.eq(employee_model.office_email),
        mobile.eq(employee_model.mobile),
    );

    // Try to insert the new employee
    match insert_into(employee).values(values).execute(conn.deref()) {
        Ok(_) => {
            return Ok(Response::builder(200).body("ok".to_string()).build());
        },
        Err(err) => {
            match err {
                diesel::result::Error::DatabaseError(kind, info) => {
                    match kind {
                        diesel::result::DatabaseErrorKind::UniqueViolation => {
                            return Ok(Response::builder(409).body(format!("{:?}", info)).build());
                        },
                        _ => {
                    println!("{:?}", info);
                            return Ok(Response::builder(500).body(format!("{:?}", info)).build());
                        },
                    }
                },
                error => {
                    println!("{:?}", error);
                    return Ok(Response::builder(500).body(format!("{}", error)).build());
                },
            }
        }
    }
}

async fn handle_get_all_employees(req: Request<SharedSyncState>) -> Result<Response> {

    // This locks the 'state' and unlocks it when returning from function
    let state = req.state().lock().unwrap();

    // Get a conneciton from the pool
    let conn = state.conn_pool.get().expect("Cannot create connection!");

    use schema::employee::dsl::*;

    // Read all employees
    match employee.load::<Employee>(conn.deref()) {
        Ok(employees) => {
            let to_return: Vec<EmployeeModel> = employees.iter().map(|e|
                EmployeeModel {
                    id: Some(e.id),
                    employee_nr: e.employee_nr.clone(),
                    first_name: e.first_name.clone(),
                    second_name: e.second_name.clone(),
                    username: e.username.clone(),
                    office_email: e.office_email.clone(),
                    mobile: e.mobile.clone(),
            }).collect();
            match serde_json::to_string(&to_return) {
                Ok(body_str) => {
                    return Ok(Response::builder(200).body(body_str).build());
                },
                Err(err) => {
                    return Ok(Response::builder(500).body(format!("{}", err)).build());
                },
            };
        },
        Err(err) => {
            return Ok(Response::builder(500).body(format!("{}", err)).build());
        }
    }
}

async fn handle_hello(mut req: Request<SharedSyncState>) -> Result<String> {
    let hello_req: HelloRequest = req.body_json().await?;
    let hello_resp = HelloResponse { greeting: format!("Hello, {}!", hello_req.name), };
    Ok(serde_json::to_string(&hello_resp)?)
}

#[async_std::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    let state = Arc::new(Mutex::new(ServiceState::new()));
    let mut app = tide::with_state(state);
    app.at("/api/hello").post(handle_hello);
    app.at("/api/employee").post(handle_add_employee);
    app.at("/api/employee/all").get(handle_get_all_employees);
    app.listen("0.0.0.0:9090").await?;
    Ok(())
}
