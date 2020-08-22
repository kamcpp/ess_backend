mod models;
mod dao;
mod diesel_dao;
mod service;

use std::sync::{Arc, Mutex};
use std::ops::Deref;
use std::env;

use diesel_dao::DieselEmployeeDao;
use service::Service;
use chrono::{Utc, Duration, NaiveDateTime, DateTime};
use rand::Rng;
use rand::distributions::Alphanumeric;
use tide::{Request, Response, Result};
use common::schema;
use common::domain::{Employee, IdentityVerifyRequest};
use models::{HelloRequest, HelloResponse, EmployeeModel,
             NewIdentityVerifyRequestModel, NewIdentityVerifyResponseModel,
             CheckIdentityVerifyRequestModel};
use diesel::{insert_into, update, delete};
use diesel::result::{Error, DatabaseErrorKind};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

type SharedConnPool = Arc<Mutex<r2d2::Pool<ConnectionManager<PgConnection>>>>;

struct ServiceState {
    conn_pool: SharedConnPool,
    service: Service<diesel::result::Error>,
}

impl ServiceState {
    fn new() -> Self {
        let user = env::var("POSTGRES_USER").unwrap_or("simurgh_da".to_string());
        let password = env::var("POSTGRES_PASSWORD").unwrap_or("not-set".to_string());
        let addr = env::var("POSTGRES_ADDR").unwrap_or("localhost".to_string());
        let db = env::var("POSTGRES_DB").unwrap_or("simurgh_db".to_string());

        let database_url = format!("postgres://{}:{}@{}/{}", user, password, addr, db);
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        println!("Conneciton pool is created.");
        let conn_pool = Arc::new(Mutex::new(r2d2::Pool::builder().build(manager).expect("Failed to create PostgreSQL connection pool!")));
        let conn = conn_pool.lock().unwrap().get().expect("Cannot get a connection from pool!");
        Self {
            conn_pool: conn_pool.clone(),
            service: Service::new(
                Box::new(DieselEmployeeDao::new(conn)),
            ),
        }
    }
}

type SharedSyncState = Arc<Mutex<ServiceState>>;

enum VariantError {
    DieselError(Error),
    IdentityVerifyError,
}

impl From<Error> for VariantError {
    fn from(error: Error) -> Self {
        VariantError::DieselError(error)
    }
}

impl std::fmt::Display for VariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariantError::DieselError(err) => write!(f, "{}", err),
            VariantError::IdentityVerifyError => write!(f, "identity verification error"),
        }
    }
}

fn gen_rand_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect::<String>()
}

async fn handle_new_id_verify_req(mut req: Request<SharedSyncState>) -> Result<Response> {
    // Try to parse the request body containing thet model
    let model: NewIdentityVerifyRequestModel = match req.body_json().await {
        Ok(parsed_model) => parsed_model,
        Err(err) => return Ok(Response::builder(500).body(format!("{}", err)).build()),
    };
    let now = Utc::now();
    let client_now = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(model.client_utc_dt, 0), Utc);
    if now.signed_duration_since(client_now).num_minutes().abs() > 5 {
        return Ok(Response::builder(400).body("".to_string()).build());
    }
    println!("{:?}", model);
    // This locks the 'state' and unlocks it when returning from function
    let state = req.state().lock().unwrap();
    // Get a conneciton from the pool
    let conn = state.conn_pool.lock().unwrap().get().expect("Cannot get a connection from pool!");
    use schema::employee::dsl::*;
    use schema::id_verify_request::dsl::*;
    use schema::notify_request::dsl::*;
    match conn.transaction::<_, _, _>(|| {
        let employees = employee.filter(username.eq(model.username)).load::<Employee>(conn.deref())?;
        if employees.len() == 0 {
            return Err(Error::NotFound);
        }
        let target_employee = &employees[0];
        let secret_value = gen_rand_string(8);
        let reference_value = gen_rand_string(16);
        update(id_verify_request.filter(schema::id_verify_request::dsl::employee_id.eq(target_employee.id))).set((
            active.eq(false),
        )).execute(conn.deref())?;
        insert_into(id_verify_request).values((
            reference.eq(reference_value.clone()),
            secret.eq(secret_value.clone()),
            active.eq(true),
            schema::id_verify_request::dsl::create_utc_dt.eq(now.naive_utc()),
            schema::id_verify_request::dsl::expire_utc_dt.eq((now + Duration::minutes(5)).naive_utc()),
            schema::id_verify_request::dsl::employee_id.eq(target_employee.id),
        )).execute(conn.deref())?;
        insert_into(notify_request).values((
            title.eq("Simurgh Identity Verification System"),
            body.eq(format!("Your code: {}", secret_value)),
            schema::notify_request::dsl::create_utc_dt.eq(now.naive_utc()),
            schema::notify_request::dsl::expire_utc_dt.eq((now + Duration::minutes(15)).naive_utc()),
            schema::notify_request::dsl::employee_id.eq(target_employee.id),
        )).execute(conn.deref())?;
        Ok(reference_value)
    }) {
        Ok(reference_value) => {
            let response = NewIdentityVerifyResponseModel {
                reference: reference_value,
                server_utc_dt: now.timestamp(),
            };
            match serde_json::to_string(&response) {
                Ok(body_str) => return Ok(Response::builder(200).body(body_str).build()),
                Err(err) => return Ok(Response::builder(500).body(format!("{}", err)).build()),
            };
        },
        Err(err) => match err {
            diesel::result::Error::NotFound => return Ok(Response::builder(404).body("not found".to_string()).build()),
            error => return Ok(Response::builder(500).body(format!("{}", error)).build()),
        },
    }
}

