use std::io::Write;

pub fn make_cmd_add(filename: String, task: String) {
    if task.is_empty() {
        return;
    }

    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
    {
        if let Err(e) = writeln!(file, "{}", task) {
            eprintln!("Error: {}", e);
            return;
        }

        println!("Task added: {}", task);
    } else {
        eprintln!("Error: Unable to open or create file.");
    }
}
