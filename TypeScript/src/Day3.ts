import Day from "./Day";
import * as MyMap from "./Map";
import * as Plane from "./Plane";

class Day3 implements Day<MyMap.Map<boolean>>{
    parseInput(text: string): MyMap.Map<boolean> {
        let baseMap: boolean[][] = text.split(/\r?\n/)
            .map((line: string) => this.toHasTreeArray(line));
        return new MyMap.LoopingGrid<boolean>(baseMap);
    }

    private toHasTreeArray(line: string): boolean[]{
        return Array.from(line)
            .map((c: string) => this.isTree(c));
    }

    private isTree(character: string): boolean{
        return character == '#';
    }

    solvePart1(input: MyMap.Map<boolean>): string {
        let startPoint: Plane.Point = {x: 0, y: 0};
        let direction: Plane.Vector = {x: 3, y: 1};
        return this.countTreesInDirection(input, startPoint, direction).toString();
    }

    private countTreesInDirection(slope: MyMap.Map<boolean>, startPoint: Plane.Point, step: Plane.Vector): number{
        let direction = new Plane.IntegralDirection(step);
        return this.countTreesAtSteps(slope, startPoint, direction);
    }

    private countTreesAtSteps(slope: MyMap.Map<boolean>, startPoint: Plane.Point, step: Plane.Vector): number{
        if(step.y <= 0){
            throw new Error("slpoe must be downhill, i.e. y must be greater than zero.");
        }

        let currentPoint: Plane.Point = startPoint;

        let treesEncountered: number = 0;
        while (currentPoint.y < slope.height){
            if(slope.atPoint(currentPoint)){
                treesEncountered++;
            }
            currentPoint = Plane.add(currentPoint, step);
        }

        return treesEncountered;
    }

    solvePart2(input: MyMap.Map<boolean>): string {
        let startPoint: Plane.Point = {x: 0, y: 0};
        let directions: Plane.Vector[] = [{x: 1, y: 1}, {x: 3, y: 1}, {x: 5, y: 1}, {x: 7, y: 1}, {x: 1, y: 2}];
        let treesEncountered: number[] = directions.map((direction: Plane.Vector) => this.countTreesInDirection(input, startPoint, direction));
        return treesEncountered.reduce((accumulator: number, currentValue:number) => accumulator * currentValue).toString();
    }

}



export default Day3;