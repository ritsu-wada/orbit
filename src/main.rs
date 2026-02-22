use clap::{Parser, Subcommand};
// use std::path::PathBuf;
mod db;
use db::*;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    actions: Actions,
}

#[derive(Subcommand)]
enum Actions {
    Show {
        #[arg(short, long)]
        amount: String,
    },
    Start,
    Cmp,
    Add {
        #[arg(short, long)]
        data: String,
    },
}

fn main() {
    let args = Cli::parse();
    let conn = match setup_db() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    match args.actions {
        Actions::Show { amount } => {
            println!("show the tasks: {}", amount);
        }
        Actions::Start => {
            println!("start the task");
        }
        Actions::Cmp => {
            println!("nice job !");
        }
        Actions::Add { data } => {
            println!("adding data");
            add_data(&conn, data);
        }
    }
}
