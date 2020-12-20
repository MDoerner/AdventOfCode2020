import Day from "./Day";
import * as Parser from "./ParserGenerator";


interface MessageData{
    grammarText: string;
    messages: string[];
}

class Day19 implements Day<MessageData>{
    parseInput(text: string): MessageData {
        const parts: string[] = text.split(/\r?\n\r?\n/);
        const messages: string[] = parts[1].split(/\r?\n/);
        return {grammarText: parts[0], messages: messages};
    }

    solvePart1(input: MessageData): string {
        const grammarParser: Parser.GrammarParser = new Parser.GrammarParser();
        const grammar: Parser.Grammar = grammarParser.parse(input.grammarText);
        const parser: Parser.Parser = new Parser.Parser();
        const validMessages: string[] = input.messages
            .filter((message: string) => parser.parse(message, grammar, "0").length > 0);
        const result = validMessages.length;
        return result.toString();
    }

    solvePart2(input: MessageData): string {
        const ruleCorrections: string = `8: 42 | 42 8
11: 42 31 | 42 11 31`;
        const grammarParser: Parser.GrammarParser = new Parser.GrammarParser();
        const grammar: Parser.Grammar = grammarParser.parse(input.grammarText, ruleCorrections);
        const parser: Parser.Parser = new Parser.Parser();
        const validMessages: string[] = input.messages
            .filter((message: string) => parser.parse(message, grammar, "0").length > 0);
        const result = validMessages.length;
        return result.toString();
    }

}



export default Day19;