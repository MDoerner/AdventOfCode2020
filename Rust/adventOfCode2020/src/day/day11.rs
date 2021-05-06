
use std::convert::TryFrom;
use std::iter;

use crate::grid::{self, Grid};
use crate::space::{self, Point, Vector};
use crate::game_of_life;


#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub enum SeatState{
    Floor,
    Empty,
    Occupied,
}

impl Default for SeatState{
    fn default() -> Self {
        SeatState::Floor
    }
}

impl From<char> for SeatState{
    fn from(character: char) -> Self {
        match character{
            'L' => SeatState::Empty,
            '#' => SeatState::Occupied,
            _ => SeatState::default(),
        }
    }
}


pub struct Day11 {}

impl super::Day for Day11{
    type PuzzleInput = grid::OutsideDefaultGrid<i128, SeatState, 2>;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        if text.is_empty(){
            return default_grid();
        }

        let seat_rows: Vec<&str> = text.lines().collect();
        let maybe_row_count = i128::try_from(seat_rows.len());
        if maybe_row_count.is_err(){
            return default_grid();
        }
        let row_count = maybe_row_count.unwrap();

        let maybe_column_count = i128::try_from(seat_rows[0].len());
        if maybe_column_count.is_err(){
            return default_grid();
        }
        let column_count = maybe_column_count.unwrap();
        if column_count == 0{
            return default_grid();
        }

        let known_seats = seat_rows.iter()
            .enumerate()
            .map(|(row, line)| line.chars()
                .map(SeatState::from)
                .enumerate()
                .filter_map(move |(column, seat)| {
                    let maybe_column = i128::try_from(column);
                    let maybe_row = i128::try_from(row);
                    if maybe_column.is_err() || maybe_row.is_err(){
                        return None;
                    }
                    let coordinate_x = maybe_column.unwrap();
                    let coordinate_y = maybe_row.unwrap();
                    Some((Point::new([coordinate_x, coordinate_y]), seat))
                }))
            .flatten();
        let coordinate_ranges = [
            grid::CoordinateRange {lower_bound: 0, upper_bound: column_count},
            grid::CoordinateRange {lower_bound: 0, upper_bound: row_count}
        ];
        grid::OutsideDefaultGrid::new(coordinate_ranges, SeatState::default(), known_seats)
    }

    fn solve_part1(&self, seating_area: Self::PuzzleInput) -> std::string::String {
        let seating_bounds = seating_area.coordinate_ranges();
        let seats: Vec<Point<i128, 2>> = (seating_bounds[0].lower_bound..seating_bounds[0].upper_bound)
            .map(|x| (seating_bounds[1].lower_bound..seating_bounds[1].upper_bound)
                .map(move |y| Point::new([x,y])))
            .flatten()
            .filter(|point| match seating_area.at_point(point){
                    SeatState::Floor => false,
                    SeatState::Empty => true,
                    SeatState::Occupied => true,
                })
            .collect();
        let initially_occupied_seats: Vec<Point<i128, 2>> = seats.iter()
            .filter(|point| match seating_area.at_point(point){
                SeatState::Floor => false,
                SeatState::Empty => false,
                SeatState::Occupied => true,
            })
            .map(|point| point.to_owned())
            .collect();
        let game_of_life_rules = SimpleSeatingRules {seating_area};
        let game_of_life_runner = game_of_life::GameOfLife::new(game_of_life_rules);
        let stabelized_occupied_seats = game_of_life_runner.active_items_after_stabelizing(initially_occupied_seats.iter(), seats.iter());
        let number_of_occupied_seats = stabelized_occupied_seats.count();
        number_of_occupied_seats.to_string()
    }

    fn solve_part2(&self, seating_area: Self::PuzzleInput) -> std::string::String {
        let seating_bounds = seating_area.coordinate_ranges();
        let seats: Vec<Point<i128, 2>> = (seating_bounds[0].lower_bound..seating_bounds[0].upper_bound)
            .map(|x| (seating_bounds[1].lower_bound..seating_bounds[1].upper_bound)
                .map(move |y| Point::new([x,y])))
            .flatten()
            .filter(|point| match seating_area.at_point(point){
                    SeatState::Floor => false,
                    SeatState::Empty => true,
                    SeatState::Occupied => true,
                })
            .collect();
        let initially_occupied_seats: Vec<Point<i128, 2>> = seats.iter()
            .filter(|point| match seating_area.at_point(point){
                SeatState::Floor => false,
                SeatState::Empty => false,
                SeatState::Occupied => true,
            })
            .map(|point| point.to_owned())
            .collect();
        let game_of_life_rules = AdvancesSeatingRules {seating_area};
        let game_of_life_runner = game_of_life::GameOfLife::new(game_of_life_rules);
        let stabelized_occupied_seats = game_of_life_runner.active_items_after_stabelizing(initially_occupied_seats.iter(), seats.iter());
        let number_of_occupied_seats = stabelized_occupied_seats.count();
        number_of_occupied_seats.to_string()
    }
}

