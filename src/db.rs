use rusqlite::{Connection, Result};

struct Task {
    id: i32,
    name: String,
}

pub fn setup_db() -> Result<Connection> {
    let conn = Connection::open("test.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
        )",
        (),
    )?;
    Ok(conn)
}

pub fn add_data(conn: &Connection, name: String) -> Result<()> {
    conn.execute("INSERT INTO tasks (name) VALUES (?1)", (name,))?;
    Ok(())
}

pub fn get_data(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, FROM tasks")?;
    let _task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    Ok(())
}
