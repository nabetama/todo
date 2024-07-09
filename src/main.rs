use clap::{Parser, Subcommand};
use std::env;

mod cmd_add;
mod cmd_list;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    List,
    Add { task: Vec<String> },
    Update { index: u8, task: Vec<String> },
    Delete { index: u8 },
    Done { index: u8 },
    Undone { index: u8 },
    Sort,
}

const TODO_FILE: &str = ".todo";

fn main() {
    let mut filename = String::new();
    let mut exist_current_todo = false;

    if let Ok(cur_dir) = env::current_dir() {
        let path = cur_dir.join(TODO_FILE);
        filename = path.to_str().unwrap_or("").to_string();
        exist_current_todo = true;
    }

    if !exist_current_todo {
        let home = dirs::home_dir();
        if let Some(home_dir) = home {
            let path = home_dir.join(TODO_FILE);
            filename = path.to_str().unwrap_or("").to_string();
        }
    }

    let args = Cli::parse();

    match args.command {
        Commands::List => {
            cmd_list::make_cmd_list(&filename);
        }
        Commands::Add { task } => {
            let _ = cmd_add::make_cmd_add(&filename, task.join(" "));
        }
        Commands::Update { index, task } => {
            println!("Update: {} {}", index, task.join(" "));
        }
        Commands::Delete { index } => {
            println!("Delete: {}", index);
        }
        Commands::Done { index } => {
            println!("Done: {}", index);
        }
        Commands::Undone { index } => {
            println!("Undone: {}", index);
        }
        Commands::Sort => {
            println!("Sort");
        }
    }
}

#[cfg(test)]
mod tests {
    pub mod common;
}
