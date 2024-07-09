use std::{
    fs::OpenOptions,
    io::{self, Write},
};

pub fn make_cmd_add(filename: &String, task: String) -> io::Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;

    use serial_test::serial;

    use crate::tests::common::{
        create_test_file, delete_test_file, get_test_file_path, read_file_to_string,
    };

    const TEST_FILE: &str = ".todo_test";

    #[test]
    #[serial]
    fn test_make_cmd_add() {
        let filename = get_test_file_path(&TEST_FILE.to_string());

        create_test_file(&filename, "");

        make_cmd_add(&filename, "Task 1".to_string()).unwrap();
        make_cmd_add(&filename, "Task 2".to_string()).unwrap();

        let content = read_file_to_string(&filename);
        assert_eq!(content, "Task 1\nTask 2\n");

        delete_test_file(&filename);
    }

    #[test]
    #[serial]
    fn test_make_cmd_add_empty_task() {
        let filename = get_test_file_path(&TEST_FILE.to_string());

        create_test_file(&filename, "");

        make_cmd_add(&filename, "".to_string()).unwrap();

        let content = read_file_to_string(&filename);
        assert_eq!(content, "");

        delete_test_file(&filename);
    }
}
