use regex;

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
        .map(|item_elements| match &item_elements[..]{
                &[item_type, item_value, ..] => (item_type, item_value),
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
        birth_year: birth_year,
        issue_year: issue_year,
        expiration_year: expiration_year,
        height: height,
        hair_colour: hair_colour,
        eye_colour: eye_colour,
        passport_id: passport_id,
        country_id: country_id,
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
    let year_regex = regex::Regex::new(r"^\d{4}$").unwrap();
    if !year_regex.is_match(text){
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
    let height_regex = regex::Regex::new(r"^(?P<value>\d+)(?P<unit>in|cm)$").unwrap();
    let captures: regex::Captures;
    match height_regex.captures(text){
        Some(cap) => captures = cap,
        None => return false
    }
    let height_value: u16;
    match captures.name("value").unwrap().as_str().parse::<u16>(){
        Ok(height) => height_value = height,
        Err(_) => return false,
    }
    match captures.name("unit").unwrap().as_str(){
        "cm" => 150 <= height_value && height_value <= 193,
        "in" => 59 <= height_value && height_value <= 76,
        _ => false,
    }
}

fn is_valid_hair_color(text: &str) -> bool{
    let hair_color_regex = regex::Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    hair_color_regex.is_match(text)
}

const VALID_EYE_COLORS: &'static [&'static str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
fn is_valid_eye_color(text: &str) -> bool{
    VALID_EYE_COLORS.iter()
        .any(|&item| item == text)
}

fn is_valid_passport_id(text: &str) -> bool{
    let passport_id_regex = regex::Regex::new(r"^\d{9}$").unwrap();
    passport_id_regex.is_match(text)
}

