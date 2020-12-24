import Day from "./Day";
import * as Util from "./Util";


class Day23 implements Day<number[]>{
    parseInput(text: string): number[] {
        return Array.from(text).map((digit: string) => parseInt(digit));
    }

    solvePart1(input: number[]): string {
        const resultSequence: number[] = this.playGame(input, 100);
        const result: string = this.sequenceString(resultSequence).map((digit: number) => digit.toString()).join('');
        return result.toString();
    }

    private playGame(digits: number[], numberOfRounds: number): number[]{
        let currentSequence: number[] = digits.map((digit: number) => digit - 1);
        for(let round: number = 1; round <= numberOfRounds; round++){
            currentSequence = this.playRound(currentSequence);
        }
        return currentSequence.map((digit: number) => digit + 1);
    }

    //We always shift the new current position to the start.
    private playRound(digits: number[]): number[]{
        const extractedDigits: number[] = digits.slice(1, 4);
        const destination: number = this.destination(digits);
        if(destination == 1){
            return digits.slice(1).concat(digits[0]);
        }
        return digits.slice(4, destination + 1)
            .concat(extractedDigits)
            .concat(digits.slice(destination + 1))
            .concat(digits[0]);
    }

    private destination(digits: number[]): number{
        let destinationDigit: number = Util.moduloP(digits[0] - 1, digits.length);
        let destination: number = digits.indexOf(destinationDigit);
        while(1 <= destination && destination <= 3){
            destinationDigit = Util.moduloP(destinationDigit - 1, digits.length);
            destination = digits.indexOf(destinationDigit);
        }
        return destination;
    }

    private sequenceString(digits: number[]): number[]{
        const startIndex: number = digits.indexOf(1);
        return digits.slice(startIndex + 1).concat(digits.slice(0, startIndex));
    }

    solvePart2(input: number[]): string {
        throw new Error("Method not implemented.");
    }
}


export default Day23;