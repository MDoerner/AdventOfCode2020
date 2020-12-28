use crate::grid;
use crate::plane;


pub struct Day3 {}

impl super::Day for Day3{
    type PuzzleInput = Option<grid::LoopingGrid<usize,bool>>;

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
            .map(|(y, xs)| xs.into_iter().map(move |x| plane::Point2d{x: x, y: y}))
            .flatten()
            .collect::<Vec<plane::Point2d<usize>>>();
        let grid = grid::LoopingGrid::new(height, width, false, points_with_trees.into_iter().map(|point| (point, true)));
        Some(grid)
    }

    fn solve_part1(&self, maybe_grid: Self::PuzzleInput) -> std::string::String {
        if maybe_grid.is_none() {
            return String::from("Invalid input!");
        }
        let slope = maybe_grid.unwrap();
        let start_point = plane::Point2d {x: 0, y: 0};
        let direction = plane::Vector2d {x: 3, y: 1};
        let tree_count = trees_in_direction(start_point, direction, &slope);
        tree_count.to_string()
    }

    fn solve_part2(&self, maybe_grid: Self::PuzzleInput) -> std::string::String {
        if maybe_grid.is_none() {
            return String::from("Invalid input!");
        }
        let slope = maybe_grid.unwrap();
        let start_point = plane::Point2d {x: 0, y: 0};
        let directions: Vec<plane::Vector2d<usize>>= vec![plane::Vector2d {x: 1, y: 1}, plane::Vector2d {x: 3, y: 1}, plane::Vector2d {x: 5, y: 1}, plane::Vector2d {x: 7, y: 1}, plane::Vector2d {x: 1, y: 2}];
        let tree_counts = directions.into_iter().map(|direction| trees_in_direction(start_point, direction, &slope));
        let result = tree_counts.fold(1, |product, next_count| product * next_count);
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

fn trees_in_direction(start_point: plane::Point2d<usize>, direction: plane::Vector2d<usize>, slope: &impl grid::Grid2d<bool, CoordinateType=usize>) -> usize{
    let normalized_direction = direction.to_direction();
    trees_with_step(start_point, normalized_direction, slope)
}

fn trees_with_step(start_point: plane::Point2d<usize>, step: plane::Vector2d<usize>, slope: &impl grid::Grid2d<bool, CoordinateType=usize>) -> usize{
    if step.y <= 0{
        return 0;
    }
    let mut current_point = start_point;
    let mut trees_encountered = 0;
    while current_point.y < slope.height(){
        if *slope.at_point(&current_point){
            trees_encountered += 1;
        }
        current_point = current_point + step;
    }
    trees_encountered
}