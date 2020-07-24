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
    pub id: Option<u64>,
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

