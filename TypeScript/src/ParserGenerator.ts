import { notEmpty } from "./Util";

export enum RuleType{
    Lexer,
    Parser,
}

export interface GrammarRule{
    readonly name: string;
    readonly type: RuleType;
}

export class LexerRule implements GrammarRule{
    readonly name: string;
    readonly type: RuleType;

    readonly text: string;

    constructor(name: string, text:string){
        this.type = RuleType.Lexer;
        this.name = name;
        this.text = text;
    }
}

export class ParserRule implements GrammarRule{
    readonly name: string;
    readonly type: RuleType;

    readonly alternatives: [RuleType, number][][];

    constructor(name: string, alternatives: [RuleType, number][][]){
        this.type = RuleType.Parser;
        this.name = name;
        this.alternatives = alternatives;
    }
}

class PreParserRule implements GrammarRule{
    readonly name: string;
    readonly type: RuleType;

    readonly alternativeNames: string[][];

    constructor(name: string, alternativeNames: string[][]){
        this.type = RuleType.Parser;
        this.name = name;
        this.alternativeNames = alternativeNames;
    }
}

export interface Grammar{
    lexerRules: LexerRule[];
    parserRules: ParserRule[];
    lexerRuleIndeces: Map<string,number>;
    parserRuleIndeces: Map<string,number>;
}

export class GrammarParser{
    /**
     * Parses a multi-line input into a parser grammar.
     *
     * @param text Text representing a number of grammar rules. 
     * Each line represents a rule of one of the following two forms:
     *
     * Lexer rule: <rule name>: "<matched text>"
     * Parser rule: <rule name>: (<referenced rule name>+ )( | <referenced rule name>)*
     *
     * Rule names can contain any character except ':', '"' and whitespace.
     * Matched text can contain any printable character.
     *
     * Malformed rules are ignored.
     * Referenced rule names must exist. Otherwise, an error is thrown.
     *
     * @param ruleCorrections Additional text containing rules to overwrite the ones in the main input.
     *
     * @returns A grammar containing the rules specified in the argument.
     */
    parse(text: string, ruleCorrections: string = ""): Grammar{
        let lexerRules: LexerRule[] = [];
        let preParserRules: PreParserRule[] = [];
        const lexerRuleIndeces: Map<string, number> = new Map<string, number>();
        const parserRuleIndeces: Map<string, number> = new Map<string, number>();
        let nextLexerRuleIndex: number = 0;
        let nextParserRuleIndex: number = 0;
        const overwriteRules: Map<string, (LexerRule | PreParserRule)> = this.parseOverwritePreRules(ruleCorrections);
        const baseRules: (LexerRule | PreParserRule)[] = this.parsePreRules(text);
        for(let rule of baseRules){
            if(lexerRuleIndeces.has(rule.name) || parserRuleIndeces.has(rule.name)){
                throw Error("Duplicate grammar rule: " + rule.name);
            }
            if(overwriteRules.has(rule.name)){
                rule = overwriteRules.get(rule.name) as (LexerRule | PreParserRule);
            }
            switch(rule.type){
                case RuleType.Lexer:
                    lexerRules.push(rule as LexerRule);
                    lexerRuleIndeces.set(rule.name, nextLexerRuleIndex);
                    nextLexerRuleIndex++;
                    break;
                case RuleType.Parser:
                    preParserRules.push(rule as PreParserRule);
                    parserRuleIndeces.set(rule.name, nextParserRuleIndex);
                    nextParserRuleIndex++;
                    break;
                default:
                    throw Error("Unknown rule type: " + rule.type);
            }
        }
        const parserRules: ParserRule[] = preParserRules.map((rule: PreParserRule) => this.resolvePreParserRule(rule, lexerRuleIndeces, parserRuleIndeces));
        return {lexerRules: lexerRules, parserRules: parserRules, lexerRuleIndeces: lexerRuleIndeces, parserRuleIndeces: parserRuleIndeces};
    }

