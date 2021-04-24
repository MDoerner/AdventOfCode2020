use crate::grid;
use crate::space;


pub struct Day3 {}

impl super::Day for Day3{
    type PuzzleInput = Option<grid::LoopingGrid<usize,bool,2>>;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        let lines = text.lines().collect::<Vec<&str>>();
        let height = lines.len();
        if height == 0{
            return None;
        }
        let width = lines[0].len();
        let points_with_trees = lines.iter()
            .enumerate()
            .map(|(y, line)| (y, tree_indices(&line)))
            .map(|(y, xs)| xs.into_iter().map(move |x| space::Point::new([x,y])))
            .flatten();
        let grid = grid::LoopingGrid::new([width, height], false, points_with_trees.map(|point| (point, true)));
        Some(grid)
    }

    fn solve_part1(&self, maybe_grid: Self::PuzzleInput) -> std::string::String {
        if maybe_grid.is_none() {
            return String::from("Invalid input!");
        }
        let slope = maybe_grid.unwrap();
        let start_point = space::Point::new([0usize,0usize]);
        let direction = space::Vector::new([3usize,1usize]);
        let tree_count = trees_in_direction(start_point, &direction, &slope);
        tree_count.to_string()
    }

    fn solve_part2(&self, maybe_grid: Self::PuzzleInput) -> std::string::String {
        if maybe_grid.is_none() {
            return String::from("Invalid input!");
        }
        let slope = maybe_grid.unwrap();
        let start_point = space::Point::new([0usize, 0usize]);
        let directions = [space::Vector::new([1usize, 1usize]), space::Vector::new([3usize, 1usize]), space::Vector::new([5usize, 1usize]), space::Vector::new([7usize, 1usize]), space::Vector::new([1usize, 2usize])];
        let tree_counts = directions.iter().map(|direction| trees_in_direction(start_point, direction, &slope));
        let result: usize = tree_counts.product();
        result.to_string()
    }
}

fn tree_indices(line: &str) -> Vec<usize>{
    line.chars()
        .enumerate()
        .filter(|(_, value)| is_tree(value))
        .map(|(index, _)| index)
        .collect()
}

fn is_tree(c: &char) -> bool{
    c == &'#'
}

fn trees_in_direction(start_point: space::Point<usize, 2>, direction: &space::Vector<usize, 2>, slope: &impl grid::Grid<bool, 2, CoordinateType=usize>) -> usize{
    let normalized_direction = direction.to_direction();
    trees_with_step(start_point, normalized_direction, slope)
}

fn trees_with_step(start_point: space::Point<usize, 2>, step: space::Vector<usize, 2>, slope: &impl grid::Grid<bool, 2, CoordinateType=usize>) -> usize{
    if step[1] == 0{
        return 0;
    }
    let mut current_point = start_point;
    let mut trees_encountered = 0;
    while current_point[1] < slope.width()[1]{
        if *slope.at_point(&current_point){
            trees_encountered += 1;
        }
        current_point = current_point + step;
    }
    trees_encountered
}

#[cfg(test)]
mod day3_tests {
    use super::*;
    use crate::input;
    use crate::day;

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day3{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 3, part: 1}).unwrap();
        let expected_result = String::from("209");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day3{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 3, part: 2}).unwrap();
        let expected_result = String::from("1574890240");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}