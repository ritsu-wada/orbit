use rusqlite::{Connection, Result};

pub fn setup_db() -> Result<Connection> {
    let conn = Connection::open("tasks.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (),
    )?;
    Ok(conn)
}

// pub fn get_data(&conn) -> Result<()> {
//     conn.execute()?;
//     Ok(())
// }