fn default_grid() -> grid::OutsideDefaultGrid<i128, SeatState, 2>{
    let coordinate_ranges: [grid::CoordinateRange<i128>; 2] = [
        grid::CoordinateRange::default(),
        grid::CoordinateRange::default()
    ];
    grid::OutsideDefaultGrid::<i128, SeatState, 2>::new(coordinate_ranges, SeatState::default(), iter::empty::<(Point<i128, 2>, SeatState)>())
}

struct SimpleSeatingRules<T>{
    seating_area: T,
}

impl<T: grid::MutGrid<SeatState,2, CoordinateType=i128>> game_of_life::GameOfLifeRules for SimpleSeatingRules<T>{
    type ItemType = Point<i128,2>;

    fn neighbours<'a>(&self, item: &'a Self::ItemType) -> Vec<Self::ItemType> where Self::ItemType: 'a {
        space::neighbour_offsets_full::<i128,2>().unwrap()
            .into_iter()
            .map(|offset| item.to_owned() + offset)
            .filter(|neighbour| self.seating_area.at_point(neighbour) != &SeatState::Floor)
            .collect()
    }

    fn flip_active(&self, active_neighbour_count: usize) -> bool {
        active_neighbour_count >= 4
    }

    fn flip_inactive(&self, active_neighbour_count: usize) -> bool {
        active_neighbour_count == 0
    }
}

struct AdvancesSeatingRules<T>{
    seating_area: T,
}

impl<T: grid::MutGrid<SeatState,2, CoordinateType=i128>> game_of_life::GameOfLifeRules for AdvancesSeatingRules<T>{
    type ItemType = Point<i128,2>;

    fn neighbours<'a>(&self, item: &'a Self::ItemType) -> Vec<Self::ItemType> where Self::ItemType: 'a {
        space::neighbour_offsets_full::<i128,2>().unwrap()
            .into_iter()
            .filter_map(|offset| neighbour_in_direction(item.to_owned(),  offset, &self.seating_area))
            .collect()
    }

    fn flip_active(&self, active_neighbour_count: usize) -> bool {
        active_neighbour_count >= 5
    }

    fn flip_inactive(&self, active_neighbour_count: usize) -> bool {
        active_neighbour_count == 0
    }
}

fn neighbour_in_direction(item: Point<i128,2>, direction: Vector<i128,2>, grid: & impl grid::Grid<SeatState, 2, CoordinateType=i128>) -> Option<Point<i128,2>>{
    if direction == Vector::new([0,0]){
        return None;
    }
    let mut potential_neighbour = item + direction;
    while grid.is_on_main_grid(&potential_neighbour){
        match grid.at_point(&potential_neighbour){
            SeatState::Occupied => return Some(potential_neighbour),
            SeatState::Empty => return Some(potential_neighbour),
            SeatState::Floor => (),
        }
        potential_neighbour = potential_neighbour + direction
    }
    None
}





#[cfg(test)]
mod day11_tests {
    use super::*;
    use crate::input;
    use crate::day;

    fn example_input() -> String{
        String::from(
"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL")
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day11{});
        let problem_input = example_input();
        let expected_result = 37.to_string();
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day11{});
        let problem_input = example_input();
        let expected_result = 26.to_string();
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day11{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 11, part: 1}).unwrap();
        let expected_result = String::from("2310");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day11{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 11, part: 2}).unwrap();
        let expected_result = String::from("2074");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}