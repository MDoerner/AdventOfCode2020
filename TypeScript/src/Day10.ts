import Day from "./Day";
import * as Util from "./Util";


class Day10 implements Day<number[]>{
    parseInput(text: string): number[] {
        return text.split(/\r?\n/)
            .map((s: string) => parseInt(s));
    }

    solvePart1(input: number[]): string {
        let sortedInput: number[] = input.sort((m: number, n: number) => m - n);
        let joltageGapDistribution: Map<number,number> = this.joltageGapDistribution(sortedInput);
        let oneCount = Util.zeroIfInvalid(joltageGapDistribution.get(1));
        let threeCount = Util.zeroIfInvalid(joltageGapDistribution.get(3));
        return (oneCount * threeCount).toString();
    }

    private joltageGapDistribution(sortedAdapters: number[]): Map<number,number>{
        let distribution: Map<number,number> = new Map<number,number>();
        for(let index = 1; index < sortedAdapters.length; index++){
            let joltageGap: number = sortedAdapters[index] - sortedAdapters[index - 1];
            Util.addCount(distribution, joltageGap, 1);
        }
        Util.addCount(distribution, sortedAdapters[0], 1);
        Util.addCount(distribution, 3, 1);
        return distribution;
    }



    solvePart2(input: number[]): string {
        let sortedInput: number[] = input.sort((m: number, n: number) => m - n);
        let numberOfPossibilities = this.numberOfPossibleAdapterCombinations(sortedInput);
        return numberOfPossibilities.toString();
    }

    private numberOfPossibleAdapterCombinations(sortedAdapters: number[]) : number{
        if(sortedAdapters.length <= 0){
            return 0;
        }

        let combinationsByPreviousGap: [number, number, number] = [1, 1, 1];
        let index: number = sortedAdapters.length - 2;
        let currentGap: number;
        while(index >= 0){
            currentGap = sortedAdapters[index + 1] - sortedAdapters[index];
            combinationsByPreviousGap = this.combinationsByPreviousGap(combinationsByPreviousGap, currentGap);
            index--;
        }
        return combinationsByPreviousGap[sortedAdapters[0]-1];
    }

    private combinationsByPreviousGap(priorCombinationsByPreviousGap: [number, number, number], currentGap: number): [number, number, number]{
        switch(currentGap){
            case 1:
                return [
                    priorCombinationsByPreviousGap[0] + priorCombinationsByPreviousGap[1],
                    priorCombinationsByPreviousGap[0] + priorCombinationsByPreviousGap[2],
                    priorCombinationsByPreviousGap[0]
                ];
            case 2:
                return [
                    priorCombinationsByPreviousGap[1] + priorCombinationsByPreviousGap[2],
                    priorCombinationsByPreviousGap[1],
                    priorCombinationsByPreviousGap[1]
                ]
            case 3:
                return [
                    priorCombinationsByPreviousGap[2],
                    priorCombinationsByPreviousGap[2],
                    priorCombinationsByPreviousGap[2]
                ]
            default:
                return [0,0,0]
        }
    }
}


export default Day10;