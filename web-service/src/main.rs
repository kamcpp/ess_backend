mod models;
mod dao;
mod diesel_dao;
mod service;

use service::{
    Service,
    ServiceError,
};
use models::{
    HelloRequest,
    HelloResponse,
    EmployeeModel,
    NewIdentityVerifyRequestModel,
    CheckIdentityVerifyRequestModel,
};
use diesel_dao::{
    DieselTransactionContextBuilder,
    DieselTransactionContext,
    DieselEmployeeDao,
    DieselIdentityVerifyRequestDao,
    DieselNotifyRequestDao,
};

use std::sync::{Arc, Mutex};
use std::env;

use chrono::{Utc, NaiveDateTime, DateTime};
use diesel::result::{Error, DatabaseErrorKind};
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use tide::{Request, Response, Result};

struct AppState {
    service: Service<diesel::result::Error, DieselTransactionContext>,
}

impl AppState {
    fn new() -> Self {
        let user = env::var("POSTGRES_USER").unwrap_or("simurgh_da".to_string());
        let password = env::var("POSTGRES_PASSWORD").unwrap_or("not-set".to_string());
        let addr = env::var("POSTGRES_ADDR").unwrap_or("localhost".to_string());
        let db = env::var("POSTGRES_DB").unwrap_or("simurgh_db".to_string());

        let database_url = format!("postgres://{}:{}@{}/{}", user, password, addr, db);
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        println!("Conneciton pool is created.");
        let conn_pool = r2d2::Pool::builder().build(manager).expect("Failed to create PostgreSQL connection pool!");
        Self {
            service: Service::new(
                Box::new(DieselTransactionContextBuilder::new(conn_pool)),
                Box::new(DieselEmployeeDao::new()),
                Box::new(DieselIdentityVerifyRequestDao::new()),
                Box::new(DieselNotifyRequestDao::new()),
            ),
        }
    }
}

type SharedSyncState = Arc<Mutex<AppState>>;

async fn handle_new_id_verify_req(mut req: Request<SharedSyncState>) -> Result<Response> {
    let model: NewIdentityVerifyRequestModel = match req.body_json().await {
        Ok(parsed_model) => parsed_model,
        Err(err) => return Ok(Response::builder(500).body(format!("{}", err)).build()),
    };
    let now = Utc::now();
    let client_now = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(model.client_utc_dt, 0), Utc);
    if now.signed_duration_since(client_now).num_minutes().abs() > 5 {
        return Ok(Response::builder(400).body("".to_string()).build());
    }
    let mut state = req.state().lock().unwrap();
    match state.service.new_id_verify_req(model) {
        Ok(response) => {
            match serde_json::to_string(&response) {
                Ok(body_str) => return Ok(Response::builder(200).body(body_str.to_string()).build()),
                Err(err) => return Ok(Response::builder(500).body(format!("{}", err)).build()),
            }
        },
        Err(err) => match err {
            ServiceError::DaoError(err) => match err {
                diesel::result::Error::NotFound => return Ok(Response::builder(404).body("not found".to_string()).build()),
                error => return Ok(Response::builder(500).body(format!("{}", error)).build()),
            },
            _ => return Ok(Response::builder(500).body("unknown error").build()),
        },
    }
}

async fn handle_check_id_verify_req(mut req: Request<SharedSyncState>) -> Result<Response> {
    let model: CheckIdentityVerifyRequestModel = match req.body_json().await {
        Ok(parsed_model) => parsed_model,
        Err(err) => return Ok(Response::builder(500).body(format!("{}", err)).build()),
    };
    let now = Utc::now();
    let client_now = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(model.client_utc_dt, 0), Utc);
    if now.signed_duration_since(client_now).num_minutes().abs() > 5 {
        return Ok(Response::builder(400).body("".to_string()).build());
    }
    let mut state = req.state().lock().unwrap();
    match state.service.check_id_verify_req(model) {
        Ok(_) => return Ok(Response::builder(200).body("".to_string()).build()),
        Err(err) => match err {
            ServiceError::IdentityVerifyError => return Ok(Response::builder(403).body("identity verification failed".to_string()).build()),
            ServiceError::DaoError(Error::NotFound) => return Ok(Response::builder(404).body("not found".to_string()).build()),
            error => return Ok(Response::builder(500).body(format!("{}", error)).build()),
        },
    }
}

async fn handle_add_employee(mut req: Request<SharedSyncState>) -> Result<Response> {
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
            ServiceError::DaoError(err) => match err {
                Error::DatabaseError(kind, info) => match kind {
                    DatabaseErrorKind::UniqueViolation => return Ok(Response::builder(409).body(format!("{:?}", info)).build()),
                    _ => return Ok(Response::builder(500).body(format!("{:?}", info)).build()),
                },
                error => return Ok(Response::builder(500).body(format!("{}", error)).build()),
            },
            _ => return Ok(Response::builder(500).body("unknown error").build()),
        },
    }
}

async fn handle_update_employee(mut req: Request<SharedSyncState>) -> Result<Response> {
    let mut employee_model: EmployeeModel = match req.body_json().await {
        Ok(parsed_employee_model) => parsed_employee_model,
        Err(err) => return Ok(Response::builder(500).body(format!("{}", err)).build()),
    };
    let employee_id: i32 = match req.param("id") {
        Ok(value) => value,
        Err(err) => return Ok(Response::builder(400).body(format!("'id' is mandatory and must be provided as part of path: {}", err)).build()),
    };
    employee_model.id = Some(employee_id);
    let mut state = req.state().lock().unwrap();
    match state.service.update_employee(employee_model) {
        Ok(_) => return Ok(Response::builder(200).body("ok".to_string()).build()),
        Err(err) => match err {
            ServiceError::DaoError(err) => match err {
                Error::DatabaseError(kind, info) => match kind {
                    DatabaseErrorKind::UniqueViolation => return Ok(Response::builder(409).body(format!("{:?}", info)).build()),
                    _ => return Ok(Response::builder(500).body(format!("{:?}", info)).build()),
                },
                Error::NotFound => return Ok(Response::builder(404).body("not found".to_string()).build()),
                error => return Ok(Response::builder(500).body(format!("{}", error)).build()),
            },
            _ => return Ok(Response::builder(500).body("unknown error").build()),
        },
    }
}

async fn handle_delete_employee(req: Request<SharedSyncState>) -> Result<Response> {
    let employee_id: i32 = match req.param("id") {
        Ok(value) => value,
        Err(err) => return Ok(Response::builder(400).body(format!("'id' is mandatory and must be provided as part of path: {}", err)).build()),
    };
    let mut state = req.state().lock().unwrap();
    match state.service.delete_employee(employee_id) {
        Ok(_) => return Ok(Response::builder(200).body("ok".to_string()).build()),
        Err(err) => match err {
            ServiceError::DaoError(err) => match err {
                Error::NotFound => return Ok(Response::builder(404).body("not found".to_string()).build()),
                error => return Ok(Response::builder(500).body(format!("{}", error)).build()),
            },
            _ => return Ok(Response::builder(500).body("unknown error").build()),
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
            ServiceError::DaoError(err) => match err {
                Error::NotFound => return Ok(Response::builder(404).body("not found".to_string()).build()),
                error => return Ok(Response::builder(500).body(format!("{}", error)).build()),
            },
            _ => return Ok(Response::builder(500).body("unknown error").build()),
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
    let state = Arc::new(Mutex::new(AppState::new()));
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
