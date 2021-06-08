use crate::schema::employee;

#[derive(Identifiable, Queryable, Associations)]
#[primary_key("username")]
#[table_name="employee"]
pub struct Employee {
    pub first_name: String,
    pub second_name: String,
    pub username: String,
    pub totp_secret: String,
}
