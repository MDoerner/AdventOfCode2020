import * as Plane from "./Plane";


export interface Map<T>{
    readonly height: number
    readonly width: number
    atPoint(point: Plane.Point): T
}

export interface MutableMap<T> extends Map<T>{
    setPoint(point: Plane.Point, value: T): boolean;
}

export class LoopingGrid<T> implements Map<T>{
    private baseMap: T[][];
    readonly height: number;
    readonly width: number;


    constructor(baseMap: T[][]){
        this.baseMap = baseMap;
        this.height = this.baseMap.length;
        this.width = (this.height == 0 ? 0 : this.baseMap[0].length);
    }

    atPoint(point: Plane.Point): T{
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

    atPoint(point: Plane.Point): T {
        return !isOnMainGrid(point, this)
            ? this.defaultValue
            : this.baseMap[point.y][point.x];
    }

    setPoint(point: Plane.Point, value: T): boolean {
        if(!isOnMainGrid(point, this)){
            return false;
        }
        this.baseMap[point.y][point.x] = value;
        return true;
    }
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

export function isOnMainGrid<T>(point: Plane.Point, grid: Map<T>){
    return point.x >= 0
        && point.x < grid.width
        && point.y >= 0
        && point.y < grid.height;
}