use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct TicketRule{
    name: String,
    low_range: (u32, u32),
    high_range: (u32, u32),
}

#[derive(Debug)]
pub struct Ticket{
    entries: Vec<u32>,
}

#[derive(Debug)]
pub struct TicketData{
    rules: Vec<TicketRule>,
    own_ticket: Ticket,
    other_tickets: Vec<Ticket>,
}

pub struct Day16 {}

impl super::Day for Day16{
    type PuzzleInput = Option<TicketData>;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        let sections: Vec<&str> = text.split("\n\n").collect();
        if sections.len() < 3{
            return None;
        }
        let rules = parsed_ticket_rules(sections[0]);

        let my_ticket_text = sections[1].lines().nth(1)?;
        let own_ticket = parsed_ticket(my_ticket_text);

        let other_tickets = sections[2]
            .lines()
            .skip(1)
            .map(parsed_ticket)
            .collect();

        Some(TicketData {rules, own_ticket, other_tickets})
    }

    fn solve_part1(&self, maybe_ticket_data: Self::PuzzleInput) -> std::string::String {
        if maybe_ticket_data.is_none(){
            return String::from("Invalid input!");
        }
        let ticket_data = maybe_ticket_data.unwrap();
        let rules = ticket_data.rules;
        let invalid_entries = ticket_data.other_tickets.iter()
            .map(|ticket| ticket.entries
                .iter()
                .filter(|entry| !is_valid_by_some_rule(entry, &rules))
            ).flatten();
        let error_rate: u32 = invalid_entries.sum();
        error_rate.to_string()
    }

    fn solve_part2(&self, maybe_ticket_data: Self::PuzzleInput) -> std::string::String {
        if maybe_ticket_data.is_none(){
            return String::from("Invalid input!");
        }
        let ticket_data = maybe_ticket_data.unwrap();

        let rules = ticket_data.rules;
        let own_ticket = ticket_data.own_ticket;
        let mut rules_for_entries:Vec<HashSet<TicketRule>> = own_ticket
            .entries
            .iter()
            .map(|entry| rules.iter()
                .map(|rule| rule.to_owned())
                .filter(|rule| is_valid_by_rule(entry, rule))
                .collect())
            .collect();

        for ticket in ticket_data.other_tickets.iter().filter(|ticket| is_valid(ticket, &rules)){
            for (index, entry) in  ticket.entries.iter().enumerate(){
                let rules = &mut rules_for_entries[index];
                remove_invalid_rules(entry, rules);
            }
        }

        reduce_by_uniqueness_of_position(&mut rules_for_entries);

        let maybe_assigned_rules = unique_rules_for_entries(&rules_for_entries);
        if maybe_assigned_rules.is_none(){
            return String::from("No unique solution could be found!");
        }
        let assigned_rules = maybe_assigned_rules.unwrap();

        let result: u128 = assigned_rules.iter()
            .enumerate()
            .map(|(index, rule)| if rule.name.starts_with("departure"){
                u128::from(own_ticket.entries[index])
            } else {
                1
            }).product();
        result.to_string()
    }
}

fn parsed_ticket_rules(rules_text: &str) -> Vec<TicketRule>{
    rules_text.lines()
        .filter_map(parsed_ticket_rule)
        .collect()
}

fn parsed_ticket_rule(rule_text: &str) -> Option<TicketRule>{
    lazy_static! {
        static ref RULE_RE: regex::Regex = regex::Regex::new(r"^((\w|\s)+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    }
    let captures: regex::Captures = RULE_RE.captures(rule_text)?;
    let name = captures[1].to_owned();
    let low_lower_bound = captures[3].parse::<u32>().ok()?;
    let low_upper_bound = captures[4].parse::<u32>().ok()? + 1; //Rust upper bounds are usually not inclusive.
    let high_lower_bound = captures[5].parse::<u32>().ok()?;
    let high_upper_bound = captures[6].parse::<u32>().ok()? + 1;
    Some(TicketRule {name, low_range: (low_lower_bound, low_upper_bound), high_range: (high_lower_bound, high_upper_bound)})
}

fn parsed_ticket(ticket_text: &str) -> Ticket{
    let ticket_entries = ticket_text.split(',')
        .filter_map(|item| item.parse::<u32>().ok())
        .collect();
    Ticket {entries: ticket_entries}
}



fn is_valid_by_some_rule(ticket_entry: &u32, rules: &[TicketRule]) -> bool{
    rules.iter().any(|rule| is_valid_by_rule(ticket_entry, rule))
}

fn is_valid_by_rule(ticket_entry: &u32, rule: &TicketRule) -> bool{
    let (low_lower_bound, low_upper_bound) = rule.low_range;
    let (high_lower_bound, high_upper_bound) = rule.high_range;
    (low_lower_bound..low_upper_bound).contains(ticket_entry)
        || (high_lower_bound..high_upper_bound).contains(ticket_entry)
}

fn is_valid(ticket: &Ticket, rules: &[TicketRule]) -> bool{
    ticket.entries.iter().all(|entry| is_valid_by_some_rule(entry, rules))
}

fn remove_invalid_rules(ticket_entry: &u32, rules: &mut HashSet<TicketRule>){
    let invalid_rules: Vec<TicketRule> = rules.iter()
        .cloned()
        .filter(|rule| !is_valid_by_rule(ticket_entry, rule))
        .collect();
    for rule in invalid_rules{
        rules.remove(&rule);
    }
}

fn reduce_by_uniqueness_of_position(rules_by_entry: &mut [HashSet<TicketRule>]){
    let mut processed_indices: HashSet<usize> = HashSet::new();
    let mut continue_to_search = true;
    while continue_to_search {
        continue_to_search = false;
        for index in 0..rules_by_entry.len(){
            if rules_by_entry[index].len() == 1 && !processed_indices.contains(&index){
                let single_rule = rules_by_entry[index].iter().next().unwrap().to_owned();
                remove_rule(&single_rule, index, rules_by_entry);
                processed_indices.insert(index);
                continue_to_search = true;
            }
        }
    }
}

fn remove_rule(rule: &TicketRule, index_to_skip:usize, rules_by_entry: &mut [HashSet<TicketRule>]){
    for (index, rule_set) in rules_by_entry.iter_mut().enumerate(){
        if index != index_to_skip{
            rule_set.remove(rule);
        }
    }
}

fn unique_rules_for_entries(rules_by_entry: &[HashSet<TicketRule>]) -> Option<Vec<TicketRule>>{
    let mut rules = vec![];
    for rule_set in rules_by_entry{
        if rule_set.len() != 1{
            return None;
        }
        let unique_item = rule_set.iter().next().unwrap().to_owned();
        rules.push(unique_item);
    }
    Some(rules)
}


#[cfg(test)]
mod day16_tests {
    use super::*;
    use crate::input;
    use crate::day;

    fn example_input() -> String{
        String::from(
"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12")
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day16{});
        let problem_input = example_input();
        let expected_result = 71.to_string();
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day16{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 16, part: 1}).unwrap();
        let expected_result = String::from("26941");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day16{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 16, part: 2}).unwrap();
        let expected_result = String::from("634796407951");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}