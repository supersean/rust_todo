use crate::db::models::{NewTodo, Todo};
use chrono::Utc;
use diesel::{prelude::*, update};

pub fn get_todos(conn: &mut PgConnection) -> Vec<Todo> {
    use crate::db::schema::todos::dsl::*;
    let results = todos
        .filter(completed.eq(false))
        .load::<Todo>(conn)
        .expect("Error loading todos");
    return results;
}

pub fn get_completed_todos(conn: &mut PgConnection) -> Vec<Todo> {
    use crate::db::schema::todos;
    let results = todos::dsl::todos
        .filter(todos::dsl::completed.eq(true))
        .load::<Todo>(conn)
        .expect("Error loading todos");
    return results;
}

pub fn add_todo(conn: &mut PgConnection, text: &str) -> Todo {
    use crate::db::schema::todos;
    let new_todo = NewTodo { text };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result(conn)
        .expect("Error saving new todo")
}

pub fn complete_todo(conn: &mut PgConnection, todo_id: i32) -> Result<Todo, diesel::result::Error> {
    use crate::db::schema::todos;
    // use crate::db::schema::todos::dsl::*;

    // let todo = todos.find(todo_id).
    return diesel::update(todos::table.find(todo_id))
        .set((
            todos::completed.eq(true),
            todos::completed_on.eq(Utc::now().naive_utc()),
        ))
        .get_result::<Todo>(conn);
}

pub fn get_todo_by_id(conn: &mut PgConnection, id: i32) -> Result<Todo, diesel::result::Error> {
    use crate::db::schema::todos::dsl::*;
    return todos.find(id).first(conn);
}
