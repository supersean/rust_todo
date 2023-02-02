use crate::db::schema::todos;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Debug)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub completed: bool,
    pub completed_on: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub text: &'a str,
}

pub struct CompleteTodo {
    pub completed: bool,
}
