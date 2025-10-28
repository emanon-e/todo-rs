use super::todo::Todo;
use rusqlite::{Connection, fallible_iterator::FallibleIterator, params};

pub fn connect_to_database(path: &str) -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open(path)?;

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

pub fn update_todo(conn: &Connection, todo: &Todo) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "UPDATE TODOS SET
        TEXT = ?1,
        IS_COMPLETED = ?2
        WHERE ID = ?3",
        params![&todo.text, todo.is_completed, todo.id],
    )
}

pub fn delete_todo(conn: &Connection, todo_id: u32) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM TODOS WHERE ID = ?1", [todo_id])
}

pub fn add_todo(conn: &Connection, todo_text: &String) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "INSERT INTO TODOS(TEXT) VALUES(:todo_text)",
        &[(":todo_text", todo_text)],
    )
}

pub fn get_all_todos(conn: &Connection) -> Result<Vec<Todo>, rusqlite::Error> {
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
