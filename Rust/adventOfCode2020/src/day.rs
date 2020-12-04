mod day1;

pub trait DaySolver{
    fn solvePart1($self, input: String) -> String;
    fn solvePart2($self, input: String) -> String;
}

pub trait Day<T>{
    fn parseInput($self, input: String) -> T;
    fn solvePart1($self, input: T) -> String;
    fn solvePart2($self, input: T) -> String;
}

impl DaySolver for Day<T> {
    fn solvePart1($self, input: String) -> String {
        Day<T>::solvePart1(parseInput(input));
    }

    fn solvePart2($self, input: String) -> String {
        Day<T>::solvePart2(parseInput(input));
    }
}

pub fn getDay(day: i32) -> Option<impl DaySolver>{
    match day{
        1 => Some(Day1 {});
        _ => None;
    }
}