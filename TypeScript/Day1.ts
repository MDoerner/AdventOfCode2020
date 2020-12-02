import Day from "./Day";

class Day1 implements Day<number[]>{
    constructor(){}

    parseInput(input: string): number[]{
        return input.split(/\r?\n/).map((s: string) => parseInt(s));
    }

    solvePart1(input: number[]): string{
        let summingValues: [number, number] = this.findSummingPair(input, 2020);
        return (summingValues[0] * summingValues[1]).toString();
    }

    private findSummingPair(input: number[], desiredSum: number): [number, number]{
        let sortedInput: number[] = input.sort((m: number, n: number) => m - n);
        return this.findSummingPairInSorted(sortedInput, desiredSum);
    }

    private findSummingPairInSorted(sortedInput: number[], desiredSum: number): [number, number]{
        if(sortedInput.length < 2){
            return null;
        }
        
        let lowerIndex: number = 0;
        let higherIndex: number = sortedInput.length - 1;
        let currentLowerValue = sortedInput[lowerIndex];
        let currentHigherValue = sortedInput[higherIndex];
        while(higherIndex > lowerIndex){
            let currentSum = currentLowerValue + currentHigherValue
            if(currentSum > desiredSum){
                higherIndex -= 1;
                currentHigherValue = sortedInput[higherIndex];
            } else if (currentSum < desiredSum){
                lowerIndex += 1;
                currentLowerValue = sortedInput[lowerIndex];
            } else {
                return [currentLowerValue, currentHigherValue];
            }
        }
        
        return null;
    }

    solvePart2(input: number[]): string{
        let summingValues: [number, number, number] = this.findSummingTriple(input, 2020);
        return (summingValues[0] * summingValues[1] * summingValues[2]).toString();
    }

    private findSummingTriple(input: number[], desiredSum: number): [number, number, number]{
        let sortedInput: number[] = input.sort((m: number, n: number) => m - n);
        return this.findSummingTripleInSorted(sortedInput, desiredSum);
    }

    private findSummingTripleInSorted(sortedInput: number[], desiredSum: number): [number, number, number]{
        if(sortedInput.length < 3){
            return null;
        }
        
        let remainaingCandidates: number[] = sortedInput;

        while(remainaingCandidates.length >= 3){
            let lowestValue = remainaingCandidates[0];
            remainaingCandidates = remainaingCandidates.slice(1);

            let otherValues: [number, number] = this.findSummingPairInSorted(remainaingCandidates, desiredSum - lowestValue);

            if(otherValues){
                return [lowestValue, otherValues[0], otherValues[1]];
            }
        }
        
        return null;
    }
}


export default Day1;