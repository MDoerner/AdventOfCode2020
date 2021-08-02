use std::{collections::{HashMap, HashSet}, iter::repeat};

use crate::space::{Point, Vector};


const MONSTER_CHARACTER: char = '#';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Border{
    Upper = 0,
    Right = 1,
    Lower = 2,
    Left = 3,
}

impl std::convert::From<u8> for Border{
    fn from(value: u8) -> Self {
        match value % 4{
            0 => Border::Upper,
            1 => Border::Right,
            2 => Border::Lower,
            3 => Border::Left,
            _ => unreachable!(),
        }
    }
}

impl std::convert::From<Border> for u8{
    fn from(border: Border) -> Self {
        border.value()
    }
}

impl Border{
    fn value(self) -> u8{
        self as u8
    }

    fn opposite(self) -> Border{
        (self.value() + 2).into()
    }

    fn rotate(self, steps: u8) -> Border{
        (self.value() + steps).into()
    }

    fn flip_on_y_axis(self) -> Border{
        let value = self.value();
        if value % 2 == 0{
            self
        } else {
            (value + 2).into()
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Tile{
    id: u64,
    image: [[bool;10];10],
    border_codes: [u16;4],
    flip_border_codes: [u16;4],
}

impl Tile{
    pub fn new(id: u64, image: [[bool;10];10]) -> Tile{
        let mut border_codes = [0;4];
        let mut flip_border_codes = [0;4];

        border_codes[Border::Upper as usize] = to_bismask(image[0].iter().copied());
        flip_border_codes[Border::Upper as usize] = to_bismask(image[0].iter().copied().rev());

        border_codes[Border::Right as usize] = to_bismask(image.iter().map(|row| row[9]));
        flip_border_codes[Border::Right as usize] = to_bismask(image.iter().map(|row| row[9]).rev());

        border_codes[Border::Lower as usize] = to_bismask(image[9].iter().copied().rev());
        flip_border_codes[Border::Lower as usize] = to_bismask(image[9].iter().copied());

        border_codes[Border::Left as usize] = to_bismask(image.iter().map(|row| row[0]).rev());
        flip_border_codes[Border::Left as usize] = to_bismask(image.iter().map(|row| row[0]));

        Tile {id, image, border_codes, flip_border_codes}
    }

    pub fn is_monster(&self, point: Point<usize, 2>) -> bool{
        let x = point[0];
        let y = point[1];
        if x >= 10 || y >= 10 {
            false
        } else {
            self.image[y][x]
        }
    }
}

fn to_bismask(seq: impl Iterator<Item = bool>) -> u16{
    let mut result = 0;
    for b in seq{
        result <<= 1;
        if b{
            result += 1;
        }
    }
    result
}


#[derive(Debug)]
pub struct SeaMonsterMask{
    width: usize,
    height: usize,
    monster_points: HashSet<Vector<usize, 2>>,
}


pub struct Day20 {}

impl super::Day for Day20{
    type PuzzleInput = (HashMap<u64, Tile>, SeaMonsterMask);

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        let parts: Vec<&str> = text.split("\n\n").collect();
        let sea_monster = parse_sea_monster_mask(parts[0]);
        let tiles: HashMap<u64, Tile> = parts[1..].iter()
            .map(|text| parse_tile(text))
            .map(|tile| (tile.id, tile))
            .collect();
        (tiles, sea_monster)
    }

    fn solve_part1(&self, data: Self::PuzzleInput) -> std::string::String {
        let (tiles, _) = data;
        let ids_by_border_code = tiles_by_border_code(&tiles);
        let border_ids_with_border_codes = border_tiles_with_border_codes(&ids_by_border_code);
        let corner_ids = corner_ids(&border_ids_with_border_codes);
        let result: u64 = corner_ids.iter().product();
        result.to_string()
    }

    fn solve_part2(&self, data: Self::PuzzleInput) -> std::string::String {
        let (tiles, mask) = data;
        let image = assembled_image(&tiles);
        let monster_pixel_count = image.iter()
            .flat_map(|row| row.iter())
            .filter(|pixel| **pixel)
            .count();
        let pixels_in_monsters = pixels_with_monster(&image, &mask);
        let pixel_with_monster_count = pixels_in_monsters.len();
        let result = monster_pixel_count - pixel_with_monster_count;
        result.to_string()
    }
}

fn parse_sea_monster_mask(text: &str) -> SeaMonsterMask{
    let height = text.lines().count();
    let width = text.lines().next().unwrap().len();
    let monster_points = text.lines().enumerate()
        .flat_map(|(row, line )| line.chars()
            .enumerate()
            .filter_map(move |(column, c)| if c == MONSTER_CHARACTER { Some(Vector::new([column, row])) } else  { None } ))
        .collect();
    SeaMonsterMask {width, height, monster_points}
}

fn parse_tile(text: &str) -> Tile{
    let lines: Vec<&str> = text.lines().collect();
    let id = parse_tile_id(lines[0]).unwrap();
    if lines.len() != 10 + 1{
        panic!("Each tile must be 10x10!");
    }
    let mut image = [[false;10];10];

    for (row, line) in lines[1..].iter().enumerate(){
        if line.len() != 10{
            panic!("Each tile must be 10x10!");
        }
        for (column, c) in line.chars().enumerate(){
            if c == MONSTER_CHARACTER{
                image[row][column] = true;
            }
        }
    }

    Tile::new(id, image)
}

fn parse_tile_id(text: &str) -> Option<u64>{
    lazy_static! {
        static ref TILE_ID_RE: regex::Regex = regex::Regex::new(r#"^Tile (\d+):$"#).unwrap();
    }
    let captures: regex::Captures = TILE_ID_RE.captures(text)?;
    let id = captures[1].parse::<u64>().ok()?;
    Some(id)
}

///
/// Orientation of a tile relative to its recorded form.
///
/// * `rotation` - Number of 90 degree rotations in counter-clockwise direction.
/// * `flip` - Indicatore whether a the tile has been mirrored along the y-axis before rotation.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Orientation{
    rotation: u8,
    flip: bool,
}

impl Orientation{
    ///
    /// Returns an orientation such that the operations revert those corresponding to self.
    ///
    #[allow(dead_code)]
    fn reverse(&self) -> Orientation{
        if self.flip {
            Orientation { rotation: self.rotation(), flip: true}
        } else {
            let rotation = (4 - self.rotation()) % 4;
            Orientation { rotation, flip: false}
        }
    }

    fn possible_values() -> Vec<Orientation>{
        let mut possibilities = vec![];
        for rotation in 0..4{
            possibilities.push(Orientation {rotation, flip:true});
            possibilities.push(Orientation {rotation, flip:false});
        }
        possibilities
    }

    pub fn rotation(&self) -> u8{
        self.rotation % 4
    }

    pub fn flip(&self) -> bool{
        self.flip
    }
}

fn reoriented_coordinates(point: Point<usize, 2>, orientation: &Orientation, width: usize, height: usize) -> Point<usize, 2>{
    let rotation_steps = orientation.rotation();
    let x_after_flip = if orientation.flip { width - point[0] - 1 } else { point[0] };
    let y_after_flip = point[1];
    match rotation_steps{
        0 => Point::new([x_after_flip, y_after_flip]),
        1 => Point::new([height - y_after_flip - 1, x_after_flip]),
        2 => Point::new([width - x_after_flip - 1, height - y_after_flip - 1]),
        3 => Point::new([y_after_flip, width - x_after_flip - 1]),
        _ => unreachable!("Modulo 4 cannot yield this value."),
    }
}


fn tiles_by_border_code(tiles_by_id: &HashMap<u64, Tile>) -> HashMap<u16, Vec<(u64, Orientation)>>{
    let mut ids_by_border_code = HashMap::<u16, Vec<(u64, Orientation)>>::new();
    for (code, id) in tiles_by_id.values()
        .flat_map(|tile| {
            let tile_id = tile.id;
            tile.border_codes.iter()
                .enumerate()
                .map(|(direction, code)| (code, Orientation {rotation: (direction % 4) as u8, flip:false}))
            .chain(tile.flip_border_codes.iter()
                .enumerate()
                .map(|(direction, code)| (code, Orientation {rotation: (direction % 4) as u8, flip:true})))
                    .map(move |(code, or)| (*code, (tile_id, or)))
        }){
        match ids_by_border_code.get_mut(&code){
            Some(ids) => ids.push(id),
            None => {
                let ids = vec![id];
                ids_by_border_code.insert(code, ids);
            }
        };
    }
    ids_by_border_code
}

fn corner_ids(border_ids_with_border_codes: &HashMap<u64, Vec<(u16, Orientation)>>) -> Vec<u64>{
    border_ids_with_border_codes.iter()
        .filter(|(_id, codes)| codes.len() == 4) // 4 instead of 2 since the flipped ones are there, too.
        .map(|(id, _)| *id)
        .collect()
}

fn border_tiles_with_border_codes(ids_by_border_code: &HashMap::<u16, Vec<(u64, Orientation)>>) -> HashMap<u64, Vec<(u16, Orientation)>>{
    let mut border_ids_with_codes = HashMap::<u64, Vec<(u16, Orientation)>>::new();
    for (code, (id, or)) in outer_border_codes_with_tiles(ids_by_border_code).iter(){
        match border_ids_with_codes.get_mut(id){
            Some(codes) => codes.push((*code, or.to_owned())),
            None => {
                let codes = vec![(*code, or.to_owned())];
                border_ids_with_codes.insert(*id, codes);
            }
        };
    }
    border_ids_with_codes
}

fn outer_border_codes_with_tiles(ids_by_border_code: &HashMap::<u16, Vec<(u64, Orientation)>>) -> HashMap<u16, (u64, Orientation)>{
    ids_by_border_code.iter()
        .filter(|(_code, ids)| ids.len() == 1)
        .map(|(code, ids)| (*code, ids[0]))
        .collect()
}


fn assembled_image(tiles_by_id: &HashMap<u64, Tile>) -> Vec<Vec<bool>>{
    let arranged_tiles = assembled_tiles(tiles_by_id);
    let mut rows = vec![];
    for tile_row in arranged_tiles.iter(){
        let mut row_batch: Vec<Vec<bool>> = repeat(vec![]).take(8).collect();
        for (tile_id, tile_or) in tile_row.iter(){
            let tile = tiles_by_id.get(tile_id).unwrap();
            for y in 1..9{
                for x in 1..9{
                    let lookup_point = reoriented_coordinates(Point::new([x,y]), tile_or,10, 10);
                    row_batch[y-1].push(tile.is_monster(lookup_point));
                }
            }
        }
        rows.append(&mut row_batch);
    }
    rows
}

///Returns an arrangement of the tiles with fitting borders between the tiles.
///The orientation describes the transformation from the tile in the assembled image to the original tile.
fn assembled_tiles(tiles_by_id: &HashMap<u64, Tile>) -> Vec<Vec<(u64, Orientation)>>{
    let mut rows = vec![];
    let ids_by_border_code = tiles_by_border_code(&tiles_by_id);
    let border_ids_with_border_codes = border_tiles_with_border_codes(&ids_by_border_code);
    let mut corner_ids = corner_ids(&border_ids_with_border_codes);
    corner_ids.sort_unstable(); //This is here to make the results deterministic.
    let start_corner = corner_ids[0];
    let first_row = arranged_first_row(start_corner, &tiles_by_id, &ids_by_border_code, &border_ids_with_border_codes);

    let mut maybe_current_row = Some(first_row);
    while maybe_current_row.is_some(){
        let current_row = maybe_current_row.unwrap();
        let maybe_next_row: Option<Vec<(u64, Orientation)>> = current_row.iter()
            .map(|(id, or)| tile_in_direction(Border::Lower, *id, or.to_owned(), tiles_by_id, &ids_by_border_code))
            .collect();
        rows.push(current_row);
        maybe_current_row = maybe_next_row;
    }

    rows
}

fn arranged_first_row(
    start_corner_id: u64,
    tiles_by_id: &HashMap<u64, Tile>,
    ids_by_border_code: &HashMap<u16, Vec<(u64, Orientation)>>,
    border_ids_with_border_codes: &HashMap<u64, Vec<(u16, Orientation)>>
) -> Vec<(u64, Orientation)>{
    let start_corner_orientation = upper_left_corner_orientation(start_corner_id, border_ids_with_border_codes);
    let mut first_row = vec![];
    let mut maybe_current_tile = Some((start_corner_id, start_corner_orientation));
    while let Some(current_tile) = maybe_current_tile{
        first_row.push(current_tile);
        let (current_id, current_or) = current_tile;
        maybe_current_tile = tile_in_direction(
            Border::Right,
            current_id,
            current_or,
            tiles_by_id,
            ids_by_border_code);
    }

    first_row
}

fn upper_left_corner_orientation(corner_id: u64, boder_ids_with_border_codes: &HashMap<u64, Vec<(u16, Orientation)>>) -> Orientation{
    let mut non_flip_rotations: Vec<u8> = boder_ids_with_border_codes.get(&corner_id).unwrap()
        .iter()
        .filter(|(_, or)| !or.flip)
        .map(|(_, or)| or.rotation())
        .collect();
    if non_flip_rotations.len() != 2{
        panic!("Unexpected number of non-flip border codes!")
    }
    non_flip_rotations.sort_unstable();
    if non_flip_rotations[1] == (Border::Left.into())
        &&  non_flip_rotations[0] == (Border::Upper.into()){
            Orientation { rotation: 0, flip: false }
    } else {
        Orientation { rotation: non_flip_rotations[0] + 1, flip: false }
    }
}

/// Returns the next tile in the direction together with its orientation.
fn tile_in_direction(
    direction: Border,
    tile_id: u64,
    orientation: Orientation,
    tiles_by_id: &HashMap<u64, Tile>,
    ids_by_border_code: &HashMap<u16, Vec<(u64, Orientation)>>,
) -> Option<(u64, Orientation)>{
    let border_code = flip_border_code_in_direction(direction, tile_id, orientation, tiles_by_id);
    let potential_tiles = ids_by_border_code.get(&border_code)?;
    let other_tiles: Vec<&(u64, Orientation)> = potential_tiles.iter()
        .filter(|(id, _or)| *id != tile_id)
        .collect();
    if other_tiles.is_empty(){
        return None;
    }
    if other_tiles.len() > 1{
        panic!("The edges of the tiles are not unique!");
    }
    let (next_tile, border_or) = other_tiles[0];

    let start_border = direction.opposite();
    let start_border_after_potential_flip = if border_or.flip() { start_border.flip_on_y_axis() } else { start_border };
    let rotation = (border_or.rotation() + 4 - start_border_after_potential_flip.value()) % 4;
    let tile_or = Orientation { rotation, flip: border_or.flip()};

    Some((*next_tile, tile_or))
}

fn border_after_orientation(border: Border, orientation: Orientation) -> Border{
    let border_after_potential_flip = if orientation.flip() { border.flip_on_y_axis() } else { border };
    border_after_potential_flip.rotate(orientation.rotation())
}

fn flip_border_code_in_direction(direction: Border, tile_id: u64, orientation: Orientation, tiles_by_id: &HashMap<u64, Tile>) -> u16{
    let tile = tiles_by_id.get(&tile_id).unwrap();
    let border_on_tile = border_after_orientation(direction, orientation);
    if orientation.flip{
        tile.border_codes[border_on_tile as usize]
    } else {
        tile.flip_border_codes[border_on_tile as usize]
    }
}

fn pixels_with_monster(image: &[Vec<bool>], mask: &SeaMonsterMask) -> HashSet<Point<usize, 2>>{
    Orientation::possible_values().iter()
        .flat_map(|or| pixels_with_monster_in_orientation(image, mask, or))
        .collect()
}

fn pixels_with_monster_in_orientation(image: &[Vec<bool>], mask: &SeaMonsterMask, orientation: &Orientation) -> Vec<Point<usize, 2>>{
    let image_height = image.len();
    if image_height == 0 {
        return vec![];
    }
    let image_width = image[0].len();

    let transformed_mask_height = if orientation.rotation % 2 == 0 { mask.height } else { mask.width };
    let transformed_mask_width = if orientation.rotation % 2 == 0 { mask.width } else { mask.height };

    if transformed_mask_width > image_width || transformed_mask_height > image_height{
        return vec![];
    }

    let mask_base_point = Point::new([0,0]);
    let reoriented_mask: HashSet<Vector<usize, 2>> = mask.monster_points.iter()
        .map(|offset| reoriented_coordinates(mask_base_point + *offset, orientation, mask.width, mask.height) - mask_base_point)
        .collect();

    let base_points = (0..(image_width - transformed_mask_width))
        .flat_map(|x| (0..(image_height - transformed_mask_height))
            .map(move |y| Point::new([x, y])));

    let base_points_with_monster = base_points
        .filter(|point| reoriented_mask.iter()
            .map(|offset| *point + *offset)
            .all(|point| image[point[1]][point[0]]));

    base_points_with_monster
        .flat_map(|point| reoriented_mask.iter()
            .map(move |offset| point + *offset))
        .collect()
}


#[cfg(test)]
mod day20_tests {
    use super::*;
    use crate::input;
    use crate::day;

    fn example_input() -> String{
        String::from(
r#"                  # 
#    ##    ##    ###
 #  #  #  #  #  #   

Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#)
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day20{});
        let problem_input = example_input();
        let expected_result = 20899048083289u64.to_string();
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day20{});
        let problem_input = example_input();
        let expected_value =
            ".####...#####..#...###..
#####..#..#.#.####..#.#.
.#.#...#.###...#.##.O#..
#.O.##.OO#.#.OO.##.OOO##
..#O.#O#.O##O..O.#O##.##
...#.#..##.##...#..#..##
#.##.#..#.#..#..##.#.#..
.###.##.....#...###.#...
#.####.#.#....##.#..#.#.
##...#..#....#..#...####
..#.##...###..#.#####..#
....#.##.#.#####....#...
..##.##.###.....#.##..#.
#...#...###..####....##.
.#.##...#.##.#.#.###...#
#.###.#..####...##..#...
#.###...#.##...#.##O###.
.O##.#OO.###OO##..OOO##.
..O#.O..O..O.#O##O##.###
#.#..##.########..#..##.
#.#####..#.#...##..#....
#....##..#.#########..##
#...#.....#..##...###.##
#..###....##.#...##.##.#"
            .chars().filter(|c| *c == MONSTER_CHARACTER).count();
        let expected_result = expected_value.to_string();
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day20{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 20, part: 1}).unwrap();
        let expected_result = String::from("63187742854073");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day20{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 20, part: 2}).unwrap();
        let expected_result = String::from("2152");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}