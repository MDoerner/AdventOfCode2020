use std::{path::Path, path::PathBuf};
use std::fs;
pub struct PuzzleConfiguration {
    pub day: i32,
    pub part: i32
}

pub fn puzzle_input(config: &PuzzleConfiguration) -> Option<String>{
    let path: PathBuf = puzzle_file_path(&config);
    match fs::read_to_string(path){
        Err(_) => None,
        Ok(text) => Some(text)
    }
}

fn puzzle_file_path(config: &PuzzleConfiguration) -> PathBuf{
    let filename = puzzle_file_name(&config);
    let mut path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
                            .parent().unwrap()
                            .parent().unwrap()
                            .to_path_buf();
    path.push("Input");
    path.push(filename);
    path
}

fn puzzle_file_name(config: &PuzzleConfiguration) -> String{
    let day: &str = &config.day.to_string();
    ["Day", day, ".txt"].join("")
}