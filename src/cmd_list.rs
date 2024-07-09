use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

const DONE_MARK_1: &str = "☐";
const DONE_MARK_2: &str = "☑";

pub fn make_cmd_list(filename: String) {
    let mut todo = Vec::new();
    let mut index = 1;

    if let Ok(lines) = read_lines(&*filename) {
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

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
