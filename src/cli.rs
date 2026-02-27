use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};

use super::db::*;

#[derive(Parser)]
#[command(
    name= "tm",
    after_help = format!("UTC now: {}", Utc::now().to_rfc3339())
)]
struct Cli {
    #[command(subcommand)]
    actions: Actions,
}

#[derive(Subcommand)]
enum Actions {
    Show,
    Start,
    Cmp,
    /// タスクの追加
    Add {
        /// UTC example: 1995-08-25T03:00:00Z (JST,UTC+9) 1995-08-25T03:00:00+09:00
        #[arg(short, long)]
        deadline: DateTime<Utc>,
        /// タスクの内容
        #[arg(short, long)]
        content: String,
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
        Actions::Show => match get_data(&conn) {
            Ok(c) => c,
            Err(e) => {
                println!("Error: {}", e)
            }
        },
        Actions::Add { deadline, content } => {
            println!("adding data");
            match add_data(&conn, deadline, content) {
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
