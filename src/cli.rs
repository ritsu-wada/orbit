use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};

use super::db::*;

#[derive(Parser)]
#[command(
    name= "ns",
    after_help = format!("UTC now: {}", Utc::now().to_rfc3339())
)]
struct Cli {
    #[command(subcommand)]
    actions: Actions,
}

#[derive(Subcommand)]
enum Actions {
    /// show list of tasks
    #[command(alias = "l")] // alias for list
    List {
        /// show all data
        #[arg(short, long)]
        all: bool,
    },
    /// add hope
    #[command(alias = "ah")]
    AddHope {
        #[arg(short, long)]
        title: String,
        /// UTC
        #[arg(short, long)]
        deadline: DateTime<Utc>,
    },
    /// add process
    #[command(alias = "ap")]
    AddProcess {
        /// the process's title
        #[arg(short, long)]
        title: String,
        /// related hope's ID
        #[arg(short, long)]
        id: i32,
    },
    /// タスクの追加
    #[command(alias = "at")]
    AddTask {
        /// タスクのタイトル
        #[arg(short, long)]
        title: String,
        /// 準備、必要なもの場所
        #[arg(short, long)]
        input: String,
        /// 何をする作業？
        #[arg(short, long)]
        action: String,
        /// 何がゴール？
        #[arg(short, long)]
        output: String,
        /// 1: 確実に1時間で終わる 2: 1時間で終わるだろうが不安 3: 未知の作業
        #[arg(short, long, default_value_t = 1)]
        weight: i32,
        /// related Process's ID
        #[arg(short, long)]
        id: Option<i32>,
    },
    Start {
        /// your target task's ID
        #[arg(short, long)]
        id: i32,
    },
    /// change to state complete
    Cmp {
        /// your target task's ID
        #[arg(short, long)]
        id: i32,
    },
    /// delete hopes
    #[command(alias = "dh")]
    DeleteHope {
        /// your target's ID
        #[arg(short, long)]
        id: i32,
    },
    /// delete process
    #[command(alias = "dp")]
    DeleteProcess {
        /// your target's ID
        #[arg(short, long)]
        id: i32,
    },
    /// delete task
    #[command(alias = "dt")]
    DeleteTask {
        /// your target task's ID
        #[arg(short, long)]
        id: i32,
    },
}

pub fn parse_cli() {
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
                let print_tasks = |task: &Task| {
                    println!("　　├─[Task] ID: {} -", task.id);
                    println!("　　│  Title: {}", task.title);
                    println!("　　│  Input: {}", task.input);
                    println!("　　│  Action: {}", task.action);
                    println!("　　└  Output: {}", task.output);
                };
                for hope in &hopes {
                    println!("[Hope ID:{}] ", hope.id);
                    println!(" TITLE: {} ", hope.title);
                    let related_processes: Vec<&Process> =
                        processes.iter().filter(|t| t.hope_id == hope.id).collect();

                    for process in related_processes {
                        println!("　├─[Process] ID: {}", process.id);
                        println!("　└  Title: {}", process.title);
                        let related_tasks: Vec<&Task> = tasks
                            .iter()
                            .filter(|t| t.process_id == Some(process.id))
                            .collect();

                        for task in &related_tasks {
                            print_tasks(&task);
                        }
                    }
                }
                println!("=== Standalone Tasks ===");
                let standalone_tasks: Vec<&Task> =
                    tasks.iter().filter(|t| t.process_id == None).collect();
                for task in standalone_tasks {
                    print_tasks(&task);
                }
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
            id,
        } => match add_task(&conn, title, input, action, output, weight, id) {
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
