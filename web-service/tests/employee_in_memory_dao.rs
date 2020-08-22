use crate::dao::{DaoResult, EmployeeDao};
use crate::in_memory_dao::{Identifiable, Appliable};

use std::vec::Vec;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

impl Identifiable for EmployeeModel {
    fn id(&self) -> Option<i32> {
        self.id
    }

    fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }
}

struct InMemoryEmployeeDao {
    dao: Arc<Mutex<InMemoryDao<EmployeeModel>>>,
}

impl EmployeeDao for InMemoryEmployeeDao {

    type ErrorType = InMemoryDaoError;

    fn insert_into(&mut self, employee_model: EmployeeModel) -> DaoResult<(), Self::ErrorType> {
        let dao = self.dao.lock().unwrap();
        dao.insert_into(employee_model)
    }

    fn update(&mut self, employee_model: EmployeeModel) -> DaoResult<(), Self::ErrorType> {
        let dao = self.dao.lock().unwrap();
        dao.update(employee_model)
    }

    fn delete(&mut self, id: i32) -> DaoResult<(), Self::ErrorType> {
        let dao = self.dao.lock().unwrap();
        dao.delete(id)
    }

    fn get_by_username(&self, username: String) -> DaoResult<EmployeeModel, Self::ErrorType> {
        let dao = self.dao.lock().unwrap();
        dao.get_one(|e| e.username == username)
    }

    fn get_one(&self, id: i32) -> DaoResult<EmployeeModel, Self::ErrorType> {
        let dao = self.dao.lock().unwrap();
        dao.get_one_by_id(id)
    }

    fn get_all(&self) -> DaoResult<Vec<EmployeeModel>, Self::ErrorType> {
        let dao = self.dao.lock().unwrap();
        dao.get_all()
    }
}
