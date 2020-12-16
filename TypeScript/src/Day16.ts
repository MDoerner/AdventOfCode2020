import Day from "./Day";
import * as Util from "./Util";

interface TicketRule{
    name: string;
    lowerRange: [number,number];
    higherRange: [number, number];
}

interface TicketData{
    rules: TicketRule[];
    ownTicket: number[];
    otherTickets: number[][];
}

class Day16 implements Day<TicketData>{
    parseInput(text: string): TicketData {
        const specificationParts: string[] = text.split(/\r?\n\r?\n/);
        const rules: TicketRule[] = this.parseRules(specificationParts[0]);
        const ownTicket: number[] = this.parseOwnTicket(specificationParts[1]);
        const otherTickets: number[][] = this.parseOtherTickets(specificationParts[2]);
        return {rules: rules, ownTicket: ownTicket, otherTickets: otherTickets};
    }

    private parseRules(text: string): TicketRule[]{
        return text.split(/\r?\n/)
            .map((line: string) => this.parseRule(line))
            .filter(Util.notEmpty);
    }

    private parseRule(text: string): TicketRule | null{
        const pattern: RegExp = /^((\w|\s)+): (\d+)-(\d+) or (\d+)-(\d+)$/;
        const match: RegExpMatchArray | null = text.match(pattern);
        if(!match){
            return null;
        }
        const name: string = match[1];
        const lowerRange: [number, number] = [parseInt(match[3]), parseInt(match[4])];
        const higherRange: [number, number] = [parseInt(match[5]), parseInt(match[6])];
        return {name: name, lowerRange: lowerRange, higherRange: higherRange};
    }

    private parseOwnTicket(text: string): number[]{
        const ownTicketText: string = text.split(/\r?\n/)[1];
        return this.parseTicket(ownTicketText);
    }

    private parseTicket(text: string): number[]{
        return text.split(',')
            .map((ticketItem: string) => parseInt(ticketItem));
    }

    private parseOtherTickets(text: string): number[][]{
        return text.split(/\r?\n/)
            .slice(1)
            .map((line: string) => this.parseTicket(line));
    }

    solvePart1(input: TicketData): string {
        const rules: TicketRule[] = input.rules;
        const otherTickets: number[][] = input.otherTickets;
        const invalidTicketItems: number[] = otherTickets
            .map((ticket: number[]) => this.invalidValues(ticket, rules))
            .reduce((flatInvalid: number[], currentInvalid: number[]) => flatInvalid.concat(currentInvalid));
        const invalidSum: number = invalidTicketItems.reduce((sum: number, invalid: number) => sum + invalid);
        return invalidSum.toString();
    }

    private invalidValues(ticket: number[], rules: TicketRule[]): number[]{
        return ticket.filter((ticketItem: number) => !this.isValidBySomeRule(ticketItem, rules));
    }

    private isValidBySomeRule(ticketItem: number, rules: TicketRule[]): boolean{
        for(let rule of rules){
            if(this.isValidByRule(ticketItem, rule)){
                return true;
            }
        }
        return false;
    }

    private isValidByRule(ticketItem: number, rule: TicketRule): boolean{
        return rule.lowerRange[0] <= ticketItem && ticketItem <= rule.lowerRange[1]
            || rule.higherRange[0] <= ticketItem && ticketItem <= rule.higherRange[1];
    }

    solvePart2(input: TicketData): string {
        const validOtherTickets: number[][] = input.otherTickets
            .filter((ticket: number[]) => this.isValidTicket(ticket, input.rules));
        let validRulesForItems: Set<TicketRule>[] = input.ownTicket.map((_: number) => new Set<TicketRule>(input.rules));
        this.removeInvalidRules(input.ownTicket, validRulesForItems);
        for(let ticket of validOtherTickets){
            this.removeInvalidRules(ticket, validRulesForItems);
        }
        this.reduceByUniquenessOfRulePosition(validRulesForItems);
        if(!this.isUniqueRulePerTicketItem(validRulesForItems)){
            return "No unique ruleassignment found!";
        }
        const assignedRules: TicketRule[] = validRulesForItems.reduce((rules: TicketRule[], ruleForTicket: Set<TicketRule>) => Util.concatSet(rules, ruleForTicket), []);
        const result: number = assignedRules
            .reduce((product: number, rule: TicketRule, index: number) =>
                rule.name.indexOf('departure') >= 0
                ? product * input.ownTicket[index]
                : product, 1)
        return result.toString();
    }

    private isValidTicket(ticket: number[], rules: TicketRule[]): boolean{
        return this.invalidValues(ticket, rules).length == 0;
    }

    private removeInvalidRules(ticket: number[], rulesByTicketItem: Set<TicketRule>[]): void{
        for(let index in ticket){
            this.removeRulesInvalidForTicketItem(ticket[index], rulesByTicketItem[index]);
        }
    }

    private removeRulesInvalidForTicketItem(ticketItem: number, rules: Set<TicketRule>): void{
        let invalidRules: TicketRule[] = [];
        for(let rule of rules){
            if(!this.isValidByRule(ticketItem, rule)){
                invalidRules.push(rule);
            }
        }
        for(let rule of invalidRules){
            rules.delete(rule);
        }
    }

    private reduceByUniquenessOfRulePosition(rulesByTicketItem: Set<TicketRule>[]): void{
        let processedIndices: Set<number> = new Set<number>();
        let continueToSearch: boolean = true;
        while(continueToSearch){
            continueToSearch = false;
            for(let index: number = 0; index < rulesByTicketItem.length; index++){
                if(rulesByTicketItem[index].size == 1 && !processedIndices.has(index)){
                    //ForEach because it is inconvenient to extract the single element of a set.
                    rulesByTicketItem[index].forEach((rule: TicketRule) => this.removeRule(rule, index, rulesByTicketItem));
                    processedIndices.add(index);
                    continueToSearch = true;
                }
            }
        }
    }

    private removeRule(rule: TicketRule, indexToSkip: number, rulesByTicketItem: Set<TicketRule>[]): void{
        for(let index: number = 0; index < rulesByTicketItem.length; index++){
            if(index != indexToSkip && rulesByTicketItem[index].has(rule)){
                rulesByTicketItem[index].delete(rule);
            }
        }
    }

    private isUniqueRulePerTicketItem(rulesByTicketItem: Set<TicketRule>[]){
        for(let rules of rulesByTicketItem){
            if(rules.size != 1){
                return false;
            }
        }
        return true;
    }
}


export default Day16;