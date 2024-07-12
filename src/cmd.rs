use std::error::Error;

/// This module contains the command interface and its implementations.
use crate::{cmd_add, cmd_clean, cmd_delete, cmd_done, cmd_list, cmd_sort, cmd_undone, cmd_update};

pub trait Command<E: Error> {
    fn execute(&self, filename: &str) -> Result<(), E>;
}

pub struct ListCommand;

impl Command<std::io::Error> for ListCommand {
    fn execute(&self, filename: &str) -> Result<(), std::io::Error> {
        cmd_list::make_cmd_list(filename)
    }
}

pub struct AddCommand {
    pub task: String,
}

impl Command<std::io::Error> for AddCommand {
    fn execute(&self, filename: &str) -> Result<(), std::io::Error> {
        cmd_add::make_cmd_add(filename, &self.task)
    }
}

pub struct CleanCommand;

impl Command<std::io::Error> for CleanCommand {
    fn execute(&self, filename: &str) -> Result<(), std::io::Error> {
        cmd_clean::make_cmd_clean(filename)
    }
}

pub struct UpdateCommand {
    pub index: u8,
    pub task: String,
}

impl Command<std::io::Error> for UpdateCommand {
    fn execute(&self, filename: &str) -> Result<(), std::io::Error> {
        cmd_update::make_cmd_update(filename, self.index, &self.task)
    }
}

pub struct DeleteCommand {
    pub ids: Vec<String>,
}

impl Command<std::io::Error> for DeleteCommand {
    fn execute(&self, filename: &str) -> Result<(), std::io::Error> {
        cmd_delete::make_cmd_delete(filename, &self.ids)
    }
}

pub struct DoneCommand {
    pub ids: Vec<String>,
}

impl Command<std::io::Error> for DoneCommand {
    fn execute(&self, filename: &str) -> Result<(), std::io::Error> {
        cmd_done::make_cmd_done(filename, &self.ids)
    }
}

pub struct UndoneCommand {
    pub ids: Vec<String>,
}

impl Command<std::io::Error> for UndoneCommand {
    fn execute(&self, filename: &str) -> Result<(), std::io::Error> {
        cmd_undone::make_cmd_undone(filename, &self.ids)
    }
}

pub struct SortCommand;

impl Command<std::io::Error> for SortCommand {
    fn execute(&self, filename: &str) -> Result<(), std::io::Error> {
        cmd_sort::make_cmd_sort(filename)
    }
}
