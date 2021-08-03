use std::collections::{HashSet, VecDeque};




pub struct Day22 {}

impl super::Day for Day22{
    type PuzzleInput = Vec<VecDeque<u8>>;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        text.split("\n\n")
            .map(|part| parse_deck(part))
            .collect()
    }

    fn solve_part1(&self, mut decks: Self::PuzzleInput) -> std::string::String {
        play_game(&mut decks);
        let scores = decks.iter().map(|deck| deck_score(deck));
        let result = scores.max().unwrap();
        result.to_string()
    }

    fn solve_part2(&self, mut decks: Self::PuzzleInput) -> std::string::String {
        play_recursive_game(&mut decks);
        let scores = decks.iter().map(|deck| deck_score(deck));
        let result = scores.max().unwrap();
        result.to_string()
    }
}

fn parse_deck(text: &str) -> VecDeque<u8>{
    text.lines()
        .skip(1)
        .filter_map(|line| line.parse::<u8>().ok())
        .collect()
}

fn play_game(decks: &mut[VecDeque<u8>]){
    while !game_has_ended(decks){
        play_round(decks);
    }
}

fn game_has_ended(decks: &[VecDeque<u8>]) -> bool{
    decks.iter().any(|deck| deck.is_empty())
}

/// Plays one round and changes the decks accordingly.
///
/// The game must not have ended, i.e. no deck may be empty.
fn play_round(decks: &mut[VecDeque<u8>]){
    let cards_played: Vec<u8> = decks.iter_mut().map(|deck| deck.pop_front().unwrap()).collect();
    let (winner, cards_to_add) = evaluate_round(cards_played);
    let winning_deck = &mut decks[winner];
    winning_deck.reserve(cards_to_add.len());
    for card in cards_to_add{
        winning_deck.push_back(card);
    }
}

/// Returns the index of the winning playern and the cards to add.
fn evaluate_round(cards_played: Vec<u8>) -> (usize, Vec<u8>){
    let winning_player = cards_played.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index)
        .unwrap();
    let mut cards_to_add = cards_played;
    cards_to_add.sort_unstable_by(|a, b| b.cmp(a));
    (winning_player, cards_to_add)
}

fn deck_score(deck: &VecDeque<u8>) -> usize{
    deck.iter()
        .rev()
        .enumerate()
        .map(|(index, card)| (index + 1) * usize::from(*card))
        .sum()
}

/// Returns whether the game has been played to the end.
fn play_recursive_game(decks: &mut[VecDeque<u8>]) -> bool{
    let mut known_configurations = HashSet::new();
    while !game_has_ended(decks){
        let current_configuration = configuration_key(decks);
        if known_configurations.contains(&current_configuration){
            return false;
        }
        known_configurations.insert(current_configuration);
        play_recursive_round(decks);
    }
    true
}

fn configuration_key(decks: &[VecDeque<u8>]) -> String{
    let deck_keys: Vec<_> = decks.iter()
        .map(deck_key)
        .collect();
    deck_keys.join("|")
}

fn deck_key(deck: &VecDeque<u8>) -> String{
    let card_keys: Vec<_> = deck.range(..)
        .map(|card| card.to_string())
        .collect();
    card_keys.join(",")
}

/// Plays one round and changes the decks accordingly.
///
/// The game must not have ended, i.e. no deck may be empty.
fn play_recursive_round(decks: &mut[VecDeque<u8>]){
    let cards_played: Vec<u8> = decks.iter_mut().map(|deck| deck.pop_front().unwrap()).collect();
    let (winner, cards_to_add) = evaluate_recursive_round(cards_played, decks);
    let winning_deck = &mut decks[winner];
    winning_deck.reserve(cards_to_add.len());
    for card in cards_to_add{
        winning_deck.push_back(card);
    }
}

/// Returns the index of the winning playern and the cards to add.
fn evaluate_recursive_round(cards_played: Vec<u8>, decks: &[VecDeque<u8>]) -> (usize, Vec<u8>){
    let deck_sizes: Vec<_> = decks.iter().map(|deck| deck.len()).collect();
    if !can_play_recursive_game(&cards_played, &deck_sizes){
        return evaluate_round(cards_played);
    }
    let mut recursive_decks = recursive_decks(&cards_played, decks);
    let game_played_to_the_end = play_recursive_game(&mut recursive_decks);
    if game_played_to_the_end {
        evaluate_recursive_game(cards_played, &recursive_decks)
    } else {
        (0, cards_played)
    }
}

fn can_play_recursive_game(cards_played: &[u8], deck_sizes: &[usize]) -> bool{
    cards_played.iter()
        .zip(deck_sizes.iter())
        .all(|(card, deck_size)| usize::from(*card) <= *deck_size)
}

fn recursive_decks(cards_played: &[u8], decks: &[VecDeque<u8>]) -> Vec<VecDeque<u8>>{
    decks.iter()
        .zip(cards_played.iter())
        .map(|(deck, card)| deck.range(..usize::from(*card)).copied().collect())
        .collect()
}

fn evaluate_recursive_game(cards_played: Vec<u8>, recursive_decks: &[VecDeque<u8>]) -> (usize, Vec<u8>){
    let scores: Vec<_> = recursive_decks.iter().map(|deck| deck_score(deck)).collect();
    let winning_player = scores.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index)
        .unwrap();
    let mut cards_with_scores: Vec<_> = cards_played.iter()
        .zip(scores.iter())
        .collect();
    cards_with_scores.sort_by(|(_card1, score1), (_card2, score2)| score2.cmp(score1));
    let cards_to_add = cards_with_scores.iter().map(|(card, _score)| **card).collect();
    (winning_player, cards_to_add)
}



#[cfg(test)]
mod day22_tests {
    use super::*;
    use crate::input;
    use crate::day;

    fn example_input() -> String{
        String::from(
r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#)
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day22{});
        let problem_input = example_input();
        let expected_result = 306.to_string();
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day22{});
        let problem_input = example_input();
        let expected_result = 291.to_string();
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day22{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 22, part: 1}).unwrap();
        let expected_result = String::from("32033");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day22{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 22, part: 2}).unwrap();
        let expected_result = String::from("34901");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}