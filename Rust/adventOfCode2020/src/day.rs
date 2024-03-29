mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day18_v2;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;


pub trait DaySolver{
    fn solve_part1(&self, input: String) -> String;
    fn solve_part2(&self, input: String) -> String;
}

pub trait Day{
    type PuzzleInput;
    fn parse_input(&self, input: String) -> Self::PuzzleInput;
    fn solve_part1(&self, input: Self::PuzzleInput) -> String;
    fn solve_part2(&self, input: Self::PuzzleInput) -> String;
}

impl<T> DaySolver for T where T: Day {
    fn solve_part1(&self, input: String) -> String {
        self.solve_part1(self.parse_input(input))
    }

    fn solve_part2(&self, input: String) -> String {
        self.solve_part2(self.parse_input(input))
    }
}

pub fn get_day(&day: &i32) -> Option<Box<dyn DaySolver>>{
    match day{
        1 => Some(Box::new(day1::Day1 {})),
        2 => Some(Box::new(day2::Day2 {})),
        3 => Some(Box::new(day3::Day3 {})),
        4 => Some(Box::new(day4::Day4 {})),
        5 => Some(Box::new(day5::Day5 {})),
        6 => Some(Box::new(day6::Day6 {})),
        7 => Some(Box::new(day7::Day7 {})),
        8 => Some(Box::new(day8::Day8 {})),
        9 => Some(Box::new(day9::Day9 {})),
        10 => Some(Box::new(day10::Day10 {})),
        11 => Some(Box::new(day11::Day11 {})),
        12 => Some(Box::new(day12::Day12 {})),
        13 => Some(Box::new(day13::Day13 {})),
        14 => Some(Box::new(day14::Day14 {})),
        15 => Some(Box::new(day15::Day15 {})),
        16 => Some(Box::new(day16::Day16 {})),
        17 => Some(Box::new(day17::Day17 {})),
        18 => Some(Box::new(day18_v2::Day18 {})),
        19 => Some(Box::new(day19::Day19 {})),
        20 => Some(Box::new(day20::Day20 {})),
        21 => Some(Box::new(day21::Day21 {})),
        22 => Some(Box::new(day22::Day22 {})),
        23 => Some(Box::new(day23::Day23 {})),
        24 => Some(Box::new(day24::Day24 {})),
        25 => Some(Box::new(day25::Day25 {})),
        _ => None
    }
}