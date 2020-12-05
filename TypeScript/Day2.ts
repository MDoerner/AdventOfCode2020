import Day from "./Day";

interface PasswordRule{
    character: string
    minNumber: number
    maxNumber: number
}

interface PasswordData{
    rule: PasswordRule
    password: string
}

class Day2 implements Day<(PasswordData | null)[]>{
    constructor(){}

    parseInput(input: string): (PasswordData | null)[]{
        return input.split(/\r?\n/).map((s: string) => this.parsePasswordData(s));
    }

    private parsePasswordData(line: string): PasswordData | null{
        let parts = line.match(/(\d+)-(\d+) (\w): (.*)/);
        if(!parts){
            return null;
        }
        let rule: PasswordRule = {character: parts[3], minNumber: parseInt(parts[1]), maxNumber: parseInt(parts[2]) };
        return {rule: rule, password: parts[4]};
    }

    solvePart1(input: (PasswordData | null)[]): string{
        return input
        .filter((data: PasswordData | null) => data && this.validateSledPasswordData(data))
        .length
        .toString();
    }

    private validateSledPasswordData(data: PasswordData): boolean{
        let rule: PasswordRule = data.rule;
        let occurrenceCount: number = this.countOccurences(data.password, rule.character);
        return occurrenceCount >= rule.minNumber
            && occurrenceCount <= rule.maxNumber;
    }

    private countOccurences(toEvaluate: string, character: string){
        let count: number = 0;
        for(let index: number = 0; index < toEvaluate.length; index++){
            if(toEvaluate.charAt(index) == character){
                count++;
            }
        }
        return count;
    }

    solvePart2(input: (PasswordData | null)[]): string{
        return input
        .filter((data: PasswordData | null) => data && this.validateTobogganPasswordData(data))
        .length
        .toString();
    }

    private validateTobogganPasswordData(data: PasswordData): boolean{
        let rule: PasswordRule = data.rule;
        let password: string = data.password;
        return password.charAt(rule.minNumber-1) == rule.character && password.charAt(rule.maxNumber-1) != rule.character
            || password.charAt(rule.minNumber-1) != rule.character && password.charAt(rule.maxNumber-1) == rule.character;
    }
}


export default Day2;