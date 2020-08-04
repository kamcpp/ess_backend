use chrono::NaiveDateTime;

use crate::schema::employee;
use crate::schema::id_verify_request;
use crate::schema::notify_request;

#[derive(Identifiable, Queryable, Associations)]
#[table_name="employee"]
pub struct Employee {
    pub id: i32,
    pub employee_nr: String,
    pub first_name: String,
    pub second_name: String,
    pub username: String,
    pub office_email: String,
    pub mobile: String,
}

#[derive(Identifiable, Queryable, Associations)]
#[table_name="id_verify_request"]
#[belongs_to(Employee)]
pub struct IdentityVerifyRequest {
    pub id: i32,
    pub reference: String,
    pub secret: String,
    pub active: bool,
    pub create_utc_dt: NaiveDateTime,
    pub expire_utc_dt: NaiveDateTime,
    pub verified_utc_dt: Option<NaiveDateTime>,
    pub employee_id: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[table_name="notify_request"]
#[belongs_to(Employee)]
pub struct NotifyRequest {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub create_utc_dt: NaiveDateTime,
    pub expire_utc_dt: NaiveDateTime,
    pub send_utc_dt: Option<NaiveDateTime>,
    pub employee_id: i32,
}
