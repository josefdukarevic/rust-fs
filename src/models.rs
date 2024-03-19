use crate::schema::personal_info;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Debug, Queryable, Insertable, Clone)]
#[diesel(table_name = personal_info)]
pub struct PersonalInfo {
    id: i32,
    full_name: String,
    age: i32,
    email: String,
    phone: String,
    address: String,
    profession: String,
    birth_date: NaiveDate
}