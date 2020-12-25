import Day from "./Day";

interface DoorCard{
    readonly cardPublicKey: number,
    readonly doorPublicKey: number,
    readonly base: number,
    readonly keySpaceSize: number,
}

class Day25 implements Day<DoorCard>{
    parseInput(text: string): DoorCard {
        const publicKeys: number[] = text.split(/\r?\n/)
            .map((line: string) => parseInt(line));
        return {
            cardPublicKey: publicKeys[0],
            doorPublicKey: publicKeys[1],
            base: 7,
            keySpaceSize: 20201227
        };
    }

    solvePart1(card: DoorCard): string {
        const encryptionKey: number | null = this.encryptionKey(card);
        if(encryptionKey == null){
            return "Unable to find the encrytion key.";
        }
        return encryptionKey.toString();
    }

    private encryptionKey(card: DoorCard): number | null{
        const cardLoopSize: number | null = this.bruteForcedLoopSize(card.cardPublicKey, card.base, card.keySpaceSize);
        if(cardLoopSize == null)
        {
            return null;
        }
        return this.transformKey(card.doorPublicKey, cardLoopSize, card.keySpaceSize);
    }

    private bruteForcedLoopSize(key: number, subjectNumber: number, keySpaceSize: number): number | null{
        let comparisonKey: number = 1;
        let loopSize: number = 0
        while(loopSize < keySpaceSize){
            if(comparisonKey == key){
                return loopSize;
            }
            comparisonKey = (comparisonKey * subjectNumber) % keySpaceSize;
            loopSize++;
        }
        return null;
    }

    private transformKey(key: number, loopSize: number, keySpaceSize: number): number{
        let resultKey: number = 1;
        for(let round: number = 0; round < loopSize; round++){
            resultKey = (resultKey * key) % keySpaceSize;
        }
        return resultKey;
    }

    solvePart2(card: DoorCard): string {
        //Nothing to do!
        throw new Error("Method not implemented.");
    }
}






export default Day25;