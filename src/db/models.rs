use crate::db::schema::todos;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, AsChangeset, Debug, Serialize)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub completed_on: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, AsChangeset, Debug)]
#[diesel(table_name = todos)]
pub struct FormTodo {
    pub text: Option<String>,
    pub completed_on: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = todos)]
pub struct NewTodo {
    pub text: String,
}

