// use chrono::*;
use rusqlite::Connection;

use super::db::*;
use super::models::*;

// fn make_today_list(conn: &Connection, tree: Vec<HopeBlock>)-> Vec<HopeBlock> {

// }

pub fn eliminate_done(tree: &mut Vec<HopeBlock>) {
    for hope_block in tree.into_iter() {
        hope_block.tasks.retain(|t| t.is_done);
        for process_block in &mut hope_block.process {
            process_block.tasks.retain(|t| t.is_done);
        }
    }
}

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

pub fn get_standalone_tasks(conn: &Connection) -> Vec<Task> {
    let tasks: Vec<Task> = match get_tasks(&conn) {
        Ok(tasks) => tasks,
        Err(e) => {
            eprintln!("Error: {}", e);
            Vec::new()
        }
    };
    tasks
        .into_iter()
        .filter(|t| t.hope_id == None && t.process_id == None)
        .collect()
}
