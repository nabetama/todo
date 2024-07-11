use std::{
    fs::OpenOptions,
    io::{self, Write},
};

pub fn make_cmd_add(filename: &str, task: &str) -> io::Result<()> {
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

    fn setup() -> String {
        let filename = get_test_file_path(TEST_FILE);
        create_test_file(&filename, "");
        filename
    }

    fn teardown(filename: &str) {
        delete_test_file(filename);
    }

    #[test]
    #[serial]
    fn test_make_cmd_add() {
        let filename = setup();

        make_cmd_add(&filename, "Task 1").unwrap();
        make_cmd_add(&filename, "Task 2").unwrap();

        let content = read_file_to_string(&filename);
        assert_eq!(content, "Task 1\nTask 2\n");

        teardown(&filename);
    }

    #[test]
    #[serial]
    fn test_make_cmd_add_empty_task() {
        let filename = setup();

        make_cmd_add(&filename, "").unwrap();
        let content = read_file_to_string(&filename);
        assert_eq!(content, "");

        teardown(&filename);
    }
}
