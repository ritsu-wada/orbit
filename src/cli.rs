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
    #[command(alias = "ap")]
    /// add process
    AddProcess {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        hope_id: i32,
    },
    #[command(alias = "at")]
    /// タスクの追加
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
        #[arg(short, long)]
        process_id: Option<i32>,
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
    #[command(alias = "dh")]
    /// delete hopes
    DeleteHope {
        /// your target's ID
        #[arg(short, long)]
        id: i32,
    },
    #[command(alias = "dp")]
    /// delete process
    DeleteProcess {
        /// your target's ID
        #[arg(short, long)]
        id: i32,
    },
    #[command(alias = "dt")]
    ///タスク削除
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
        Actions::List { all } => match get_tasks(&conn) {
            Ok(tasks) => {
                // 三回イテレータ回すよりからの変数を作ってmatchで振り分けるほうがいい
                // けどイテレータの勉強したかったのでこうなった
                let untouch_task: Vec<&Task> = tasks.iter().filter(|&n| !n.is_done).collect();
                let complete_task: Vec<&Task> = tasks.iter().filter(|&n| n.is_done).collect();
                // taskを表示するクロージャを作ってみてる
                let print_tasks = |task: &Task| {
                    println!("- ID: {} -", task.id);
                    println!("Title: {}", task.title);
                    println!("Input: {}", task.input);
                    println!("Action: {}", task.action);
                    println!("Output: {}", task.output);
                };
                println!("= Untouched =");
                for task in untouch_task {
                    print_tasks(task);
                }
                if all {
                    println!("= Completed =");
                    for task in complete_task {
                        print_tasks(task);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e)
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
        Actions::AddProcess { title, hope_id } => match add_process(&conn, title, hope_id) {
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
            process_id,
        } => match add_task(&conn, title, input, action, output, weight, process_id) {
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
