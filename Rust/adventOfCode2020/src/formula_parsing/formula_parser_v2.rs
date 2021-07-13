use std::rc::Rc;

use super::Formula;
use super::formula_expression::{BinaryOperator, FormulaExpression};
use super::formula_lexer::{FormulaLexer, Token};


pub trait FormulaParser{
    fn parse(&self, formula_text: &Formula) -> Option<FormulaExpression>;
}

pub trait BinaryOpParsingStrategy<T>{
    fn parse_binary_op(&self, base: &RightToLeftFormulaParser<T>, operator_index: usize, right_context: FormulaContext, tokenstream: &[Token])-> Option<FormulaContext>;
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormulaContext{
    start_token_index: usize,
    stop_token_index: usize,
    expression: FormulaExpression,
}

fn generate_integer_context(index: usize, value: i128) -> FormulaContext{
    let expression = FormulaExpression::Integer(value);
    FormulaContext {
        start_token_index: index,
        stop_token_index: index,
        expression
    }
}

fn generate_parenthesized_context(inner_context: FormulaContext) -> FormulaContext{
    let parenthesized_expression = FormulaExpression::ParenthesizedExpression(Rc::new(inner_context.expression));
    FormulaContext {
        start_token_index: inner_context.start_token_index - 1,
        stop_token_index: inner_context.stop_token_index + 1,
        expression: parenthesized_expression,
    }
}

fn generate_binary_operation_context(left_context: FormulaContext, operator: BinaryOperator, right_context: FormulaContext) -> FormulaContext{
    let expression = FormulaExpression::BinaryOperation(Rc::new(left_context.expression), operator, Rc::new(right_context.expression));
    let start_token_index = left_context.start_token_index;
    let stop_token_index = right_context.stop_token_index;
    FormulaContext {start_token_index, stop_token_index, expression}
}

fn binary_operator_from_token(token: Token) -> Option<BinaryOperator>{
    match token{
        Token::Mult => Some(BinaryOperator::Mult),
        Token::Plus => Some(BinaryOperator::Plus),
        _ => None
    }
}



pub struct RightToLeftFormulaParser<T>{
    binary_op_parsing_strategy: T,
}

impl<T: BinaryOpParsingStrategy<T>> RightToLeftFormulaParser<T>{
    pub fn new(binary_op_parsing_strategy: T) -> RightToLeftFormulaParser<T>{
        RightToLeftFormulaParser {binary_op_parsing_strategy}
    }

    fn parse_tokenstream(&self, tokenstream: &[Token]) -> Option<FormulaExpression>{
        let entire_formula_context = self.parse_expression(tokenstream.len() - 1, tokenstream)?;
        Some(entire_formula_context.expression)
    }

    fn parse_expression(&self, end_index: usize, tokenstream: &[Token]) -> Option<FormulaContext>{
        let operand_context = self.parse_operand_expression(end_index, tokenstream)?;

        if operand_context.start_token_index == 0{
            return Some(operand_context);
        }

        let previous_token_index = operand_context.start_token_index - 1;

        match tokenstream[previous_token_index]{
            Token::Mult | Token::Plus => self.parse_binary_operation_expression(previous_token_index, operand_context, tokenstream),
            _ => Some(operand_context),
        }
    }

    fn parse_operand_expression(&self, end_index: usize, tokenstream: &[Token]) -> Option<FormulaContext>{
        match tokenstream[end_index]{
            Token::Integer(number) => Some(generate_integer_context(end_index, number)),
            Token::RParen => self.parse_parenthesized_expression(end_index, tokenstream),
            _ => None,
        }
    }

    fn parse_parenthesized_expression(&self, end_index: usize, tokenstream: &[Token]) -> Option<FormulaContext>{
        let inner_context = self.parse_expression(end_index - 1, tokenstream)?;
        let parenthesized_context = generate_parenthesized_context(inner_context);
        Some(parenthesized_context)
    }

    fn parse_binary_operation_expression(&self, end_index: usize, right_context: FormulaContext, tokenstream: &[Token]) -> Option<FormulaContext>{
        self.binary_op_parsing_strategy.parse_binary_op(self, end_index, right_context, tokenstream)
    }
}

impl<T: BinaryOpParsingStrategy<T>> FormulaParser for RightToLeftFormulaParser<T>{
    fn parse(&self, formula_text: &Formula) -> Option<FormulaExpression> {
        let lexer = FormulaLexer::new();
        let tokenstream = lexer.lex(formula_text)?;
        self.parse_tokenstream(&tokenstream)
    }
}


pub struct LeftToRightEvaluationBinaryOpParsingStrategy {}

impl LeftToRightEvaluationBinaryOpParsingStrategy{
    pub fn new() -> LeftToRightEvaluationBinaryOpParsingStrategy{
        LeftToRightEvaluationBinaryOpParsingStrategy {}
    }
}

impl BinaryOpParsingStrategy<Self> for LeftToRightEvaluationBinaryOpParsingStrategy{
    fn parse_binary_op(&self, base: &RightToLeftFormulaParser<Self>, operator_index: usize, right_context: FormulaContext, tokenstream: &[Token])-> Option<FormulaContext> {
        let left_context = base.parse_expression(operator_index - 1, tokenstream)?;
        let operator_token = tokenstream[operator_index];
        let operator = binary_operator_from_token(operator_token)?;
        let context = generate_binary_operation_context(left_context, operator, right_context);
        Some(context)
    }
}


pub struct PlusBeforeMultEvaluationBinaryOpParsingStrategy{}

impl PlusBeforeMultEvaluationBinaryOpParsingStrategy{
    pub fn new() -> PlusBeforeMultEvaluationBinaryOpParsingStrategy{
        PlusBeforeMultEvaluationBinaryOpParsingStrategy {}
    }
}

impl BinaryOpParsingStrategy<Self> for PlusBeforeMultEvaluationBinaryOpParsingStrategy{
    fn parse_binary_op(&self, base: &RightToLeftFormulaParser<Self>, operator_index: usize, right_context: FormulaContext, tokenstream: &[Token])-> Option<FormulaContext> {
        if operator_index == 0 || operator_index > tokenstream.len(){
            return None;
        }

        let mut current_operator_index = operator_index;
        let mut current_operator_token = tokenstream[operator_index];
        let mut current_right_context = right_context;

        while current_operator_token == Token::Plus {
            let next_operand = base.parse_operand_expression(current_operator_index - 1, tokenstream)?;
            current_right_context = generate_binary_operation_context(next_operand, BinaryOperator::Plus, current_right_context.clone());
            if current_right_context.start_token_index == 0{
                break;
            }
            current_operator_index = current_right_context.start_token_index - 1;
            current_operator_token = tokenstream[current_operator_index]
        }

        if current_right_context.start_token_index == 0
            || current_operator_token == Token::LParen{
            //We have consumed the entire (sub-)formula in the current scope defined by parenthesese.
            Some(current_right_context)
        } else if current_operator_token == Token::Mult{
            let left_context = base.parse_expression(current_operator_index - 1, tokenstream)?;
            let mult_context = generate_binary_operation_context(left_context, BinaryOperator::Mult, current_right_context);
            Some(mult_context)
        } else {
            //This only happens for malformed formulas.
            None
        }
    }
}