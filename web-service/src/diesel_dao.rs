use crate::{SharedConnPool, VariantError};
use crate::dao::{DaoResult, EmployeeDao};
use crate::models::EmployeeModel;

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
use r2d2_diesel::ConnectionManager;

pub struct DieselEmployeeDao {
    conn_pool: SharedConnPool,
}

impl DieselEmployeeDao {
    pub fn new(conn_pool: SharedConnPool) -> Self {
        Self {
            conn_pool
        }
    }
}

impl EmployeeDao for DieselEmployeeDao {
    type ErrorType = diesel::result::Error;

    fn insert_into(&mut self, employee_model: EmployeeModel) -> DaoResult<(), Self::ErrorType> {
        let conn_pool = self.conn_pool.lock().unwrap();
        let conn = conn_pool.get().expect("Cannot get a connection from pool!");
        use schema::employee::dsl::*;
        let values = (
            employee_nr.eq(employee_model.employee_nr.unwrap()),
            first_name.eq(employee_model.first_name.unwrap()),
            second_name.eq(employee_model.second_name.unwrap()),
            username.eq(employee_model.username.unwrap()),
            office_email.eq(employee_model.office_email.unwrap()),
            mobile.eq(employee_model.mobile.unwrap()),
        );
        conn.transaction::<_, diesel::result::Error, _>(|| {
            insert_into(employee).values(values).execute(conn.deref())
        }).map(|_| ())
    }

    fn update(&mut self, employee_model: EmployeeModel) -> DaoResult<(), Self::ErrorType> {
        let conn_pool = self.conn_pool.lock().unwrap();
        let conn = conn_pool.get().expect("Cannot get a connection from pool!");
        use schema::employee::dsl::*;
        let employee_id = employee_model.id.unwrap();
        conn.transaction::<_, diesel::result::Error, _>(|| {
            if employee_model.employee_nr.is_some() {
                update(employee.filter(id.eq(employee_id))).set(employee_nr.eq(employee_model.employee_nr.unwrap())).execute(conn.deref())?;
            }
            if employee_model.first_name.is_some() {
                update(employee.filter(id.eq(employee_id))).set(first_name.eq(employee_model.first_name.unwrap())).execute(conn.deref())?;
            }
            if employee_model.second_name.is_some() {
                update(employee.filter(id.eq(employee_id))).set(second_name.eq(employee_model.second_name.unwrap())).execute(conn.deref())?;
            }
            if employee_model.username.is_some() {
                update(employee.filter(id.eq(employee_id))).set(username.eq(employee_model.username.unwrap())).execute(conn.deref())?;
            }
            if employee_model.office_email.is_some() {
                update(employee.filter(id.eq(employee_id))).set(office_email.eq(employee_model.office_email.unwrap())).execute(conn.deref())?;
            }
            if employee_model.mobile.is_some() {
                update(employee.filter(id.eq(employee_id))).set(mobile.eq(employee_model.mobile.unwrap())).execute(conn.deref())?;
            }
            Ok(())
        })
    }

    fn delete(&mut self, id: i32) -> DaoResult<(), Self::ErrorType> {
        let conn_pool = self.conn_pool.lock().unwrap();
        let conn = conn_pool.get().expect("Cannot get a connection from pool!");
        use schema::employee::dsl::*;
        conn.transaction::<_, diesel::result::Error, _>(|| {
            delete(employee.filter(id.eq(id))).execute(conn.deref())
        }).map(|_| ())
    }

    fn get_by_username(&self, username: String) -> DaoResult<EmployeeModel, Self::ErrorType> {
        Err(diesel::result::Error::NotFound)
    }

    fn get_one(&self, id: i32) -> DaoResult<EmployeeModel, Self::ErrorType> {
        let conn_pool = self.conn_pool.lock().unwrap();
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
        }
        Ok(employees[0].clone())
    }

    fn get_all(&self) -> DaoResult<Vec<EmployeeModel>, Self::ErrorType> {
        let conn_pool = self.conn_pool.lock().unwrap();
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
        )
    }
}

