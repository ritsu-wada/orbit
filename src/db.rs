use chrono::{DateTime, Local, Utc};
use rusqlite::{Connection, Result};

struct Task {
    id: i32,
    deadline: DateTime<Utc>,
    name: String,
    description: Option<String>,
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
            name TEXT NOT NULL,
            discription TEXT,
            state TEXT NOT NULL
        )",
        (),
    )?;
    Ok(conn)
}

pub fn add_data(conn: &Connection, name: String) -> Result<()> {
    // let now = Utc::now();
    conn.execute("INSERT INTO tasks (name) VALUES (?1)", (name,))?;
    Ok(())
}

pub fn get_data(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, discription, state FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            deadline: row.get(1)?,
            name: row.get(2)?,
            description: Some(row.get(3)?),
            state: {
                let state_str: String = row.get(4)?;
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
            "ID: {} DeadLine: {} Name: {} State: {:?}",
            t.id, t.deadline, t.name, t.state
        );
        if let Some(desc) = t.description {
            println!("Description: /n {}", desc)
        }
    }
    Ok(())
}
