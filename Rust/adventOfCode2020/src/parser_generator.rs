use std::collections::HashMap;



pub enum GrammarRule{
    LexerRule {id: u64, text: String},
    PaserRule {id: u64, alternatives: Vec<Vec<u64>>},
}

impl GrammarRule {
    pub fn id(&self) -> u64{
        match self {
            GrammarRule::LexerRule { id, .. } => *id,
            GrammarRule::PaserRule { id, .. } => *id,
        }
    }
}


pub fn parse_grammar(rule_specification: &str, rule_specification_corrections: Option<&str>) -> HashMap<u64, GrammarRule>{
    let mut grammar = HashMap::<u64, GrammarRule>::new();
    let rules = rule_specification.lines()
        .filter_map(|line| parse_rule(line));
    for rule in rules{
        grammar.insert(rule.id(), rule);
    }

    if let Some(rule_specification_corrections_text) = rule_specification_corrections{
        let rule_corrections = rule_specification_corrections_text.lines()
            .filter_map(|line| parse_rule(line));
        for rule in rule_corrections{
            grammar.insert(rule.id(), rule);
        }
    }

    grammar
}


fn parse_rule(rule_specification: &str) -> Option<GrammarRule>{
    lazy_static! {
        static ref RULE_RE: regex::Regex = regex::Regex::new(r#"^([^:"\s]+): ("(.+)"|((([^:"\s]+\s*)+\|?)+))$"#).unwrap();
    }
    let captures: regex::Captures = RULE_RE.captures(rule_specification)?;
    let rule_id = captures.get(1)?.as_str().parse::<u64>().ok()?;

    if let Some(token_text_match) = captures.get(3){
        return Some(GrammarRule::LexerRule { id: rule_id, text: token_text_match.as_str().to_owned() });
    }

    let alternatives_text = captures.get(4)?.as_str();
    let alterative_texts = alternatives_text.split('|');
    let maybe_alternatives: Option<Vec<Vec<u64>>> = alterative_texts
        .map(|rule_sequence_text| rule_sequence_text
            .trim()
            .split(' ')
            .map(|rule_id_text| rule_id_text.parse::<u64>().ok()
        ).collect())
        .collect();
    if let Some(alternatives) = maybe_alternatives{
        return Some(GrammarRule::PaserRule { id: rule_id, alternatives });
    }

    None
}


#[derive(Debug, Clone)]
pub enum GrammarContext{
    ParserContext {
        start_index: usize,
        stop_index: usize,
        rule_id: u64,
        children: Vec<GrammarContext>,
    },
    LexerContext {
        start_index: usize,
        stop_index: usize,
        rule_id: u64,
    },
}




impl GrammarContext{
    #[allow(dead_code)]
    pub fn start_index(&self) -> usize{
        match self{
            GrammarContext::ParserContext { start_index, ..} | GrammarContext::LexerContext { start_index, ..} => *start_index,
        }
    }

    pub fn stop_index(&self) -> usize{
        match self{
            GrammarContext::ParserContext { stop_index, ..} | GrammarContext::LexerContext { stop_index, ..} => *stop_index,
        }
    }
}


pub struct Parser{
    grammar: HashMap<u64, GrammarRule>,
}

impl Parser{
    /// Returns a parser based on the grammar provided.
    ///
    /// # Arguments
    /// * `grammar` - The grammar rules describing the language. It must not contain left-recursive rules.
    ///
    pub fn new(grammar: HashMap<u64, GrammarRule>) -> Parser{
        Parser { grammar }
    }

    /// Returns all possible parse trees representing the input text.
    ///
    /// # Arguments
    /// * `input_text` - Text to parse. It must contain only ASCII characters.
    /// * `start_rule_id` - Rule to match to the text.
    ///
    pub fn parse(&self, input_text: &str, start_rule_id: u64) -> Vec<GrammarContext>{
        let maybe_start_rule = self.grammar.get(&start_rule_id);
        if maybe_start_rule.is_none(){
            return vec![];
        }
        let start_rule = maybe_start_rule.unwrap();
        self.parse_rule(input_text, start_rule, 0).into_iter()
            .filter(|context| context.stop_index() == input_text.len())
            .collect()
    }

    fn parse_rule(&self, input_text: &str, rule: &GrammarRule, start_index: usize) -> Vec<GrammarContext>{
        match rule{
            GrammarRule::LexerRule { id, text } => match Parser::parse_lexer_rule(input_text, start_index, *id, &text){
                Some(context) => vec![context],
                None => vec![],
            },
            GrammarRule::PaserRule { id, alternatives } => self.parse_parser_rule(input_text, start_index, *id, &alternatives),
        }
    }

    fn parse_lexer_rule(input_text: &str, start_index: usize, rule_id: u64, rule_text: &str) -> Option<GrammarContext>{
        if input_text[start_index..].starts_with(rule_text){
            let context = GrammarContext::LexerContext {
                start_index,
                stop_index: start_index + rule_text.len(),
                rule_id,
            };
            Some(context)
        } else {
            None
        }
    }

    fn parse_parser_rule(&self, input_text: &str, start_index: usize, rule_id: u64, alternatives: &[Vec<u64>]) -> Vec<GrammarContext>{
        alternatives.iter()
            .flat_map(|rule_id_sequence| self.parse_alternative(input_text, start_index, rule_id_sequence))
            .map(|possible_child_contexts| {
                let stop_index = match possible_child_contexts.last(){
                    Some(context) => context.stop_index(),
                    None => start_index,
                };
                GrammarContext::ParserContext {
                    start_index,
                    stop_index,
                    rule_id,
                    children: possible_child_contexts,
                }
            }).collect()
    }

    fn parse_alternative(&self, input_text: &str, start_index: usize, rule_id_sequence: &[u64]) -> Vec<Vec<GrammarContext>>{
        if rule_id_sequence.is_empty(){
            return vec![vec![]];    // An empty alternative is always a match without contexts.
        }
        let maybe_first_rule = self.grammar.get(&rule_id_sequence[0]);
        if maybe_first_rule.is_none(){
            return vec![];
        }
        let first_rule = maybe_first_rule.unwrap();
        let possible_first_rule_contexts = self.parse_rule(input_text, first_rule, start_index);
        possible_first_rule_contexts.iter()
            .map(|first_context| {
                let remaining_contexts = self.parse_alternative(input_text, first_context.stop_index(), &rule_id_sequence[1..]);
                remaining_contexts.into_iter().map(|mut contexts| {
                    let mut with_first = vec![first_context.clone()];
                    with_first.append(&mut contexts);
                    with_first
                }).collect::<Vec<Vec<GrammarContext>>>()
            }).flatten()
            .collect()
    }
}