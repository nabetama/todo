use std::io::Write;
use std::{fs::OpenOptions, io};

use crate::utils::read_lines;

pub fn make_cmd_undone(filename: &str, ids: &[String]) -> io::Result<()> {
    if ids.is_empty() {
        println!("No task id provided");
        return Ok(());
    }

    let ids: Vec<u8> = ids.iter().map(|id| id.parse::<u8>().unwrap_or(0)).collect();
    let tmp_file = format!("{}_", filename);

    let mut w = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&tmp_file)?;

    if let Ok(lines) = read_lines(filename) {
        for (i, line) in lines.map_while(Result::ok).enumerate() {
            let mut matched = false;

            for id in &ids {
                if i + 1 == *id as usize {
                    matched = true;
                }
            }

            if matched && line.starts_with('-') {
                writeln!(w, "{}", &line[1..])?;
                println!("Task {} marked as undone: {}", i + 1, &line[1..]);
            } else {
                writeln!(w, "{}", line)?;
            }
        }
    }
    std::fs::remove_file(filename)?;
    std::fs::rename(tmp_file, filename)
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
        let filename = get_test_file_path(TEST_FILE);
        create_test_file(&filename, "-task 1\ntask 2\ntask 3\ntask 4\ntask 5\n");
        filename
    }

    fn teardown(filename: &str) {
        delete_test_file(filename);
    }

    #[test]
    #[serial]
    fn test_make_cmd_undone_success() {
        let filename = setup();

        make_cmd_undone(&filename, &["1".to_string(), "3".to_string()]).unwrap();

        let content = read_file_to_string(&filename);
        assert_eq!(content, "task 1\ntask 2\ntask 3\ntask 4\ntask 5\n");

        teardown(&filename);
    }

    #[test]
    #[serial]
    fn test_make_cmd_undone_no_task_id() {
        let filename = setup();

        make_cmd_undone(&filename, &[]).unwrap();
        let content = read_file_to_string(&filename);
        assert_eq!(content, "-task 1\ntask 2\ntask 3\ntask 4\ntask 5\n");

        teardown(&filename);
    }
}
