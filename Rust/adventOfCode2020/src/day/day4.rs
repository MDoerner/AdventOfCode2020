use regex::Regex;


#[derive(Eq, PartialEq, Hash, Debug)]
pub struct TravelDocument{
    pub birth_year: Option<String>,
    pub issue_year: Option<String>,
    pub expiration_year: Option<String>,
    pub height: Option<String>,
    pub hair_colour: Option<String>,
    pub eye_colour: Option<String>,
    pub passport_id: Option<String>,
    pub country_id: Option<String>,
}



pub struct Day4 {}

impl super::Day for Day4{
    type PuzzleInput = Vec<TravelDocument>;
    fn parse_input(&self, text: std::string::String) -> Self::PuzzleInput {
        let document_delimiter_regex = regex::Regex::new(r"\r?\n\r?\n").unwrap();
        document_delimiter_regex.split(&text)
            .map(|document_text| parse_document(document_text))
            .collect()
    }

    fn solve_part1(&self, documents: Self::PuzzleInput) -> String {
        let number_of_passports = documents.iter()
            .filter(|document| is_passport(document))
            .count();
        number_of_passports.to_string()
    }

    fn solve_part2(&self, documents: Self::PuzzleInput) -> String {
        let number_of_passports = documents.iter()
            .filter(|document| is_valid_passport(document))
            .count();
        number_of_passports.to_string()
    }

}

fn parse_document(text: &str) -> TravelDocument{
    let document_items = text.lines()
        .map(|line| line.split(' '))
        .flatten()
        .map(|item| item.split(':').collect::<Vec<&str>>())
        .filter(|item_elements| item_elements.len() == 2)
        .map(|item_elements| match item_elements[..]{
                [item_type, item_value, ..] => (item_type, item_value),
                _ => unreachable!(),
            });
    document_from_item_texts(document_items)
}

fn document_from_item_texts<'a>(items_on_document: impl IntoIterator<Item=(&'a str, &'a str)>) -> TravelDocument{
    let mut birth_year: Option<String> = None;
    let mut issue_year: Option<String> = None;
    let mut expiration_year: Option<String> = None;
    let mut height: Option<String> = None;
    let mut hair_colour: Option<String> = None;
    let mut eye_colour: Option<String> = None;
    let mut passport_id: Option<String> = None;
    let mut country_id: Option<String> = None;

    for (item_type, item_value) in items_on_document{
        match item_type{
            "byr" => birth_year = Some(item_value.to_owned()),
            "iyr" => issue_year = Some(item_value.to_owned()),
            "eyr" => expiration_year = Some(item_value.to_owned()),
            "hgt" => height = Some(item_value.to_owned()),
            "hcl" => hair_colour = Some(item_value.to_owned()),
            "ecl" => eye_colour = Some(item_value.to_owned()),
            "pid" => passport_id = Some(item_value.to_owned()),
            "cid" => country_id = Some(item_value.to_owned()),
            _ => (),
        }
    }

    TravelDocument {
        birth_year,
        issue_year,
        expiration_year,
        height,
        hair_colour,
        eye_colour,
        passport_id,
        country_id,
    }
}

fn is_passport(document: &TravelDocument) -> bool{
    document.birth_year.is_some()
        && document.issue_year.is_some()
        && document.expiration_year.is_some()
        && document.height.is_some()
        && document.hair_colour.is_some()
        && document.eye_colour.is_some()
        && document.passport_id.is_some()
}

fn is_valid_passport(document: &TravelDocument) -> bool{
    is_passport(document)
        && is_valid_birth_year(document.birth_year.as_ref().unwrap())
        && is_valid_issue_year(document.issue_year.as_ref().unwrap())
        && is_valid_expiration_year(document.expiration_year.as_ref().unwrap())
        && is_valid_height(document.height.as_ref().unwrap())
        && is_valid_hair_color(document.hair_colour.as_ref().unwrap())
        && is_valid_eye_color(document.eye_colour.as_ref().unwrap())
        && is_valid_passport_id(document.passport_id.as_ref().unwrap())
}

fn is_valid_birth_year(text: &str) -> bool{
    is_valid_year(text, 1920, 2002)
}

fn is_valid_year(text: &str, min_year: u16, max_year: u16) -> bool{
    lazy_static! {
        static ref YEAR_RE: Regex = Regex::new(r"^\d{4}$").unwrap();
    }
    if !YEAR_RE.is_match(text){
        return false;
    }
    let parsed_year = text.parse::<u16>();
    match parsed_year{
        Ok(year) =>  min_year <= year && year <= max_year,
        Err(_) => false,
    }
}

fn is_valid_issue_year(text: &str) -> bool{
    is_valid_year(text, 2010, 2020)
}

fn is_valid_expiration_year(text: &str) -> bool{
    is_valid_year(text, 2020, 2030)
}

fn is_valid_height(text: &str) -> bool{
    lazy_static! {
        static ref HEIGHT_RE: Regex = Regex::new(r"^(?P<value>\d+)(?P<unit>in|cm)$").unwrap();
    }
    let captures: regex::Captures;
    match HEIGHT_RE.captures(text){
        Some(cap) => captures = cap,
        None => return false
    }
    let height_value: u16;
    match captures.name("value").unwrap().as_str().parse::<u16>(){
        Ok(height) => height_value = height,
        Err(_) => return false,
    }
    match captures.name("unit").unwrap().as_str(){
        "cm" => (150..=193).contains(&height_value),
        "in" => (59..=76).contains(&height_value),
        _ => false,
    }
}

fn is_valid_hair_color(text: &str) -> bool{
    lazy_static! {
        static ref HAIR_COLOR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    HAIR_COLOR_RE.is_match(text)
}

const VALID_EYE_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
fn is_valid_eye_color(text: &str) -> bool{
    VALID_EYE_COLORS.iter()
        .any(|&item| item == text)
}

fn is_valid_passport_id(text: &str) -> bool{
    lazy_static! {
        static ref PASSPORT_ID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    PASSPORT_ID_RE.is_match(text)
}

#[cfg(test)]
mod day4_tests {
    use super::*;
    use crate::input;
    use crate::day;


    fn example_input() -> String{
        String::from(
"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in")
    }

    fn example_valid() -> String{
        String::from(
"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719")
    }

    fn example_invalid() -> String{
        String::from(
"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007")
    }

    #[test]
    fn example_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day4{});
        let problem_input = example_input();
        let expected_result = String::from("2");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example_part2_valid() {
        let day: Box<dyn day::DaySolver> = Box::new(Day4{});
        let problem_input = example_valid();
        let expected_result = String::from("4");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn example_part2_invalid() {
        let day: Box<dyn day::DaySolver> = Box::new(Day4{});
        let problem_input = example_invalid();
        let expected_result = String::from("0");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn correct_part1() {
        let day: Box<dyn day::DaySolver> = Box::new(Day4{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 4, part: 1}).unwrap();
        let expected_result = String::from("182");
        let actual_result = day.solve_part1(problem_input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn correct_part2() {
        let day: Box<dyn day::DaySolver> = Box::new(Day4{});
        let problem_input = input::puzzle_input(&input::PuzzleConfiguration{day: 4, part: 2}).unwrap();
        let expected_result = String::from("109");
        let actual_result = day.solve_part2(problem_input);
        assert_eq!(actual_result, expected_result);
    }
}

