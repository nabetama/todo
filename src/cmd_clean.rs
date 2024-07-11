use std::io::Write;
use std::{fs::OpenOptions, io};

use crate::utils::read_lines;

pub fn make_cmd_clean(filename: &String) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{}_", filename))?;

    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            if !line.starts_with('-') {
                writeln!(file, "{}", line)?;
            }
        }
        println!("Tasks cleaned.");
    }

    std::fs::remove_file(filename)?;
    std::fs::rename(format!("{}_", filename), filename)
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        os::unix::fs::PermissionsExt,
    };

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
    fn test_make_cmd_clean_success() {
        let filename = setup();

        make_cmd_clean(&filename).unwrap();

        let content = read_file_to_string(&filename);
        assert_eq!(content, "Task 1\nTask 3\n");

        teardown(&filename);
    }

    #[test]
    #[serial]
    fn test_make_cmd_clean_open_error() {
        let filename = "/non_exist_dir/test_clean.todo";
        let result = make_cmd_clean(&filename.to_string());
        assert!(result.is_err());

        teardown(&filename);
    }

    #[test]
    #[serial]
    fn test_make_cmd_clean_write_error() {
        let filename = setup();

        let temp_filename = format!("{}_", filename);
        let _ = File::create(&temp_filename);

        fs::set_permissions(&temp_filename, fs::Permissions::from_mode(0o444))
            .expect("Unable to set permissions");

        let result = make_cmd_clean(&filename.to_string());
        assert!(result.is_err());

        delete_test_file(&filename.to_string());
        delete_test_file(&temp_filename);
    }

    #[test]
    #[serial]
    fn test_make_cmd_clean_remove_file_error() {
        let filename = setup();

        let temp_filename = format!("{}_", filename);
        let _ = File::create(&temp_filename);

        fs::set_permissions(&temp_filename, fs::Permissions::from_mode(0o444))
            .expect("Unable to set permissions");

        let result = make_cmd_clean(&filename.to_string());
        assert!(result.is_err());

        fs::set_permissions(&temp_filename, fs::Permissions::from_mode(0o666))
            .expect("Unable to set permissions");

        delete_test_file(&temp_filename);
        delete_test_file(&filename.to_string());
    }

    // todo: fix this test.
    // It failes, so the file was already removed from function make_cmd_clean().
    // #[test]
    // #[serial]
    // fn test_make_cmd_clean_rename_error() {
    //     let filename = "test_clean_rename_error.txt";
    //     let contents = "task1\n-task2\ntask3\n-task4\n";
    //     create_test_file(&filename.to_string(), contents);

    //     let temp_filename = format!("{}_", filename);
    //     let _ = File::create(&temp_filename);

    //     // removed original file to simulate rename error
    //     fs::remove_file(&temp_filename).expect("Unable to remove temp file");

    //     let result = make_cmd_clean(&filename.to_string());
    //     assert!(result.is_err());

    //     delete_test_file(&filename.to_string());
    // }
}
