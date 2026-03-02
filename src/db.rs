use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result};

pub struct Task {
    id: i32,
    deadline: DateTime<Utc>,
    content: String,
    state: State,
}

#[derive(Debug)]
enum State {
    Active,
    Complite,
    Unknown,
}

pub fn setup_db() -> Result<Connection> {
    let conn = Connection::open("test.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            deadline DATETIME NOT NULL,
            content TEXT NOT NULL,
            state TEXT NOT NULL
        )",
        (),
    )?;
    Ok(conn)
}

pub fn add_data(conn: &Connection, deadline: DateTime<Utc>, content: String) -> Result<()> {
    // 静的ステークホルダー、配列化タプルを渡すことができる
    conn.execute(
        "INSERT INTO tasks (deadline, content, state) VALUES (?1,?2,?3)",
        (deadline, content, "active"),
    )?;
    Ok(())
}

pub fn get_data(conn: &Connection) -> Result<()> {
    let mut stmt =
        conn.prepare("SELECT id, deadline, content, state FROM tasks ORDER BY deadline DESC")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            deadline: row.get(1)?,
            content: row.get(2)?,
            state: {
                let state_str: String = row.get(3)?;
                match state_str.as_str() {
                    "active" => State::Active,
                    "complite" => State::Complite,
                    _ => State::Unknown,
                }
            },
        })
    })?;
    for task in task_iter {
        let t = task?;
        println!(
            "ID: {} DeadLine: {} Content: {} State: {:?}",
            t.id, t.deadline, t.content, t.state
        );
    }
    Ok(())
}
