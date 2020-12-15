import Day from "./Day";


class Day15 implements Day<number[]>{
    parseInput(text: string): number[] {
        return text.split(',')
        .map((item: string) => parseInt(item));
    }

    solvePart1(input: number[]): string {
        const result: number | null = this.numberSpoken(input, 2020);
        if(!result){
            return "Unable to determine result!";
        }
        return result.toString();
    }

    private numberSpoken(input: number[], position:number): number | null{
        const memory: Map<number, number> = new Map<number, number>();
        const startValues: [number, number] | null = this.loadMemory(memory, input);
        if(!startValues){
            return null;
        }
        const result: number | null = this.numberAtPosition(memory, startValues[1], startValues[0], position);
        if(!result){
            return null;
        }
        return result;
    }

    //Loads all inputs except the last one.
    //Returns the position after the last input and the last input.
    private loadMemory(memory: Map<number, number>, input: number[]): [number, number] | null{
        if(input.length == 0){
            return null;
        }

        for(let numberPosition: number = 1; numberPosition < input.length; numberPosition++){
            memory.set(input[numberPosition - 1], numberPosition);
        }

        return [input.length + 1, input[input.length - 1]];
    }

    private numberAtPosition(memory: Map<number,number>, initialLastNumber: number, startPosition: number, targetPosition: number): number | null{
        if(targetPosition < startPosition){
            return null;
        }

        let lastNumber: number = initialLastNumber;
        let currentNumber: number;
        for(let currentPosition: number = startPosition; currentPosition <= targetPosition; currentPosition++){
            currentNumber = this.numberAtNextPosition(memory, currentPosition - 1, lastNumber);
            memory.set(lastNumber, currentPosition - 1);
            lastNumber = currentNumber;
        }

        return lastNumber;
    }

    private numberAtNextPosition(memory: Map<number,number>, position: number, currentNumber: number): number{
        return memory.has(currentNumber) ? position - (memory.get(currentNumber) as number) : 0;
    }

    solvePart2(input: number[]): string {
        const result: number | null = this.numberSpoken(input, 30000000);
        if(!result){
            return "Unable to determine result!";
        }
        return result.toString();
    }
}


export default Day15;