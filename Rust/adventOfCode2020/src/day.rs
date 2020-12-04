mod day1;

pub trait DaySolver{
    fn solve_part1(&self, input: String) -> String;
    fn solve_part2(&self, input: String) -> String;
}

pub trait Day<T>{
    fn parse_input(&self, input: String) -> T;
    fn solve_part1(&self, input: T) -> String;
    fn solve_part2(&self, input: T) -> String;
}

impl<U, T> DaySolver for U where U: Day<T> {
    fn solve_part1(&self, input: String) -> String {
        self.solve_part1(self.parse_input(input))
    }

    fn solve_part2(&self, input: String) -> String {
        self.solve_part2(self.parse_input(input))
    }
}

pub fn getDay(&day: &i32) -> Option<Box<dyn DaySolver>>{
    match day{
        1 => Some(Box::new(day1::Day1 {})),
        _ => None
    }
}