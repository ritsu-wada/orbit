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
    Show {
        /// show all data
        #[arg(short, long)]
        all: bool,
        /// show current tasks only
        #[arg(short, long)]
        current: bool,
        /// show untouched tasks only
        #[arg(short, long)]
        untouched: bool,
    },
    Start,
    Cmp,
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
        Actions::Show {
            all,
            current,
            untouched,
        } => match get_data(&conn) {
            Ok(tasks) => {
                // 三回イテレータ回すよりからの変数を作ってmatchで振り分けるほうがいい
                // けどイテレータの勉強したかったのでこうなった
                let untouch_task: Vec<&Task> = tasks.iter().filter(|&n| n.state == 0).collect();
                let active_task: Vec<&Task> = tasks.iter().filter(|&n| n.state == 1).collect();
                let complete_task: Vec<&Task> = tasks.iter().filter(|&n| n.state == 2).collect();
                // taskを表示するクロージャを作ってみてる
                let print_tasks = |task: &Task| {
                    println!("- ID: {} -", task.id);
                    println!("Title: {}", task.title);
                    println!("Input: {}", task.input);
                    println!("Action: {}", task.action);
                    println!("Output: {}", task.output);
                };
                if !untouched {
                    println!("= Active =");
                    for task in active_task {
                        print_tasks(task);
                    }
                }
                if !current {
                    println!("= Untouched =");
                    for task in untouch_task {
                        print_tasks(task);
                    }
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
        Actions::Start => {
            println!("start the task");
        }
        Actions::Cmp => {
            println!("nice job !");
        }
    }
}
