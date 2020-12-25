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
        const numberOfCups: number = 1000000;
        const numberOfRounds: number = 10000000;
        const linkedCupList: number[] = this.linkedCupList(input, numberOfCups);
        this.playCrabGame(linkedCupList, input[0] - 1, numberOfRounds);

        const sequenceStart: number[] = [];
        let cup: number = 0;
        for(let counter: number = 0; counter < input.length; counter++){
            cup = linkedCupList[cup];
            sequenceStart.push(cup + 1);
        }
        console.log(sequenceStart.join(','));

        const nextLabel: number = linkedCupList[0] + 1;
        const secondNextLabel: number = linkedCupList[nextLabel - 1] + 1;
        const result: number = nextLabel * secondNextLabel;
        return result.toString();
    }

    /**
     * @returns Returns a list where the value at each index shows which index is next in the cup order.
     * The indeces are always one less than the corresponding labels.
     */
    private linkedCupList(startCups: number[], numberOfCups: number): number[]{
        const startNumber: number = startCups[0] - 1;
        const nextAfterStartCups: number = numberOfCups <= startCups.length
            ? startNumber
            : startCups.length;
        const linkedList: number[] = startCups.map((number: number, index: number) =>
            [
                number - 1,
                index == startCups.length - 1
                    ? nextAfterStartCups
                    : startCups[index + 1] - 1
            ]).sort(([label, ]: number[], [otherLabel, ]: number[]) => label - otherLabel)
                .map(([, nextLabel]: number[]) => nextLabel);
        if(numberOfCups > startCups.length){
            for(let furtherLabel: number = startCups.length + 1; furtherLabel < numberOfCups; furtherLabel++){
                linkedList.push(furtherLabel);
            }
            linkedList.push(startNumber);
        }
        return linkedList;
    }

    private playCrabGame(linkedCupList: number[], startCup: number, numberOfRounds: number): void{
        let currentCup: number = startCup;
        const numberOfCups: number = linkedCupList.length;
        for(let round: number = 1; round <= numberOfRounds; round++){
            currentCup = this.playCrabRound(linkedCupList, currentCup, numberOfCups);
        }
    }

    /**
     * @returns Returns the new current cup.
     */
    private playCrabRound(linkedCupList: number[], currentCup: number, numberOfCups: number): number{
        let destination: number = currentCup == 0
            ? numberOfCups - 1
            : currentCup - 1;
        const firstMovedCup: number = linkedCupList[currentCup];
        const midMovedCup: number = linkedCupList[firstMovedCup];
        const lastMovedCup: number = linkedCupList[midMovedCup];

        const nextCurrentCup: number = linkedCupList[lastMovedCup];
        linkedCupList[currentCup] = nextCurrentCup;

        while(destination == firstMovedCup
            || destination == midMovedCup
            || destination == lastMovedCup){
                destination = destination == 0
                    ? numberOfCups - 1
                    : destination - 1;
        }

        const afterDestinationCup: number = linkedCupList[destination];
        linkedCupList[destination] = firstMovedCup;
        linkedCupList[lastMovedCup] = afterDestinationCup;

        return nextCurrentCup;
    }
}


export default Day23;