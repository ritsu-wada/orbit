use rusqlite::{Connection, Result};

pub fn setup_db() -> Result<Connection> {
    let conn = Connection::open("test.db")?;
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

pub fn add_data(conn: &Connection, name: String) -> Result<()> {
    conn.execute("INSERT INTO tasks (name) VALUES (?1)", (name,))?;
    Ok(())
}

// pub fn get_data(conn: &Connection) -> Result<()> {
//     conn.execute("SELECT * ")
// }
