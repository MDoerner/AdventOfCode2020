export interface Point{
    readonly x: number
    readonly y: number
}

export interface Vector extends Point{}
export interface Direction extends Vector{}

export function copyVector(vector: Vector): Vector{
    return {x: vector.x, y: vector.y};
}

export function add(base: Point, offset: Vector): Point{
    return {x: base.x + offset.x, y: base.y + offset.y};
}

export function scaledVector(vector: Vector, scale: number): Vector{
    return {x: vector.x * scale, y: vector.y * scale};
}

export function move(toMove: Point, offset: Vector, amplitude: number): Point{
    return add( toMove, scaledVector(offset, amplitude));
}

export function rotatedVector(vector: Vector, angleToAdd: number): Vector{
    //Rotation is a linear operation.
    const rotatedX: Vector = scaledVector(new AngleDirection(angleToAdd), vector.x);
    const rotatedY: Vector = scaledVector(new AngleDirection(angleToAdd + 90), vector.y);
    return add(rotatedX, rotatedY);
}

//Retirns undefined exactly for the zero vector.
export function angleOfVector(vector: Vector): number | undefined{
    if(vector.x == 0 && vector.y == 0){
        return undefined;
    }

    if(vector.x == 0){
        return vector.y > 0 ? 90 : -90;
    }

    //This has been added to get the cardinal directions right although computations using the functions and constants in Math suffer from numeric errors.
    if(vector.y == 0){
        return vector.x > 0 ? 0 : 180;
    }

    if(vector.x > 0){
        return Math.atan(vector.y/vector.x) * 180/Math.PI;
    }

    if(vector.x < 0){
        return Math.atan(vector.y/vector.x) * 180/Math.PI + 180;
    }
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
    x: number = 0;
    y: number = 0;

    constructor(initialAngle: number){
        this.angle = initialAngle;
        this.updateXY();
    }

    //This is necessary because Math.cos nd Math.sin do not return exact results for k/2 * Math.PI where k is a whole number.
    private updateXY(): void{
        switch(this.angle % 360){
            case 0:
                this.x = 1;
                this.y = 0;
                break;
            case 90:
            case -270:
                this.x = 0;
                this.y = 1;
                break;
            case 180:
            case -180:
                this.x = -1;
                this.y = 0;
                break;
            case 270:
            case -90:
                this.x = 0;
                this.y = -1;
                break;
            default:
                this.x = Math.cos(Math.PI * this.angle/180);
                this.y = Math.sin(Math.PI * this.angle/180);
                break;
        }
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

export function l1Norm(vector: Vector): number{
    return Math.abs(vector.x) + Math.abs(vector.y);
}

export function l2Norm(vector: Vector): number{
    return Math.sqrt((vector.x)^2 + (vector.y)^2);
}

export function manhattanDistance(point: Point, otherPoint: Point): number{
    const differenceVector: Vector = add(point, scaledVector(otherPoint, -1));
    return l1Norm(differenceVector);
}