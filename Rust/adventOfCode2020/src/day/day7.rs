use std::collections::{HashMap, HashSet};
use std::hash::Hash;


#[derive(PartialEq, Eq, Hash, Debug, Clone, Default)]
struct BagType{
    appearance: String,
    color: String,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Default)]
pub struct LuggageRule{
    containing_bag: BagType,
    contained_bags: Vec<(BagType, usize)>,
}


pub struct Day7 {}

impl super::Day for Day7{
    type PuzzleInput = Vec<LuggageRule>;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        text.lines()
            .filter_map(|line| parsed_luggage_rule(line))
            .collect()
    }

    fn solve_part1(&self, rules: Self::PuzzleInput) -> std::string::String {
        let my_bag = BagType {appearance: String::from("shiny"), color: String::from("gold")};
        let graph = contained_graph(&rules);
        let containing_bags = descendants(&graph, &my_bag);
        let result = containing_bags.len();
        result.to_string()
    }

    fn solve_part2(&self, rules: Self::PuzzleInput) -> std::string::String {
        let my_bag = BagType {appearance: String::from("shiny"), color: String::from("gold")};
        let graph = containing_graph(&rules);
        let containing_bags = total_contained_bags(&graph, &my_bag);
        let result: usize = containing_bags.values().sum();
        result.to_string()
    }
}

fn parsed_luggage_rule(line: &str) -> Option<LuggageRule>{
    lazy_static! {
        static ref LUGGAGE_RULE_RE: regex::Regex = regex::Regex::new(r"(\w+) (\w+) bags contain(( no other bags)|(,? \d+ \w+ \w+ bags?)+)").unwrap();
    }
    let captures: regex::Captures = LUGGAGE_RULE_RE.captures(line)?;
    let containing_bag = BagType{ appearance: captures[1].to_owned(), color: captures[2].to_owned()};
    let contained_bags = parsed_contained_bags(&captures[3])?;
    Some(LuggageRule {containing_bag, contained_bags })
}

fn parsed_contained_bags(bags_text: &str) -> Option<Vec<(BagType, usize)>>{
    if bags_text == " no other bags"{
        return Some(Vec::new());
    }

    let contained_bag_definitions = bags_text.split(',');
    let contained_bag_specifications = contained_bag_definitions
        .filter_map(|bag_definition| parsed_contained_bag(bag_definition))
        .collect();
        Some(contained_bag_specifications)
}

fn parsed_contained_bag(bag_text: &str) -> Option<(BagType, usize)>{
    lazy_static! {
        static ref BAG_RE: regex::Regex = regex::Regex::new(r"(\d+) (\w+) (\w+) bags?").unwrap();
    }
    let captures: regex::Captures = BAG_RE.captures(bag_text)?;
    let contained_bag_type = BagType {appearance: captures[2].to_owned(), color: captures[3].to_owned()};
    let number_of_contained_bags = captures[1].parse::<usize>().ok()?;
    Some((contained_bag_type, number_of_contained_bags))
}

fn contained_graph(rules: &[LuggageRule]) -> HashMap<BagType, Vec<(BagType, usize)>>{
    let mut graph = HashMap::new();
    for rule in rules.iter(){
        let containing_bag = &rule.containing_bag;
        for (contained_bag, bag_count) in &rule.contained_bags{
            add_directed_edge(&mut graph, contained_bag, containing_bag, bag_count);
        }
    }
    graph
}

fn add_directed_edge<TVertex: Eq + Hash + Clone, TEdgeData: Copy>(graph: &mut HashMap<TVertex, Vec<(TVertex, TEdgeData)>>, start_vertex: &TVertex, end_vertex: &TVertex, edge_data: &TEdgeData){
    let children = graph.get_mut(start_vertex);
    match children{
        Some(ch) => ch.push((end_vertex.to_owned(), edge_data.to_owned())),
        None => {graph.insert(start_vertex.to_owned(), vec![(end_vertex.to_owned(), edge_data.to_owned())]);},
    }
}

fn descendants<'a, TVertex: Hash + Eq, TEdgeData>(graph: &'a HashMap<TVertex, Vec<(TVertex, TEdgeData)>>, node: &TVertex) -> HashSet<&'a TVertex>{
    let mut descendant_set = HashSet::new();
    add_descendants(graph, node, &mut descendant_set);
    descendant_set
}

fn add_descendants<'a, TVertex: Hash + Eq, TEdgeData>(graph: &'a HashMap<TVertex, Vec<(TVertex, TEdgeData)>>, node: &TVertex, nodes: &mut HashSet<&'a TVertex>){
    let maybe_children = graph.get(node);
    if let Some(children) = maybe_children {
        for (child, _edge_data) in children{
            nodes.insert(child);
            add_descendants(graph, child, nodes);
        }
    }
}

fn containing_graph(rules: &[LuggageRule]) -> HashMap<BagType, Vec<(BagType, usize)>>{
    let mut graph = HashMap::new();
    for rule in rules.iter(){
        let containing_bag = &rule.containing_bag;
        for (contained_bag, bag_count) in &rule.contained_bags{
            add_directed_edge(&mut graph, containing_bag, contained_bag, bag_count);
        }
    }
    graph
}

fn total_contained_bags(rules_dac: &HashMap<BagType, Vec<(BagType, usize)>>, node: &BagType) -> HashMap<BagType, usize>{
    let mut total_contained_bag_by_node: HashMap<BagType, HashMap<BagType, usize>> = HashMap::new();
    add_contained_bags(&mut total_contained_bag_by_node, node, rules_dac);
    total_contained_bag_by_node.remove(node).unwrap()
}

fn add_contained_bags(total_contained_bags: &mut HashMap<BagType, HashMap<BagType, usize>>, node: &BagType, rules_dac: &HashMap<BagType, Vec<(BagType, usize)>>){
    let maybe_children = rules_dac.get(node);
    if let Some(children) = maybe_children{
        let mut contained_bags: HashMap<BagType, usize> = HashMap::new();
        for (child, bag_count) in children{
            if !total_contained_bags.contains_key(child){
                add_contained_bags(total_contained_bags,child, rules_dac);
            }
            for (bag, &count) in total_contained_bags.get(child).unwrap(){
                add_to_bags( &mut contained_bags, bag, count * bag_count);
            }
            add_to_bags(&mut contained_bags, child, *bag_count);
        }
        total_contained_bags.insert(node.to_owned(), contained_bags);
    } else {
        total_contained_bags.insert(node.to_owned(), HashMap::new());
    }
}

fn add_to_bags(contained_bags: &mut HashMap<BagType, usize>, bag_type: &BagType, bag_count: usize){
    match contained_bags.get_mut(bag_type){
        Some(current_count) => *current_count += bag_count,
        None => {contained_bags.insert(bag_type.to_owned(), bag_count);},
    }
}



#[cfg(test)]
mod day7_tests {
    use super::*;
    use crate::input;
    use crate::day;

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day7{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 7, part: 1}).unwrap();
        let expected_result = String::from("161");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day7{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 7, part: 2}).unwrap();
        let expected_result = String::from("30899");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}