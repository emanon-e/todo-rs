use rusqlite::Connection;

const DB_PATH: &str = "db/sqlite.db";

enum TodoError {
    DatabaseError,
}

fn prepare_db(conn: &Connection) -> Result<(), TodoError> {
    let res = conn.execute(
        "CREATE TABLE IF NOT EXISTS
        todos (
            id   INTEGER PRIMARY KEY,
            text TEXT NOT NULL,
            is_completed INTEGER NOT NULL
        )",
        (),
    );

    match res {
        Ok(_) => (),
        Err(_) => return Err(TodoError::DatabaseError),
    };

    Ok(())
}

fn main() {
    let conn = match Connection::open(DB_PATH) {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    match prepare_db(&conn) {
        Ok(_) => (),
        Err(_) => println!("A database error occurred"),
    };
}
