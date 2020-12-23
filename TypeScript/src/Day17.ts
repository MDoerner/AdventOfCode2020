import Day from "./Day";
import * as Util from "./Util";
import * as Plane from "./Plane4d";


class Day17 implements Day<Util.StructSet<Plane.Point>>{
    parseInput(text: string): Util.StructSet<Plane.Point> {
        const activePoints: Util.StructSet<Plane.Point> = new Util.StructSet<Plane.Point>();
        const inputPlane: boolean[][] = this.parsePlane(text);
        for(let y: number = 0; y < inputPlane.length; y++){
            for(let x: number = 0; x < inputPlane[y].length; x++){
                if(inputPlane[y][x]){
                    activePoints.add({x: x, y: y, z: 0, w: 0});
                }
            }
        }
        return activePoints;
    }

    private parsePlane(text: string): boolean[][]{
        return text.split(/\r?\n/)
            .reduce((rows: boolean[][], line: string) => {
                rows.push(this.parseRow(line));
                return rows;
            }, [])
    }

    private parseRow(line: string): boolean[]{
        return Array.from(line).reduce((row: boolean[], character: string) => {
                row.push(this.isActiveMarker(character));
                return row;
            }, []);
    }

    private isActiveMarker(character: string): boolean{
        return character == '#';
    }

    solvePart1(input: Util.StructSet<Plane.Point>): string {
        const activePoints: Util.StructSet<Plane.Point> = input;
        for(let iteration: number = 1; iteration <= 6; iteration++){
            this.executeStep(activePoints, Plane.gridNeighbours3d);
        }
        const result: number = activePoints.size;
        return result.toString();
    }

    private executeStep(activePoints: Util.StructSet<Plane.Point>, neighboursFunction: (point: Plane.Point) => Plane.Point[]): void{
        const relevantPoints: Util.StructSet<Plane.Point> = this.relevantPoints(activePoints, neighboursFunction);
        const changingPoints: Plane.Point[] = relevantPoints.toArray()
            .filter((point: Plane.Point) => this.pointChanges(point, activePoints, neighboursFunction));
        this.changePointStates(changingPoints, activePoints);
    }

    private relevantPoints(activePoints: Util.StructSet<Plane.Point>, neighboursFunction: (point: Plane.Point) => Plane.Point[]): Util.StructSet<Plane.Point>{
        const points: Util.StructSet<Plane.Point> = new Util.StructSet<Plane.Point>();
        for(let activePoint of activePoints.toArray()){
            points.add(activePoint);
            for(let neighbour of neighboursFunction(activePoint)){
                points.add(neighbour);
            }
        }
        return points;
    }

    private pointChanges(point: Plane.Point, activePoints: Util.StructSet<Plane.Point>, neighboursFunction: (point: Plane.Point) => Plane.Point[]): boolean{
        const activeNeighbourCount: number = this.activeNeighbourCount(point, activePoints, neighboursFunction);
        if(activePoints.has(point)){
            return activeNeighbourCount < 2
                || activeNeighbourCount > 3;
        } else {
            return activeNeighbourCount == 3;
        }
    }

    private activeNeighbourCount(point: Plane.Point, activePoints: Util.StructSet<Plane.Point>, neighboursFunction: (point: Plane.Point) => Plane.Point[]){
        return neighboursFunction(point)
            .filter((neighbour: Plane.Point) => activePoints.has(neighbour))
            .length;
    }

    private changePointStates(changingPoints: Plane.Point[], activePoints: Util.StructSet<Plane.Point>): void{
        for(let changingPoint of changingPoints){
            if(activePoints.has(changingPoint)){
                activePoints.delete(changingPoint);
            } else {
                activePoints.add(changingPoint);
            }
        }
    }

    solvePart2(input: Util.StructSet<Plane.Point>): string {
        const activePoints: Util.StructSet<Plane.Point> = input;
        for(let iteration: number = 1; iteration <= 6; iteration++){
            this.executeStep(activePoints, Plane.gridNeighbours);
        }
        const result: number = activePoints.size;
        return result.toString();
    }

}


export default Day17;