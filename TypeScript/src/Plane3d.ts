
export interface Point{
    readonly x: number;
    readonly y: number;
    readonly z: number;
}

export interface Vector extends Point{}

export function copyVector(vector: Vector): Vector{
    return {x: vector.x, y: vector.y, z: vector.z};
}

export function add(base: Point, offset: Vector): Point{
    return {x: base.x + offset.x, y: base.y + offset.y, z: base.z + offset.z};
}

export function scaledVector(vector: Vector, scale: number): Vector{
    return {x: vector.x * scale, y: vector.y * scale, z: vector.z * scale};
}

export function move(toMove: Point, offset: Vector, amplitude: number): Point{
    return add( toMove, scaledVector(offset, amplitude));
}

export function gridNeighbours(point: Point): Point[]{
    const offsets: Vector[] = gridNeighbourOffsets;
    return offsets.map((offset: Vector) => add(point, offset));
}

const gridNeighbourOffsets: Vector[] = generateGridNeighbourOffsets();

function generateGridNeighbourOffsets(): Vector[]{
    let offsets: Vector[] = [];
    for(let x: number = -1; x <= +1; x++){
        for(let y: number = -1; y <= +1; y++){
            for(let z: number = -1; z <= +1; z++){
                if(x != 0 || y != 0 || z != 0){
                    offsets.push({x: x, y: y, z: z});
                }
            }
        }
    }
    return offsets;
}

export function l1Norm(vector: Vector): number{
    return Math.abs(vector.x) + Math.abs(vector.y) + Math.abs(vector.z);
}

export function l2Norm(vector: Vector): number{
    return Math.sqrt((vector.x)^2 + (vector.y)^2+ (vector.z)^2);
}

export function manhattanDistance(point: Point, otherPoint: Point): number{
    const differenceVector: Vector = add(point, scaledVector(otherPoint, -1));
    return l1Norm(differenceVector);
}