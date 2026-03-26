// use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result};

pub struct Task {
    pub id: i32,
    pub title: String,
    pub input: String,
    pub action: String,
    pub output: String,
    pub weight: i32,
    pub is_done: bool,
    pub process_id: Option<i32>,
}

// const TASK_STATE: [&str; 3] = ["Untouched", "Active", "Complete"];

pub fn setup_db() -> Result<Connection> {
    let conn = Connection::open("test.db")?;
    conn.execute("PRAGMA foreign_keys = ON;", [])?;
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
            hope_id INTEGER,
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
            is_done BOOLEAN NOT NULL DEFAULT 0,
            process_id INTEGER,
            FOREIGN KEY (process_id) REFERENCES processes(id) ON DELETE CASCADE
        )",
        (),
    )?;

    // weight: タスクの重さ、1~3の三段階、1時間で終わるかの自信度

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
        "INSERT INTO tasks (title, input, action, output, weight, process_id) VALUES (?1,?2,?3,?4,?5,?6)",
        (title, input, action, output, weight, process_id),
    )?;
    Ok(())
}

pub fn get_data(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, input, action, output, weight, is_done, process_id FROM tasks ORDER BY is_done ASC",
    )?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            input: row.get(2)?,
            action: row.get(3)?,
            output: row.get(4)?,
            weight: row.get(5)?,
            is_done: row.get(6)?,
            process_id: row.get(7)?,
        })
    })?;

    let tasks: Result<Vec<Task>> = task_iter.collect();

    tasks
}

pub fn complete_task(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("UPDATE tasks SET is_done = 1 WHERE id = (?1)", (id,))?;
    Ok(())
}

pub fn delete_task(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM tasks WHERE id = (?1)", (id,))?;
    Ok(())
}

// pub fn update_data(
//     conn: &Connection,
//     id: i32,
//     title: String,
//     input: String,
//     action: String,
//     output: String,
//     weight: i32,
// ) -> Result<()> {
//     conn.execute("UPDATE tasks SET (?1) = (?2) WHERE id = (?3)", (

//     ))?;
//     Ok(())
// }
