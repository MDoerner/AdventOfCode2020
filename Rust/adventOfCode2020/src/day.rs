mod day1;
mod day2;

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
        _ => None
    }
}