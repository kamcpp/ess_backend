use crate::dao::{
    TransactionContext,
    TransactionContextBuilder,
    EmployeeDao,
    IdentityVerifyRequestDao,
    NotifyRequestDao
};
use crate::models::{
    EmployeeModel,
    IdentityVerifyRequestModel,
    CheckIdentityVerifyRequestModel,
    NotifyRequestModel,
    NewIdentityVerifyRequestModel,
    NewIdentityVerifyResponseModel
};

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

pub enum ServiceError<DaoErrorType> {
    DaoError(DaoErrorType),
    IdentityVerifyError,
}

impl<DaoErrorType> std::fmt::Display for ServiceError<DaoErrorType>
where
    DaoErrorType: std::fmt::Display {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::DaoError(err) => write!(f, "{}", err),
            ServiceError::IdentityVerifyError => write!(f, "identity verification error"),
        }
    }
}

pub struct Service<DaoErrorType, TransactionContextType> {
    transaction_context_builder: Box<dyn TransactionContextBuilder<TransactionContextType> + Send>,
    employee_dao: Box<dyn EmployeeDao<ErrorType = DaoErrorType, TransactionContextType = TransactionContextType> + Send>,
    id_verify_req_dao: Box<dyn IdentityVerifyRequestDao<ErrorType = DaoErrorType, TransactionContextType = TransactionContextType> + Send>,
    notify_req_dao: Box<dyn NotifyRequestDao<ErrorType = DaoErrorType, TransactionContextType = TransactionContextType> + Send>,
}

type ServiceResult<R, E> = std::result::Result<R, E>;

