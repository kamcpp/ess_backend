use crate::schema::employee;

#[derive(Insertable)]
#[table_name ="employee"]
pub struct Employee {
    pub id: Option<i32>,
    pub employee_nr: String,
    pub first_name: String,
    pub second_name: String,
    pub username: String,
    pub office_email: String,
    pub mobile: String,
}

