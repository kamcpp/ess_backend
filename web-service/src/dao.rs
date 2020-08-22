use crate::models::{EmployeeModel, IdentityVerifyRequestModel, NotifyRequestModel};
use std::vec::Vec;

pub type DaoResult<ReturnType, ErrorType> = std::result::Result<ReturnType, ErrorType>;

pub trait TransactionalDao {
    type ErrorType;

    fn begin_transaction(&mut self) -> DaoResult<(), Self::ErrorType>;
    fn commit(&mut self) -> DaoResult<(), Self::ErrorType>;
    fn rollback(&mut self) -> DaoResult<(), Self::ErrorType>;
}

pub trait EmployeeDao {
    type ErrorType;

    fn insert_into(&mut self, employee_model: EmployeeModel) -> DaoResult<(), Self::ErrorType>;
    fn update(&mut self, employee_model: EmployeeModel) -> DaoResult<(), Self::ErrorType>;
    fn delete(&mut self, id: i32) -> DaoResult<(), Self::ErrorType>;
    fn get_by_username(&self, username: String) -> DaoResult<EmployeeModel, Self::ErrorType>;
    fn get_one(&self, id: i32) -> DaoResult<EmployeeModel, Self::ErrorType>;
    fn get_all(&self) -> DaoResult<Vec<EmployeeModel>, Self::ErrorType>;
}

pub trait IdentityVerifyRequestDao {
    type ErrorType;

    fn insert_into(&mut self, id_verify_req_model: IdentityVerifyRequestModel) -> DaoResult<(), Self::ErrorType>;
    fn deactivate_all_requests(&mut self, employee_id: i32) -> DaoResult<(), Self::ErrorType>;
    fn verify_request(&mut self, id: i32) -> DaoResult<(), Self::ErrorType>;
    fn get_active_request_by_reference(&self, reference: String) -> DaoResult<IdentityVerifyRequestModel, Self::ErrorType>;
}

pub trait NotifyRequestDao {
    type ErrorType;

    fn insert_into(&mut self, notify_req_model: NotifyRequestModel) -> DaoResult<(), Self::ErrorType>;
    fn mark_as_sent(&mut self, id: i32) -> DaoResult<(), Self::ErrorType>;
    fn get_not_sent_requests(&self) -> DaoResult<Vec<NotifyRequestModel>, Self::ErrorType>;
}

pub trait TransactionalEmployeeDao<E>: TransactionalDao<ErrorType = E> + EmployeeDao<ErrorType = E> {}
pub trait TransactionalIdentityVerifyRequestDao<E>: TransactionalDao<ErrorType = E> + IdentityVerifyRequestDao<ErrorType = E> {}
pub trait TransactionalNotifyRequestDao<E>: TransactionalDao<ErrorType = E> + NotifyRequestDao<ErrorType = E> {}
