import Day from "./Day";
import * as sumFinder from "./Day1";


class Day9 implements Day<number[]>{
    parseInput(text: string): number[] {
        return text.split(/\r?\n/)
            .map((s: string) => parseInt(s));
    }

    solvePart1(input: number[]): string {
        const firstItemFailingValidation: number | null = this.firstInvalidEntry(input, 0, 25);
        if(firstItemFailingValidation){
            return firstItemFailingValidation.toString()
        }
        return "No invalid entry found!";
    }

    private firstInvalidEntry(input: number[], startIndex: number, preambleLength: number): number | null{
        let currentComparisonRangeStart: number = startIndex;
        let currentItemIndex: number = startIndex + preambleLength;
        const comparisonRange: SortedNumberList = new SortedNumberList(input.slice(currentComparisonRangeStart, currentItemIndex));
        let currentValue: number;
        while(currentItemIndex < input.length){
            currentValue = input[currentItemIndex];
            if(!this.isValidEntry(currentValue, comparisonRange.asArray())){
                return currentValue;
            }
            comparisonRange.remove(input[currentComparisonRangeStart]);
            currentComparisonRangeStart++;
            comparisonRange.add(currentValue);
            currentItemIndex++;
        }

        return null;
    }

    private isValidEntry(item: number, comparisonRange: number[]): boolean{
        let summingPair: [number, number] | null = sumFinder.findSummingPairInSorted(comparisonRange, item);
        return summingPair != null && summingPair[0] != summingPair[1];
    }

    solvePart2(input: number[]): string {
        const firstItemFailingValidation: number | null = this.firstInvalidEntry(input, 0, 25);
        if(!firstItemFailingValidation){
            return "No invalid entry found!";
        }
        const rangeBounds: [number, number] | null = this.firstRangeWithSum(input, firstItemFailingValidation, 0);
        if(!rangeBounds){
            return "No range summing to " + firstItemFailingValidation + " found!";
        }
        const range: number[] = input.slice(rangeBounds[0], rangeBounds[1]);
        const lowestItem: number = Math.min(...range);
        const highestItem: number = Math.max(...range);
        return (lowestItem + highestItem).toString();
    }

    private firstRangeWithSum(input: number[], desiredSum: number, startIndex:number): [number, number] | null{
        let lowIndex: number = startIndex;
        let highIndex: number = startIndex + 1;

        if(highIndex > input.length || startIndex < 0){
            return null;
        }

        let currentSum: number = input[lowIndex] + input[highIndex];
        while(true){
            if(currentSum == desiredSum){
                return [lowIndex, highIndex];
            }
            if(currentSum < desiredSum){
                highIndex++;
                if(highIndex >= input.length){
                    return null;
                }
                currentSum += input[highIndex];
            } else {
                currentSum -= input[lowIndex];
                lowIndex++;
            }

            if(lowIndex == highIndex){
                highIndex++;
                if(highIndex >= input.length){
                    return null;
                }
                currentSum += input[highIndex];
            }
        }
    }


}


class SortedNumberList{
    //This wants to be a btree for performance, but for this coding chellange, a simple array will
    private items: number[];
    length: number;

    constructor(initialItems: number[] = []){
        this.items = initialItems.sort((m: number, n: number) => m - n);
        this.length = initialItems.length;
    }

    //This is unsafe because it allows the caller to manipulate the array.
    //However, I wanted to reuse the work from day 1 without refactoring it to use a SortedNumberList.
    asArray(): number[] {
        return this.items;
    }

    get(index: number): number | undefined {
        if(index < 0 || index >= this.length){
            return undefined;
        }
        return this.items[index];
    }

    add(item: number): void {
        const insertionIndex: number = this.insertionIndex(item);
        this.items.splice(insertionIndex, 0, item);
        this.length++;
    }

    //Either the index that already holds the item or the lowest index greater than the item.
    private insertionIndex(item: number): number{
        const lowestValue = this.get(0);
        if(lowestValue == undefined || lowestValue >= item){
            return 0;
        }
        const highestValue = this.get(this.length - 1);
        if(highestValue == undefined || highestValue <= item){
            return this.length;
        }

        return this.insertionIndexBetweenIndices(item, 0, this.length - 1);
    }

    //This method expects that this.get(lowerBound) <= item <= this.get(upperBound).
    private insertionIndexBetweenIndices(item: number, lowerBound: number, upperBound: number): number{
        if(lowerBound == upperBound){
            return upperBound;
        }

        const candidate: number = Math.floor((lowerBound + upperBound)/2);
        const itemAtCandidate: number | undefined = this.get(candidate) as number;
        if(itemAtCandidate == item){
            return candidate;
        }
        if(upperBound == candidate + 1){
            return itemAtCandidate > item ? candidate : upperBound;
        }
        if(itemAtCandidate > item){
            return this.insertionIndexBetweenIndices(item, lowerBound, candidate);
        }
        return this.insertionIndexBetweenIndices(item, candidate, upperBound);
    }

    indexOf(item: number): number | null{
        const candidate = this.insertionIndex(item);
        return this.get(candidate) == item ? candidate : null;
    }

    has(item: number): boolean {
        return this.insertionIndex(item) != null;
    }

    remove(item: number): void {
        const candidateIndex: number = this.insertionIndex(item);
        if(this.get(candidateIndex) == item){
            this.items.splice(candidateIndex, 1);
            this.length--;
        }
    }
}


export default Day9;