mod models;

use std::fs::File;
use std::sync::{Arc, Mutex};
use std::ops::Deref;
use std::env;

use async_std::task;

use tide::{Request, Response, Result};

use tide_rustls::rustls;
use tide_rustls::TlsListener;

use common::schema;
use common::domain::{Employee};

use models::{EmployeeModel, IdentityVerificationRequestModel};

use diesel::{insert_into, update, delete};
use diesel::result::{Error, DatabaseErrorKind};
use diesel::prelude::*;
use diesel::pg::PgConnection;

use r2d2_diesel::ConnectionManager;

use google_authenticator::{GoogleAuthenticator, ErrorCorrectionLevel};

#[derive(Debug, Clone)]
struct IdentityVerificationError;

struct ServiceState {
    conn_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl ServiceState {
    fn new() -> Self {

        std::thread::sleep(std::time::Duration::from_millis(10000));

        let user = env::var("POSTGRES_USER").unwrap_or("ess_da".to_string());
        let password = env::var("POSTGRES_PASSWORD").unwrap_or("not-set".to_string());
        let addr = env::var("POSTGRES_ADDR").unwrap_or("localhost".to_string());
        let db = env::var("POSTGRES_DB").unwrap_or("ess_db".to_string());

        let database_url = format!("postgres://{}:{}@{}/{}", user, password, addr, db);
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        println!("Conneciton pool is created.");
        Self {
            conn_pool: r2d2::Pool::builder().build(manager).expect("Failed to create PostgreSQL connection pool!"),
        }
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
    // Try to parse the request body containing thet model
    let model: IdentityVerificationRequestModel = match req.body_json().await {
        Ok(parsed_model) => parsed_model,
        Err(err) => return Ok(Response::builder(500).body(format!("{}", err)).build()),
    };
    // This locks the 'state' and unlocks it when returning from function
    let state = req.state().lock().unwrap();
    // Get a conneciton from the pool
    let conn = state.conn_pool.get().expect("cannot get a connection from pool!");
    use schema::employee::dsl::*;
    match conn.transaction::<_, _, _>(|| {
        let employees = employee.filter(username.eq(model.username)).load::<Employee>(conn.deref())?;
        if employees.len() == 0 {
            return Err(VariantError::DieselError(Error::NotFound));
        }
        let target_employee = &employees[0];
        let ga = GoogleAuthenticator::new();
        if ga.verify_code(&target_employee.totp_secret, &model.totp_code, 3, 0) {
            return Ok(())
        }
        Err(VariantError::IdentityVerificationError)
    }) {
        Ok(_) => {
            return Ok(Response::builder(200).body("").build())
        },
        Err(err) => match err {
            VariantError::IdentityVerificationError => return Ok(Response::builder(403).body("identity verification failed".to_string()).build()),
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
    if employee_model.first_name.is_none() {
        return Ok(Response::builder(400).body("'first_name' is mandatory").build());
    }
    if employee_model.second_name.is_none() {
        return Ok(Response::builder(400).body("'second_name' is mandatory").build());
    }
    if employee_model.username.is_none() {
        return Ok(Response::builder(400).body("'username' is mandatory").build());
    }
    // This locks the 'state' and unlocks it when returning from function
    let state = req.state().lock().unwrap();
    // Get a conneciton from the pool
    let conn = state.conn_pool.get().expect("cannot get a connection from pool!");
    use schema::employee::dsl::*;
    // Generate QR secret URL
    let ga = GoogleAuthenticator::new();
    let secret = ga.create_secret(32);
    let qr_secret_url = ga.qr_code_url(
        &secret,
        "Encryptizer Inc.",
        "ESS Secret",
        400,
        400,
        ErrorCorrectionLevel::Medium,
    );
    // let secret_value = gen_rand_string(64);
    // Create the values for the new employee
    let values = (
        first_name.eq(employee_model.first_name.unwrap()),
        second_name.eq(employee_model.second_name.unwrap()),
        username.eq(employee_model.username.unwrap()),
        totp_secret.eq(&secret),
    );
    // Try to insert the new employee in a transaction
    match conn.transaction::<_, Error, _>(|| {
        insert_into(employee).values(values).execute(conn.deref())
    }) {
        Ok(_) => return Ok(Response::builder(200).body(qr_secret_url).build()),
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
    let employee_model: EmployeeModel = match req.body_json().await {
        Ok(parsed_employee_model) => parsed_employee_model,
        Err(err) => return Ok(Response::builder(500).body(format!("{}", err)).build()),
    };
    // Read 'employee username' from the path
    let employee_username: String = match req.param("username") {
        Ok(value) => value.to_string(),
        Err(err) => return Ok(Response::builder(400).body(format!("'username' is mandatory and must be provided as part of path: {}", err)).build()),
    };
    use schema::employee::dsl::*;
    // This locks the 'state' and unlocks it when returning from function
    let state = req.state().lock().unwrap();
    // Get a conneciton from the pool
    let conn = state.conn_pool.get().expect("cannot get a connection from pool!");
    // Try to update the employee
    match conn.transaction::<_, Error, _>(|| {
        if employee_model.first_name.is_some() {
            update(employee.filter(username.eq(employee_username.clone()))).set(first_name.eq(employee_model.first_name.unwrap())).execute(conn.deref())?;
        }
        if employee_model.second_name.is_some() {
            update(employee.filter(username.eq(employee_username.clone()))).set(second_name.eq(employee_model.second_name.unwrap())).execute(conn.deref())?;
        }
        Ok(())
    }) {
        Ok(_) => return Ok(Response::builder(200).body("".to_string()).build()),
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
    // Read 'employee username' from the path
    let employee_username: String = match req.param("username") {
        Ok(value) => value.to_string(),
        Err(err) => return Ok(Response::builder(400).body(format!("'username' is mandatory and must be provided as part of path: {}", err)).build()),
    };
    use schema::employee::dsl::*;
    // This locks the 'state' and unlocks it when returning from function
    let state = req.state().lock().unwrap();
    // Get a conneciton from the pool
    let conn = state.conn_pool.get().expect("cannot get a connection from pool!");
    // Try to delete the employee
    match conn.transaction::<_, Error, _>(|| {
        delete(employee.filter(username.eq(employee_username))).execute(conn.deref())
    }) {
        Ok(_) => return Ok(Response::builder(200).body("".to_string()).build()),
        Err(err) => match err {
            Error::NotFound => return Ok(Response::builder(404).body("not found".to_string()).build()),
            error => return Ok(Response::builder(500).body(format!("{}", error)).build()),
        },
    }
}

async fn handle_get_all_employees(req: Request<SharedSyncState>) -> Result<Response> {
    // This locks the 'state' and unlocks it when returning from function
    let state = req.state().lock().unwrap();
    // Get a conneciton from the pool
    let conn = state.conn_pool.get().expect("cannot get a connection from pool!");
    use schema::employee::dsl::*;
    // Read all employees
    match conn.transaction::<_, Error, _>(|| {
        employee.load::<Employee>(conn.deref())
    }) {
        Ok(employees) => {
            let to_return: Vec<EmployeeModel> = employees.iter().map(|e|
                EmployeeModel {
                    first_name: Some(e.first_name.clone()),
                    second_name: Some(e.second_name.clone()),
                    username: Some(e.username.clone()),
            }).collect();
            match serde_json::to_string(&to_return) {
                Ok(body_str) => return Ok(Response::builder(200).body(body_str).build()),
                Err(err) => return Ok(Response::builder(500).body(format!("{}", err)).build()),
            };
        },
        Err(err) => return Ok(Response::builder(500).body(format!("{}", err)).build()),
    }
}

async fn handle_get_employee(req: Request<SharedSyncState>) -> Result<Response> {
    // Read 'employee username' from the path
    let employee_username: String = match req.param("username") {
        Ok(value) => value.to_string(),
        Err(err) => return Ok(Response::builder(400).body(format!("'username' is mandatory and must be provided as part of path: {}", err)).build()),
    };
    // This locks the 'state' and unlocks it when returning from function
    let state = req.state().lock().unwrap();
    // Get a conneciton from the pool
    let conn = state.conn_pool.get().expect("cannot get a connection from pool!");
    use schema::employee::dsl::*;
    // Read just one employee
    match conn.transaction::<_, Error, _>(|| {
        employee.filter(username.eq(employee_username)).load::<Employee>(conn.deref())
    }) {
        Ok(employees) => {
            let to_return: Vec<EmployeeModel> = employees.iter().map(|e|
                EmployeeModel {
                    first_name: Some(e.first_name.clone()),
                    second_name: Some(e.second_name.clone()),
                    username: Some(e.username.clone()),
            }).collect();
            if to_return.len() == 0 {
                return Ok(Response::builder(404).body("no employee found with this username!".to_string()).build());
            }
            // The following condition MUST NEVER happen!
            if to_return.len() > 1 {
                return Ok(Response::builder(500).body("more than one employee have been found with the same username!".to_string()).build());
            }
            match serde_json::to_string(&to_return) {
                Ok(body_str) => return Ok(Response::builder(200).body(body_str).build()),
                Err(err) => return Ok(Response::builder(500).body(format!("{}", err)).build()),
            };
        },
        Err(err) => return Ok(Response::builder(500).body(format!("{}", err)).build()),
    }
}

#[async_std::main]
async fn main() -> std::result::Result<(), std::io::Error> {

    println!("starting ESS web services ...");

    // read server's cert and key
    // read this link to get more insight on how all this works: https://github.com/http-rs/tide-rustls/blob/main/src/tls_listener.rs
    let server_cert_path = env::var("SERVER_CERT_PATH").unwrap();
    let server_key_path = env::var("SERVER_KEY_PATH").unwrap();
    let server_cert_chain = rustls::internal::pemfile::certs(&mut std::io::BufReader::new(File::open(server_cert_path)?)).unwrap();
    let mut server_keys = rustls::internal::pemfile::rsa_private_keys(&mut std::io::BufReader::new(File::open(server_key_path)?)).unwrap();

    let state = Arc::new(Mutex::new(ServiceState::new()));

    {
        let server_cert_chain = server_cert_chain.clone();
        let mut server_keys = server_keys.clone();
        let pam_state = state.clone();
        task::spawn(async move {
            println!("starting PAM services ...");
            let mut pam_app = tide::with_state(pam_state);
            pam_app.at("/api/pam/verify").post(handle_id_verify_req);

            // create pam's server config
            let mut pam_server_config = rustls::ServerConfig::new(rustls::NoClientAuth::new());
            pam_server_config.set_single_cert(server_cert_chain, server_keys.remove(0)).unwrap();

            pam_app.listen(
                TlsListener::build()
                    .addrs("0.0.0.0:443")
                    .config(pam_server_config)
            ).await.expect("Could not start pam web server!");
        });
    }
    {
        println!("starting admin services ...");
        let mut admin_app = tide::with_state(state);
        admin_app.at("/api/admin/employee").post(handle_add_employee);
        admin_app.at("/api/admin/employee/:username").put(handle_update_employee);
        admin_app.at("/api/admin/employee/:username").delete(handle_delete_employee);
        admin_app.at("/api/admin/employee/all").get(handle_get_all_employees);
        admin_app.at("/api/admin/employee/:username").get(handle_get_employee);

        // create the root cert store
        let mut root_cert_store = rustls::RootCertStore::empty();
        let root_certs_path = env::var("ROOT_CERTS_PATH").unwrap();
        let mut root_certs_file = std::io::BufReader::new(File::open(root_certs_path).unwrap());
        let root_certs = rustls::internal::pemfile::certs(&mut root_certs_file).unwrap();
        for root_cert in root_certs {
            root_cert_store.add(&root_cert).unwrap();
        }

        // create admin's server config
        let mut admin_server_config = rustls::ServerConfig::new(rustls::AllowAnyAuthenticatedClient::new(root_cert_store));
        admin_server_config.set_single_cert(server_cert_chain, server_keys.remove(0)).unwrap();

        admin_app.listen(
            TlsListener::build()
                .addrs("0.0.0.0:444")
                .config(admin_server_config)
        ).await.expect("Could not start admin web server!");
    }

    Ok(())
}
