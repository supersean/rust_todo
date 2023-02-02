use std::io::stdin;

use db::models::Todo;
use diesel::PgConnection;

pub mod db;
pub mod warehouse;

fn main() {
    let conn = &mut db::establish_connection();
    loop {
        do_action(conn);
    }
}

pub struct Action {
    key: String,
    text: String,
    action: fn(&mut PgConnection),
}
impl Action {
    fn do_action(&self, conn: &mut PgConnection) {
        (self.action)(conn)
    }
}

fn do_action(conn: &mut PgConnection) {
    let insert_action = Action {
        key: String::from("i"),
        text: String::from("insert"),
        action: add_todo,
    };
    let get_action = Action {
        key: String::from("g"),
        text: String::from("get"),
        action: print_todos,
    };
    let complete_action = Action {
        key: "c".to_string(),
        text: "complete todo".to_string(),
        action: complete_todo,
    };
    let show_completed = Action {
        key: "sc".to_string(),
        text: "show completed todos".to_string(),
        action: show_completed_todos,
    };
    let mut actions: Vec<Action> = vec![insert_action, get_action, complete_action, show_completed];

    println!("\nwhat action would you like to take?");
    println!("\nactions:");
    for action in &actions {
        println!("({}):{}", action.key, action.text);
    }

    let mut action = String::new();
    let action = read_input(&mut action);

    actions
        .iter_mut()
        .find(|x| x.key == action)
        .unwrap()
        .do_action(conn);
}

fn print_todos(conn: &mut PgConnection) {
    let todos = warehouse::get_todos(conn);

    for todo in todos {
        println!("{:#?}", todo)
    }
}

fn add_todo(conn: &mut PgConnection) {
    println!("\nwhat would you like the text to be?");
    let mut text = String::new();
    let text = read_input(&mut text);

    warehouse::add_todo(conn, &text);
}

fn complete_todo(conn: &mut PgConnection) {
    loop {
        println!("\nWhich todo would you like to complete?");

        let todos = warehouse::get_todos(conn);
        show_todo_list(&todos);

        let mut text = String::new();
        let text = read_input(&mut text);

        let Ok(choice) = text.parse::<usize>() else { 
            println!("You must select a todo from the list.");
            continue;
        };
        
        let todo_to_complete = &todos[choice - 1];

        let Ok(_) = warehouse::complete_todo(conn, todo_to_complete.id) else {
            println!("error updating todo");
            break;
        };

        println!("success");

        let todos = warehouse::get_todos(conn);
        show_todo_list(&todos);
        
        break;

    }
}

fn show_todo_list(todos: &Vec<Todo>) {
    println!("\nTodos:");
    for (i, todo) in todos.iter().enumerate() {
        println!("{}: {}", i + 1, todo.text);
    }
}

fn show_completed_todos(conn: &mut PgConnection) {
    let todos = warehouse::get_completed_todos(conn);
    for todo in todos {
        println!("{} : {}", todo.text, todo.completed_on)
    }
}

fn read_input(input: &mut String) -> &str {
    stdin().read_line(input).unwrap();
    return input.trim_end();
}
