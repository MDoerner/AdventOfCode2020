import Day from "./Day";
import { GameOfLife } from "./GameOfLife";
import { add, Point, Vector } from "./Plane";
import * as Util from "./Util";

enum HexagonalDirection{
    West = 'w',
    NorthWest = 'nw',
    NorthEast = 'ne',
    East = 'e',
    SouthEast = 'se',
    SouthWest = 'sw',
}

function offsetFromHexDirection(hexDirection: HexagonalDirection): Vector{
    switch(hexDirection){
        case HexagonalDirection.West:
            return {x: -1, y: 0};
        case HexagonalDirection.East:
            return {x: 1, y: 0};
        case HexagonalDirection.NorthWest:
            return {x: 0, y: 1};
        case HexagonalDirection.NorthEast:
            return {x: 1, y: 1};
        case HexagonalDirection.SouthWest:
            return {x: -1, y: -1};
        case HexagonalDirection.SouthEast:
            return {x: 0, y: -1};
        default:
            throw Error("Unknown hexagonal direction: " + hexDirection);
    }
}

const hexDirections: HexagonalDirection[] = [
    HexagonalDirection.NorthEast,
    HexagonalDirection.NorthWest,
    HexagonalDirection.SouthEast,
    HexagonalDirection.SouthWest,
    HexagonalDirection.East,
    HexagonalDirection.West
]

function hexNeighbours(point: Point): Point[]{
    return hexDirections.map((hexDirection: HexagonalDirection) => add(point, offsetFromHexDirection(hexDirection)));
}

function endPoint(startPoint: Point, hexPath: Iterable<HexagonalDirection>): Point{
    return Util.reduce(hexPath, (currentPoint: Point, nextDirection: HexagonalDirection) => add(currentPoint, offsetFromHexDirection(nextDirection)), startPoint);
}

class Day24 implements Day<HexagonalDirection[][]>{
    parseInput(text: string): HexagonalDirection[][] {
        return text.split(/\r?\n/)
            .map((line: string) => this.parseHexPath(line));
    }

    private parseHexPath(text: string): HexagonalDirection[]{
        const hexPath: HexagonalDirection[] = [];
        let startIndex: number = 0;
        while(startIndex < text.length){
            if(text.startsWith(HexagonalDirection.NorthEast, startIndex)){
                hexPath.push(HexagonalDirection.NorthEast);
                startIndex += 2;
            } else if(text.startsWith(HexagonalDirection.NorthWest, startIndex)){
                hexPath.push(HexagonalDirection.NorthWest);
                startIndex += 2;
            } else if(text.startsWith(HexagonalDirection.SouthEast, startIndex)){
                hexPath.push(HexagonalDirection.SouthEast);
                startIndex += 2;
            } else if(text.startsWith(HexagonalDirection.SouthWest, startIndex)){
                hexPath.push(HexagonalDirection.SouthWest);
                startIndex += 2;
            } else if(text.startsWith(HexagonalDirection.East, startIndex)){
                hexPath.push(HexagonalDirection.East);
                startIndex++;
            } else if(text.startsWith(HexagonalDirection.West, startIndex)){
                hexPath.push(HexagonalDirection.West);
                startIndex++;
            } else {
                throw Error("Hex path starts withunknown direction: " + text.slice(startIndex));
            }
        }
        return hexPath;
    }

    solvePart1(hexPaths: HexagonalDirection[][]): string {
        const origin: Point = {x: 0, y: 0};
        const blackTiles: Iterable<Point> = this.blackTiles(origin, hexPaths);
        const result: number = Util.count(blackTiles);
        return result.toString();
    }

    private blackTiles(origin: Point, hexPaths: Iterable<Iterable<HexagonalDirection>>): Iterable<Point>{
        const flipCounts: Util.StructMap<Point,number> = this.flipCounts(origin, hexPaths);
        return Util.map(
                Util.filter(flipCounts, ([, flipCount]: [Point, number]) => flipCount % 2 == 1),
                ([tile, ]: [Point, number]) => tile
        );
    }

    private flipCounts(origin: Point, hexPaths: Iterable<Iterable<HexagonalDirection>>): Util.StructMap<Point,number>{
        const flipCounts: Util.StructMap<Point,number> = new Util.StructMap<Point,number>();
        for(let hexPath of hexPaths){
            const tile: Point = endPoint(origin, hexPath);
            this.addOrIncrement(tile, flipCounts);
        }
        return flipCounts;
    }

    private addOrIncrement(point: Point, countMap: Util.StructMap<Point,number>): void{
        if(countMap.has(point)){
            const currentCount: number = countMap.get(point) as number;
            countMap.set(point, currentCount + 1);
        } else{
            countMap.set(point, 1);
        }
    }

    solvePart2(hexPaths: HexagonalDirection[][]): string {
        const origin: Point = {x: 0, y: 0};
        const initiallyBlackTiles: Iterable<Point> = this.blackTiles(origin, hexPaths);
        const gameOfLife = new HexGameOfLife();
        const blackTiles: Iterable<Point> = gameOfLife.activeItemsAfterPlaying(initiallyBlackTiles, 100);
        const result: number = Util.count(blackTiles);
        return result.toString();
    }

}

class HexGameOfLife extends GameOfLife<Point>{
    neighbours(point: Point): Iterable<Point> {
        return hexNeighbours(point);
    }

    flipActive(activeNeighbourCount: number): boolean {
        return activeNeighbourCount <= 0
            || activeNeighbourCount > 2;
    }

    flipInactive(activeNeighbourCount: number): boolean {
        return activeNeighbourCount == 2;
    }
}


export default Day24;