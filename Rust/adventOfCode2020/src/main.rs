mod day;
mod input;
mod util;
mod algebra;
mod space;
mod grid;
mod game_of_life;

#[macro_use] extern crate lazy_static;
extern crate regex;

#[cfg(test)] extern crate rstest;


use std::env;
use std::time;
use input::PuzzleConfiguration;

fn main() {
    let args: Vec<String> = env::args().collect();
    let maybe_config: Option<PuzzleConfiguration> = puzzle_config(args);
    let start_time = time::Instant::now();
    let output: String;
    match maybe_config{
        None => return,
        Some(config) => output = puzzle_output(config),
    };
    println!("{}", output);
    let runtime = start_time.elapsed();
    println!("{:?}", runtime);
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

    Some(PuzzleConfiguration {day, part})
}

fn puzzle_output(config: PuzzleConfiguration) -> String{
    let solver: Box<dyn day::DaySolver>;
    match day::get_day(&config.day){
        Some(day_solver) => solver = day_solver,
        None => return String::from("")
    };

    let input: String;
    match input::puzzle_input(&config){
        Some(text) => input = text,
        None => return String::from("")
    }

    match config.part{
        1 => (*solver).solve_part1(input),
        2 => (*solver).solve_part2(input),
        _ => String::from("")
    }
}