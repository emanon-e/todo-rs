use core::fmt;

use colored::Colorize;
use dialoguer::{Input, Select};
use rusqlite::{Connection, fallible_iterator::FallibleIterator, params};

const DB_PATH: &str = "db/sqlite.db";

enum OperationResult<T> {
    Success(T),
    Quit,
    Error(String),
}

enum MenuOperation {
    ListAllTodos,
    AddTodo,
}

impl fmt::Display for MenuOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            MenuOperation::ListAllTodos => "List all todos",
            MenuOperation::AddTodo => "Add todo",
        };
        write!(f, "{}", text)
    }
}

const MENU_OPERATIONS: [&'static MenuOperation; 2] =
    [&MenuOperation::ListAllTodos, &MenuOperation::AddTodo];

enum TodoOperation {
    ToggleIsCompleted,
    EditText,
    Delete,
}

impl fmt::Display for TodoOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text: &'static str = match self {
            TodoOperation::ToggleIsCompleted => "Toggle is completed",
            TodoOperation::EditText => "Edit todo text",
            TodoOperation::Delete => "Delete",
        };
        write!(f, "{}", text)
    }
}

const TODO_OPERATIONS: [&'static TodoOperation; 3] = [
    &TodoOperation::ToggleIsCompleted,
    &TodoOperation::EditText,
    &TodoOperation::Delete,
];

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

fn select_menu_operation() -> OperationResult<&'static MenuOperation> {
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

    match menu_operation {
        Ok(Some(idx)) => OperationResult::Success(MENU_OPERATIONS[idx]),
        Ok(None) => OperationResult::Quit,
        Err(e) => OperationResult::Error(e.to_string()),
    }
}

fn select_todo(todos: &Vec<Todo>) -> OperationResult<usize> {
    let selected_todo_index = Select::new()
        .with_prompt(format!(
            "{}\n{}",
            "Press Esc or q to go to the menu".yellow(),
            "Select a todo".cyan()
        ))
        .default(0)
        .report(false)
        .items(todos)
        .max_length(5)
        .interact_opt();

    match selected_todo_index {
        Ok(Some(idx)) => OperationResult::Success(idx),
        Ok(None) => OperationResult::Quit,
        Err(e) => OperationResult::Error(e.to_string()),
    }
}

fn select_todo_operation() -> OperationResult<&'static TodoOperation> {
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

    match todo_operation {
        Ok(Some(idx)) => OperationResult::Success(TODO_OPERATIONS[idx]),
        Ok(None) => OperationResult::Quit,
        Err(e) => OperationResult::Error(e.to_string()),
    }
}

fn get_input(prompt: &str, initial_text: &String) -> OperationResult<String> {
    let input = Input::<String>::new()
        .with_prompt(prompt)
        .report(false)
        .with_initial_text(initial_text)
        .interact_text();

    match input {
        Ok(v) => OperationResult::Success(v),
        Err(e) => OperationResult::Error(e.to_string()),
    }
}

fn list_all_todos(conn: &Connection) -> OperationResult<()> {
    let mut todos = match get_all_todos(&conn) {
        Ok(v) => v,
        Err(e) => return OperationResult::Error(e.to_string()),
    };

    loop {
        let selected_todo_index = match select_todo(&todos) {
            OperationResult::Success(idx) => idx,
            OperationResult::Quit => break,
            OperationResult::Error(e) => return OperationResult::Error(e),
        };

        let todo_operation = match select_todo_operation() {
            OperationResult::Success(v) => v,
            OperationResult::Quit => break,
            OperationResult::Error(e) => return OperationResult::Error(e),
        };

        match todo_operation {
            TodoOperation::ToggleIsCompleted => {
                let selected_todo = &mut todos[selected_todo_index];

                selected_todo.is_completed = !selected_todo.is_completed;
                match update_todo(&conn, &selected_todo) {
                    Err(e) => return OperationResult::Error(e.to_string()),
                    _ => (),
                };
            }
            TodoOperation::EditText => {
                let selected_todo = &mut todos[selected_todo_index];

                selected_todo.text = match get_input("Enter a new todo text", &selected_todo.text) {
                    OperationResult::Success(v) => v,
                    OperationResult::Quit => continue,
                    OperationResult::Error(e) => return OperationResult::Error(e),
                };
                match update_todo(&conn, &selected_todo) {
                    Err(e) => return OperationResult::Error(e.to_string()),
                    _ => (),
                };
            }
            TodoOperation::Delete => {
                match delete_todo(&conn, todos[selected_todo_index].id) {
                    Err(e) => return OperationResult::Error(e.to_string()),
                    _ => (),
                }
                todos.remove(selected_todo_index);
            }
        }
    }

    OperationResult::Success(())
}

fn main() {
    let conn = match prepare_db() {
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
