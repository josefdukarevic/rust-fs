use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct PersonalInfo {
    id: i32,
    full_name: String,
    age: i32,
    email: String,
    phone: String,
    address: String,
    profession: String,
    birthday: String
}