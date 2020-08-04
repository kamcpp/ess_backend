use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct HelloRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct HelloResponse {
    pub greeting: String,
}

#[derive(Deserialize, Serialize)]
pub struct EmployeeModel {
    pub id: Option<i32>,
    #[serde(rename = "employeeNr")]
    pub employee_nr: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "secondName")]
    pub second_name: String,
    pub username: String,
    #[serde(rename = "officeEmail")]
    pub office_email: String,
    pub mobile: String,
}

#[derive(Deserialize)]
pub struct NewIdentityVerifyRequestModel {
    pub username: String,
    #[serde(rename = "clientUtcDateTime")]
    pub client_utc_dt: i64,
}

#[derive(Serialize)]
pub struct NewIdentityVerifyResponseModel {
    pub reference: String,
    #[serde(rename = "serverUtcDateTime")]
    pub server_utc_dt: i64,
}

#[derive(Deserialize)]
pub struct CheckIdentityVerifyRequestModel {
    pub reference: String,
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    #[serde(rename = "clientUtcDateTime")]
    pub client_utc_dt: i64,
}
