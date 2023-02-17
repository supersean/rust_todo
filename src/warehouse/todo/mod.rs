use crate::{
    db::models::{FormTodo, NewTodo, Todo},
    errors::ServerError,
};
use chrono::Utc;
use diesel::{delete, prelude::*};

pub fn get_todos(conn: &mut PgConnection) -> Result<Vec<Todo>, ServerError> {
    use crate::db::schema::todos::dsl::*;
    Ok(todos.filter(completed_on.is_null()).load::<Todo>(conn)?)
}

pub fn get_all_todos(
    conn: &mut PgConnection,
) -> Result<Vec<Todo>, ServerError> {
    use crate::db::schema::todos::dsl::*;
    Ok(todos.load::<Todo>(conn)?)
}

pub fn get_completed_todos(
    conn: &mut PgConnection,
) -> Result<Vec<Todo>, ServerError> {
    use crate::db::schema::todos;

    Ok(todos::dsl::todos
        .filter(todos::dsl::completed_on.is_not_null())
        .load::<Todo>(conn)?)
}

pub fn add_todo_from_form(
    conn: &mut PgConnection,
    todo: &NewTodo,
) -> Result<Todo, ServerError> {
    use crate::db::schema::todos;

    Ok(diesel::insert_into(todos::table)
        .values(todo)
        .get_result(conn)?)
}
pub fn add_todo(
    conn: &mut PgConnection,
    text: &str,
) -> Result<Todo, ServerError> {
    use crate::db::schema::todos;
    let new_todo = NewTodo {
        text: text.to_string(),
    };

    Ok(diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result(conn)?)
}

pub fn complete_todo(
    conn: &mut PgConnection,
    todo_id: i32,
) -> Result<Todo, ServerError> {
    use crate::db::schema::todos;
    Ok(diesel::update(todos::table.find(todo_id))
        .set((todos::completed_on.eq(Utc::now().naive_utc()),))
        .get_result::<Todo>(conn)?)
}

pub fn get_todo_by_id(
    conn: &mut PgConnection,
    id: i32,
) -> Result<Todo, ServerError> {
    use crate::db::schema::todos::dsl;
    return Ok(dsl::todos.find(id).first(conn)?);
}

pub fn delete_todo_by_id(
    conn: &mut PgConnection,
    id: i32,
) -> Result<usize, ServerError> {
    use crate::db::schema::todos::dsl;

    let todo = dsl::todos.find(id).first::<Todo>(conn)?;

    Ok(delete(&todo).execute(conn)?)
}

pub fn update_todo(
    conn: &mut PgConnection,
    id: i32,
    update_todo: &FormTodo,
) -> Result<Todo, ServerError> {
    use crate::db::schema::todos::dsl;

    // let todo = dsl::todos.find(id).first::<Todo>(conn)?;

    Ok(diesel::update(dsl::todos.find(id))
        .set(update_todo)
        .get_result::<Todo>(conn)?)
}
