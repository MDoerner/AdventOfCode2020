import Day from "./Day";
import * as MyMap from "./Map";
import * as Util from "./Util";

enum SeatState{
    Floor,
    Empty,
    Occupied,
}

class Day11 implements Day<MyMap.MutableMap<SeatState>>{
    parseInput(text: string): MyMap.MutableMap<SeatState> {
        let baseMap: SeatState[][] = text.split(/\r?\n/)
            .map((line: string) => this.toSeatStateArray(line));
        return new MyMap.OutsideDefaultGrid<SeatState>(baseMap, SeatState.Floor);
    }

    private toSeatStateArray(line: string): SeatState[]{
        return Array.from(line)
            .map((c: string) => this.toSeatState(c));
    }

    private toSeatState(character: string): SeatState{
        switch(character){
            case "L":
                return SeatState.Empty;
            case "#":
                return SeatState.Occupied;
            default:
                return SeatState.Floor;
        }
    }

    solvePart1(input: MyMap.MutableMap<SeatState>): string {
        this.runTillEquilibrium(input, 3, nonFloorNeighbours);
        return MyMap.gridOccurrencCount(input, SeatState.Occupied).toString();
    }

    private runTillEquilibrium(seatStateMap: MyMap.MutableMap<SeatState>, toleratedNeighbours: number, neighbourFunction: (point: MyMap.Point, seatStateGrid: MyMap.Map<SeatState>) => MyMap.Point[]){
        const occupiedNeighbourCounts: MyMap.MutableMap<number> = this.occupiedNeighboursCountGrid(seatStateMap, neighbourFunction);
        let seatsToEvaluate: Set<string> = this.seats(seatStateMap);
        while(seatsToEvaluate.size > 0){
            let seatsToSwitch: Set<string> = this.switchingSeats(seatStateMap, occupiedNeighbourCounts, toleratedNeighbours, seatsToEvaluate);
            let seatsWithChangedNeighbourCounts: Set<string> = this.switchSeatsAndReturnChangedNeighbourCountSeats(seatStateMap, occupiedNeighbourCounts, seatsToSwitch, neighbourFunction);
            seatsToEvaluate = Util.unionWith(seatsToSwitch, seatsWithChangedNeighbourCounts);
        }
    }

    //string because we cannot overwrite the hashing for an object like Point.
    private seats(seatStateGrid: MyMap.MutableMap<SeatState>): Set<string>{
        const points: Set<string> = new Set<string>();
        for(let y = 0; y < seatStateGrid.height; y++){
            for(let x = 0; x < seatStateGrid.width; x++){
                let point: MyMap.Point = {x: x, y: y};
                if(seatStateGrid.atPoint(point) != SeatState.Floor){
                    points.add(JSON.stringify(point));
                }
            }
        }
        return points;
    }

    private occupiedNeighboursCountGrid(seatStateGrid: MyMap.MutableMap<SeatState>, neighbourFunction: (point: MyMap.Point, seatStateGrid: MyMap.Map<SeatState>) => MyMap.Point[]): MyMap.MutableMap<number>{
        let baseMap: number[][] = [];
        for(let y = 0; y < seatStateGrid.height; y++){
            let countsRow: number[] = [];
            for(let x = 0; x < seatStateGrid.width; x++){
                countsRow.push(this.occupiedNeighboursCount({x: x, y: y}, seatStateGrid, neighbourFunction));
            }
            baseMap.push(countsRow);
        }
        return new MyMap.OutsideDefaultGrid(baseMap, -1);
    }

    private occupiedNeighboursCount(point: MyMap.Point, grid: MyMap.Map<SeatState>, neighbourFunction: (point: MyMap.Point, seatStateGrid: MyMap.Map<SeatState>) => MyMap.Point[]) : number{
        if(grid.atPoint(point) == SeatState.Floor){
            return -1;
        }
        const neighbours: MyMap.Point[] = neighbourFunction(point, grid);
        return neighbours.filter((neighbour: MyMap.Point) => grid.atPoint(neighbour) == SeatState.Occupied).length;
    }

