pub mod console;
pub mod db;
pub mod errors;
pub mod warehouse;
pub mod webapp;
use actix_cors::Cors;
use actix_web::http::header::ContentType;
use actix_web::{http, HttpResponse};
use actix_web::{middleware::Logger, web, App, HttpServer};
use db::establish_connection;
use diesel::PgConnection;
use env_logger::Env;
use errors::ServerError;
use serde::Serialize;
use std::sync::Mutex;
use std::sync::MutexGuard;
use webapp::controllers::todo_controller;

// fn main() {
//     let conn = &mut db::establish_connection();
//     loop {
//         console::do_action(conn);
//     }
// }

pub struct DbConnectionError {}

pub struct AppState {
    conn: Mutex<PgConnection>,
}
impl AppState {
    pub fn retrieve_conn(
        &self,
    ) -> Result<MutexGuard<PgConnection>, ServerError> {
        Ok(self.conn.lock().or_else(|_| {
            return Err(ServerError::SeanError {});
        })?)
    }
}

pub fn ok_response<T>(body: &T) -> Result<HttpResponse, ServerError>
where
    T: Serialize,
{
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serialize_object(body)?))
}
fn serialize_object<T>(object: &T) -> Result<String, ServerError>
where
    T: Serialize,
{
    Ok(serde_json::to_string(object)?)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = establish_connection();
    let app_state = web::Data::new(AppState {
        conn: Mutex::new(conn),
    });

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
            ])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(
                web::scope("/api").configure(todo_controller::configure_scope), // .service(hello)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
