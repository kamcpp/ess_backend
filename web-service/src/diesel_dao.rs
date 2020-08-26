use crate::dao::{
    DaoResult,
    TransactionContext,
    TransactionContextBuilder,
    EmployeeDao,
    IdentityVerifyRequestDao,
    NotifyRequestDao
};
use crate::models::{
    EmployeeModel,
    IdentityVerifyRequestModel,
    NotifyRequestModel,
};
use crate::service::{
    ServiceError,
};

use std::vec::Vec;
use std::ops::DerefMut;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::ops::Deref;

use common::schema;
use common::domain::Employee;
use diesel::{insert_into, update, delete};
use diesel::result::{Error, DatabaseErrorKind};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::connection::TransactionManager;
use r2d2::{PooledConnection, Pool};
use r2d2_diesel::ConnectionManager;

impl From<diesel::result::Error> for ServiceError<diesel::result::Error> {
    fn from(error: diesel::result::Error) -> Self {
        ServiceError::DaoError(error)
    }
}

pub struct DieselTransactionContext {
    conn: PooledConnection<ConnectionManager<PgConnection>>,
}

impl TransactionContext for DieselTransactionContext {
    type ErrorType = diesel::result::Error;

    fn begin(&mut self) -> DaoResult<(), Self::ErrorType> {
        let tm = self.conn.transaction_manager();
        tm.begin_transaction(self.conn.deref())
    }

    fn commit(&mut self) -> DaoResult<(), Self::ErrorType> {
        let tm = self.conn.transaction_manager();
        tm.commit_transaction(self.conn.deref())
    }

    fn rollback(&mut self) -> DaoResult<(), Self::ErrorType> {
        let tm = self.conn.transaction_manager();
        tm.rollback_transaction(self.conn.deref())
    }
}

pub struct DieselTransactionContextBuilder {
    conn_pool: Pool<ConnectionManager<PgConnection>>,
}

impl DieselTransactionContextBuilder {
    pub fn new(conn_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            conn_pool
        }
    }
}

impl TransactionContextBuilder<DieselTransactionContext> for DieselTransactionContextBuilder {
    fn build(&self) -> DieselTransactionContext {
        let conn = self.conn_pool.get().expect("Cannot get a connection from pool!");
        DieselTransactionContext { conn }
    }
}

// ========================== Employee Dao ===================================

pub struct DieselEmployeeDao {
}

impl DieselEmployeeDao {
    pub fn new() -> Self {
        Self {}
    }
}

impl EmployeeDao for DieselEmployeeDao {
    type ErrorType = diesel::result::Error;
    type TransactionContextType = DieselTransactionContext;

    fn insert_into(&mut self, transaction_context: &mut Self::TransactionContextType, employee_model: EmployeeModel) -> DaoResult<(), Self::ErrorType> {
        use schema::employee::dsl::*;
        let values = (
            employee_nr.eq(employee_model.employee_nr.unwrap()),
            first_name.eq(employee_model.first_name.unwrap()),
            second_name.eq(employee_model.second_name.unwrap()),
            username.eq(employee_model.username.unwrap()),
            office_email.eq(employee_model.office_email.unwrap()),
            mobile.eq(employee_model.mobile.unwrap()),
        );
        insert_into(employee).values(values).execute(transaction_context.conn.deref()).map(|_| {})
    }

    fn update(&mut self, transaction_context: &mut Self::TransactionContextType, employee_model: EmployeeModel) -> DaoResult<(), Self::ErrorType> {
        use schema::employee::dsl::*;
        let employee_id = employee_model.id.unwrap();
        if employee_model.employee_nr.is_some() {
            update(employee.filter(id.eq(employee_id))).set(employee_nr.eq(employee_model.employee_nr.unwrap())).execute(transaction_context.conn.deref())?;
        }
        if employee_model.first_name.is_some() {
            update(employee.filter(id.eq(employee_id))).set(first_name.eq(employee_model.first_name.unwrap())).execute(transaction_context.conn.deref())?;
        }
        if employee_model.second_name.is_some() {
            update(employee.filter(id.eq(employee_id))).set(second_name.eq(employee_model.second_name.unwrap())).execute(transaction_context.conn.deref())?;
        }
        if employee_model.username.is_some() {
            update(employee.filter(id.eq(employee_id))).set(username.eq(employee_model.username.unwrap())).execute(transaction_context.conn.deref())?;
        }
        if employee_model.office_email.is_some() {
            update(employee.filter(id.eq(employee_id))).set(office_email.eq(employee_model.office_email.unwrap())).execute(transaction_context.conn.deref())?;
        }
        if employee_model.mobile.is_some() {
            update(employee.filter(id.eq(employee_id))).set(mobile.eq(employee_model.mobile.unwrap())).execute(transaction_context.conn.deref())?;
        }
        Ok(())
    }

