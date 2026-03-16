use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result};

pub struct Task {
    id: i32,
    title: String,
    input: String,
    action: String,
    output: String,
    weight: i32,
    state: i32,
    process_id: Option<i32>,
}

const TASK_STATE: [&str; 3] = ["Untouched", "Active", "Complete"];

pub fn setup_db() -> Result<Connection> {
    let conn = Connection::open("test.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS hopes (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            deadline DATETIME NOT NULL
        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS processes (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            FOREIGN KEY (hope_id) REFERENCES hopes(id) ON DELETE CASCADE
        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            input TEXT NOT NULL,
            action TEXT NOT NULL,
            output TEXT NOT NULL,
            weight INTEGER NOT NULL, 
            status TEXT NOT NULL,
            FOREIGN KEY (processes_id) REFERENCES processes(id) ON DELETE CASCADE
        )",
        (),
    )?;

    // weight: タスクの重さ、1~3の三段階、1時間で終わるかの自信度
    // status: 完了済み、取り組み中、未着手に分けたい,

    Ok(conn)
}

pub fn add_task(
    conn: &Connection,
    title: String,
    input: String,
    action: String,
    output: String,
    weight: i32,
    process_id: Option<i32>,
) -> Result<()> {
    // 静的ステークホルダー、配列化タプルを渡すことができる
    conn.execute(
        "INSERT INTO tasks (title, input, action, output, weight, status) VALUES (?1,?2,?3,?4,?5,?6,?7)",
        (title, input, action, output, weight, 0, process_id),
    )?;
    Ok(())
}

pub fn get_data(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, title, input, action, output, weight, status FROM tasks ORDER BY deadline DESC",
    )?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            input: row.get(2)?,
            action: row.get(3)?,
            output: row.get(4)?,
            weight: row.get(5)?,
            state: row.get(6)?,
            process_id: row.get(7)?,
        })
    })?;
    for task in task_iter {
        let t = task?;
        println!(
            "ID: {} Title: {} Input: {} Action: {} Output: {} Weight: {} State: {}",
            t.id, t.title, t.input, t.action, t.output, t.weight, t.state,
        );
    }
    Ok(())
}
