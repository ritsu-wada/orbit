use clap::{Parser, Subcommand};

use super::db::*;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    actions: Actions,
}

#[derive(Subcommand)]
enum Actions {
    Show {
        #[arg(short, long)]
        all: bool,
    },
    Start,
    Cmp,
    Add {
        #[arg(short, long)]
        data: String,
    },
}

pub fn parse_cli() {
    let args = Cli::parse();
    let conn = match setup_db() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    match args.actions {
        Actions::Show { all } => {
            if all {
                match get_data(&conn) {
                    Ok(c) => c,
                    Err(e) => {
                        println!("Error: {}", e)
                    }
                }
            }
        }
        Actions::Start => {
            println!("start the task");
        }
        Actions::Cmp => {
            println!("nice job !");
        }
        Actions::Add { data } => {
            println!("adding data");
            match add_data(&conn, data) {
                Ok(c) => c,
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}
