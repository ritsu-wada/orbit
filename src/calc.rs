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

pub fn make_tree(conn: &Connection) -> Vec<HopeBlock> {
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

    let blocks: Vec<HopeBlock> = hopes
        .into_iter()
        .map(|hope| {
            let hope_id = hope.id;
            let related_processes: Vec<Process> = processes
                .iter()
                .filter(|p| p.hope_id == hope_id)
                .cloned()
                .collect();

            let process_block: Vec<ProcessBlock> = related_processes
                .into_iter()
                .map(|process| {
                    let process_id = process.id;
                    let related_task: Vec<Task> = tasks
                        .iter()
                        .filter(|t| t.process_id == Some(process_id))
                        .cloned()
                        .collect();
                    ProcessBlock {
                        process: process,
                        tasks: related_task,
                    }
                })
                .collect();

            let related_tasks: Vec<Task> = tasks
                .iter()
                .filter(|t| t.hope_id == Some(hope.id))
                .cloned()
                .collect();

            HopeBlock {
                hope,
                process: process_block,
                tasks: related_tasks,
            }
        })
        .collect();

    blocks
}
