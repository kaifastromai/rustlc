///A simple application to count the lines of code in a Rust project.
/// The idea is to recursively walk the directory tree, and for every .rs file, count the lines of code,
/// accumulating the total in a `lines` variable.
/// We ignore the target folder.
/// Must be run from the root of the project.
///
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let mut line_count = 0usize;
    let mut char_count = 0usize;
    let proj_root = env::current_dir().unwrap();
    //print the project root
    println!("Counting lines of {}", proj_root.display());
    fn count_lines(path: &Path, line_count: &mut usize, char_count: &mut usize) {
        if path.is_dir() {
            for entry in path.read_dir().unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                count_lines(&path, line_count, char_count);
            }
        } else if path.is_file() {
            match path.extension() {
                Some(ext) => {
                    if ext == "rs" {
                        let mut file = File::open(path).unwrap();
                        let mut contents = String::new();
                        file.read_to_string(&mut contents).unwrap();
                        *line_count += contents.lines().count();
                        *char_count += contents.len();
                    }
                }
                None => {}
            }
        }
    }
    count_lines(&proj_root, &mut line_count, &mut char_count);
    println!("{} lines of code, {} characters", line_count, char_count);
}
