use crate::models::{EmployeeModel, IdentityVerifyRequestModel, NotifyRequestModel};
use crate::dao::{DaoResult, TransactionalDao, EmployeeDao};

use std::vec::Vec;
use std::ops::DerefMut;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub trait Identifiable {
    fn id(&self) -> Option<i32>;
    fn set_id(&mut self, id: i32);
}

pub trait Appliable {
    fn apply(&mut self, other: &Self);
}

#[derive(Debug)]
pub struct InMemoryDaoError {
    code: i32,
    msg: String,
}

#[allow(dead_code)]
impl InMemoryDaoError {
    fn new(code: i32, msg: String) -> Self {
        Self {
            code,
            msg,
        }
    }

    pub fn get_code(&self) -> i32 {
        self.code
    }

    pub fn get_msg(&self) -> String {
        self.msg.clone()
    }

    pub fn entity_not_found() -> Self {
        Self::new(1000, "Entity not found".to_string())
    }

    pub fn id_field_must_be_none() -> Self {
        Self::new(2000, "The field 'id' must be none".to_string())
    }

    pub fn id_field_must_not_be_none() -> Self {
        Self::new(3000, "The field 'id' must not be none".to_string())
    }

    pub fn more_than_one_entity_found() -> Self {
        Self::new(4000, "More than one entity found".to_string())
    }
}

impl std::fmt::Display for InMemoryDaoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (Error Code: {})", self.msg, self.code)
    }
}

type Predicate<I, O> = dyn FnMut(&I) -> O;

pub struct InMemoryDb<EntityModelType> {
    id_seq: i32,
    entities: HashMap<i32, EntityModelType>,
}

#[allow(dead_code)]
impl<EntityModelType> InMemoryDb<EntityModelType>
where
    EntityModelType: Identifiable + Appliable + Clone {

    pub fn new() -> Self {
        Self {
            id_seq: 0,
            entities: HashMap::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.entities.len()
    }

    pub fn insert_into(&mut self, mut values: EntityModelType) -> DaoResult<(), InMemoryDaoError> {
        if values.id().is_some() {
            return Err(InMemoryDaoError::id_field_must_be_none());
        }
        self.id_seq += 1;
        let id = self.id_seq;
        values.set_id(id);
        self.entities.insert(id, values);
        Ok(())
    }

    pub fn update(&mut self, set_values: EntityModelType) -> DaoResult<(), InMemoryDaoError> {
        if set_values.id().is_none() {
            return Err(InMemoryDaoError::id_field_must_not_be_none());
        }
        match self.entities.get_mut(&set_values.id().unwrap()) {
            Some(entity) => {
                entity.apply(&set_values);
                Ok(())
            }
            None => Err(InMemoryDaoError::entity_not_found()),
        }
    }

    pub fn delete(&mut self, id: i32) -> DaoResult<(), InMemoryDaoError> {
        self.entities.retain(|key, _| *key != id);
        Ok(())
    }

    pub fn get_one_by_id(&self, id: i32) -> DaoResult<EntityModelType, InMemoryDaoError> {
        for (entity_id, entity) in &self.entities {
            if *entity_id == id {
                return Ok(entity.clone());
            }
        }
        Err(InMemoryDaoError::entity_not_found())
    }

    pub fn get(&self, predicate: &mut Predicate<EntityModelType, bool>) -> DaoResult<Vec<EntityModelType>, InMemoryDaoError> {
        let mut results = Vec::new();
        for (_, entity) in &self.entities {
            if predicate(entity) {
                results.push(entity.clone());
            }
        }
        Ok(results)
    }

    pub fn get_one(&self, predicate: &mut Predicate<EntityModelType, bool>) -> DaoResult<EntityModelType, InMemoryDaoError> {
        let found_entities = self.get(predicate)?;
        if found_entities.len() == 0 {
            return Err(InMemoryDaoError::entity_not_found());
        }
        if found_entities.len() > 1 {
            return Err(InMemoryDaoError::more_than_one_entity_found());
        }
        Ok(found_entities[0].clone())
    }

    pub fn get_all(&self) -> DaoResult<Vec<EntityModelType>, InMemoryDaoError> {
        let mut results = Vec::new();
        for (_, entity) in &self.entities {
            results.push(entity.clone());
        }
        Ok(results)
    }
}

// ========================================= Employee Dao =======================================

impl Identifiable for EmployeeModel {
    fn id(&self) -> Option<i32> {
        self.id
    }

    fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }
}

impl Appliable for EmployeeModel {
    fn apply(&mut self, other: &Self) {
    }
}

pub struct InMemoryEmployeeDao {
    db: InMemoryDb<EmployeeModel>,
}

impl InMemoryEmployeeDao {
    pub fn new() -> Self {
        Self {
            db: InMemoryDb::new(),
        }
    }
}

impl TransactionalDao for InMemoryEmployeeDao {
    type ErrorType = InMemoryDaoError;

    fn begin() -> DaoResult<(), Self::ErrorType> {
        Ok(())
    }

    fn commit() -> DaoResult<(), Self::ErrorType> {
        Ok(())
    }

    fn rollback() -> DaoResult<(), Self::ErrorType> {
        Ok(())
    }
}

impl EmployeeDao for InMemoryEmployeeDao {
    type ErrorType = InMemoryDaoError;

    fn insert_into(&mut self, employee_model: EmployeeModel) -> DaoResult<(), Self::ErrorType> {
        Ok(())
    }

    fn update(&mut self, employee_model: EmployeeModel) -> DaoResult<(), Self::ErrorType> {
        Ok(())
    }

    fn delete(&mut self, id: i32) -> DaoResult<(), Self::ErrorType> {
        Ok(())
    }

    fn get_by_username(&self, username: String) -> DaoResult<EmployeeModel, Self::ErrorType> {
        Err(InMemoryDaoError::entity_not_found())
    }

    fn get_one(&self, id: i32) -> DaoResult<EmployeeModel, Self::ErrorType> {
        Err(InMemoryDaoError::entity_not_found())
    }

    fn get_all(&self) -> DaoResult<Vec<EmployeeModel>, Self::ErrorType> {
        Ok(Vec::new())
    }
}

