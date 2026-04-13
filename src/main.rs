mod calc;
mod cli;
mod db;
mod models;

use calc::*;
use clap::Parser;
use cli::*;
use db::*;

fn main() {
    let conn = match setup_db() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    let args = Cli::parse();
    match args.actions {
        // need to change
        Actions::List { all } => {
            if all {
                let blocks = make_tree(&conn);
                print_all_task(blocks);
            } else {
                println!("Sorry I need --all or -a option to show data");
            }
        }
        // need to change
        Actions::AddHope { title, deadline } => match add_hope(&conn, title, deadline) {
            Ok(c) => {
                println!("Add hope");
                c
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        },
        Actions::AddProcess { title, id } => match add_process(&conn, title, id) {
            Ok(c) => {
                println!("Add process");
                c
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        },
        Actions::AddTask {
            title,
            input,
            action,
            output,
            weight,
            hope_id,
            process_id,
        } => match add_task(
            &conn, title, input, action, output, weight, hope_id, process_id,
        ) {
            Ok(c) => {
                println!("adding data");
                c
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        },
        Actions::Start { id } => {
            println!("!!! Start a task !!! ID: {}", id);
            println!("now happend nothing");
        }
        Actions::Cmp { id } => match complete_task(&conn, id) {
            Ok(c) => {
                println!("Good job !! You Complete ID: {}", id);
                c
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        },
        Actions::DeleteHope { id } => match delete_hope(&conn, id) {
            Ok(c) => {
                println!("Delete hope ID: {}", id);
                c
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        },
        Actions::DeleteProcess { id } => match delete_process(&conn, id) {
            Ok(c) => {
                println!("Delete process ID: {}", id);
                c
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        },
        Actions::DeleteTask { id } => match delete_task(&conn, id) {
            Ok(c) => {
                println!("Deleted task ID: {}", id);
                c
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        },
    }
}
