use std::io::Write;
use std::{fs::OpenOptions, io};

use crate::utils::read_lines;

pub fn make_cmd_update(filename: &String, index: u8, task: &str) -> io::Result<()> {
    if task.is_empty() {
        println!("Task cannot be empty");
        return Ok(());
    }

    let mut w = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{}_", filename))?;

    if let Ok(lines) = read_lines(filename) {
        for (i, line) in lines.map_while(Result::ok).enumerate() {
            if i + 1 == index as usize {
                let mut new_task = task.to_string();

                if line.starts_with('-') {
                    new_task = format!("-{}", task);
                }
                writeln!(w, "{}", new_task)?;
                println!("Task {:03} updated: {}", i + 1, task);
            } else {
                writeln!(w, "{}", line)?;
            }
        }
    }

    std::fs::remove_file(filename)?;
    std::fs::rename(format!("{}_", filename), filename)
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;

    use crate::tests::common::{
        create_test_file, delete_test_file, get_test_file_path, read_file_to_string,
    };

    const TEST_FILE: &str = ".todo_test";

    fn setup() -> String {
        let filename = get_test_file_path(&TEST_FILE.to_string());
        create_test_file(&filename, "Task 1\n- Task 2\nTask 3\n- Task 4\n");
        filename
    }

    fn teardown(filename: &str) {
        delete_test_file(&filename.to_string());
    }

    #[test]
    #[serial]
    fn test_make_cmd_update_success() {
        let filename = setup();
        make_cmd_update(&filename, 2, "Task 5").unwrap();

        let contents = read_file_to_string(&filename);
        assert_eq!(contents, "Task 1\n-Task 5\nTask 3\n- Task 4\n");

        teardown(&filename);
    }

    #[test]
    #[serial]
    fn test_make_cmd_update_fail_task_is_empty() {
        let filename = setup();

        make_cmd_update(&filename, 2, "").unwrap();

        let contents = read_file_to_string(&filename);
        assert_eq!(contents, "Task 1\n- Task 2\nTask 3\n- Task 4\n");

        teardown(&filename);
    }
}
