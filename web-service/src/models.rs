use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EmployeeModel {
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "secondName")]
    pub second_name: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IdentityVerificationRequestModel {
    pub username: String,
    #[serde(rename = "totp")] // Time-based One-Time Password
    pub totp: String,
}
