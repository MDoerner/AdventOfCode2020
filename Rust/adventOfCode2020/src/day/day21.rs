use std::collections::{HashMap, HashSet};



#[derive(Debug, Clone)]
pub struct Recipe{
    ingredients: Vec<String>,
    allergens: Vec<String>,
}


pub struct Day21 {}

impl super::Day for Day21{
    type PuzzleInput = Vec<Recipe>;

    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        text.lines()
            .filter_map(|line| parse_recipe(line))
            .collect()
    }

    fn solve_part1(&self, recipies: Self::PuzzleInput) -> std::string::String {
        let ingredients_with_allergens: HashSet<String> = ingredients_with_allergens(&recipies).values().cloned().collect();
        let result = recipies.iter()
            .flat_map(|recipe| recipe.ingredients.iter())
            .filter(|ingredient| !ingredients_with_allergens.contains(*ingredient))
            .count();
        result.to_string()
    }

    fn solve_part2(&self, recipies: Self::PuzzleInput) -> std::string::String {
        let ingredients_with_allergens = ingredients_with_allergens(&recipies);
        let mut allergens: Vec<String> = ingredients_with_allergens.keys().cloned().collect();
        allergens.sort_unstable();
        let ingredients_sorted_by_allergens: Vec<String> = allergens.iter().map(|allergen| ingredients_with_allergens.get(allergen).unwrap().to_owned()).collect();
        ingredients_sorted_by_allergens.join(",")
    }
}

fn parse_recipe(text: &str) -> Option<Recipe>{
    lazy_static! {
        static ref RECIPE_RE: regex::Regex = regex::Regex::new(r#"(\w+( \w+)*) \(contains (\w+(, \w+)*)\)"#).unwrap();
    }
    let captures: regex::Captures = RECIPE_RE.captures(text)?;
    let ingredients = captures[1].split(' ').map(|ingredient| ingredient.to_owned()).collect();
    let allergens = captures[3].split(", ").map(|allergens| allergens.to_owned()).collect();
    Some(Recipe { ingredients, allergens })
}

fn ingredients_with_allergens(recipes: &[Recipe]) -> HashMap<String, String>{
    let mut determined_ingredients_by_allergen = HashMap::new();
    let recipe_indices_by_allergen = recipe_indices_by_allergen(recipes);
    let mut possible_ingredients_by_allergen = possible_ingredients_by_allergen(recipes, &recipe_indices_by_allergen);
    let mut new_allergens_with_one_possible_ingredient = single_value_items(&possible_ingredients_by_allergen);
    while !new_allergens_with_one_possible_ingredient.is_empty(){
        for allergen in new_allergens_with_one_possible_ingredient.iter(){
            let ingredient = possible_ingredients_by_allergen.get(allergen).unwrap().iter().cloned().next().unwrap();
            determined_ingredients_by_allergen.insert(allergen.to_owned(), ingredient.clone());
            possible_ingredients_by_allergen.remove(allergen);
            for (_allergen, ingredients) in possible_ingredients_by_allergen.iter_mut(){
                if ingredients.contains(&ingredient){
                    ingredients.remove(&ingredient);
                }
            }
        }
        new_allergens_with_one_possible_ingredient = single_value_items(&possible_ingredients_by_allergen);
    }

    determined_ingredients_by_allergen
}

fn single_value_items(items: &HashMap<String, HashSet<String>>) -> Vec<String>{
    items.iter()
        .filter(|(_item, values)| values.len() == 1)
        .map(|(item, _)| item.to_owned())
        .collect()
}

fn recipe_indices_by_allergen(recipies: &[Recipe]) -> HashMap<String, Vec<usize>>{
    let mut indices_by_allergen: HashMap<String, Vec<usize>> = HashMap::new();
    for (allergen, index) in recipies.iter()
        .enumerate()
        .flat_map(|(index, recipe)| recipe.allergens
            .iter()
            .map(move |allergen| (allergen.to_owned(), index))){
        if let Some(indices) = indices_by_allergen.get_mut(&allergen) {
            indices.push(index)
        } else {
            indices_by_allergen.insert(allergen, vec![index]);
        }
    }
    indices_by_allergen
}

fn possible_ingredients_by_allergen(recipies: &[Recipe], indices_by_allergen: &HashMap<String, Vec<usize>>) -> HashMap<String, HashSet<String>>{
    indices_by_allergen.iter()
        .map(|(allergen, indices)| (allergen.to_owned(), common_ingredients(recipies, indices)))
        .collect()
}

fn common_ingredients(recipies: &[Recipe], selected_indices: &[usize]) -> HashSet<String>{
    if selected_indices.is_empty(){
        return HashSet::new();
    }

    if selected_indices.len() == 1{
        return recipies[selected_indices[0]].ingredients.iter().cloned().collect();
    }

    let mut ingredients = HashSet::new();
    for ingredient in recipies[selected_indices[0]].ingredients.iter(){
        if selected_indices[1..].iter().all(|index| recipies[*index].ingredients.contains(ingredient)){
            ingredients.insert(ingredient.to_owned());
        }
    }

    ingredients
}


#[cfg(test)]
mod day21_tests {
    use super::*;
    use crate::input;
    use crate::day;

    fn example_input() -> String{
        String::from(
r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#)
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day21{});
        let problem_input = example_input();
        let expected_result = 5.to_string();
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day21{});
        let problem_input = example_input();
        let expected_result = String::from("mxmxvkd,sqjhc,fvjkl");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day21{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 21, part: 1}).unwrap();
        let expected_result = String::from("2265");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day21{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 21, part: 2}).unwrap();
        let expected_result = String::from("dtb,zgk,pxr,cqnl,xkclg,xtzh,jpnv,lsvlx");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}