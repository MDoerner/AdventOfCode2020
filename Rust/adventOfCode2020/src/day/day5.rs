

pub struct Seat{
    row: u16,
    column: u16,
}


pub struct Day5 {}

impl super::Day for Day5{
    type PuzzleInput = Vec<Seat>;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        text.lines()
            .map(|boarding_card_text| parse_boarding_card(boarding_card_text))
            .filter_map(Result::ok)
            .collect::<Vec<Seat>>()
    }

    fn solve_part1(&self, seats: Self::PuzzleInput) -> std::string::String {
        seats.iter()
            .map(|seat| seat.seat_id())
            .fold(0, |previous_max, next_value| previous_max.max(next_value))
            .to_string()
    }

    fn solve_part2(&self, seats: Self::PuzzleInput) -> std::string::String {
        let my_seat_id = first_free_seat_id(&seats);
        my_seat_id.to_string()
    }
}

fn parse_boarding_card(text: &str) -> Result<Seat, std::num::ParseIntError>{
    let (row_text, column_text) = text.split_at(7);
    let row = parse_seat_row(row_text)?;
    let column = parse_seat_column(column_text)?;
    Ok(Seat {row: row, column: column})
}

fn parse_seat_row(text: &str) -> Result<u16, std::num::ParseIntError>{
    let binary_representation = text.replace("F", "0").replace("B", "1");
    u16::from_str_radix(&binary_representation, 2)
}

fn parse_seat_column(text: &str) -> Result<u16, std::num::ParseIntError>{
    let binary_representation = text.replace("L", "0").replace("R", "1");
    u16::from_str_radix(&binary_representation, 2)
}

impl Seat {
    fn seat_id(&self) -> u32{
        u32::from(self.row) * 8 + u32::from(self.column)
    }
}

fn first_free_seat_id(seats: &Vec<Seat>) -> u32{
    let mut seat_ids: Vec<u32> = seats.iter()
    .map(|seat| seat.seat_id())
    .collect();
    seat_ids.sort();
    first_missing_seat_id(&seat_ids)
}

fn first_missing_seat_id(seat_ids: &Vec<u32>) -> u32{
    let mut seat_index: usize = 0;
    while seat_index + 1 < seat_ids.len()
        && seat_ids[seat_index] == seat_ids[seat_index + 1] - 1{
            seat_index += 1;
    }
    seat_ids[seat_index] + 1
}