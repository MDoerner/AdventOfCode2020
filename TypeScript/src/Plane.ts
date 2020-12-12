export interface Point{
    readonly x: number
    readonly y: number
}

export interface Vector extends Point{}
export interface Direction extends Vector{}

export function add(base: Point, offset: Vector): Point{
    return {x: base.x + offset.x, y: base.y + offset.y};
}

export function scaledVector(vector: Vector, scale: number): Vector{
    return {x: vector.x * scale, y: vector.y * scale};
}

export class IntegralDirection implements Direction{
    readonly x: number;
    readonly y: number;

    constructor(direction: Vector){
        if(direction.x == 0 && direction.y == 0){
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

export class AngleDirection implements Direction{
    private angle: number;
    x: number;
    y: number;

    constructor(initialAngle: number){
        this.angle = initialAngle;
        this.x = Math.cos(Math.PI * this.angle/180);
        this.y = Math.sin(Math.PI * this.angle/180);
    }

    addAngle(angle: number): AngleDirection{
        return new AngleDirection(this.angle + angle);
    }
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