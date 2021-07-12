use std::cell::{RefCell, RefMut};
use std::rc::{Rc, Weak};

use super::Formula;
use super::formula_expression::{BinaryOperator, FormulaExpression};
use super::formula_lexer::{FormulaLexer, Token};


pub trait FormulaParser{
    fn parse(&self, formula_text: &Formula) -> Option<FormulaExpression>;
}

pub trait BinaryOpParsingStrategy{
    fn parse_binary_op(&self, operator_index: usize, right_context: FormulaContext, tokenstream: &[Token])-> Option<FormulaContext>;
}

pub trait BaseOnitializable<T>{
    fn initialize_base(&mut self, base: Weak<RefCell<T>>);
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


pub struct RightToLeftParser<T>{
    inner_parser: Rc<RefCell<RightToLeftFormulaParser<T>>>,
}

impl<T: BinaryOpParsingStrategy + BaseOnitializable<RightToLeftFormulaParser<T>>> RightToLeftParser<T>{
    pub fn new(binary_op_parsing_strategy: T) -> RightToLeftParser<T>{
        let inner_parser = RightToLeftFormulaParser::new(binary_op_parsing_strategy);
        RightToLeftParser {inner_parser}
    }
}

impl<T: BinaryOpParsingStrategy + BaseOnitializable<RightToLeftFormulaParser<T>>> FormulaParser for RightToLeftParser<T>{
    fn parse(&self, formula_text: &Formula) -> Option<FormulaExpression> {
        let inner = self.inner_parser.borrow();
        inner.parse(formula_text)
    }
}




pub struct RightToLeftFormulaParser<T>{
    binary_op_parsing_strategy: T,
}

impl<T: BinaryOpParsingStrategy + BaseOnitializable<Self>> RightToLeftFormulaParser<T>{
    fn new(binary_op_parsing_strategy: T) -> Rc<RefCell<RightToLeftFormulaParser<T>>>{
        let parser = RightToLeftFormulaParser {binary_op_parsing_strategy};
        let parser_cell = RefCell::new(parser);
        let parser_wrapper = Rc::new(parser_cell);
        let reference_for_strategy = Rc::downgrade(&parser_wrapper);
        {
            let mut strategy: RefMut<_> = parser_wrapper.borrow_mut();
            strategy.binary_op_parsing_strategy.initialize_base(reference_for_strategy);
        }
        parser_wrapper
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
        self.binary_op_parsing_strategy.parse_binary_op(end_index, right_context, tokenstream)
    }
}

impl<T: BinaryOpParsingStrategy + BaseOnitializable<Self>> FormulaParser for RightToLeftFormulaParser<T>{
    fn parse(&self, formula_text: &Formula) -> Option<FormulaExpression> {
        let lexer = FormulaLexer::new();
        let tokenstream = lexer.lex(formula_text)?;
        self.parse_tokenstream(&tokenstream)
    }
}


pub struct LeftToRightEvaluationBinaryOpParsingStrategy{
    base_parser: Option<Weak<RefCell<RightToLeftFormulaParser<Self>>>>,
}

impl LeftToRightEvaluationBinaryOpParsingStrategy{
    pub fn new() -> LeftToRightEvaluationBinaryOpParsingStrategy{
        LeftToRightEvaluationBinaryOpParsingStrategy {base_parser: None}
    }
}

impl BaseOnitializable<RightToLeftFormulaParser<Self>> for LeftToRightEvaluationBinaryOpParsingStrategy{
    fn initialize_base(&mut self, base: Weak<RefCell<RightToLeftFormulaParser<Self>>>) {
        self.base_parser = Some(base)
    }
}

impl BinaryOpParsingStrategy for LeftToRightEvaluationBinaryOpParsingStrategy{
    fn parse_binary_op(&self, operator_index: usize, right_context: FormulaContext, tokenstream: &[Token])-> Option<FormulaContext> {
        let base_wrapper = self.base_parser.as_ref()?;
        let base_cell = base_wrapper.upgrade()?;
        let base = base_cell.borrow();
        let left_context = base.parse_expression(operator_index - 1, tokenstream)?;
        let operator_token = tokenstream[operator_index];
        let operator = binary_operator_from_token(operator_token)?;
        let context = generate_binary_operation_context(left_context, operator, right_context);
        Some(context)
    }
}


pub struct PlusBeforeMultEvaluationBinaryOpParsingStrategy{
    base_parser: Option<Weak<RefCell<RightToLeftFormulaParser<Self>>>>,
}

impl PlusBeforeMultEvaluationBinaryOpParsingStrategy{
    pub fn new() -> PlusBeforeMultEvaluationBinaryOpParsingStrategy{
        PlusBeforeMultEvaluationBinaryOpParsingStrategy {base_parser: None}
    }
}

impl BaseOnitializable<RightToLeftFormulaParser<Self>> for PlusBeforeMultEvaluationBinaryOpParsingStrategy{
    fn initialize_base(&mut self, base: Weak<RefCell<RightToLeftFormulaParser<Self>>>) {
        self.base_parser = Some(base)
    }
}

impl BinaryOpParsingStrategy for PlusBeforeMultEvaluationBinaryOpParsingStrategy{
    fn parse_binary_op(&self, operator_index: usize, right_context: FormulaContext, tokenstream: &[Token])-> Option<FormulaContext> {
        if operator_index == 0 || operator_index > tokenstream.len(){
            return None;
        }

        let base_wrapper = self.base_parser.as_ref()?;
        let base_cell = base_wrapper.upgrade()?;
        let base = base_cell.borrow();

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