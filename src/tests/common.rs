use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

pub fn get_test_file_path(filename: &str) -> String {
    let mut filepath = String::new();

    if let Ok(cur_dir) = env::current_dir() {
        let path = cur_dir.join(filename);
        filepath = path.to_str().unwrap_or("").to_string();
    }

    filepath
}

pub fn create_test_file(filename: &str, contents: &str) {
    let mut file = File::create(filename).expect("Unable to create test file");
    file.write_all(contents.as_bytes())
        .expect("Unable to write test file");
}

pub fn delete_test_file(filename: &str) {
    if Path::new(&filename).exists() {
        fs::remove_file(filename).expect("Unable to delete test file");
    }
}

pub fn read_file_to_string(filename: &str) -> String {
    let mut file = File::open(filename).expect("Unable to open test file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read test file");
    contents
}