    private parseOverwritePreRules(text: string): Map<string, (LexerRule | PreParserRule)>{
        const preRuleMap: Map<string, (LexerRule | PreParserRule)> = new Map<string, (LexerRule | PreParserRule)>();
        const preRules: (LexerRule | PreParserRule)[] = this.parsePreRules(text);
        for(let rule of preRules){
            if(preRuleMap.has(rule.name)){
                throw Error("Duplicate overwrite rule: " + rule.name);
            }
            preRuleMap.set(rule.name, rule);
        }
        return preRuleMap;
    }

    private parsePreRules(text:string): (LexerRule | PreParserRule)[]{
        return text.split(/\r?\n/)
        .map((line: string) => this.parseGrammarRule(line))
        .filter(notEmpty);
    }

    private parseGrammarRule(text: string): LexerRule | PreParserRule | null{
        const pattern: RegExp = /^([^:"\s]+): ("(.+)"|((([^:"\s]+\s*)+\|?)+))$/;
        const match: RegExpMatchArray | null = text.match(pattern);
        if(match == null){
            return null;
        }
        const ruleName: string = match[1];
        const maybeTokenText: string | undefined = match[3];
        if(maybeTokenText != undefined){
            return new LexerRule(ruleName, maybeTokenText);
        }
        const alternativesText: string = match[4];
        const alternatives: string[][] = alternativesText.split('|')
            .map((s: string) => s.trim().split(/\s+/));
        return new PreParserRule(ruleName, alternatives);
    }

    private resolvePreParserRule(rule: PreParserRule, lexerRuleIndeces: Map<string, number>, parserRuleIndeces: Map<string, number>): ParserRule{
        const alternatives: [RuleType, number][][] = rule.alternativeNames
            .map((names: string[]) => names.map((ruleName: string) => this.resolveRule(ruleName, lexerRuleIndeces, parserRuleIndeces)));
        return new ParserRule(rule.name, alternatives);
    }

    private resolveRule(ruleName: string, lexerRuleIndeces: Map<string, number>, parserRuleIndeces: Map<string, number>): [RuleType, number]{
        if(lexerRuleIndeces.has(ruleName)){
            return [RuleType.Lexer, lexerRuleIndeces.get(ruleName) as number];
        } else if(parserRuleIndeces.has(ruleName)){
            return [RuleType.Parser, parserRuleIndeces.get(ruleName) as number];
        } else {
            throw Error("Unknown rule encountered: " + ruleName);
        }
    }
}

export interface RuleContext{
    readonly startIndex: number;
    readonly stopIndex: number;
    readonly type: RuleType;
}

export class LexerRuleContext implements RuleContext{
    readonly startIndex: number;
    readonly stopIndex: number;
    readonly type: RuleType;
    readonly rule: LexerRule;

    constructor(rule: LexerRule, tokenIndex: number){
        this.type = RuleType.Lexer;
        this.startIndex = tokenIndex;
        this.stopIndex = tokenIndex + 1;
        this.rule = rule;
    }
}

export class ParserRuleContext implements RuleContext{
    readonly startIndex: number;
    readonly stopIndex: number;
    readonly type: RuleType;
    readonly rule: ParserRule;
    readonly children: RuleContext[];

    constructor(rule: ParserRule, children: RuleContext[]){
        this.type = RuleType.Parser;
        this.rule = rule;
        this.children = children;
        this.startIndex = children[0].startIndex;
        this.stopIndex = children[children.length - 1].stopIndex;
    }
}

/**
 * Parses an input text based on a grammar starting with a specified parser rule.
 *
 * @param text Text to parse into parse trees.
 * @param grammar Must not contain left-recursive rules. Otherwise, the behaviour is undefined.
 * @param startRule Must be the name of a parser rule specified in the grammar;
 * @returns All possible parse trees representing the entire input.
 */
