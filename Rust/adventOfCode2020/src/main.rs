mod day;

use std::env;
use std::fs;

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

    return Some(PuzzleConfiguration {day: day, part: part});
}

fn puzzleOutput(config: PuzzleConfiguration) -> String{
    return String::from("");
}