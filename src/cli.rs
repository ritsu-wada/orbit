use chrono::{NaiveDate, Utc};
use clap::{Parser, Subcommand};

use crate::models::*;

#[derive(Parser)]
#[command(
    name= "ns",
    after_help = format!("UTC now: {}", Utc::now().to_rfc3339())
)]
pub struct Cli {
    #[command(subcommand)]
    pub actions: Actions,
}

#[derive(Subcommand)]
pub enum Actions {
    /// show list of tasks
    #[command(alias = "ls")] // alias for list
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
        deadline: NaiveDate,
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
        #[arg(long)]
        hope_id: Option<i32>,
        /// related Process's ID
        #[arg(long)]
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

pub fn print_all_task(blocks: Vec<Block>) {
    let print_related_tasks = |task: &Task| {
        println!("　　├─[Task] ID: {} -", task.id);
        println!("　　│  Title: {}", task.title);
        println!("　　│  Input: {}", task.input);
        println!("　　│  Action: {}", task.action);
        println!("　　└  Output: {}", task.output);
    };
    let print_block = |block: &Block| {
        println!("[Hope ID:{}]:", block.hope.id);
        println!(" DeadLine: {}", block.hope.deadline);
        println!(" TITLE: {}", block.hope.title);
    };
    let print_process = |process: &Process| {
        println!("　├─[Process] ID: {}", process.id);
        println!("　└  Title: {}", process.title);
    };
    for block in blocks {
        print_block(&block);
    }
}

// fn print_standalone_tasks() {
//     println!("=== Standalone Tasks ===");
//     let print_standalone_task = |task: &Task| {
//         println!("┌─[Task] ID: {} -", task.id);
//         println!("│  Title: {}", task.title);
//         println!("│  Input: {}", task.input);
//         println!("│  Action: {}", task.action);
//         println!("└  Output: {}", task.output);
//     };
// }
