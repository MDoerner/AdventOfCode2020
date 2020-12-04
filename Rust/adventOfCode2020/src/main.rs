mod day;

use std::{path::Path, path::PathBuf, env};
use std::fs;

use day::DaySolver;

fn main() {
    let args: Vec<String> = env::args().collect();
    let maybeConfig = puzzleConfig(args);
    let output: String;
    match maybeConfig{
        None => return,
        Some(config) => output = puzzleOutput(config),
    };
    println!("{}", output);
}

struct PuzzleConfiguration {
    day: i32,
    part: i32
}

fn puzzleConfig(args: Vec<String>) -> Option<PuzzleConfiguration>{
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

fn puzzleOutput(config: PuzzleConfiguration) -> String{
    let solver: Box<dyn day::DaySolver>;
    match day::getDay(&config.day){
        Some(daySolver) => solver = daySolver,
        None => return String::from("")
    };

    let input: String;
    match puzzleInput(&config){
        Some(text) => input = text,
        None => return String::from("")
    }

    match config.part{
        1 => return (*solver).solve_part1(input),
        2 => return (*solver).solve_part2(input),
        _ => return String::from("")
    }
}

fn puzzleInput(&config: &PuzzleConfiguration) -> Option<String>{
    let path: PathBuf = puzzleFilePath(&config);
    match fs::read_to_string(path){
        Err(_) => return None,
        Ok(text) => return Some(text)
    }
}

fn puzzleFilePath(&config: &PuzzleConfiguration) -> PathBuf{
    let filename = puzzleFileName(&config);
    let path: PathBuf = Path::new(file!())
                            .parent().unwrap()
                            .parent().unwrap()
                            .parent().unwrap()
                            .parent().unwrap()
                            .to_path_buf();
    path.push("Input");
    path.push(filename);
    return path;
}

fn puzzleFileName(&config: &PuzzleConfiguration) -> String{
    let day: &str = &config.day.to_string();
    return ["Day", day, ".txt"].join("");
}