use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct HelloRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct HelloResponse {
    pub greeting: String,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize)]
pub struct CheckIdentityVerifyRequestModel {
    pub reference: String,
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    #[serde(rename = "clientUtcDateTime")]
    pub client_utc_dt: i64,
}
