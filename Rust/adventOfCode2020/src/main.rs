mod day;

use std::{path::Path, path::PathBuf, env};
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let maybe_config: Option<PuzzleConfiguration> = puzzle_config(args);
    let output: String;
    match maybe_config{
        None => return,
        Some(config) => output = puzzle_output(config),
    };
    println!("{}", output);
}

struct PuzzleConfiguration {
    day: i32,
    part: i32
}

fn puzzle_config(args: Vec<String>) -> Option<PuzzleConfiguration>{
    if args.len() < 3 {
        return None;
    }

    let day: i32;
    match args[1].parse::<i32>(){
        Ok(x) => day = x,
        Err(_) => return None,
    }

    let part: i32;
    match args[2].parse::<i32>(){
        Ok(x) => part = x,
        Err(_) => return None,
    }

    if part != 1 && part != 2
        || day < 1
        || day > 25{
            return None;
    }

    return Some(PuzzleConfiguration {day: day, part: part});
}

fn puzzle_output(config: PuzzleConfiguration) -> String{
    let solver: Box<dyn day::DaySolver>;
    match day::get_day(&config.day){
        Some(day_solver) => solver = day_solver,
        None => return String::from("")
    };

    let input: String;
    match puzzle_input(&config){
        Some(text) => input = text,
        None => return String::from("")
    }

    match config.part{
        1 => return (*solver).solve_part1(input),
        2 => return (*solver).solve_part2(input),
        _ => return String::from("")
    }
}

fn puzzle_input(config: &PuzzleConfiguration) -> Option<String>{
    let path: PathBuf = puzzle_file_path(&config);
    match fs::read_to_string(path){
        Err(_) => return None,
        Ok(text) => return Some(text)
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
    return path;
}

fn puzzle_file_name(config: &PuzzleConfiguration) -> String{
    let day: &str = &config.day.to_string();
    return ["Day", day, ".txt"].join("");
}