    //string because we cannot overwrite the hashing for an object like Point.
    private switchingSeats(seatStateGrid: MyMap.MutableMap<SeatState>, occupiedNeighbourCounts: MyMap.MutableMap<number>, toleratedNeighbours: number, seatsToConsider: Set<string>): Set<string>{
        const seatStrings: Set<string> = new Set<string>();
        for( let pointString of seatsToConsider){
            let point: MyMap.Point = JSON.parse(pointString) as MyMap.Point;
            switch(seatStateGrid.atPoint(point)){
                case SeatState.Empty:
                    if(occupiedNeighbourCounts.atPoint(point) == 0){
                        seatStrings.add(pointString);
                    }
                    break;
                case SeatState.Occupied:
                    if(occupiedNeighbourCounts.atPoint(point) > toleratedNeighbours){
                        seatStrings.add(pointString);
                    }
                    break;
            }
        }
        return seatStrings;
    }

    //string because we cannot overwrite the hashing for an object like Point.
    private switchSeatsAndReturnChangedNeighbourCountSeats(seatStateGrid: MyMap.MutableMap<SeatState>, occupiedNeighbourCounts: MyMap.MutableMap<number>, seatsToSwitch: Set<string>, neighbourFunction: (point: MyMap.Point, seatStateGrid: MyMap.Map<SeatState>) => MyMap.Point[]): Set<string>{
        const affectedNeighbours: Set<string> = new Set<string>();
        for(let pointString of seatsToSwitch){
            let point: MyMap.Point = JSON.parse(pointString) as MyMap.Point;
            switch(seatStateGrid.atPoint(point)){
                case SeatState.Empty:
                    seatStateGrid.setPoint(point, SeatState.Occupied);
                    for(let neighbour of neighbourFunction(point, seatStateGrid)){
                        occupiedNeighbourCounts.setPoint(neighbour, occupiedNeighbourCounts.atPoint(neighbour) + 1);
                        affectedNeighbours.add(JSON.stringify(neighbour));
                    }
                    break;
                case SeatState.Occupied:
                    seatStateGrid.setPoint(point, SeatState.Empty);
                    for(let neighbour of neighbourFunction(point, seatStateGrid)){
                        occupiedNeighbourCounts.setPoint(neighbour, occupiedNeighbourCounts.atPoint(neighbour) - 1);
                        affectedNeighbours.add(JSON.stringify(neighbour));
                    }
                    break;
            }
        }
        return affectedNeighbours;
    }

    solvePart2(input: MyMap.MutableMap<SeatState>): string {
        this.runTillEquilibrium(input, 4, visibleNeighbours);
        return MyMap.gridOccurrencCount(input, SeatState.Occupied).toString();
    }
}

function nonFloorNeighbours(point: MyMap.Point, seatStateGrid: MyMap.Map<SeatState>): MyMap.Point[]{
    return MyMap.gridNeighbours(point)
        .filter((neighbour: MyMap.Point) => seatStateGrid.atPoint(neighbour) != SeatState.Floor);
}

function visibleNeighbours(point: MyMap.Point, seatStateGrid: MyMap.Map<SeatState>): MyMap.Point[]{
    const directions: MyMap.Direction[] = MyMap.gridNeighbours({x: 0, y: 0});
    return directions.map((direction: MyMap.Direction) => nextVisibleSeat(point, direction, seatStateGrid))
        .filter(Util.notEmpty);
}

function nextVisibleSeat(point: MyMap.Point, direction: MyMap.Direction, seatStateGrid: MyMap.Map<SeatState>): MyMap.Point | null{
    let currentPoint: MyMap.Point = point;
    while(MyMap.isOnMainGrid(currentPoint, seatStateGrid)){
        currentPoint = MyMap.add(currentPoint, direction);
        if(seatStateGrid.atPoint(currentPoint) != SeatState.Floor){
            return currentPoint;
        }
    }
    return null;
}



export default Day11;