async fn handle_check_id_verify_req(mut req: Request<SharedSyncState>) -> Result<Response> {

    // Try to parse the request body containing thet model
    let model: CheckIdentityVerifyRequestModel = match req.body_json().await {
        Ok(parsed_model) => parsed_model,
        Err(err) => return Ok(Response::builder(500).body(format!("{}", err)).build()),
    };
    let now = Utc::now();
    let client_now = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(model.client_utc_dt, 0), Utc);
    if now.signed_duration_since(client_now).num_minutes().abs() > 5 {
        return Ok(Response::builder(400).body("".to_string()).build());
    }
    // This locks the 'state' and unlocks it when returning from function
    let state = req.state().lock().unwrap();
    // Get a conneciton from the pool
    let conn = state.conn_pool.lock().unwrap().get().expect("Cannot get a connection from pool!");
    use schema::id_verify_request::dsl::*;
    match conn.transaction::<_, _, _>(|| {
        let id_verify_requests =
            id_verify_request.filter(
                reference.eq(model.reference)
                .and(active.eq(true))
                .and(expire_utc_dt.gt(now.naive_utc())))
            .load::<IdentityVerifyRequest>(conn.deref())?;
        if id_verify_requests.len() == 0 {
            return Err(VariantError::DieselError(Error::NotFound));
        }
        let target_request = &id_verify_requests[0];
        if target_request.secret != model.client_secret {
            return Err(VariantError::IdentityVerifyError);
        }
        update(target_request).set((
            active.eq(false),
            verify_utc_dt.eq(Utc::now().naive_utc()),
        )).execute(conn.deref())?;
        Ok(())
    }) {
        Ok(_) => return Ok(Response::builder(200).body("".to_string()).build()),
        Err(err) => match err {
            VariantError::IdentityVerifyError => return Ok(Response::builder(403).body("identity verification failed".to_string()).build()),
            VariantError::DieselError(Error::NotFound) => return Ok(Response::builder(404).body("not found".to_string()).build()),
            error => return Ok(Response::builder(500).body(format!("{}", error)).build()),
        },
    }
}

async fn handle_add_employee(mut req: Request<SharedSyncState>) -> Result<Response> {
    // Try to parse the request body containing the employee model
    let employee_model: EmployeeModel = match req.body_json().await {
        Ok(parsed_employee_model) => parsed_employee_model,
        Err(err) => return Ok(Response::builder(500).body(format!("{}", err)).build()),
    };
    if employee_model.id.is_some() {
        return Ok(Response::builder(400).body("'id' must not be provided").build());
    }
    if employee_model.employee_nr.is_none() {
        return Ok(Response::builder(400).body("'employee_nr' is mandatory").build());
    }
    if employee_model.first_name.is_none() {
        return Ok(Response::builder(400).body("'first_name' is mandatory").build());
    }
    if employee_model.second_name.is_none() {
        return Ok(Response::builder(400).body("'second_name' is mandatory").build());
    }
    if employee_model.username.is_none() {
        return Ok(Response::builder(400).body("'username' is mandatory").build());
    }
    if employee_model.office_email.is_none() {
        return Ok(Response::builder(400).body("'office_email' is mandatory").build());
    }
    if employee_model.mobile.is_none() {
        return Ok(Response::builder(400).body("'mobile' is mandatory").build());
    }
    let mut state = req.state().lock().unwrap();
    match state.service.add_employee(employee_model) {
        Ok(_) => return Ok(Response::builder(200).body("ok".to_string()).build()),
        Err(err) => match err {
            Error::DatabaseError(kind, info) => match kind {
                DatabaseErrorKind::UniqueViolation => return Ok(Response::builder(409).body(format!("{:?}", info)).build()),
                _ => return Ok(Response::builder(500).body(format!("{:?}", info)).build()),
            },
            error => return Ok(Response::builder(500).body(format!("{}", error)).build()),
        },
    }
}

