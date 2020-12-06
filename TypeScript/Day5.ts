import Day from "./Day";

interface Seat{
    row: number;
    column: number;
}

class Day5 implements Day<Seat[]>{
    parseInput(text: string): Seat[] {
        return text.split(/\r?\n/)
            .map((line: string) => this.parseBoardingCard(line));
    }

    private parseBoardingCard(text: string): Seat{
        let row: number = this.parseSeatRow(text.slice(0,7));
        let column: number = this.parseSeatColumn(text.slice(-3));
        return {row: row, column: column};
    }

    private parseSeatRow(seatCode: string): number{
        let seatBinary: string = seatCode.replace(/F/g, '0').replace(/B/g,'1');
        return parseInt(seatBinary, 2);
    }

    private parseSeatColumn(seatCode: string): number{
        let seatBinary: string = seatCode.replace(/L/g, '0').replace(/R/g,'1');
        return parseInt(seatBinary, 2);
    }

    solvePart1(input: Seat[]): string {
        let ids: number[] = input.map((seat: Seat) => seatId(seat));
        return Math.max(...ids).toString();
    }

    solvePart2(input: Seat[]): string {
        let ids: number[] = input.map((seat: Seat) => seatId(seat));
        let sortedIds: number[] = ids.sort((m: number, n: number) => m - n);
        let index: number = 0;
        while(sortedIds[index] == sortedIds[index + 1] - 1){
            index++;
        }

        return (sortedIds[index] + 1).toString();
    }

}

function seatId(seat: Seat): number{
    return seat.row * 8 + seat.column;
}

export default Day5;