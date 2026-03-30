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
    targets: Targets,
}

#[derive(Subcommand)]
enum Targets {
    /// action of hope
    Hope {
        #[command(subcommand)]
        actions: HopeActions,
    },
    /// action of process
    Process {
        #[command(subcommand)]
        actions: ProcessActions,
    },
    /// action of task
    Task {
        #[command(subcommand)]
        actions: TaskActions,
    },
}

#[derive(Subcommand)]
enum ProcessActions {
    /// add process
    Add {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        hope_id: i32,
    },
    List,
    Delete {
        #[arg(short, long)]
        id: i32,
    },
}

#[derive(Subcommand)]
enum HopeActions {
    /// add process
    Add {
        #[arg(short, long)]
        title: String,
        /// UTC
        #[arg(short, long)]
        deadline: DateTime<Utc>,
    },
    List,
    Delete {
        #[arg(short, long)]
        id: i32,
    },
}

#[derive(Subcommand)]
enum TaskActions {
    /// show list of tasks
    List {
        /// show all data
        #[arg(short, long)]
        all: bool,
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
    }, // activeのタスクを完了にする
    /// タスクの追加
    Add {
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
    ///タスク削除
    Delete {
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
    match args.targets {
        Targets::Task { actions } => match actions {
            TaskActions::List { all } => match get_tasks(&conn) {
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
            TaskActions::Add {
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
            TaskActions::Start { id } => {
                println!("!!! Start a task !!! ID: {}", id);
                println!("now happend nothing");
            }
            TaskActions::Cmp { id } => match complete_task(&conn, id) {
                Ok(c) => {
                    println!("Good job !! You Complete ID: {}", id);
                    c
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            },
            TaskActions::Delete { id } => match delete_task(&conn, id) {
                Ok(c) => {
                    println!("Deleted task ID: {}", id);
                    c
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            },
        },
        Targets::Process { actions } => match actions {
            ProcessActions::Add { title, hope_id } => match add_process(&conn, title, hope_id) {
                Ok(c) => {
                    println!("Add process");
                    c
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            },
            ProcessActions::List => match get_process(&conn) {
                Ok(processes) => {
                    for process in processes {
                        print!("ID: {}", process.id);
                        print!("Title: {}", process.title);
                        print!("hope_id: {}", process.hope_id);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            },
            ProcessActions::Delete { id } => match delete_process(&conn, id) {
                Ok(c) => {
                    println!("Delete process ID: {}", id);
                    c
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            },
        },
        Targets::Hope { actions } => match actions {
            HopeActions::Add { title, deadline } => match add_hope(&conn, title, deadline) {
                Ok(c) => {
                    println!("Add hope");
                    c
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            },
            HopeActions::List => match get_hopes(&conn) {
                Ok(hopes) => {
                    for hope in hopes {
                        println!("ID: {}", hope.id);
                        println!("Title: {}", hope.title);
                        println!("DeadLine: {}", hope.deadline);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            },
            HopeActions::Delete { id } => match delete_hope(&conn, id) {
                Ok(c) => {
                    println!("Delete hope ID: {}", id);
                    c
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            },
        },
    }
}
