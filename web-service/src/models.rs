use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct HelloRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct HelloResponse {
    pub greeting: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmployeeModel {
    pub id: Option<i32>,
    #[serde(rename = "employeeNr")]
    pub employee_nr: Option<String>,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "secondName")]
    pub second_name: Option<String>,
    pub username: Option<String>,
    #[serde(rename = "officeEmail")]
    pub office_email: Option<String>,
    pub mobile: Option<String>,
}

impl EmployeeModel {
    pub fn empty() -> Self {
        Self {
            id: None,
            employee_nr: None,
            first_name: None,
            second_name: None,
            username: None,
            office_email: None,
            mobile: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IdentityVerifyRequestModel {
    pub id: Option<i32>,
    pub reference: Option<String>,
    pub secret: Option<String>,
    pub active: Option<bool>,
    pub create_utc_dt: Option<NaiveDateTime>,
    pub expire_utc_dt: Option<NaiveDateTime>,
    pub verify_utc_dt: Option<NaiveDateTime>,
    pub employee_id: Option<i32>,
}

impl IdentityVerifyRequestModel {
    pub fn empty() -> Self {
        Self {
            id: None,
            reference: None,
            secret: None,
            active: None,
            create_utc_dt: None,
            expire_utc_dt: None,
            verify_utc_dt: None,
            employee_id: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NotifyRequestModel {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub create_utc_dt: Option<NaiveDateTime>,
    pub expire_utc_dt: Option<NaiveDateTime>,
    pub send_utc_dt: Option<NaiveDateTime>,
    pub employee_id: Option<i32>,
}

impl NotifyRequestModel {
    pub fn empty() -> Self {
        Self {
            id: None,
            title: None,
            body: None,
            create_utc_dt: None,
            expire_utc_dt: None,
            send_utc_dt: None,
            employee_id: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NewIdentityVerifyRequestModel {
    pub username: String,
    #[serde(rename = "clientUtcDateTime")]
    pub client_utc_dt: i64,
}

#[derive(Debug, Serialize)]
pub struct NewIdentityVerifyResponseModel {
    pub reference: String,
    #[serde(rename = "serverUtcDateTime")]
    pub server_utc_dt: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CheckIdentityVerifyRequestModel {
    pub reference: String,
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    #[serde(rename = "clientUtcDateTime")]
    pub client_utc_dt: i64,
}
