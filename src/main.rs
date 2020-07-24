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

    use schema::employee::dsl::*;

    let employee_model: EmployeeModel = req.body_json().await?;
    let mut state = req.state().lock().unwrap();
    let conn = state.conn_pool.get().expect("Cannot create connection!");

    let new_employee = Employee {
         // The id column is serial so PostgreSQL should use an internal seuqnce to generate the value upon insertion.
        id: None,
        employee_nr: employee_model.employee_nr,
        first_name: employee_model.first_name,
        second_name: employee_model.second_name,
        username: employee_model.username,
        office_email: employee_model.office_email,
        mobile: employee_model.mobile,
    };
    match insert_into(employee).values(&new_employee).execute(conn.deref()) {
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
                            return Ok(Response::builder(500).body(format!("{:?}", info)).build());
                        },
                    }
                },
                error => {
                    return Ok(Response::builder(500).body(format!("{}", error)).build());
                },
            }
        }
    }
}

async fn handle_get_all_employees(req: Request<SharedSyncState>) -> Result<String> {
    let state = req.state().lock().unwrap();
    let to_return: Vec<EmployeeModel> = Vec::new(); /*state.employees.iter().map(|employee| EmployeeModel {
        id: Some(employee.id),
        employee_nr: employee.employee_nr.clone(),
        first_name: employee.first_name.clone(),
        second_name: employee.second_name.clone(),
        username: employee.username.clone(),
        office_email: employee.office_email.clone(),
        mobile: employee.mobile.clone(),
    }).collect();*/
    Ok(serde_json::to_string(&to_return)?)
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
