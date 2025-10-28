mod database;
mod input;
mod todo;

use database::*;
use input::*;

const DB_PATH: &str = "db/sqlite.db";

fn main() {
    let conn = match connect_to_database(DB_PATH) {
        Ok(c) => c,
        Err(e) => {
            println!("Database error: {}", e);
            return;
        }
    };

    loop {
        let menu_operation = match select_menu_operation() {
            OperationResult::Success(v) => v,
            OperationResult::Quit => break,
            OperationResult::Error(e) => {
                println!("{}", e);
                return;
            }
        };

        match menu_operation {
            MenuOperation::ListAllTodos => match list_all_todos(&conn) {
                OperationResult::Success(_) => (),
                OperationResult::Quit => continue,
                OperationResult::Error(e) => {
                    println!("{}", e);
                    return;
                }
            },
            MenuOperation::AddTodo => {
                let todo_text = match get_input("Enter a new todo", &"".to_string()) {
                    OperationResult::Success(v) => v,
                    OperationResult::Quit => continue,
                    OperationResult::Error(e) => {
                        println!("{}", e);
                        return;
                    }
                };

                match add_todo(&conn, &todo_text) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Database error: {}", e);
                        return;
                    }
                }
            }
        }
    }
}
