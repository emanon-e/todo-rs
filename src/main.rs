use core::fmt;

use colored::Colorize;
use dialoguer::{Input, Select};
use rusqlite::{Connection, fallible_iterator::FallibleIterator, params};

const DB_PATH: &str = "db/sqlite.db";
const MENU_OPERATIONS: [&str; 2] = ["List all todos", "Add todo"];
const TODO_OPERATIONS: [&str; 3] = ["Toggle is completed", "Edit todo text", "Delete"];

struct Todo {
    id: u32,
    text: String,
    is_completed: bool,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.is_completed {
            true => write!(f, "{}", self.text.strikethrough().dimmed()),
            false => write!(f, "{}", &self.text),
        }
    }
}

fn prepare_db() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS
        TODOS (
            ID   INTEGER PRIMARY KEY,
            TEXT TEXT NOT NULL,
            IS_COMPLETED INTEGER DEFAULT 0 NOT NULL
        )",
        (),
    )?;

    Ok(conn)
}

fn update_todo(conn: &Connection, todo: &Todo) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "UPDATE TODOS SET
        TEXT = ?1,
        IS_COMPLETED = ?2
        WHERE ID = ?3",
        params![&todo.text, todo.is_completed, todo.id],
    )
}

fn delete_todo(conn: &Connection, todo_id: u32) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM TODOS WHERE ID = ?1", [todo_id])
}

fn add_todo(conn: &Connection, todo_text: &String) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "INSERT INTO TODOS(TEXT) VALUES(:todo_text)",
        &[(":todo_text", todo_text)],
    )
}

fn get_all_todos(conn: &Connection) -> Result<Vec<Todo>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT * FROM TODOS")?;
    let rows: rusqlite::Rows<'_> = stmt.query([])?;

    let todos: Vec<Todo> = rows
        .map(|row| {
            Ok(Todo {
                id: row.get(0)?,
                text: row.get(1)?,
                is_completed: row.get(2)?,
            })
        })
        .collect()?;

    Ok(todos)
}

fn main() {
    let conn = match prepare_db() {
        Ok(c) => c,
        Err(e) => {
            println!("Database error: {}", e);
            return;
        }
    };

    // TODO: Extract large code blocks to functions
    loop {
        let menu_operation = Select::new()
            .with_prompt(format!(
                "{}\n{}",
                "Press Esc or q to quit".yellow(),
                "Select an operation".cyan()
            ))
            .default(0)
            .report(false)
            .items(&MENU_OPERATIONS)
            .interact_opt();

        let menu_operation = match menu_operation {
            Ok(Some(idx)) => idx,
            Ok(None) => break,
            Err(_) => {
                println!("A cli error occured");
                return;
            }
        };

        match menu_operation {
            0 => {
                let mut todos = match get_all_todos(&conn) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("Database error: {}", e);
                        return;
                    }
                };

                loop {
                    let selected_todo_index = Select::new()
                        .with_prompt(format!(
                            "{}\n{}",
                            "Press Esc or q to go to the menu".yellow(),
                            "Select a todo".cyan()
                        ))
                        .default(0)
                        .report(false)
                        .items(&todos)
                        .max_length(5)
                        .interact_opt();

                    let selected_todo_index = match selected_todo_index {
                        Ok(Some(idx)) => idx,
                        Ok(None) => break,
                        Err(_) => {
                            println!("A cli error occured");
                            return;
                        }
                    };

                    let todo_operation = Select::new()
                        .with_prompt(format!(
                            "{}\n{}",
                            "Press Esc or q to go to the menu".yellow(),
                            "Select an operation".cyan()
                        ))
                        .default(0)
                        .report(false)
                        .items(&TODO_OPERATIONS)
                        .interact_opt();

                    let todo_operation = match todo_operation {
                        Ok(Some(idx)) => idx,
                        Ok(None) => break,
                        Err(_) => {
                            println!("A cli error occured");
                            return;
                        }
                    };

                    match todo_operation {
                        0 => {
                            let selected_todo = match todos.get_mut(selected_todo_index) {
                                Some(v) => v,
                                None => {
                                    println!("Something went wrong");
                                    return;
                                }
                            };

                            selected_todo.is_completed = !selected_todo.is_completed;
                            match update_todo(&conn, &selected_todo) {
                                Err(e) => {
                                    println!("Database error: {}", e);
                                    return;
                                }
                                _ => (),
                            }
                        }
                        1 => {
                            let selected_todo = match todos.get_mut(selected_todo_index) {
                                Some(v) => v,
                                None => {
                                    println!("Something went wrong");
                                    return;
                                }
                            };

                            let todo_text = Input::<String>::new()
                                .with_prompt("Enter a new todo text")
                                .report(false)
                                .with_initial_text(&selected_todo.text)
                                .interact_text();

                            let todo_text = match todo_text {
                                Ok(v) => v,
                                Err(_) => {
                                    println!("A cli error occured");
                                    return;
                                }
                            };

                            selected_todo.text = todo_text;
                            match update_todo(&conn, &selected_todo) {
                                Err(e) => {
                                    println!("Database error: {}", e);
                                    return;
                                }
                                _ => (),
                            }
                        }
                        2 => {
                            match delete_todo(&conn, todos[selected_todo_index].id) {
                                Err(e) => {
                                    println!("Database error: {}", e);
                                    return;
                                }
                                _ => (),
                            }
                            todos.remove(selected_todo_index);
                        }
                        _ => {
                            println!("Something went wrong");
                            return;
                        }
                    }
                }
            }
            1 => {
                let todo_text = Input::<String>::new()
                    .with_prompt("Enter a new todo")
                    .report(false)
                    .interact_text();

                let todo_text = match todo_text {
                    Ok(t) => t,
                    Err(_) => {
                        println!("A cli error occured");
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
            _ => {
                println!("Something went wrong");
                return;
            }
        }
    }
}
