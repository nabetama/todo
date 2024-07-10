use crate::utils::read_lines;

const DONE_MARK_1: &str = "☐";
const DONE_MARK_2: &str = "☑";

pub fn make_cmd_list(filename: &String) {
    let mut todo = Vec::new();
    let mut index = 1;

    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            todo.push((index, line));
            index += 1;
        }
    }

    if todo.is_empty() {
        println!("No tasks found.");
    } else {
        for (index, task) in todo {
            if let Some(task) = task.strip_prefix('-') {
                println!("{} {:03} {}", DONE_MARK_2, index, task);
            } else {
                println!("{} {:03} {}", DONE_MARK_1, index, task);
            }
        }
    }
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
    fn test_make_cmd_list_no_tasks() {
        let filename = get_test_file_path(&TEST_FILE.to_string());

        create_test_file(&filename, "");

        let output = std::panic::catch_unwind(|| {
            make_cmd_list(&filename);
        });

        assert!(output.is_ok());
        delete_test_file(&filename)
    }

    #[test]
    #[serial]
    fn test_make_cmd_list_with_tasks() {
        let filename = get_test_file_path(&TEST_FILE.to_string());
        const TASK_1: &str = "task 1";

        create_test_file(&filename, TASK_1);

        let output = std::panic::catch_unwind(|| {
            make_cmd_list(&filename);
        });

        assert!(output.is_ok());
        assert_eq!(read_file_to_string(&filename), TASK_1);

        delete_test_file(&filename);
    }

    #[test]
    #[serial]
    fn test_make_cmd_list_with_various_tasks() {
        let filename = get_test_file_path(&TEST_FILE.to_string());

        const TASK_1: &str = "task 1";
        const TASK_2: &str = "task 2";
        const TASK_3: &str = "task 3";

        create_test_file(
            &filename,
            format!("{}\n{}\n{}", TASK_1, TASK_2, TASK_3).as_str(),
        );

        let output = std::panic::catch_unwind(|| {
            make_cmd_list(&filename);
        });

        assert!(output.is_ok());
        assert_eq!(
            read_file_to_string(&filename),
            format!("{}\n{}\n{}", TASK_1, TASK_2, TASK_3)
        );

        delete_test_file(&filename);
    }
}
