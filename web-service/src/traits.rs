use std::vec::Vec;

pub trait Identifiable {
    fn id(&self) -> Option<i32>;
    fn set_id(&mut self, id: i32);
}

pub trait Appliable<EntityType> {
    fn apply(&mut self, entity: &EntityType);
}

pub type DaoResult<EntityType, ErrorType> = std::result::Result<EntityType, ErrorType>;
pub type Predicate<InputType, OutputType> = dyn std::ops::FnMut(&InputType) -> OutputType;

pub trait Dao<EntityType>
where
    EntityType : Identifiable {

    type PredicateOutputType;
    type ErrorType;

    fn count(&self) -> usize;

    fn insert_into(&mut self, values: EntityType) -> DaoResult<(), Self::ErrorType>;

    fn update(&mut self, set_values: EntityType, predicate: &mut Predicate<EntityType, Self::PredicateOutputType>) -> DaoResult<(), Self::ErrorType>;
    fn update_one(&mut self, id: i32, set_values: EntityType) -> DaoResult<(), Self::ErrorType>;

    fn delete(&mut self, predicate: &mut Predicate<EntityType, Self::PredicateOutputType>) -> DaoResult<(), Self::ErrorType>;
    fn delete_one(&mut self, id: i32) -> DaoResult<(), Self::ErrorType>;

    fn get_one(&self, id: i32) -> DaoResult<EntityType, Self::ErrorType>;
    fn get(&self, predicate: &mut Predicate<EntityType, Self::PredicateOutputType>) -> DaoResult<Vec<EntityType>, Self::ErrorType>;
    fn get_all(&self) -> DaoResult<Vec<EntityType>, Self::ErrorType>;
}
