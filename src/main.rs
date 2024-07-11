use clap::{Parser, Subcommand};
use cmd::{Command, DoneCommand};
use std::env;

mod cmd;
mod cmd_add;
mod cmd_clean;
mod cmd_delete;
mod cmd_done;
mod cmd_list;
mod cmd_update;
mod utils;

use crate::cmd::{AddCommand, CleanCommand, DeleteCommand, ListCommand, UpdateCommand};

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
    Clean,
    Update { index: u8, task: Vec<String> },
    Delete { ids: Vec<String> },
    Done { ids: Vec<String> },
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

    let command: Box<dyn Command> = match args.command {
        Commands::List => Box::new(ListCommand),
        Commands::Add { task } => Box::new(AddCommand {
            task: task.join(" "),
        }),
        Commands::Clean => Box::new(CleanCommand),
        Commands::Update { index, task } => Box::new(UpdateCommand {
            index,
            task: task.join(" "),
        }),
        Commands::Delete { ids } => Box::new(DeleteCommand { ids }),
        Commands::Done { ids } => Box::new(DoneCommand { ids }),
        _ => unimplemented!(),
    };

    // todo: error handling
    command.execute(&filename);
}

#[cfg(test)]
mod tests {
    pub mod common;
}
