use crate::dao::{DaoResult, TransactionContext, TransactionContextBuilder, EmployeeDao};
use crate::models::{EmployeeModel};

use rand::Rng;
use rand::distributions::Alphanumeric;

fn gen_rand_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect::<String>()
}

#[derive(Debug, Clone)]
struct IdentityVerifyError;

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

pub struct Service<DaoErrorType> {
    transaction_context_builder: Box<dyn TransactionContextBuilder<DaoErrorType> + Send>,
    employee_dao: Box<dyn EmployeeDao<ErrorType = DaoErrorType> + Send>,
}

type ServiceResult<R, E> = std::result::Result<R, E>;

impl<DaoErrorType> Service<DaoErrorType>
where
    DaoErrorType: std::convert::Into<VariantError> {

    pub fn new(transaction_context_builder: Box<dyn TransactionContextBuilder<DaoErrorType> + Send>,
               employee_dao: Box<dyn EmployeeDao<ErrorType = DaoErrorType> + Send>) -> Self {
        Self {
            transaction_context_builder,
            employee_dao
        }
    }

    pub fn add_employee(&mut self, employee_model: EmployeeModel) -> ServiceResult<(), DaoErrorType> {
        let mut transaction_context = self.transaction_context_builder.build();
        transaction_context.begin();
        self.employee_dao.insert_into(&mut transaction_context, employee_model)
            .map(|_| {
                transaction_context.commit();
            })
            .map_err(|err| {
                transaction_context.rollback();
                err
            })
    }

    pub fn update_employee(&mut self, employee_model: EmployeeModel) -> ServiceResult<(), DaoErrorType> {
        //self.employee_dao.begin_transaction();
        self.employee_dao.update(employee_model)
            /*.map(|_| {
                self.employee_dao.commit();
            })
            .map_err(|err| {
                self.employee_dao.rollback();
                err
            })*/
    }

    pub fn delete_employee(&mut self, employee_id: i32) -> ServiceResult<(), DaoErrorType> {
        //self.employee_dao.begin_transaction();
        self.employee_dao.delete(employee_id)
        /*    .map(|_| {
                self.employee_dao.commit();
            })
            .map_err(|err| {
                self.employee_dao.rollback();
                err
            })*/
    }

    pub fn get_employee(&mut self, employee_id: i32) -> ServiceResult<EmployeeModel, DaoErrorType> {
        // self.employee_dao.begin_transaction();
        self.employee_dao.get_one(employee_id)
           /* .map(|e| {
                self.employee_dao.commit();
                e
            })
            .map_err(|err| {
                self.employee_dao.rollback();
                err
            })*/
    }

    pub fn get_all_employees(&mut self) -> ServiceResult<Vec<EmployeeModel>, DaoErrorType> {
        // self.employee_dao.begin_transaction();
        self.employee_dao.get_all()
         /*   .map(|v| {
                self.employee_dao.commit();
                v
            })
            .map_err(|err| {
                self.employee_dao.rollback();
                err
            })*/
    }

    /*pub fn new_id_verify_req(&mut self, id_veirfy_req: NewIdentityVerifyRequestModel) -> ServiceResult<NewIdentityVerifyResponseModel, DaoErrorType> {
    }*/
}
