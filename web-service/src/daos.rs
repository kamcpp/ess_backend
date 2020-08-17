use std::vec::Vec;
use std::ops::DerefMut;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::traits::{Identifiable, Appliable, Dao, DaoResult, Predicate};

pub struct InMemoryDao<EntityType> {
    id_seq: i32,
    entities: Arc<Mutex<HashMap<i32, EntityType>>>,
}

#[allow(dead_code)]
impl<EntityType> InMemoryDao<EntityType> {
    pub fn new() -> Self {
        Self {
            id_seq: 0,
            entities: Arc::new(Mutex::new(HashMap::new())),
        }
    }
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
}

impl std::fmt::Display for InMemoryDaoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (Error Code: {})", self.msg, self.code)
    }
}

impl<EntityType> Dao<EntityType> for InMemoryDao<EntityType>
where
    EntityType : Identifiable + Appliable<EntityType> + Clone {

    type PredicateOutputType = bool;
    type ErrorType = InMemoryDaoError;

    fn count(&self) -> usize {
        let entities = self.entities.lock().unwrap();
        entities.len()
    }

    fn insert_into(&mut self, mut values: EntityType) -> DaoResult<(), InMemoryDaoError> {
        if values.id().is_some() {
            return Err(InMemoryDaoError::id_field_must_be_none());
        }
        let mut entities = self.entities.lock().unwrap();
        self.id_seq += 1;
        let id = self.id_seq;
        values.set_id(id);
        entities.insert(id, values);
        Ok(())
    }

    fn update(&mut self, set_values: EntityType, predicate: &mut Predicate<EntityType, bool>) -> DaoResult<(), InMemoryDaoError> {
        if set_values.id().is_some() {
            return Err(InMemoryDaoError::id_field_must_be_none());
        }
        let mut entities = self.entities.lock().unwrap();
        for (_, entity) in entities.deref_mut() {
            if predicate(entity) {
                entity.apply(&set_values);
            }
        }
        Ok(())
    }

    fn update_one(&mut self, id: i32, set_values: EntityType) -> DaoResult<(), InMemoryDaoError> {
        if set_values.id().is_some() {
            return Err(InMemoryDaoError::id_field_must_be_none());
        }
        let mut entities = self.entities.lock().unwrap();
        match entities.get_mut(&id) {
            Some(entity) => {
                entity.apply(&set_values);
                Ok(())
            }
            None => Err(InMemoryDaoError::entity_not_found()),
        }
    }

    fn delete(&mut self, predicate: &mut Predicate<EntityType, bool>) -> DaoResult<(), InMemoryDaoError> {
        let mut entities = self.entities.lock().unwrap();
        entities.retain(|_, entity| !predicate(entity));
        Ok(())
    }

    fn delete_one(&mut self, id: i32) -> DaoResult<(), InMemoryDaoError> {
        let mut entities = self.entities.lock().unwrap();
        entities.retain(|key, _| *key != id);
        Ok(())
    }

    fn get_one(&self, id: i32) -> DaoResult<EntityType, InMemoryDaoError> {
        let mut entities = self.entities.lock().unwrap();
        for (entity_id, entity) in entities.deref_mut() {
            if *entity_id == id {
                return Ok(entity.clone());
            }
        }
        Err(InMemoryDaoError::entity_not_found())
    }

    fn get(&self, predicate: &mut Predicate<EntityType, bool>) -> DaoResult<Vec<EntityType>, InMemoryDaoError> {
        let mut entities = self.entities.lock().unwrap();
        let mut results = Vec::new();
        for (_, entity) in entities.deref_mut() {
            if predicate(entity) {
                results.push(entity.clone());
            }
        }
        Ok(results)
    }

    fn get_all(&self) -> DaoResult<Vec<EntityType>, InMemoryDaoError> {
        let mut entities = self.entities.lock().unwrap();
        let mut results = Vec::new();
        for (_, entity) in entities.deref_mut() {
            results.push(entity.clone());
        }
        Ok(results)
    }
}

