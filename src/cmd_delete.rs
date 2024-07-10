use std::io::Write;
use std::{fs::OpenOptions, io};

use crate::utils::read_lines;

pub fn make_cmd_delete(filename: &String, ids: &[String]) -> io::Result<()> {
    let tmp_file = format!("{}_", filename);

    if ids.is_empty() {
        println!("Delete [id, ...]");
        return Ok(());
    }

    let ids: Vec<u8> = ids.iter().map(|id| id.parse::<u8>().unwrap_or(0)).collect();

    let mut w = OpenOptions::new()
        .create(true)
        .append(true)
        .open(tmp_file)?;

    if let Ok(lines) = read_lines(filename) {
        for (i, line) in lines.map_while(Result::ok).enumerate() {
            let mut matched = false;

            for id in &ids {
                if i + 1 == *id as usize {
                    matched = true;
                }
            }

            if matched {
                println!("Task {:03} deleted: {}", i + 1, line);
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

    #[test]
    #[serial]
    fn test_make_cmd_delete_success() {
        let filename = get_test_file_path(&TEST_FILE.to_string());

        create_test_file(&filename, "Task 1\n- Task 2\nTask 3\n- Task 4\n");

        make_cmd_delete(&filename, &["2".to_string(), "4".to_string()]).unwrap();

        let contents = read_file_to_string(&filename);
        assert_eq!(contents, "Task 1\nTask 3\n");

        delete_test_file(&filename);
    }

    #[test]
    #[serial]
    fn test_make_cmd_delete_empty_ids() {
        let filename = get_test_file_path(&TEST_FILE.to_string());

        create_test_file(&filename, "Task 1\n- Task 2\nTask 3\n- Task 4\n");

        make_cmd_delete(&filename, &[]).unwrap();

        let contents = read_file_to_string(&filename);
        assert_eq!(contents, "Task 1\n- Task 2\nTask 3\n- Task 4\n");

        delete_test_file(&filename);
    }

    #[test]
    #[serial]
    fn test_make_cmd_delete_invalid_id() {
        let filename = get_test_file_path(&TEST_FILE.to_string());

        create_test_file(&filename, "Task 1\n- Task 2\nTask 3\n- Task 4\n");

        make_cmd_delete(&filename, &["a".to_string()]).unwrap();

        let contents = read_file_to_string(&filename);
        assert_eq!(contents, "Task 1\n- Task 2\nTask 3\n- Task 4\n");

        delete_test_file(&filename);
    }
}
