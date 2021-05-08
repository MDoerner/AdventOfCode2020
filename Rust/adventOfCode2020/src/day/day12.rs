

use crate::space::{self, Point, Vector};
use crate::util;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub enum Instruction{
    East(i64),
    North(i64),
    West(i64),
    South(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}


pub struct Day12 {}

impl super::Day for Day12{
    type PuzzleInput = Vec<Instruction>;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        text.lines()
            .filter_map(parsed_instruction)
            .collect()
    }

    fn solve_part1(&self, instructions: Self::PuzzleInput) -> std::string::String {
        let initial_position = Point::new([0, 0]);
        let initial_direction = Direction::East;
        let mut ship_state = DirectionalShipState {position: initial_position, direction: initial_direction};
        for instruction in instructions.into_iter(){
            ship_state.move_ship(instruction);
        }
        let distance = space::manhattan_metric(initial_position, ship_state.position);
        distance.to_string()
    }

    fn solve_part2(&self, instructions: Self::PuzzleInput) -> std::string::String {
        let initial_position = Point::new([0, 0]);
        let initial_waypoint = Vector::new([10,1]);
        let mut ship_state = WaypointShipState {position: initial_position, waypoint: initial_waypoint};
        for instruction in instructions.into_iter(){
            ship_state.move_ship(instruction);
        }
        let distance = space::manhattan_metric(initial_position, ship_state.position);
        distance.to_string()
    }
}

fn parsed_instruction(instruction_text: &str) -> Option<Instruction>{
    if instruction_text.len() < 2 || !instruction_text.is_char_boundary(1){
        return None;
    }
    let (instruction_type_text, amplitude_text) = instruction_text.split_at(1);
    let amplitude = amplitude_text.parse::<i64>().ok()?;
    match instruction_type_text{
        "E" => Some(Instruction::East(amplitude)),
        "N" => Some(Instruction::North(amplitude)),
        "W" => Some(Instruction::West(amplitude)),
        "S" => Some(Instruction::South(amplitude)),
        "L" => Some(Instruction::Left(amplitude)),
        "R" => Some(Instruction::Right(amplitude)),
        "F" => Some(Instruction::Forward(amplitude)),
        _ => None,
    }
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
enum Direction{
    North,
    West,
    South,
    East,
}

impl Direction {
    fn rotate(self, degrees: i64) -> Direction{
        let degrees_from_north = i64::from(self) + degrees;
        Direction::from(degrees_from_north)
    }
}

impl From<Direction> for i64{
    fn from(direction: Direction) -> Self {
        match direction{
            Direction::North => 0,
            Direction::West => 90,
            Direction::South => 180,
            Direction::East => 270,
        }
    }
}

impl From<i64> for Direction{
    fn from(degrees: i64) -> Self {
        let effective_degrees = util::modulo(degrees, 360);
        let rotation_steps = (effective_degrees + 45) / 90;
        match rotation_steps{
            0 => Direction::North,
            1 => Direction::West,
            2 => Direction::South,
            3 => Direction::East,
            _ => unreachable!("This cannot happen because of how modulo works."),
        }
    }
}

impl From<Direction> for Vector<i64, 2>{
    fn from(direction: Direction) -> Self {
        match direction{
            Direction::North => Vector::new([0, 1]),
            Direction::West => Vector::new([-1, 0]),
            Direction::South => Vector::new([0, -1]),
            Direction::East => Vector::new([1, 0]),
        }
    }
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct DirectionalShipState{
    position: Point<i64, 2>,
    direction: Direction,
}

impl DirectionalShipState{
    fn move_ship(&mut self, instruction: Instruction){
        match instruction{
            Instruction::East(amp) => self.position = self.position + Vector::from(Direction::East) * amp,
            Instruction::North(amp) => self.position = self.position + Vector::from(Direction::North) * amp,
            Instruction::West(amp) => self.position = self.position + Vector::from(Direction::West) * amp,
            Instruction::South(amp) => self.position = self.position + Vector::from(Direction::South) * amp,
            Instruction::Left(amp) => self.direction = self.direction.rotate(amp),
            Instruction::Right(amp) => self.direction = self.direction.rotate(-amp),
            Instruction::Forward(amp) => self.position = self.position + Vector::from(self.direction) * amp,
        }
    }
}



#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct WaypointShipState{
    position: Point<i64, 2>,
    waypoint: Vector<i64, 2>,
}

impl WaypointShipState{
    fn move_ship(&mut self, instruction: Instruction){
        match instruction{
            Instruction::East(amp) => self.waypoint = self.waypoint + Vector::from(Direction::East) * amp,
            Instruction::North(amp) => self.waypoint = self.waypoint + Vector::from(Direction::North) * amp,
            Instruction::West(amp) => self.waypoint = self.waypoint + Vector::from(Direction::West) * amp,
            Instruction::South(amp) => self.waypoint = self.waypoint + Vector::from(Direction::South) * amp,
            Instruction::Left(amp) => self.waypoint = self.waypoint.rotate(amp),
            Instruction::Right(amp) => self.waypoint = self.waypoint.rotate(-amp),
            Instruction::Forward(amp) => self.position = self.position + self.waypoint * amp,
        }
    }
}


#[cfg(test)]
mod day12_tests {
    use super::*;
    use crate::input;
    use crate::day;

    fn example_input() -> String{
        String::from(
"F10
N3
F7
R90
F11")
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day12{});
        let problem_input = example_input();
        let expected_result = 25.to_string();
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day12{});
        let problem_input = example_input();
        let expected_result = 286.to_string();
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day12{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 12, part: 1}).unwrap();
        let expected_result = String::from("1441");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day12{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 12, part: 2}).unwrap();
        let expected_result = String::from("61616");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}