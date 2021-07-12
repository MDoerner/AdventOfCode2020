use super::Formula;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Token{
    Plus,
    Mult,
    LParen,
    RParen,
    Integer(i128),
}

pub struct FormulaLexer{}

impl FormulaLexer{
    pub fn new() -> FormulaLexer{
        FormulaLexer {}
    }

    pub fn lex(&self, formula_text: &Formula)-> Option<Vec<Token>>{
        let mut tokens = vec![];
        let mut current_digits = vec![];
        let characters = formula_text.0.chars();
        for c in characters{
            if c.is_ascii_digit(){
                current_digits.push(c);
            } else {
                if !current_digits.is_empty(){
                    let number_text: String = current_digits.into_iter().collect();
                    let number = number_text.parse::<i128>().ok()?;
                    tokens.push(Token::Integer(number));
                    current_digits = vec![];
                }
                match c {
                    '*' => tokens.push(Token::Mult),
                    '+' => tokens.push(Token::Plus),
                    '(' => tokens.push(Token::LParen),
                    ')' => tokens.push(Token::RParen),
                    _ => {},
                }
            }
        }
        if !current_digits.is_empty(){
            let number_text: String = current_digits.into_iter().collect();
            let number = number_text.parse::<i128>().ok()?;
            tokens.push(Token::Integer(number));
        }
        Some(tokens)
    }
}