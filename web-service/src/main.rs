mod models;

use std::fs::File;
use std::sync::{Arc, Mutex};
use std::ops::Deref;
use std::env;

use env_logger;

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

>> ADD REQUIRED DEPENDENCIES HERE <<

#[derive(Debug, Clone)]
struct IdentityVerificationError;

struct ServiceState {
    conn_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl ServiceState {
    fn new() -> Self {

        std::thread::sleep(std::time::Duration::from_millis(10000));

        let user = env::var("POSTGRES_USER").unwrap_or("ess_da".to_string());
        let password = env::var("POSTGRES_PASSWORD").unwrap_or("ess_password".to_string());
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

    println!("starting ESS web services ...");

    env_logger::init();

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

        // read root cert file
        let root_certs_path = env::var("ROOT_CERTS_PATH").unwrap();
        let mut root_certs_file = std::io::BufReader::new(File::open(root_certs_path).unwrap());

        // create the root cert store
        let mut root_cert_store = rustls::RootCertStore::empty();
        root_cert_store.add_pem_file(&mut root_certs_file).unwrap();

        // create admin's server config
        let verifier = rustls::AllowAnyAuthenticatedClient::new(root_cert_store);
        let mut admin_server_config = rustls::ServerConfig::with_ciphersuites(verifier, &rustls::ALL_CIPHERSUITES);
        admin_server_config.set_single_cert(server_cert_chain, server_keys.remove(0)).unwrap();

        admin_app.listen(
            TlsListener::build()
                .addrs("0.0.0.0:444")
                .config(admin_server_config)
        ).await.expect("Could not start admin web server!");
    }

    Ok(())
}
