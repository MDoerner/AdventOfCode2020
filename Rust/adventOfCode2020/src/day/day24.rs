use std::{collections::HashSet, convert::TryFrom, error::Error, fmt::Display};

use crate::space::{Point, Vector};
use crate::game_of_life::{self, GameOfLifeRules};



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HexagonalDirection{
    West,
    NorthWest,
    NorthEast,
    East,
    SouthEast,
    SouthWest,
}

impl HexagonalDirection {
    fn all() -> Vec<HexagonalDirection>{
        vec![
            HexagonalDirection::West,
            HexagonalDirection::NorthWest,
            HexagonalDirection::NorthEast,
            HexagonalDirection::East,
            HexagonalDirection::SouthEast,
            HexagonalDirection::SouthWest,
        ]
    }
}

impl From<&HexagonalDirection> for String{
    fn from(direction: &HexagonalDirection) -> Self {
        match direction {
            HexagonalDirection::West => String::from("w"),
            HexagonalDirection::NorthWest => String::from("nw"),
            HexagonalDirection::NorthEast => String::from("ne"),
            HexagonalDirection::East => String::from("e"),
            HexagonalDirection::SouthEast => String::from("se"),
            HexagonalDirection::SouthWest => String::from("sw"),
        }
    }
}

impl TryFrom<&str> for HexagonalDirection{
    type Error = ParseHexagonalDirectionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "w" => Ok(HexagonalDirection::West),
            "nw" => Ok(HexagonalDirection::NorthWest),
            "ne" => Ok(HexagonalDirection::NorthEast),
            "e" => Ok(HexagonalDirection::East),
            "se" => Ok(HexagonalDirection::SouthEast),
            "sw" => Ok(HexagonalDirection::SouthWest),
            _ => {
                let error = ParseHexagonalDirectionError { invalid_input: String::from(value) };
                Err(error)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseHexagonalDirectionError{
    pub invalid_input: String,
}

impl Display for ParseHexagonalDirectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let valid_values: Vec<_> = HexagonalDirection::all().iter()
            .map(String::from)
            .collect();
        let valid_values_string = valid_values.join(", ");
        write!(f, "Invalid input {} to parse as a hexagonal direction. The valid values are {}.", self.invalid_input, valid_values_string)
    }
}

impl Error for ParseHexagonalDirectionError {}

impl From<&HexagonalDirection> for Vector<i64,2>{
    fn from(direction: &HexagonalDirection) -> Self {
        match direction {
            HexagonalDirection::West => Vector::new([-1, 0]),
            HexagonalDirection::NorthWest => Vector::new([0, 1]),
            HexagonalDirection::NorthEast => Vector::new([1, 1]),
            HexagonalDirection::East => Vector::new([1, 0]),
            HexagonalDirection::SouthEast => Vector::new([0, -1]),
            HexagonalDirection::SouthWest => Vector::new([-1, -1]),
        }
    }
}

pub struct HexPath(Vec<HexagonalDirection>);

pub struct Day24 {}

impl super::Day for Day24{
    type PuzzleInput = Vec<HexPath>;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        text.lines()
            .map(parse_hex_path)
            .collect()
    }

    fn solve_part1(&self, paths_to_tiles_to_flip: Self::PuzzleInput) -> std::string::String {
        let black_tiles = flipped_tiles(&paths_to_tiles_to_flip);
        let result = black_tiles.len();
        result.to_string()
    }

    fn solve_part2(&self, paths_to_tiles_to_flip: Self::PuzzleInput) -> std::string::String {
        let days_passed = 100;
        let result = solve_part2_impl(&paths_to_tiles_to_flip, days_passed);
        result.to_string()
    }
}

