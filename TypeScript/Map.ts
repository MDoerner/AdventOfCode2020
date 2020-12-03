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