export class Parser{
    parse(text: string, grammar: Grammar, startRule: string): ParserRuleContext[]{
        const lexer: Lexer = new Lexer();
        const tokenStream: number[] | null = lexer.tokenize(text, grammar);
        if(tokenStream == null || !grammar.parserRuleIndeces.has(startRule)){
            return [];
        }
        let startRuleIndex: number = grammar.parserRuleIndeces.get(startRule) as number;
        return this.parseTokenStream(tokenStream, grammar, startRuleIndex)
            .filter((context: ParserRuleContext) => context.stopIndex == tokenStream.length);
    }

    private parseTokenStream(tokenStream: number[], grammar: Grammar, startRule: number): ParserRuleContext[]{
        return this.parseRule(grammar.parserRules[startRule], 0, tokenStream, grammar);
    }

    private parseRule(rule: ParserRule, startIndex: number, tokenStream: number[], grammar: Grammar): ParserRuleContext[]{
        return rule.alternatives
            .map((alternative: [RuleType, number][]) => this.parseAlternative(alternative, startIndex, tokenStream, grammar)
                .map((resolvedAlternative: RuleContext[]) => new ParserRuleContext(rule, resolvedAlternative)))
            .reduce((possibleResults: ParserRuleContext[], possibleResultsForNextAlternative: ParserRuleContext[]) => possibleResults.concat(possibleResultsForNextAlternative), []);
    }

    private parseAlternative(alternative: [RuleType, number][] , startIndex: number, tokenStream: number[], grammar: Grammar): RuleContext[][]{
        if(alternative.length == 0){
            return [];
        }
        let firstElementResultContexts: RuleContext[] | null = this.parseAlternativeElement(alternative[0], startIndex, tokenStream, grammar);
        if(alternative.length == 1){
            return firstElementResultContexts.map((context: RuleContext) => [context]);
        }
        let resultContexts: RuleContext[][] = [];
        for(let firstContext of firstElementResultContexts){
            const remainingResults: RuleContext[][] = this.parseAlternative(alternative.slice(1), firstContext.stopIndex, tokenStream, grammar);
            const resultsWithFirstElement: RuleContext[][] = remainingResults.map((remainingResult: RuleContext[]) => [firstContext].concat(remainingResult));
            resultContexts = resultContexts.concat(resultsWithFirstElement);
        }
        return resultContexts;
    }

    private parseAlternativeElement(alternativeElement: [RuleType, number] , startIndex: number, tokenStream: number[], grammar: Grammar): RuleContext[]{
        const ruleType: RuleType = alternativeElement[0];
        switch(ruleType){
            case RuleType.Lexer:
                const lexerRuleIndex = alternativeElement[1];
                if(lexerRuleIndex != tokenStream[startIndex]){
                    return [];
                }
                return [new LexerRuleContext(grammar.lexerRules[lexerRuleIndex], startIndex)];
            case RuleType.Parser:
                const parserRuleIndex: number = alternativeElement[1];
                return this.parseRule(grammar.parserRules[parserRuleIndex], startIndex, tokenStream, grammar);
            default:
                throw Error("Unknown rule type: " + ruleType);
        }
    }
}

class Lexer{
    tokenize(text:string, grammar: Grammar): number[] | null{
        let tokenStream: number[] = [];
        let remainingString: string = text;
        while(remainingString != ""){
            let foundMatch: boolean = false;
            for(let ruleIndex: number = 0; ruleIndex < grammar.lexerRules.length; ruleIndex++){
                const ruleText: string = grammar.lexerRules[ruleIndex].text;
                if(remainingString.startsWith(ruleText)){
                    tokenStream.push(ruleIndex);
                    remainingString = remainingString.slice(ruleText.length);
                    foundMatch = true;
                    break;
                }
            }
            if(!foundMatch){
                //Cannot tokenize because of unknown text.
                return null;
            }
        }
        return tokenStream;
    }
}
