use crate::warehouse;
use crate::db::models::Todo;
use diesel::PgConnection;
use std::io::stdin;

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

pub fn do_action(conn: &mut PgConnection) {
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
    let delete_todo = Action {
        key: "d".to_string(),
        text: "delete a todo".to_string(),
        action: delete_todo,
    };
    let mut actions: Vec<Action> = vec![insert_action, get_action, complete_action, show_completed, delete_todo];

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

pub fn print_todos(conn: &mut PgConnection) {
    let Ok(todos) = warehouse::todo::get_todos(conn) else {
        println!("couldn't locate todos");
        return;
    };

    for todo in todos {
        println!("{:#?}", todo)
    }
}

pub fn add_todo(conn: &mut PgConnection) {
    println!("\nwhat would you like the text to be?");
    let mut text = String::new();
    let text = read_input(&mut text);

    warehouse::todo::add_todo(conn, &text).expect("should be able to add todo");
}

pub fn complete_todo(conn: &mut PgConnection) {
    loop {
        println!("\nWhich todo would you like to complete?");

        let Ok(todos) = warehouse::todo::get_todos(conn) else {
            println!("couldn't locate todos");
            break;
        };
        
        let Ok(choice) = select_todo(&todos) else {
            println!("you must select a todo from the list");
            continue;
        };
        

        let Ok(_) = warehouse::todo::complete_todo(conn, choice.id) else {
            println!("error updating todo");
            break;
        };

        println!("success");

        let Ok(todos) = warehouse::todo::get_todos(conn) else {
            println!("couldn't locate todos");
            break;
        };
        show_todo_list(&todos);
        
        break;

    }
}

pub fn show_todo_list(todos: &Vec<Todo>) {
    println!("\nTodos:");
    for (i, todo) in todos.iter().enumerate() {
        println!("{}: {}", i + 1, todo.text);
    }
}

pub fn show_completed_todos(conn: &mut PgConnection) {
    let Ok(todos) = warehouse::todo::get_completed_todos(conn) else {
        println!("couldnt locate todos");
        return;
    };
    for todo in todos {
        println!("{} : {}", todo.text, todo.completed_on.unwrap())
    }
}

pub fn read_input(input: &mut String) -> &str {
    stdin().read_line(input).unwrap();
    return input.trim_end();
}

pub fn delete_todo(conn: &mut PgConnection) {
    println!("Which todo would you like to delete");
    let Ok(todos) = warehouse::todo::get_todos(conn) else {
        println!("couldn't locate todos");
        return;
    };
    let Ok(todo) = select_todo(&todos) else {return;};

    let Ok(_) = warehouse::todo::delete_todo_by_id(conn, todo.id) else {
        println!("there was an error");
        return;
    };
}

pub struct TodoError;

pub fn select_todo(todos: &Vec<Todo>) -> Result<&Todo, TodoError>{
    show_todo_list(todos);

    let mut text = String::new();
    let text = read_input(&mut text);

    let Ok(choice) = text.parse::<usize>() else { 
        return Err(TodoError{});
    };

    if choice > todos.len() {
        return Err(TodoError{});
    }

    return Ok(&todos[choice - 1]);
}
