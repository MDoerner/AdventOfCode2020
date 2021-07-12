use std::rc::Rc;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinaryOperator{
    Plus,
    Mult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormulaExpression{
    Integer(i128),
    BinaryOperation(Rc<FormulaExpression>, BinaryOperator, Rc<FormulaExpression>),
    ParenthesizedExpression(Rc<FormulaExpression>),
}

impl FormulaExpression{
    pub fn evaluate(&self) -> i128{
        match self{
            FormulaExpression::Integer(number) => *number,
            FormulaExpression::BinaryOperation(left, op, right) => {
                let left_result = left.evaluate();
                let right_result = right.evaluate();
                match op{
                    BinaryOperator::Plus => left_result + right_result,
                    BinaryOperator::Mult => left_result * right_result,
                }
            },
            FormulaExpression::ParenthesizedExpression(inner) => inner.evaluate(),
        }
    }
}