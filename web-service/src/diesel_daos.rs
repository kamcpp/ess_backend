use crate::traits::{Identifiable, Appliable, Dao, DaoResult, Predicate};
use crate::schema;

use std::env;
use std::vec::Vec;
use std::ops::DerefMut;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::ops::Deref;

use common::domain::Employee;
use diesel::{insert_into, update, delete};
use diesel::result::{Error, DatabaseErrorKind};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

pub enum VariantError {
    DieselError(diesel::result::Error),
    IdentityVerifyError,
}

impl From<diesel::result::Error> for VariantError {
    fn from(error: diesel::result::Error) -> Self {
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

pub struct EmployeeDao {
    conn_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

#[allow(dead_code)]
impl EmployeeDao {
    pub fn new() -> Self {
        let user = env::var("POSTGRES_USER").unwrap_or("simurgh_da".to_string());
        let password = env::var("POSTGRES_PASSWORD").unwrap_or("not-set".to_string());
        let addr = env::var("POSTGRES_ADDR").unwrap_or("localhost".to_string());
        let db = env::var("POSTGRES_DB").unwrap_or("simurgh_db".to_string());

        let database_url = format!("postgres://{}:{}@{}/{}", user, password, addr, db);
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        println!("Conneciton pool is created.");
        Self {
            conn_pool: r2d2::Pool::builder().build(manager).expect("Failed to create PostgreSQL connection pool!"),
        }
    }
}

impl Identifiable for Employee {
    fn id(&self) -> Option<i32> {
        Some(self.id)
    }
    fn set_id(&mut self, id: i32) {
        self.id = id;
    }
}

enum ExpressionType<A, B> {
    Equal(diesel::expression::operators::Eq<A, B>),
    NotEq(diesel::expression::operators::NotEq<A, B>),
}

impl Dao<Employee> for EmployeeDao {

    type PredicateOutputType = String; //diesel::expression::Expression<SqlType = schema::employee::SqlType>;
    type ErrorType = VariantError;

    /*fn count(&self) -> usize {
        return 0;
    }

    fn insert_into(&mut self, mut values: Employee) -> DaoResult<(), VariantError> {
        Ok(())
    }*/

    fn update(&mut self, set_values: Employee, predicate: &mut Predicate<Employee, Self::PredicateOutputType>) -> DaoResult<(), VariantError> {
        let conn = self.conn_pool.get().expect("Cannot get a connection from pool!");
        use schema::employee::dsl::*;
        use schema::id_verify_request::dsl::*;
        use schema::notify_request::dsl::*;
        conn.transaction::<_, _, _>(|| {
            let aaa = predicate(&set_values);
            let aaa: String = username.eq("Hello".to_string()).and(first_name.eq("qweqwe".to_string()));
            let employees = employee.filter(aaa).load::<Employee>(conn.deref())?;
            if employees.len() == 0 {
                return Err(Error::NotFound);
            }
            Ok(())
            /*let target_employee = &employees[0];
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
            Ok(reference_value)*/
        });
        Ok(())
    }

    /*fn update_one(&mut self, id: i32, set_values: Employee) -> DaoResult<(), VariantError> {
        Ok(())
    }

    fn delete(&mut self, filter: &mut Predicate<Employee, Self::PredicateOutputType>) -> DaoResult<(), VariantError> {
        Ok(())
    }

    fn delete_one(&mut self, id: i32) -> DaoResult<(), VariantError> {
        Ok(())
    }

    fn get_one(&self, id: i32) -> DaoResult<Employee, VariantError> {
        Err(VariantError::IdentityVerifyError)
    }

    fn get(&self, filter: &mut Predicate<Employee, Self::PredicateOutputType>) -> DaoResult<Vec<Employee>, VariantError> {
        Ok(Vec::new())
    }

    fn get_all(&self) -> DaoResult<Vec<Employee>, VariantError> {
        Ok(Vec::new())
    }*/
}

