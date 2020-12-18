import Day from "./Day";
import * as Util from "./Util";
import * as Parser from "./FormulaParser";

class Day18 implements Day<string[]>{
    parseInput(text: string): string[] {
        return text.split(/\r?\n/);
    }

    solvePart1(formulas: string[]): string {
        const parser: Parser.FormulaParser = new Parser.LeftToRightEvaluationParser();
        const expressions: Parser.ExpressionContext[] = formulas.map((line: string) => parser.parse(line))
            .filter(Util.notEmpty);
        const values: number[] = expressions.map((expr: Parser.ExpressionContext) => this.evaluateExpression(expr));
        const result: number = values.reduce((sum: number, nextNumber: number) => sum + nextNumber, 0);
        return result.toString();
    }

    private evaluateExpression(expression: Parser.ExpressionContext): number{
        switch(expression.type){
            case Parser.ParserContextType.Integer:
                return (expression as Parser.IntegerContext).value;
            case Parser.ParserContextType.ParenthesizedExpression:
                return this.evaluateExpression((expression as Parser.ParenthesizedExpressionContext).innerExpression);
            case Parser.ParserContextType.BinaryOperation:
                return this.evaluateBinaryExpression((expression as Parser.BinaryOperationContext));
            default: //Should never happen.
                throw Error("Invalid ParserContextType: " + expression.type);
        }
    }

    private evaluateBinaryExpression(binaryOp: Parser.BinaryOperationContext): number{
        const leftValue: number = this.evaluateExpression(binaryOp.left);
        const rightValue: number = this.evaluateExpression(binaryOp.right);
        switch(binaryOp.operator){
            case '*':
                return leftValue * rightValue;
            case '+':
                return leftValue + rightValue;
            default: //Should never happen.
                throw Error("Illegal binary operator: " + binaryOp.operator);
        }
    }

    solvePart2(formulas: string[]): string {
        const parser: Parser.FormulaParser = new Parser.PlusBeforeMultParser();
        const expressions: Parser.ExpressionContext[] = formulas.map((line: string) => parser.parse(line))
            .filter(Util.notEmpty);
        const values: number[] = expressions.map((expr: Parser.ExpressionContext) => this.evaluateExpression(expr));
        const result: number = values.reduce((sum: number, nextNumber: number) => sum + nextNumber, 0);
        return result.toString();
    }

}

export default Day18;