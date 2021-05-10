use std::convert::TryFrom;

use crate::algebra;

pub struct Day13 {}

impl super::Day for Day13{
    type PuzzleInput = (i128, Vec<String>);

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        let mut lines = text.lines();

        let maybe_earliest_time = lines.next();
        if maybe_earliest_time.is_none() {
            return (0, vec![]);
        }
        let possibly_earliest_time = maybe_earliest_time.unwrap().parse::<i128>();
        if possibly_earliest_time.is_err(){
            return (0, vec![]);
        }
        let earliest_time = possibly_earliest_time.unwrap();

        let maybe_timetable = lines.next();
        if maybe_timetable.is_none(){
            return (earliest_time, vec![]);
        }
        let timetable: Vec<String> = maybe_timetable.unwrap()
            .split(',')
            .map(|item| item.to_owned())
            .collect();
        (earliest_time, timetable)
    }

    fn solve_part1(&self, (earliest_time, timetable): Self::PuzzleInput) -> std::string::String {
        if timetable.is_empty(){
            return String::from("No timetable!");
        }
        let bus_ids = timetable.iter()
            .filter_map(|id| id.parse::<i128>().ok());
        let ids_with_waiting_time = bus_ids.map(|id| (id, bus_waiting_time(earliest_time, id)));
        let best_connection = ids_with_waiting_time.min_by(|(_id1, time1),(_id2, time2)| time1.cmp(time2));
        if best_connection.is_none(){
            return String::from("No connections!");
        }
        let (shortest_wait, corresponding_bus) = best_connection.unwrap();
        let result = shortest_wait * corresponding_bus;
        result.to_string()
    }

    fn solve_part2(&self, (_earliest_time, timetable): Self::PuzzleInput) -> std::string::String {
        let contest_data: Vec<(i128, i128)> = timetable.into_iter()
            .enumerate()
            .filter_map(|(index, id_text)| match id_text.parse::<i128>(){
                Err(_) => None,
                Ok(id) => Some((id, index))
            })
            .filter_map(|(id, index)| match i128::try_from(index) {
                Err(_) => None,
                Ok(ind) => Some((time_since_last_departure(ind, id), id)),
            })
            .collect();
        let result = algebra::chinese_remainder(contest_data);
        match result{
            None => String::from("There is no result!"),
            Some(solution) => solution.to_string()
        }
    }
}

fn bus_waiting_time(base_time: i128, bus_id: i128) -> i128{
    bus_id - (base_time % bus_id)
}

fn time_since_last_departure(offset_from_first_bus: i128, bus_id: i128) -> i128{
    if offset_from_first_bus == 0{
        0
    } else {
        bus_id - (offset_from_first_bus % bus_id)
    }
}

#[cfg(test)]
mod day13_tests {
    use super::*;
    use crate::input;
    use crate::day;

    fn example_input() -> String{
        String::from(
"939
7,13,x,x,59,x,31,19")
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day13{});
        let problem_input = example_input();
        let expected_result = 295.to_string();
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day13{});
        let problem_input = example_input();
        let expected_result = 1068781.to_string();
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day13{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 13, part: 1}).unwrap();
        let expected_result = String::from("161");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day13{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 13, part: 2}).unwrap();
        let expected_result = String::from("213890632230818");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}