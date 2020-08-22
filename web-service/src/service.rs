use crate::dao::{DaoResult, TransactionalEmployeeDao};
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
    employee_dao: Box<dyn TransactionalEmployeeDao<DaoErrorType> + Send>,
}

type ServiceResult<R, E> = std::result::Result<R, E>;

impl<DaoErrorType> Service<DaoErrorType>
where
    DaoErrorType: std::convert::Into<VariantError> {

    pub fn new(employee_dao: Box<dyn TransactionalEmployeeDao<DaoErrorType> + Send>) -> Self {
        Self {
            employee_dao
        }
    }

    pub fn add_employee(&mut self, employee_model: EmployeeModel) -> ServiceResult<(), DaoErrorType> {
        self.employee_dao.begin_transaction();
        self.employee_dao.insert_into(employee_model)
            .map(|_| {
                self.employee_dao.commit();
            })
            .map_err(|err| {
                self.employee_dao.rollback();
                err
            })
    }

    pub fn update_employee(&mut self, employee_model: EmployeeModel) -> ServiceResult<(), DaoErrorType> {
        self.employee_dao.update(employee_model)
    }

    pub fn delete_employee(&mut self, employee_id: i32) -> ServiceResult<(), DaoErrorType> {
        self.employee_dao.delete(employee_id)
    }

    pub fn get_employee(&mut self, employee_id: i32) -> ServiceResult<EmployeeModel, DaoErrorType> {
        self.employee_dao.get_one(employee_id)
    }

    pub fn get_all_employees(&mut self) -> ServiceResult<Vec<EmployeeModel>, DaoErrorType> {
        self.employee_dao.get_all()
    }

    /*pub fn new_id_verify_req(&mut self, id_veirfy_req: NewIdentityVerifyRequestModel) -> ServiceResult<NewIdentityVerifyResponseModel, DaoErrorType> {
    }*/
}
