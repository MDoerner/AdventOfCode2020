export enum TokenType{
    Plus,
    Mult,
    LParen,
    RParen,
    Integer,
}

export interface Token{
    type: TokenType;
    text: string;
}

class FormulaLexer{
    tokenStream(text: string): Token[]{
        let tokens: Token[] = [];
        let currentDigits: string[] = [];
        for(let character of Array.from(text)){
            if(/\d/.test(character)){
                currentDigits.push(character);
            } else {
                if(currentDigits.length > 0){
                    tokens.push({type: TokenType.Integer, text: currentDigits.join()});
                    currentDigits = [];
                }
                switch(character){
                    case '*':
                        tokens.push({type: TokenType.Mult, text: character});
                        break;
                    case '+':
                        tokens.push({type: TokenType.Plus, text: character});
                        break;
                    case '(':
                        tokens.push({type: TokenType.LParen, text: character});
                        break;
                    case ')':
                        tokens.push({type: TokenType.RParen, text: character});
                        break;
                }
            }
        }
        if(currentDigits.length > 0){
            tokens.push({type: TokenType.Integer, text: currentDigits.join()});
        }
        return tokens;
    }
}

export enum ParserContextType{
    Expression = 1 << 0,
    BinaryOperation = Expression | 1 << 1,
    Integer = Expression | 1 << 2,
    ParenthesizedExpression = Expression | 1 << 3,
}

export class ParserContext{
    type: ParserContextType;
    startIndex: number;
    stopIndex: number;
    tokenStream: Token[];

    constructor(type: ParserContextType, startIndex: number, stopIndex: number, tokenStream: Token[]){
        this.type = type;
        this.startIndex = startIndex;
        this.stopIndex = stopIndex;
        this.tokenStream = tokenStream;
    }

    getText(): string{
        return this.tokenStream.slice(this.startIndex, this.stopIndex)
            .map((token: Token) => token.text)
            .join();
    }
}

export class ExpressionContext extends ParserContext{
    constructor(startIndex: number, stopIndex: number, tokenStream: Token[], expressionType: ParserContextType = ParserContextType.Expression){
        super(expressionType, startIndex, stopIndex, tokenStream);
    }
}

export class IntegerContext extends ExpressionContext{
    readonly value: number;

    constructor(integerTokenIndex: number, tokenStream: Token[]){
        super(integerTokenIndex, integerTokenIndex + 1, tokenStream, ParserContextType.Integer);
        this.value = parseInt(tokenStream[integerTokenIndex].text);
    }
}

export class BinaryOperationContext extends ExpressionContext{
    readonly operator: string;
    readonly left: ExpressionContext;
    readonly right: ExpressionContext;

    constructor(operatorIndex: number, left: ExpressionContext, right: ExpressionContext, tokenStream: Token[]){
        super(left.startIndex, right.stopIndex, tokenStream, ParserContextType.BinaryOperation);
        this.operator = tokenStream[operatorIndex].text;
        this.left = left;
        this.right = right;
    }
}

export class ParenthesizedExpressionContext extends ExpressionContext{
    readonly innerExpression: ExpressionContext;

    constructor(innerExpression: ExpressionContext, tokenStream: Token[]){
        super(innerExpression.startIndex - 1, innerExpression.stopIndex + 1, tokenStream, ParserContextType.ParenthesizedExpression);
        this.innerExpression = innerExpression;
    }
}

export interface FormulaParser{
    parse(text: string): ExpressionContext | null;
}

abstract class FormulaParserBase implements FormulaParser{
    parse(text: string): ExpressionContext | null{
        const tokenStream: Token[] = (new FormulaLexer()).tokenStream(text);
        return this.parseTokenStream(tokenStream);
    }

    private parseTokenStream(tokenStream: Token[]): ExpressionContext | null{
        return this.parseExpression(tokenStream.length-1, tokenStream);
    }

    protected parseExpression(endIndex: number, tokenStream: Token[]): ExpressionContext | null{
        let operandExpression: ExpressionContext | null = this.parseOperandExpression(endIndex, tokenStream);

        if(operandExpression == null){
            return null;
        }

        const previousTokenIndex: number = operandExpression.startIndex - 1;

        if(previousTokenIndex < 0){
            return operandExpression;
        }

        const previousToken: Token = tokenStream[previousTokenIndex];
        switch(previousToken.type){
            case TokenType.Mult:
            case TokenType.Plus:
                return this.parseBinaryOperation(previousTokenIndex, operandExpression, tokenStream);
            default:
                return operandExpression;
        }
    }

    protected parseOperandExpression(endIndex: number, tokenStream: Token[]): ExpressionContext | null{
        const endToken: Token = tokenStream[endIndex];
        switch(endToken.type){
            case TokenType.Integer:
                return new IntegerContext(endIndex, tokenStream);
            case TokenType.RParen:
                return this.parseParenthesizedExpression(endIndex, tokenStream);
            default:
                return null;
        }
    }

    protected parseParenthesizedExpression(endIndex: number, tokenStream: Token[]) : ParenthesizedExpressionContext | null{
        const innerExpression: ExpressionContext | null = this.parseExpression(endIndex - 1, tokenStream);
        return innerExpression == null ? null : new ParenthesizedExpressionContext(innerExpression, tokenStream);
    }

    protected parseBinaryOperation(operatorIndex: number, right: ExpressionContext, tokenStream: Token[]): BinaryOperationContext | null{
        const left: ExpressionContext | null = this.parseExpression(operatorIndex - 1, tokenStream);
        return left == null ? null : new BinaryOperationContext(operatorIndex, left, right, tokenStream);
    }
}

export class LeftToRightEvaluationParser extends FormulaParserBase{
    protected parseBinaryOperation(operatorIndex: number, right: ExpressionContext, tokenStream: Token[]): BinaryOperationContext | null{
        const left: ExpressionContext | null = this.parseExpression(operatorIndex - 1, tokenStream);
        return left == null ? null : new BinaryOperationContext(operatorIndex, left, right, tokenStream);
    }
}

export class PlusBeforeMultParser extends FormulaParserBase{
    protected parseBinaryOperation(operatorIndex: number, rightOperand: ExpressionContext, tokenStream: Token[]): BinaryOperationContext | null{
        let currentOperatorIndex: number = operatorIndex;
        let operator: Token = tokenStream[operatorIndex];
        let right: ExpressionContext | null = rightOperand;
        while(currentOperatorIndex >= 0 && operator.type == TokenType.Plus){
            let nextOperand: ExpressionContext | null = this.parseOperandExpression(currentOperatorIndex - 1, tokenStream);
            if(nextOperand == null){
                return null;
            }
            right = new BinaryOperationContext(currentOperatorIndex, nextOperand, right, tokenStream);
            if(right == null){
                return null;
            }
            currentOperatorIndex = right.startIndex - 1;
            operator = tokenStream[currentOperatorIndex];
        }

        if(currentOperatorIndex < 0 || operator.type != TokenType.Mult){
            return right as BinaryOperationContext;
        }

        const left: ExpressionContext | null = this.parseExpression(currentOperatorIndex - 1, tokenStream);
        return left == null ? null : new BinaryOperationContext(currentOperatorIndex, left, right, tokenStream);
    }
}