fn parse_hex_path(text: &str) -> HexPath{
    let mut path = vec![];
    let mut path_characters = text.chars();
    while let Some(c) = path_characters.next(){
        match c {
            'w' => path.push(HexagonalDirection::West),
            'e' => path.push(HexagonalDirection::East),
            'n' => {
                if let Some(next_c) = path_characters.next() {
                    match next_c{
                        'w' => path.push(HexagonalDirection::NorthWest),
                        'e' => path.push(HexagonalDirection::NorthEast),
                        _ => {},
                    }
                }
            },
            's' => {
                if let Some(next_c) = path_characters.next() {
                    match next_c{
                        'w' => path.push(HexagonalDirection::SouthWest),
                        'e' => path.push(HexagonalDirection::SouthEast),
                        _ => {},
                    }
                }
            },
            _ => {},
        }
    }
    HexPath { 0: path }
}

fn flipped_tiles(paths_to_tiles_to_flip: &[HexPath]) -> HashSet<Point<i64, 2>>{
    let mut flipped_points = HashSet::new();
    let start_point = Point::new([0,0]);
    for path in paths_to_tiles_to_flip.iter(){
        let end_point = end_point(path, start_point);
        if flipped_points.contains(&end_point){
            flipped_points.remove(&end_point);
        } else {
            flipped_points.insert(end_point);
        }
    }
    flipped_points
}

fn end_point(path: &HexPath, start_point: Point<i64, 2>) -> Point<i64, 2>{
    path.0.iter()
        .fold(start_point, |current_point, direction| current_point + Vector::from(direction))
}

fn solve_part2_impl(paths_to_tiles_to_flip: &[HexPath], days_passed: usize) -> usize{
    let initially_black_tiles = flipped_tiles(paths_to_tiles_to_flip);
    let game_of_life_rules = HexFloorTileGameofLifeRules {};
    let game_of_life_runner = game_of_life::GameOfLife::new(game_of_life_rules);
    let black_tiles = game_of_life_runner.active_items_after_playing(days_passed, initially_black_tiles.iter());
    black_tiles.count()
}


struct HexFloorTileGameofLifeRules {}

impl GameOfLifeRules for HexFloorTileGameofLifeRules{
    type ItemType = Point<i64, 2>;

    fn neighbours<'a>(&self, item: &'a Self::ItemType) -> Vec<Self::ItemType> where Self::ItemType: 'a {
        HexagonalDirection::all().iter()
            .map(|direction| item.to_owned() + Vector::from(direction))
            .collect()
    }

    fn flip_active(&self, active_neighbour_count: usize) -> bool {
        active_neighbour_count == 0 || active_neighbour_count > 2
    }

    fn flip_inactive(&self, active_neighbour_count: usize) -> bool {
        active_neighbour_count == 2
    }
}




#[cfg(test)]
mod day24_tests {
    use super::*;
    use crate::input;
    use crate::day;
    use rstest::rstest;

    fn example_input() -> String{
        String::from(
r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#)
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day24{});
        let problem_input = example_input();
        let expected_result = 10.to_string();
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[rstest]
    #[case(0, 10)]
    #[case(1, 15)]
    #[case(2, 12)]
    #[case(3, 25)]
    #[case(4, 14)]
    #[case(5, 23)]
    #[case(6, 28)]
    #[case(7, 41)]
    #[case(8, 37)]
    #[case(9, 49)]
    #[case(10, 37)]
    #[case(20, 132)]
    #[case(30, 259)]
    #[case(40, 406)]
    #[case(50, 566)]
    #[case(60, 788)]
    #[case(70, 1106)]
    #[case(80, 1373)]
    #[case(90, 1844)]
    fn basic_examples_part2(#[case] days_passed: usize, #[case] expected_result: usize) {
        let day: Box<dyn day::Day<PuzzleInput = Vec<HexPath>>> = Box::new(Day24{});
        let problem_input = example_input();
        let paths_to_tiles_to_flip = day.parse_input(problem_input);
        let actual_result = solve_part2_impl(&paths_to_tiles_to_flip, days_passed);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day24{});
        let problem_input = example_input();
        let expected_result = 2208.to_string();
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day24{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 24, part: 1}).unwrap();
        let expected_result = String::from("427");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day24{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 24, part: 2}).unwrap();
        let expected_result = String::from("3837");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}