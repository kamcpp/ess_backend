use crate::models::{EmployeeModel, IdentityVerifyRequestModel, NotifyRequestModel};
use crate::dao::{DaoResult, TransactionalDao, EmployeeDao, IdentityVerifyRequestDao};

use std::vec::Vec;
use std::ops::DerefMut;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

macro_rules! impl_identifiable {
    ($name:ident) => {
        impl Identifiable for $name {
            fn id(&self) -> Option<i32> {
                self.id
            }

            fn set_id(&mut self, id: i32) {
                self.id = Some(id);
            }
        }
    }
}

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

    pub fn update<Predicate>(&mut self, set_values: EntityModelType, mut predicate: Predicate) -> DaoResult<(), InMemoryDaoError>
    where
        Predicate: FnMut(&EntityModelType) -> bool {

        for (_, entity) in &mut self.entities {
            if predicate(entity) {
                entity.apply(&set_values);
            }
        }
        Ok(())
    }

    pub fn update_one(&mut self, set_values: EntityModelType) -> DaoResult<(), InMemoryDaoError> {
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

    pub fn get<Predicate>(&self, mut predicate: Predicate) -> DaoResult<Vec<EntityModelType>, InMemoryDaoError>
    where
        Predicate: FnMut(&EntityModelType) -> bool {

        let mut results = Vec::new();
        for (_, entity) in &self.entities {
            if predicate(entity) {
                results.push(entity.clone());
            }
        }
        Ok(results)
    }

    pub fn get_one<Predicate>(&self, mut predicate: Predicate) -> DaoResult<EntityModelType, InMemoryDaoError>
    where
        Predicate: FnMut(&EntityModelType) -> bool {

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

impl_identifiable!(EmployeeModel);

impl Appliable for EmployeeModel {
    fn apply(&mut self, other: &Self) {
        if other.first_name.is_some() {
            self.first_name = other.first_name.clone();
        }
        if other.second_name.is_some() {
            self.second_name = other.second_name.clone();
        }
        if other.employee_nr.is_some() {
            self.employee_nr = other.employee_nr.clone();
        }
        if other.username.is_some() {
            self.username = other.username.clone();
        }
        if other.office_email.is_some() {
            self.office_email = other.office_email.clone();
        }
        if other.mobile.is_some() {
            self.mobile = other.mobile.clone();
        }
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

    fn begin_transaction() -> DaoResult<(), Self::ErrorType> {
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
        self.db.insert_into(employee_model)
    }

    fn update(&mut self, employee_model: EmployeeModel) -> DaoResult<(), Self::ErrorType> {
        self.db.update_one(employee_model)
    }

    fn delete(&mut self, id: i32) -> DaoResult<(), Self::ErrorType> {
        self.db.delete(id)
    }

    fn get_by_username(&self, username: String) -> DaoResult<EmployeeModel, Self::ErrorType> {
        self.db.get_one(|employee| employee.username == Some(username.clone()))
    }

    fn get_one(&self, id: i32) -> DaoResult<EmployeeModel, Self::ErrorType> {
        self.db.get_one_by_id(id)
    }

    fn get_all(&self) -> DaoResult<Vec<EmployeeModel>, Self::ErrorType> {
        self.db.get_all()
    }
}

// ========================================= Identity Verify Request Dao =======================================

impl_identifiable!(IdentityVerifyRequestModel);

impl Appliable for IdentityVerifyRequestModel {
    fn apply(&mut self, other: &Self) {
        if other.reference.is_some() {
            self.reference = other.reference.clone();
        }
        if other.secret.is_some() {
            self.secret = other.secret.clone();
        }
        if other.active.is_some() {
            self.active = other.active.clone();
        }
        if other.create_utc_dt.is_some() {
            self.create_utc_dt = other.create_utc_dt.clone();
        }
        if other.expire_utc_dt.is_some() {
            self.expire_utc_dt = other.expire_utc_dt.clone();
        }
        if other.verified_utc_dt.is_some() {
            self.verified_utc_dt = other.verified_utc_dt.clone();
        }
        if other.employee_id.is_some() {
            self.employee_id = other.employee_id.clone();
        }
    }
}

pub struct InMemoryIdentityVerifyRequestDao {
    db: InMemoryDb<IdentityVerifyRequestModel>,
}

impl InMemoryIdentityVerifyRequestDao {
    pub fn new() -> Self {
        Self {
            db: InMemoryDb::new(),
        }
    }
}

impl TransactionalDao for InMemoryIdentityVerifyRequestDao {
    type ErrorType = InMemoryDaoError;

    fn begin_transaction() -> DaoResult<(), Self::ErrorType> {
        Ok(())
    }

    fn commit() -> DaoResult<(), Self::ErrorType> {
        Ok(())
    }

    fn rollback() -> DaoResult<(), Self::ErrorType> {
        Ok(())
    }
}

impl IdentityVerifyRequestDao for InMemoryIdentityVerifyRequestDao {
    type ErrorType = InMemoryDaoError;

    fn insert_into(&mut self, id_verify_req_model: IdentityVerifyRequestModel) -> DaoResult<(), Self::ErrorType> {
        Ok(())
    }

    fn deactivate_all_requests(&mut self, employee_id: i32) -> DaoResult<(), Self::ErrorType> {
        Ok(())
    }

    fn verify_request(&mut self, id: i32) -> DaoResult<(), Self::ErrorType> {
        Ok(())
    }

    fn get_active_request_by_reference(&self, reference: String) -> DaoResult<IdentityVerifyRequestModel, Self::ErrorType> {
        Err(InMemoryDaoError::entity_not_found())
    }
}

