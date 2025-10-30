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
            InputResult::Result(Ok(v)) => v,
            InputResult::Result(Err(e)) => {
                println!("Input error: {}", e);
                return;
            }
            InputResult::Quit => break,
        };

        match menu_operation {
            MenuOperation::ListAllTodos => {
                let mut default_selected_todo: usize = 0;

                loop {
                    let mut todos = match get_all_todos(&conn) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("Database error: {}", e);
                            return;
                        }
                    };

                    let selected_todo_index = match select_todo(&todos, default_selected_todo) {
                        InputResult::Result(Ok(idx)) => idx,
                        InputResult::Result(Err(e)) => {
                            println!("Input error: {}", e);
                            return;
                        }
                        InputResult::Quit => break,
                    };

                    let todo_operation = match select_todo_operation() {
                        InputResult::Result(Ok(v)) => v,
                        InputResult::Result(Err(e)) => {
                            println!("Input error: {}", e);
                            return;
                        }
                        InputResult::Quit => continue,
                    };

                    let selected_todo = &mut todos[selected_todo_index];
                    default_selected_todo = selected_todo_index;

                    match todo_operation {
                        TodoOperation::ToggleIsCompleted => {
                            selected_todo.is_completed = !selected_todo.is_completed;

                            match update_todo(&conn, &selected_todo) {
                                Err(e) => {
                                    println!("Database error: {}", e);
                                    return;
                                }
                                _ => (),
                            };
                        }
                        TodoOperation::EditText => {
                            selected_todo.text = match get_text_input(
                                "Enter a new todo text",
                                &selected_todo.text,
                            ) {
                                InputResult::Result(Ok(v)) => v,
                                InputResult::Result(Err(e)) => {
                                    println!("Input error: {}", e);
                                    return;
                                }
                                InputResult::Quit => continue,
                            };
                            match update_todo(&conn, &selected_todo) {
                                Err(e) => {
                                    println!("Database error: {}", e);
                                    return;
                                }
                                _ => (),
                            };
                        }
                        TodoOperation::Delete => {
                            match delete_todo(&conn, todos[selected_todo_index].id) {
                                Err(e) => {
                                    println!("Database error: {}", e);
                                    return;
                                }
                                _ => (),
                            }
                            default_selected_todo = 0;
                        }
                    }
                }
            }
            MenuOperation::AddTodo => {
                let todo_text = match get_text_input("Enter a new todo", "") {
                    InputResult::Result(Ok(v)) => v,
                    InputResult::Result(Err(e)) => {
                        println!("Input error: {}", e);
                        return;
                    }
                    InputResult::Quit => continue,
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