impl<DaoErrorType, TransactionContextType> Service<DaoErrorType, TransactionContextType>
where
    DaoErrorType: std::convert::Into<ServiceError<DaoErrorType>>,
    TransactionContextType: TransactionContext {

    pub fn new(transaction_context_builder: Box<dyn TransactionContextBuilder<TransactionContextType> + Send>,
               employee_dao: Box<dyn EmployeeDao<ErrorType = DaoErrorType, TransactionContextType = TransactionContextType> + Send>,
               id_verify_req_dao: Box<dyn IdentityVerifyRequestDao<ErrorType = DaoErrorType, TransactionContextType = TransactionContextType> + Send>,
               notify_req_dao: Box<dyn NotifyRequestDao<ErrorType = DaoErrorType, TransactionContextType = TransactionContextType> + Send>) -> Self {
        Self {
            transaction_context_builder,
            employee_dao,
            id_verify_req_dao,
            notify_req_dao,
        }
    }

    pub fn add_employee(&mut self, employee_model: EmployeeModel) -> ServiceResult<(), ServiceError<DaoErrorType>> {
        let mut transaction_context = self.transaction_context_builder.build();
        transaction_context.begin().ok();
        self.employee_dao.insert_into(&mut transaction_context, employee_model)
            .map(|_| {
                transaction_context.commit().ok();
            })
            .map_err(|err| {
                transaction_context.rollback().ok();
                err.into()
            })
    }

    pub fn update_employee(&mut self, employee_model: EmployeeModel) -> ServiceResult<(), ServiceError<DaoErrorType>> {
        let mut transaction_context = self.transaction_context_builder.build();
        transaction_context.begin().ok();
        self.employee_dao.update(&mut transaction_context, employee_model)
            .map(|_| {
                transaction_context.commit().ok();
            })
            .map_err(|err| {
                transaction_context.rollback().ok();
                err.into()
            })
    }

    pub fn delete_employee(&mut self, employee_id: i32) -> ServiceResult<(), ServiceError<DaoErrorType>> {
        let mut transaction_context = self.transaction_context_builder.build();
        transaction_context.begin().ok();
        self.employee_dao.delete(&mut transaction_context, employee_id)
            .map(|_| {
                transaction_context.commit().ok();
            })
            .map_err(|err| {
                transaction_context.rollback().ok();
                err.into()
            })
    }

    pub fn get_employee(&mut self, employee_id: i32) -> ServiceResult<EmployeeModel, ServiceError<DaoErrorType>> {
        let mut transaction_context = self.transaction_context_builder.build();
        transaction_context.begin().ok();
        self.employee_dao.get_one(&mut transaction_context, employee_id)
            .map(|result| {
                transaction_context.commit().ok();
                result
            })
            .map_err(|err| {
                transaction_context.rollback().ok();
                err.into()
            })
    }

    pub fn get_all_employees(&mut self) -> ServiceResult<Vec<EmployeeModel>, ServiceError<DaoErrorType>> {
        let mut transaction_context = self.transaction_context_builder.build();
        transaction_context.begin().ok();
        self.employee_dao.get_all(&mut transaction_context)
            .map(|result| {
                transaction_context.commit().ok();
                result
            })
            .map_err(|err| {
                transaction_context.rollback().ok();
                err.into()
            })
    }

    pub fn new_id_verify_req(&mut self, id_verify_req: NewIdentityVerifyRequestModel) -> ServiceResult<NewIdentityVerifyResponseModel, ServiceError<DaoErrorType>> {
        let mut transaction_context = self.transaction_context_builder.build();
        transaction_context.begin().ok();
        let now = chrono::Utc::now();
        match self.employee_dao.get_by_username(&mut transaction_context, id_verify_req.username)
            .and_then(|employee| {
                let mut set_values = IdentityVerifyRequestModel::empty();
                set_values.active = Some(false);
                self.id_verify_req_dao.deactivate_all_requests(&mut transaction_context, employee.id.unwrap())?;

                let secret = gen_rand_string(8);
                let reference = gen_rand_string(16);

                let mut new_id_verify_req = IdentityVerifyRequestModel::empty();
                new_id_verify_req.reference = Some(reference.clone());
                new_id_verify_req.secret = Some(secret.clone());
                new_id_verify_req.active = Some(true);
                new_id_verify_req.create_utc_dt = Some(now.naive_utc());
                new_id_verify_req.expire_utc_dt = Some((now + chrono::Duration::minutes(5)).naive_utc());
                new_id_verify_req.employee_id = employee.id;
                self.id_verify_req_dao.insert_into(&mut transaction_context, new_id_verify_req)?;

                let mut new_notify_req = NotifyRequestModel::empty();
                new_notify_req.title = Some("Simurgh Identity Verification System".to_string());
                new_notify_req.body = Some(format!("Verification code: {}", secret));
                new_notify_req.create_utc_dt = Some(now.naive_utc());
                new_notify_req.expire_utc_dt = Some((now + chrono::Duration::minutes(15)).naive_utc());
                new_notify_req.employee_id = employee.id;
                self.notify_req_dao.insert_into(&mut transaction_context, new_notify_req)?;

                Ok(reference)
            }) {
                Ok(reference) => {
                    transaction_context.commit().ok();
                    return Ok(NewIdentityVerifyResponseModel { reference, server_utc_dt: now.timestamp() });
                },
                Err(err) => {
                    transaction_context.rollback().ok();
                    return Err(err.into());
                },
            }
    }

    pub fn check_id_verify_req(&mut self, check_id_verify_req: CheckIdentityVerifyRequestModel) -> ServiceResult<(), ServiceError<DaoErrorType>> {
        let mut transaction_context = self.transaction_context_builder.build();
        transaction_context.begin().ok();
        match self.id_verify_req_dao.get_active_request_by_reference(&mut transaction_context, check_id_verify_req.clone().reference)
            .map_err(|err| err.into())
            .and_then(|id_verify_req | {
                if id_verify_req.secret.unwrap() != check_id_verify_req.client_secret {
                    return Err(ServiceError::<DaoErrorType>::IdentityVerifyError);
                }
                self.id_verify_req_dao.deactivate_all_requests(&mut transaction_context, id_verify_req.employee_id.unwrap()).map_err(|err| err.into())
            }) {
                Ok(_) => {
                    transaction_context.commit().ok();
                    return Ok(());
                },
                Err(err) => {
                    transaction_context.rollback().ok();
                    return Err(err);
                },
            }
    }
}
