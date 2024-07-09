use std::{
    fs::OpenOptions,
    io::{self, Write},
};

pub fn make_cmd_add(filename: String, task: String) -> io::Result<()> {
    if task.is_empty() {
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;

    writeln!(file, "{}", task)?;
    println!("Task added: {}", task);
    Ok(())
}