async fn handle_update_employee(mut req: Request<SharedSyncState>) -> Result<Response> {
    // Try to parse the request body containing the employee model
    let mut employee_model: EmployeeModel = match req.body_json().await {
        Ok(parsed_employee_model) => parsed_employee_model,
        Err(err) => return Ok(Response::builder(500).body(format!("{}", err)).build()),
    };
    // Read 'employee id' from the path
    let employee_id: i32 = match req.param("id") {
        Ok(value) => value,
        Err(err) => return Ok(Response::builder(400).body(format!("'id' is mandatory and must be provided as part of path: {}", err)).build()),
    };
    employee_model.id = Some(employee_id);
    let mut state = req.state().lock().unwrap();
    match state.service.update_employee(employee_model) {
        Ok(_) => return Ok(Response::builder(200).body("ok".to_string()).build()),
        Err(err) => match err {
            Error::DatabaseError(kind, info) => match kind {
                DatabaseErrorKind::UniqueViolation => return Ok(Response::builder(409).body(format!("{:?}", info)).build()),
                _ => return Ok(Response::builder(500).body(format!("{:?}", info)).build()),
            },
            Error::NotFound => return Ok(Response::builder(404).body("not found".to_string()).build()),
            error => return Ok(Response::builder(500).body(format!("{}", error)).build()),
        },
    }
}

async fn handle_delete_employee(req: Request<SharedSyncState>) -> Result<Response> {
    // Read 'employee id' from the path
    let employee_id: i32 = match req.param("id") {
        Ok(value) => value,
        Err(err) => return Ok(Response::builder(400).body(format!("'id' is mandatory and must be provided as part of path: {}", err)).build()),
    };
    let mut state = req.state().lock().unwrap();
    match state.service.delete_employee(employee_id) {
        Ok(_) => return Ok(Response::builder(200).body("ok".to_string()).build()),
        Err(err) => match err {
            Error::NotFound => return Ok(Response::builder(404).body("not found".to_string()).build()),
            error => return Ok(Response::builder(500).body(format!("{}", error)).build()),
        },
    }
}

async fn handle_get_all_employees(req: Request<SharedSyncState>) -> Result<Response> {
    let mut state = req.state().lock().unwrap();
    match state.service.get_all_employees() {
        Ok(employees) => {
            match serde_json::to_string(&employees) {
                Ok(body_str) => return Ok(Response::builder(200).body(body_str).build()),
                Err(err) => return Ok(Response::builder(500).body(format!("{}", err)).build()),
            };
        },
        Err(err) => return Ok(Response::builder(500).body(format!("{}", err)).build()),
    }
}

async fn handle_get_employee(req: Request<SharedSyncState>) -> Result<Response> {
    // Read 'employee id' from the path
    let employee_id: i32 = match req.param("id") {
        Ok(value) => value,
        Err(err) => return Ok(Response::builder(400).body(format!("'id' is mandatory and must be provided as part of path: {}", err)).build()),
    };
    let mut state = req.state().lock().unwrap();
    match state.service.get_employee(employee_id) {
        Ok(employee) => {
            match serde_json::to_string(&employee) {
                Ok(body_str) => return Ok(Response::builder(200).body(body_str).build()),
                Err(err) => return Ok(Response::builder(500).body(format!("{}", err)).build()),
            };
        },
        Err(err) => match err {
            Error::NotFound => return Ok(Response::builder(404).body("not found".to_string()).build()),
            error => return Ok(Response::builder(500).body(format!("{}", error)).build()),
        },
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
    println!("Simurgh web service is running now ...");
    let mut app = tide::with_state(state);
    app.at("/api/pam/hello").post(handle_hello);
    app.at("/api/pam/id_verify_req/new").post(handle_new_id_verify_req);
    app.at("/api/pam/id_verify_req/check").post(handle_check_id_verify_req);
    app.at("/api/admin/employee").post(handle_add_employee);
    app.at("/api/admin/employee/:id").put(handle_update_employee);
    app.at("/api/admin/employee/:id").delete(handle_delete_employee);
    app.at("/api/admin/employee/all").get(handle_get_all_employees);
    app.at("/api/admin/employee/:id").get(handle_get_employee);
    app.listen("0.0.0.0:9876").await.expect("Could not start web server!");
    Ok(())
}
