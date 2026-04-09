// use chrono::*;
use rusqlite::Connection;

use super::db::*;
use super::models::*;

// fn make_today_list(conn: &Connection) {
//     let hopes: Vec<Hope> = match get_hopes(&conn) {
//         Ok(hopes) => hopes,
//         Err(e) => {
//             eprintln!("Error: {}", e);
//             Vec::new()
//         }
//     };
//     let processes: Vec<Process> = match get_process(&conn) {
//         Ok(process) => process,
//         Err(e) => {
//             eprintln!("Error: {}", e);
//             Vec::new()
//         }
//     };
//     let tasks: Vec<Task> = match get_tasks(&conn) {
//         Ok(tasks) => tasks,
//         Err(e) => {
//             eprintln!("Error: {}", e);
//             Vec::new()
//         }
//     };
// }

pub fn make_block(conn: &Connection) -> Vec<Block> {
    let hopes: Vec<Hope> = match get_hopes(&conn) {
        Ok(hopes) => hopes,
        Err(e) => {
            eprintln!("Error: {}", e);
            Vec::new()
        }
    };
    let processes: Vec<Process> = match get_process(&conn) {
        Ok(process) => process,
        Err(e) => {
            eprintln!("Error: {}", e);
            Vec::new()
        }
    };
    let tasks: Vec<Task> = match get_tasks(&conn) {
        Ok(tasks) => tasks,
        Err(e) => {
            eprintln!("Error: {}", e);
            Vec::new()
        }
    };

    let blocks: Vec<Block> = hopes
        .into_iter()
        .map(|hope| {
            let hope_id = hope.id;
            let related_processes: Vec<Process> = processes
                .iter()
                .filter(|p| p.hope_id == hope_id)
                .cloned()
                .collect();
            let related_tasks: Vec<Task> = tasks
                .iter()
                .filter(|t| t.hope_id == Some(hope.id))
                .cloned()
                .collect();
            Block {
                hope,
                process: related_processes,
                task: related_tasks,
            }
        })
        .collect();
    eprintln!("processに基づいているタスクをひょうじできてない！");
    blocks
}
