use actix_web::{App, HttpServer};
use actix_web::web::Data; // Fix: Change the import statement to actix_web::web::Data
use diesel::r2d2::{self, ConnectionManager};
use std::{env, io};
use diesel::pg::PgConnection;
use tokio::runtime::Runtime;

use backend::routes::personal_info::submit_personal_info;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
fn main() -> io::Result<()> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        HttpServer::new(move || {
            App::new()
                .app_data(Data::new(pool.clone()))
                .service(submit_personal_info)
                
        })
        .bind("127.0.0.1:8090")?
        .run()
        .await
    })
    
}
