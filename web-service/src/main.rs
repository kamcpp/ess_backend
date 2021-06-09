mod models;

use std::sync::{Arc, Mutex};
use std::ops::Deref;
use std::env;

use tide::{Request, Response, Result};

use common::schema;
use common::domain::{Employee};

use models::{EmployeeModel, IdentityVerificationRequestModel};

use google_authenticator::{GoogleAuthenticator, ErrorCorrectionLevel};

>> ADD REQUIRED DEPENDENCIES HERE <<

#[derive(Debug, Clone)]
struct IdentityVerificationError;

struct ServiceState {
    conn_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl ServiceState {
    fn new() -> Self {
        let user = env::var("POSTGRES_USER").unwrap_or("ess_da".to_string());
        let password = env::var("POSTGRES_PASSWORD").unwrap_or("not-set".to_string());
        let addr = env::var("POSTGRES_ADDR").unwrap_or("localhost".to_string());
        let db = env::var("POSTGRES_DB").unwrap_or("ess_db".to_string());

        >> FILL HERE <<
    }
}

type SharedSyncState = Arc<Mutex<ServiceState>>;

enum VariantError {
    DieselError(Error),
    IdentityVerificationError,
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
            VariantError::IdentityVerificationError => write!(f, "identity verification error"),
        }
    }
}

async fn handle_id_verify_req(mut req: Request<SharedSyncState>) -> Result<Response> {
    >> FILL HERE <<
}

async fn handle_add_employee(mut req: Request<SharedSyncState>) -> Result<Response> {
    >> FILL HERE <<
}

async fn handle_update_employee(mut req: Request<SharedSyncState>) -> Result<Response> {
    >> FILL HERE <<
}

async fn handle_delete_employee(req: Request<SharedSyncState>) -> Result<Response> {
    >> FILL HERE <<
}

async fn handle_get_all_employees(req: Request<SharedSyncState>) -> Result<Response> {
    >> FILL HERE <<
}

async fn handle_get_employee(req: Request<SharedSyncState>) -> Result<Response> {
    >> FILL HERE <<
}

#[async_std::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    let state = Arc::new(Mutex::new(ServiceState::new()));
    println!("ESS web service is running now ...");
    let mut app = tide::with_state(state);
    app.at("/api/admin/employee").post(handle_add_employee);
    app.at("/api/admin/employee/:username").put(handle_update_employee);
    app.at("/api/admin/employee/:username").delete(handle_delete_employee);
    app.at("/api/admin/employee/all").get(handle_get_all_employees);
    app.at("/api/admin/employee/:username").get(handle_get_employee);
    app.at("/api/pam/verify").post(handle_id_verify_req);
    app.listen("0.0.0.0:9876").await.expect("Could not start web server!");
    Ok(())
}
