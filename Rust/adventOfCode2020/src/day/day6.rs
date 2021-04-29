use std::collections::{HashSet, HashMap};




#[derive(PartialEq, Eq, Hash, Debug, Clone, Default)]
pub struct TravelGroup{
    customer_forms: Vec<String>
}


pub struct Day6 {}

impl super::Day for Day6{
    type PuzzleInput = Vec<TravelGroup>;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        let lines = text.lines();
        let mut travel_groups: Vec<TravelGroup> = vec![];
        let mut current_customer_forms: Vec<String> = vec![];
        for line in lines{
            if line.is_empty(){
                let new_group = TravelGroup {customer_forms: current_customer_forms};
                travel_groups.push(new_group);
                current_customer_forms = vec![];
            } else {
                current_customer_forms.push(line.to_string())
            }
        }
        let final_group = TravelGroup {customer_forms: current_customer_forms};
        travel_groups.push(final_group);
        travel_groups
    }

    fn solve_part1(&self, travel_groups: Self::PuzzleInput) -> std::string::String {
        let unique_items_per_group = travel_groups.iter().map(|group| unique_characters(&group.customer_forms));
        let group_items_sum: usize = unique_items_per_group
            .map(|group_items| group_items.len())
            .sum();
        group_items_sum.to_string()
    }

    fn solve_part2(&self, travel_groups: Self::PuzzleInput) -> std::string::String {
        let common_items_per_group = travel_groups.iter().map(|group| common_characters(&group.customer_forms));
        let group_items_sum: usize = common_items_per_group
            .map(|group_items| group_items.len())
            .sum();
        group_items_sum.to_string()
    }
}

fn unique_characters(strings:&[String]) -> HashSet<char>{
    let characters = strings.iter().map(|s| s.chars()).flatten();
    characters.collect()
}

fn common_characters(strings: &[String]) -> HashSet<char>{
    let size = strings.len();
    let character_counts = unique_characters_with_count(strings);
    character_counts.iter()
        .filter(|(_, count)| **count == size)
        .map(|(k, _)| *k)
        .collect()
}

fn unique_characters_with_count(strings:&[String]) -> HashMap<char, usize>{
    let characters = strings.iter().map(|s| s.chars()).flatten();
    let mut item_counts: HashMap<char, usize> = HashMap::new();
    for character in characters{
        increment_item_count(&mut item_counts, character);
    }
    item_counts
}

fn increment_item_count(item_counts: &mut HashMap<char, usize>, item: char) -> (){
    let current_item_count = item_counts.get(&item);
    match current_item_count {
        Some(old_count) => item_counts.insert(item, old_count + 1),
        None => item_counts.insert(item, 1),
    };
}



#[cfg(test)]
mod day6_tests {
    use super::*;
    use crate::input;
    use crate::day;

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day6{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 6, part: 1}).unwrap();
        let expected_result = String::from("6763");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day6{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 6, part: 2}).unwrap();
        let expected_result = String::from("3512");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}