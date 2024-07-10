/// This module contains the command interface and its implementations.
use crate::{cmd_add, cmd_clean, cmd_list, cmd_update};

pub trait Command {
    fn execute(&self, filename: &str);
}

pub struct ListCommand;

impl Command for ListCommand {
    fn execute(&self, filename: &str) {
        // Change the type of filename parameter to &str
        cmd_list::make_cmd_list(&filename.to_string());
    }
}

pub struct AddCommand {
    pub task: String,
}

impl Command for AddCommand {
    fn execute(&self, filename: &str) {
        let _ = cmd_add::make_cmd_add(&filename.to_string(), self.task.clone());
    }
}

pub struct CleanCommand;

impl Command for CleanCommand {
    fn execute(&self, filename: &str) {
        // Change the type of filename parameter to &str
        if let Ok(()) = cmd_clean::make_cmd_clean(&filename.to_string()) {
            println!("Tasks cleaned");
        }
    }
}

pub struct UpdateCommand {
    pub index: u8,
    pub task: String,
}

impl Command for UpdateCommand {
    fn execute(&self, filename: &str) {
        let _ = cmd_update::make_cmd_update(&filename.to_string(), self.index, &self.task);
    }
}