    fn delete(&mut self, transaction_context: &mut Self::TransactionContextType, id: i32) -> DaoResult<(), Self::ErrorType> {
        /*let conn_pool = self.conn_pool.lock().unwrap();
        let conn = conn_pool.get().expect("Cannot get a connection from pool!");
        use schema::employee::dsl::*;
        conn.transaction::<_, diesel::result::Error, _>(|| {
            delete(employee.filter(id.eq(id))).execute(conn.deref())
        }).map(|_| ())*/
        Ok(())
    }

    fn get_by_username(&self, transaction_context: &mut Self::TransactionContextType, username: String) -> DaoResult<EmployeeModel, Self::ErrorType> {
        Err(diesel::result::Error::NotFound)
    }

    fn get_one(&self, transaction_context: &mut Self::TransactionContextType, id: i32) -> DaoResult<EmployeeModel, Self::ErrorType> {
        /*let conn_pool = self.conn_pool.lock().unwrap();
        let conn = conn_pool.get().expect("Cannot get a connection from pool!");
        use schema::employee::dsl::*;
        let employees: Vec<EmployeeModel> = conn.transaction::<_, diesel::result::Error, _>(|| {
            employee.filter(id.eq(id)).load::<Employee>(conn.deref())
        }).map(|employees|
            employees.iter().map(|e|
                EmployeeModel {
                    id: Some(e.id),
                    employee_nr: Some(e.employee_nr.clone()),
                    first_name: Some(e.first_name.clone()),
                    second_name: Some(e.second_name.clone()),
                    username: Some(e.username.clone()),
                    office_email: Some(e.office_email.clone()),
                    mobile: Some(e.mobile.clone()),
            }).collect()
        )?;
        if employees.len() == 0 {
            return Err(diesel::result::Error::NotFound)
        }*/
        // Ok(employees[0].clone())
        Err(diesel::result::Error::NotFound)
    }

    fn get_all(&self, transaction_context: &mut Self::TransactionContextType) -> DaoResult<Vec<EmployeeModel>, Self::ErrorType> {
        /*let conn_pool = self.conn_pool.lock().unwrap();
        let conn = conn_pool.get().expect("Cannot get a connection from pool!");
        use schema::employee::dsl::*;
        conn.transaction::<_, diesel::result::Error, _>(|| {
            employee.load::<Employee>(conn.deref())
        }).map(|employees|
            employees.iter().map(|e|
                EmployeeModel {
                    id: Some(e.id),
                    employee_nr: Some(e.employee_nr.clone()),
                    first_name: Some(e.first_name.clone()),
                    second_name: Some(e.second_name.clone()),
                    username: Some(e.username.clone()),
                    office_email: Some(e.office_email.clone()),
                    mobile: Some(e.mobile.clone()),
            }).collect()
        )*/
        Ok(Vec::new())
    }
}

// ========================== Identity Verify Request Dao ===================================

pub struct DieselIdentityVerifyRequestDao {
}

impl DieselIdentityVerifyRequestDao {
    pub fn new() -> Self {
        Self {}
    }
}

impl IdentityVerifyRequestDao for DieselIdentityVerifyRequestDao {
    type ErrorType = diesel::result::Error;
    type TransactionContextType = DieselTransactionContext;

    fn insert_into(&mut self, transaction_context: &mut Self::TransactionContextType, id_verify_req_model: IdentityVerifyRequestModel) -> DaoResult<(), Self::ErrorType> {
        Ok(())
    }

    fn deactivate_all_requests(&mut self, transaction_context: &mut Self::TransactionContextType, employee_id: i32) -> DaoResult<(), Self::ErrorType> {
        Ok(())
    }

    fn verify_request(&mut self, transaction_context: &mut Self::TransactionContextType, id: i32) -> DaoResult<(), Self::ErrorType> {
        Ok(())
    }

    fn get_active_request_by_reference(&self, transaction_context: &mut Self::TransactionContextType, reference: String) -> DaoResult<IdentityVerifyRequestModel, Self::ErrorType> {
        Err(diesel::result::Error::NotFound)
    }
}

// ========================== Identity Verify Request Dao ===================================

pub struct DieselNotifyRequestDao {
}

impl DieselNotifyRequestDao {
    pub fn new() -> Self {
        Self {}
    }
}

impl NotifyRequestDao for DieselNotifyRequestDao {
    type ErrorType = diesel::result::Error;
    type TransactionContextType = DieselTransactionContext;

    fn insert_into(&mut self, transaction_context: &mut Self::TransactionContextType, notify_req_model: NotifyRequestModel) -> DaoResult<(), Self::ErrorType> {
        Ok(())
    }

    fn mark_as_sent(&mut self, transaction_context: &mut Self::TransactionContextType, id: i32) -> DaoResult<(), Self::ErrorType> {
        Ok(())
    }

    fn get_not_sent_requests(&self, transaction_context: &mut Self::TransactionContextType) -> DaoResult<Vec<NotifyRequestModel>, Self::ErrorType> {
        Ok(Vec::new())
    }

}
