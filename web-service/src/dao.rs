use crate::models::{EmployeeModel, IdentityVerifyRequestModel, NotifyRequestModel};
use std::vec::Vec;

pub type DaoResult<ReturnType, ErrorType> = std::result::Result<ReturnType, ErrorType>;

pub trait TransactionContext {
    type ErrorType;

    fn begin(&mut self) -> DaoResult<(), Self::ErrorType>;
    fn commit(&mut self) -> DaoResult<(), Self::ErrorType>;
    fn rollback(&mut self) -> DaoResult<(), Self::ErrorType>;
}

pub trait TransactionContextBuilder<TransactionContextType> {
    fn build(&self) -> TransactionContextType;
}

pub trait EmployeeDao {
    type ErrorType;
    type TransactionContextType;

    fn insert_into(&mut self, transaction_context: &mut Self::TransactionContextType, employee_model: EmployeeModel) -> DaoResult<(), Self::ErrorType>;
    fn update(&mut self, transaction_context: &mut Self::TransactionContextType, employee_model: EmployeeModel) -> DaoResult<(), Self::ErrorType>;
    fn delete(&mut self, transaction_context: &mut Self::TransactionContextType, id: i32) -> DaoResult<(), Self::ErrorType>;
    fn get_by_username(&self, transaction_context: &mut Self::TransactionContextType, username: String) -> DaoResult<EmployeeModel, Self::ErrorType>;
    fn get_one(&self, transaction_context: &mut Self::TransactionContextType, id: i32) -> DaoResult<EmployeeModel, Self::ErrorType>;
    fn get_all(&self, transaction_context: &mut Self::TransactionContextType) -> DaoResult<Vec<EmployeeModel>, Self::ErrorType>;
}

pub trait IdentityVerifyRequestDao {
    type ErrorType;
    type TransactionContextType;

    fn insert_into(&mut self, transaction_context: &mut Self::TransactionContextType, id_verify_req_model: IdentityVerifyRequestModel) -> DaoResult<(), Self::ErrorType>;
    fn deactivate_all_requests(&mut self, transaction_context: &mut Self::TransactionContextType, employee_id: i32) -> DaoResult<(), Self::ErrorType>;
    fn verify_request(&mut self, transaction_context: &mut Self::TransactionContextType, id: i32) -> DaoResult<(), Self::ErrorType>;
    fn get_active_request_by_reference(&self, transaction_context: &mut Self::TransactionContextType, reference: String) -> DaoResult<IdentityVerifyRequestModel, Self::ErrorType>;
}

pub trait NotifyRequestDao {
    type ErrorType;
    type TransactionContextType;

    fn insert_into(&mut self, transaction_context: &mut Self::TransactionContextType, notify_req_model: NotifyRequestModel) -> DaoResult<(), Self::ErrorType>;
    fn mark_as_sent(&mut self, transaction_context: &mut Self::TransactionContextType, id: i32) -> DaoResult<(), Self::ErrorType>;
    fn get_not_sent_requests(&self, transaction_context: &mut Self::TransactionContextType) -> DaoResult<Vec<NotifyRequestModel>, Self::ErrorType>;
}
