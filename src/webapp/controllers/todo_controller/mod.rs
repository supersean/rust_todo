use crate::db::models::FormTodo;
use crate::db::models::NewTodo;
use crate::errors::ServerError;
use crate::ok_response;
use crate::warehouse;
use crate::AppState;
use actix_web::delete;
use actix_web::patch;
use actix_web::{get, post, web, HttpResponse, Responder, Result};

pub fn configure_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .service(all)
            .service(get_all_uncompleted)
            .service(create)
            .service(get)
            .service(delete)
            .service(update)
            .service(complete_todo),
    );
}

#[get("/all")]
pub async fn all(
    data: web::Data<AppState>,
) -> Result<impl Responder, ServerError> {
    let mut conn = data.retrieve_conn()?;

    let todos = &warehouse::todo::get_all_todos(&mut conn)?;

    Ok(ok_response(&todos)?)
}

#[get("/")]
pub async fn get_all_uncompleted(
    data: web::Data<AppState>,
) -> Result<impl Responder, ServerError> {
    let mut conn = data.retrieve_conn()?;

    let todos = &warehouse::todo::get_todos(&mut conn)?;

    Ok(ok_response(&todos)?)
}

#[get("/{id}")]
pub async fn get(
    data: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<impl Responder, ServerError> {
    let mut conn = data.retrieve_conn()?;

    let (id,) = path.into_inner();

    let todo = warehouse::todo::get_todo_by_id(&mut conn, id)?;

    Ok(ok_response(&todo)?)
}
#[post("/")]
pub async fn create(
    data: web::Data<AppState>,
    new_todo: web::Json<NewTodo>,
) -> Result<impl Responder, ServerError> {
    let mut conn = data.retrieve_conn()?;

    let todo = warehouse::todo::add_todo_from_form(&mut conn, &new_todo)?;

    Ok(ok_response(&todo)?)
}

#[patch("/{id}")]
pub async fn update(
    data: web::Data<AppState>,
    path: web::Path<(i32,)>,
    form_todo: web::Json<FormTodo>,
) -> Result<impl Responder, ServerError> {
    println!("{:#?}", form_todo);
    let mut conn = data.retrieve_conn()?;

    let (id,) = path.into_inner();

    let todo = warehouse::todo::update_todo(&mut conn, id, &form_todo)?;

    Ok(ok_response(&todo)?)
}

#[patch("/{id}/complete")]
pub async fn complete_todo(
    data: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<impl Responder, ServerError> {
    let mut conn = data.retrieve_conn()?;
    let (id,) = path.into_inner();
    let todo = warehouse::todo::complete_todo(&mut conn, id)?;
    Ok(ok_response(&todo)?)
}

#[delete("/{id}")]
pub async fn delete(
    data: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<impl Responder, ServerError> {
    let mut conn = data.retrieve_conn()?;

    let (id,) = path.into_inner();

    warehouse::todo::delete_todo_by_id(&mut conn, id)?;

    Ok(HttpResponse::Ok())
}
