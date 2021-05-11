use std::convert::TryFrom;

use crate::game_of_life;
use crate::space::{self, Point, Vector};


pub struct Day17 {}

impl super::Day for Day17{
    type PuzzleInput = Vec<Point<i64,2>>;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        text.lines()
            .enumerate()
            .filter_map(|(y, line)| match i64::try_from(y) {
                Ok(y_value) => Some((y_value, line)),
                Err(_) => None,
            })
            .map(|(y, line)| line.chars()
                .enumerate()
                .filter_map(|(x, line)| match i64::try_from(x) {
                    Ok(x_value) => Some((x_value, line)),
                    Err(_) => None,
                })
                .filter_map(move |(x, c)| match c{
                    '#' => Some(Point::new([x,y])),
                    _ => None,
                }))
            .flatten()
            .collect()
    }

    fn solve_part1(&self, active_plane_points: Self::PuzzleInput) -> std::string::String {
        let initially_active_points: Vec<Point<i64,3>> = active_plane_points.into_iter()
            .map(|point| Point::new([point[0], point[1], 0]))
            .collect();
        let conway_cube = ConwayCube::<3> {};
        let game_of_life_runner = game_of_life::GameOfLife::new(conway_cube);
        let active_points = game_of_life_runner.active_items_after_playing(6, initially_active_points.iter());
        let result = active_points.count();
        result.to_string()
    }

    fn solve_part2(&self, active_plane_points: Self::PuzzleInput) -> std::string::String {
        let initially_active_points: Vec<Point<i64,4>> = active_plane_points.into_iter()
            .map(|point| Point::new([point[0], point[1], 0, 0]))
            .collect();
        let conway_cube = ConwayCube::<4> {};
        let game_of_life_runner = game_of_life::GameOfLife::new(conway_cube);
        let active_points = game_of_life_runner.active_items_after_playing(6, initially_active_points.iter());
        let result = active_points.count();
        result.to_string()
    }
}

struct ConwayCube<const N: usize> {}

impl<const N: usize> game_of_life::GameOfLifeRules for ConwayCube<N>{
    type ItemType = Point<i64, N>;

    fn neighbours<'a>(&self, item: &'a Self::ItemType) -> Vec<Self::ItemType> where Self::ItemType: 'a {
        let offsets = space::neighbour_offsets_full().unwrap();
        offsets.into_iter()
            .map(|offset| item.to_owned() + offset)
            .collect()
    }

    fn flip_active(&self, active_neighbour_count: usize) -> bool {
        active_neighbour_count != 2 && active_neighbour_count != 3
    }

    fn flip_inactive(&self, active_neighbour_count: usize) -> bool {
        active_neighbour_count == 3
    }
}




#[cfg(test)]
mod day16_tests {
    use super::*;
    use crate::input;
    use crate::day;

    fn example_input() -> String{
        String::from(
".#.
..#
###")
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day17{});
        let problem_input = example_input();
        let expected_result = 112.to_string();
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day17{});
        let problem_input = example_input();
        let expected_result = 848.to_string();
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day17{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 17, part: 1}).unwrap();
        let expected_result = String::from("232");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day17{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 17, part: 2}).unwrap();
        let expected_result = String::from("1620");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}