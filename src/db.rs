use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::env;
use crate::models::PersonalInfo;

pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        
}

pub fn insert_personal_info(conn: &mut PgConnection, new_info: &PersonalInfo) -> QueryResult<PersonalInfo> {
    use crate::schema::personal_info;

    diesel::insert_into(personal_info::table)
        .values(new_info)
        .get_result(conn)
}

pub fn get_personal_info(conn: &mut PgConnection) -> QueryResult<Vec<PersonalInfo>> {
    use crate::schema::personal_info::dsl::*;

    personal_info
        .load::<PersonalInfo>(conn)
}
