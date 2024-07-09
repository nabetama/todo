use clap::{Parser, Subcommand};
use std::env;

mod cmd_list;
mod command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    List,
    Add { value: Vec<String> },
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
            let cmd = cmd_list::make_cmd_list(filename);
            cmd.dispatch();
        }
        Commands::Add { value } => {
            println!("Add: {}", value.join(" "));
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
    use super::*;
    use assert_cmd::Command;

    #[test]
    fn test_list() {
        let mut cmd = Command::cargo_bin("todo").unwrap();
        cmd.arg("list")
            .assert()
            .success()
            .stdout("No tasks found.\n");
    }

    #[test]
    fn test_add() {
        let mut cmd = Command::cargo_bin("todo").unwrap();
        cmd.args(["add", "new todo"])
            .assert()
            .success()
            .stdout("Add: new todo\n");
    }

    #[test]
    fn test_update() {
        let mut cmd = Command::cargo_bin("todo").unwrap();
        cmd.args(["update", "1", "updated todo"])
            .assert()
            .success()
            .stdout("Update: 1 updated todo\n");
    }

    #[test]
    fn test_delete() {
        let mut cmd = Command::cargo_bin("todo").unwrap();
        cmd.args(["delete", "1"])
            .assert()
            .success()
            .stdout("Delete: 1\n");
    }

    #[test]
    fn test_done() {
        let mut cmd = Command::cargo_bin("todo").unwrap();
        cmd.args(["done", "1"])
            .assert()
            .success()
            .stdout("Done: 1\n");
    }

    #[test]
    fn test_undone() {
        let mut cmd = Command::cargo_bin("todo").unwrap();
        cmd.args(["undone", "1"])
            .assert()
            .success()
            .stdout("Undone: 1\n");
    }

    #[test]
    fn test_sort() {
        let mut cmd = Command::cargo_bin("todo").unwrap();
        cmd.arg("sort").assert().success().stdout("Sort\n");
    }
}
