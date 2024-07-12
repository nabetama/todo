use std::fs::{remove_file, rename, OpenOptions};
use std::io::Write;

use crate::utils::read_lines;

pub fn make_cmd_sort(filename: &str) -> Result<(), std::io::Error> {
    let mut bottom = String::new();
    let tmp_file = format!("{}_", filename);

    let mut w = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&tmp_file)?;

    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            if line.starts_with('-') {
                writeln!(w, "{}", line)?;
            } else {
                bottom.push_str(format!("{}\n", line).as_str());
            }
        }
    }

    write!(w, "{}", bottom)?;

    remove_file(filename)?;
    rename(tmp_file, filename)
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
        create_test_file(&filename, "task 1\ntask 2\n-task 3\ntask 4\ntask 5\n");
        filename
    }

    fn teardown(filename: &str) {
        delete_test_file(filename);
    }

    #[test]
    #[serial]
    fn test_make_cmd_sort_success() {
        let filename = setup();

        make_cmd_sort(&filename).unwrap();

        let contents = read_file_to_string(&filename);
        assert_eq!(contents, "-task 3\ntask 1\ntask 2\ntask 4\ntask 5\n");

        teardown(&filename);
    }
}
