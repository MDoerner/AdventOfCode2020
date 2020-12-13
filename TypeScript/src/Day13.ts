import Day from "./Day";
import * as Util from "./Util";

interface ContestData{
    busId: bigint;
    timeSinceLastDeparture: bigint;
}

class Day13 implements Day<[number, string[]]>{
    parseInput(text: string): [number, string[]] {
        const lines: string[] = text.split(/\r?\n/);
        const earliestTime: number = parseInt(lines[0]);
        const busIds: string[] = this.extractBusIDs(lines[1]);
        return [earliestTime, busIds];
    }

    private extractBusIDs(text: string): string[]{
        const pattern: RegExp = /(\d+|x),?/g;
        let ids: string[] = [];
        for(let match of text.matchAll(pattern)){
            ids.push(match[1]);
        }
        return ids;
    }

    solvePart1(input: [number, string[]]): string {
        const earliestPossibleTime: number = input[0];
        const numericIds: number[] = this.parseBusIDs(input[1]);
        const nextBusWithWaitingime: number[] = numericIds  //number[] instead of [number, number] because the type checker cannot recognize lambdas returning [number, number].
            .reduce(
                (previousBest: number[], busId: number) => {
                    const waitTime: number = this.timeToWait(earliestPossibleTime, busId);
                    if(waitTime < previousBest[1] || previousBest[1] < 0){
                        return [busId, waitTime];
                    }
                    return previousBest;
                },
                [-1, -1]);
        return (nextBusWithWaitingime[0] * nextBusWithWaitingime[1]).toString();
    }

    private parseBusIDs(ids: string[]): number[]{
        return ids.filter((id: string) => id != 'x')
            .map((id: string) => parseInt(id));
    }

    private timeToWait(startTime: number, busId: number): number{
        return busId - (startTime % busId);
    }

    //This only solves the case of coprime periods for now since all input ids are prime.
    //I will figure out the general case later.
    solvePart2(input: [number, string[]]): string {
        const contestInput: ContestData[] = this.parseContestData(input[1]);
        const remainderModuloPairs: [bigint, bigint][] = contestInput.map((contestData: ContestData) => [contestData.timeSinceLastDeparture, contestData.busId]);
        const result: bigint = Util.chineseRemainder(remainderModuloPairs);

        for(let contestData of contestInput){
            console.log(contestData.busId + ', ' + contestData.timeSinceLastDeparture + ': ' + Util.modulo(result, contestData.busId));
        }

        return result.toString();
    }

    private parseContestData(busTable: string[]): ContestData[]{
        let contestData: ContestData[] = [];
        for(let index in busTable){
            if(busTable[index] != 'x'){
                const busId: bigint = Util.parseBigInt(busTable[index]);
                contestData.push({busId: busId, timeSinceLastDeparture: this.timeSinceLastDeparture(Util.parseBigInt(index), busId)})
            }
        }
        return contestData;
    }

    private timeSinceLastDeparture(offsetFromFirstBus: bigint, busId: bigint): bigint{
        return offsetFromFirstBus == BigInt(0) ? BigInt(0) : busId - (offsetFromFirstBus % busId);
    }

}



export default Day13;