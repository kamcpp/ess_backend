#[macro_use]
extern crate diesel;

use std::sync::{Arc, Mutex};
use std::ops::Deref;
use std::env;
use std::cmp::PartialEq;
use std::hash::{Hash, Hasher};

use common::schema;
use common::domain::NotifyRequest;
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum MessageType {
    Email,
    Sms,
}

#[derive(Debug, Clone, Eq)]
struct Message {
    id: i32,
    msg_type: MessageType,
    recipient: String,
    title: String,
    body: String,
    send_utc_dt: Option<NaiveDateTime>,
}

impl Message {
    fn is_sent(&self) -> bool {
        self.send_utc_dt.is_some()
    }
}

impl Hash for Message {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

struct ServiceState {
    conn_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
    notify_queue: Vec<Message>,
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
            notify_queue: Vec::new(),
        }
    }
}

fn main() -> std::result::Result<(), std::io::Error> {
    let state = Arc::new(Mutex::new(ServiceState::new()));
    let mut rt = tokio::runtime::Runtime::new()?;
    let local = task::LocalSet::new();
    local.block_on(&mut rt, async move {
        println!("Simurgh notifier is running now ...");
        let reader_job;
        {
            let state = state.clone();
            reader_job = task::spawn_local(async move {
                let mut interval = time::interval(std::time::Duration::from_secs(10));
                loop {
                    interval.tick().await;
                    let state = state.lock().unwrap();
                    let now = Utc::now();
                    let conn = state.conn_pool.get().expect("Cannot get a connection from pool!");
                    use schema::notify_request::dsl::*;
                    conn.transaction::<_, Error, _>(|| {
                        let notify_requests =
                            notify_request.filter(
                                expire_utc_dt.gt(now.naive_utc())
                                .and(send_utc_dt.is_null())
                            ).load::<NotifyRequest>(conn.deref())?;
                        if notify_requests.len() == 0 {
                            return Ok(());
                        }
                        notify_requests.iter().for_each(|nr| {
                            // TODO
                        });
                        Ok(())
                    });
                }
            });
        }
        let notifier_job;
        {
            let state = state.clone();
            notifier_job = task::spawn_local(async move {
                let mut interval = time::interval(std::time::Duration::from_secs(10));
                loop {
                    interval.tick().await;
                    println!("Sending ...");
                }
            });
        }
        let (r1, r2) = join!(reader_job, notifier_job);
        r1.expect("Could not start the reader job!");
        r2.expect("Could not start the notifier job!");
    });
    Ok(())
}
