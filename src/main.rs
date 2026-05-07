mod calc;
mod cli;
mod db;
mod models;

use calc::*;
use clap::Parser;
use cli::*;
use db::*;

fn main() {
    if cfg!(debug_assertions) {
        println!("!!= Now is debug build =!!");
    }
    let data_path = get_db_path();
    let conn = match setup_db(data_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    let args = Cli::parse();
    match args.actions {
        // need to change
        Actions::List { target } => match target {
            Target::Hope { id } => match id {
                Some(value) => {
                    println!("print hope id = {}", value);
                }
                None => match get_hopes(&conn) {
                    Ok(hope_vec) => print_hope_list(hope_vec),
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                },
            },
            Target::All => {
                let tree = make_tree(&conn);
                print_all_task(tree);
                let standalone_tasks = get_standalone_tasks(&conn);
                print_standalone_tasks(standalone_tasks);
            }
            _ => {
                let mut tree = make_tree(&conn);
                eliminate_done(&mut tree);
                print_all_task(tree);
                let standalone_tasks = get_standalone_tasks(&conn);
                print_standalone_tasks(standalone_tasks);
            }
        },
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
            h_id,
            p_id,
        } => match add_task(&conn, title, input, action, output, weight, h_id, p_id) {
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
        Actions::Delete { target } => match target {
            Target::Hope { id } => match id {
                Some(value) => match delete_hope(&conn, value) {
                    Ok(_c) => (),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                },
                None => {}
            },
            Target::Process { id } => match id {
                Some(value) => match delete_process(&conn, value) {
                    Ok(_c) => (),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                },
                None => {}
            },
            Target::Task { id } => match id {
                Some(value) => match delete_task(&conn, value) {
                    Ok(_c) => (),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                },
                None => {}
            },
            Target::All => {
                println!("Are you Ok ? you wanna to delete all data??");
                println!("Sorry I cant sport that function")
            }
        },
    }
}
