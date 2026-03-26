use chrono::Utc;
use clap::{Parser, Subcommand};

use super::db::*;

#[derive(Parser)]
#[command(
    name= "wish",
    after_help = format!("UTC now: {}", Utc::now().to_rfc3339())
)]
struct Cli {
    #[command(subcommand)]
    actions: Actions,
}

#[derive(Subcommand)]
enum Actions {
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
        #[arg(short, long)]
        weight: i32,
        // UTC example: 1995-08-25T03:00:00Z (JST,UTC+9) 1995-08-25T03:00:00+09:00
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
    match args.actions {
        Actions::List { all } => match get_data(&conn) {
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
                    println!("flag: {}", task.is_done);
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
        Actions::Add {
            title,
            input,
            action,
            output,
            weight,
        } => {
            println!("adding data");
            match add_task(&conn, title, input, action, output, weight, None) {
                Ok(c) => c,
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
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
        Actions::Delete { id } => match delete_task(&conn, id) {
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
