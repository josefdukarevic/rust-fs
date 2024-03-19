use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::PersonalInfo;
use crate::db;
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/submit_personal_info")]
pub async fn submit_personal_info(
    pool: web::Data<DbPool>,
    new_info: web::Json<PersonalInfo>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let personal_info = new_info.into_inner();
    let info_clone = personal_info.clone();

    let result = web::block(move || db::insert_personal_info(&mut conn, &info_clone))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        });
    
    match result {
        Ok(_) => HttpResponse::Ok().json(personal_info),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/get_personal_info")]
pub async fn get_personal_info(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let result = web::block(move || db::get_personal_info(&mut conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        });

        match result {
            Ok(Ok(info)) => HttpResponse::Ok().json(info), // Successfully fetched and serialized
            Ok(Err(e)) => {
                eprintln!("Database error: {:?}", e);
                HttpResponse::InternalServerError().json("Error fetching personal information")
            },
            Err(e) => {
                eprintln!("Blocking error: {:?}", e);
                HttpResponse::InternalServerError().json("Internal server error")
            }
        }
    }
