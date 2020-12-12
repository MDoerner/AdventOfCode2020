export interface Point{
    readonly x: number
    readonly y: number
}

export interface Vector extends Point{}
export interface Direction extends Vector{}

export function add(base: Point, offset: Vector): Point{
    return {x: base.x + offset.x, y: base.y + offset.y};
}


export interface Map<T>{
    readonly height: number
    readonly width: number
    atPoint(point: Point): T
}

export interface MutableMap<T> extends Map<T>{
    setPoint(point: Point, value: T): boolean;
}

export class LoopingIntegralMap<T> implements Map<T>{
    private baseMap: T[][];
    readonly height: number;
    readonly width: number;


    constructor(baseMap: T[][]){
        this.baseMap = baseMap;
        this.height = this.baseMap.length;
        this.width = (this.height == 0 ? 0 : this.baseMap[0].length);
    }

    atPoint(point: Point): T{
        let x: number = point.x % this.width;
        let y: number = point.y % this.height;
        return this.baseMap[y][x];
    }
}

export class OutsideDefaultGrid<T> implements MutableMap<T>{
    private baseMap: T[][];
    private defaultValue: T;
    readonly height: number;
    readonly width: number;


    constructor(baseMap: T[][], defaultValue: T){
        this.baseMap = baseMap;
        this.defaultValue = defaultValue;
        this.height = this.baseMap.length;
        this.width = (this.height == 0 ? 0 : this.baseMap[0].length);
    }

    atPoint(point: Point): T {
        return !isOnMainGrid(point, this)
            ? this.defaultValue
            : this.baseMap[point.y][point.x];
    }

    setPoint(point: Point, value: T): boolean {
        if(!isOnMainGrid(point, this)){
            return false;
        }
        this.baseMap[point.y][point.x] = value;
        return true;
    }
}

export class IntegralDirection implements Direction{
    readonly x: number;
    readonly y: number;

    constructor(direction: Vector){
        if(direction.x == 0 && direction.y ==0){
            this.x = 0;
            this.y = 0;
            return;
        }

        if(direction.x == 0){
            this.x = 0;
            this.y = (direction.y > 0 ? 1 : -1);
            return;
        }

        if(direction.y == 0){
            this.y = 0;
            this.x = (direction.x > 0 ? 1 : -1);
            return;
        }

        let divisor: number = gcd(Math.abs(direction.x), Math.abs(direction.y));

        this.x = direction.x / divisor;
        this.y = direction.y / divisor;
    }
}

function gcd(a: number, b:number): number{
    if(b == 0){
        return a;
    }

    return gcd(b, a % b);
}

export function gridNeighbours(point: Point): Point[]{
    const offsets: Vector[] = gridNeighbourOffsets;
    return gridNeighbourOffsets.map((offset: Vector) => add(point, offset));
}

const gridNeighbourOffsets: Vector[] = generateGridNeighbourOffsets();

function generateGridNeighbourOffsets(): Vector[]{
    let offsets: Vector[] = [];
    for(let x: number = -1; x <= +1; x++){
        for(let y: number = -1; y <= +1; y++){
            if(x != 0 || y != 0){
                offsets.push({x: x, y: y});
            }
        }
    }
    return offsets;
}

export function gridOccurrencCount<T>(grid: Map<T>, value: T): number{
    let count: number = 0;
    for(let x: number = 0; x < grid.width; x++){
        for(let y: number = 0; y < grid.height; y++){
            if(grid.atPoint({x: x, y: y}) == value){
                count++;
            }
        }
    }
    return count;
}



export function isOnMainGrid<T>(point: Point, grid: Map<T>){
    return point.x >= 0
        && point.x < grid.width
        && point.y >= 0
        && point.y < grid.height;
}