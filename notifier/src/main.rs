#[macro_use]
extern crate diesel;

use std::sync::{Arc, Mutex};
use std::ops::Deref;
use std::env;

use chrono::{Utc, Duration, NaiveDateTime, DateTime};

use rand::Rng;
use rand::distributions::Alphanumeric;

use diesel::{insert_into, update, delete};
use diesel::result::{Error, DatabaseErrorKind};
use diesel::prelude::*;
use diesel::pg::PgConnection;

use r2d2_diesel::ConnectionManager;

use tokio::time;
use tokio::task;

use futures::join;

#[derive(Debug, Clone)]
struct IdentityVerifyError;

struct ServiceState {
    conn_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
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
        Self {
            conn_pool: r2d2::Pool::builder().build(manager).expect("Failed to create PostgreSQL connection pool."),
        }
    }
}

type SharedSyncState = Arc<Mutex<ServiceState>>;

fn main() -> std::result::Result<(), std::io::Error> {
    let state = Arc::new(Mutex::new(ServiceState::new()));
    let mut rt = tokio::runtime::Runtime::new()?;
    let local = task::LocalSet::new();
    local.block_on(&mut rt, async move {
        println!("Simurgh notifier is running now ...");
        let notifier_job = task::spawn_local(async move {
            let mut interval = time::interval(std::time::Duration::from_secs(10));
            loop {
                interval.tick().await;
                println!("Sending ...");
            }
        });
        let (r1,) = join!(notifier_job);
        r1.expect("Could not start notifier job!");
    });
    Ok(())
}
