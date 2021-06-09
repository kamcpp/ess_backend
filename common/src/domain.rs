use crate::schema::employee;

#[derive(Identifiable, Queryable, Associations)]
#[primary_key("username")]
#[table_name="employee"]
pub struct Employee {
    >> FILL HERE <<
}
