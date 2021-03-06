import * as Util from "./Util";


export function gcd(a: number, b:number): number{
    if(b == 0){
        return a;
    }

    return gcd(b, a % b);
}

export function extendedEuclid(a: bigint, b:bigint): [bigint, bigint, bigint]{
    if(a == 0n){
        return [b, 0n, 1n];
    }

    const recursiveResult: [bigint, bigint, bigint] = extendedEuclid(b % a, a);
    const remainder: bigint = recursiveResult[0];
    const s: bigint = recursiveResult[1];
    const t: bigint = recursiveResult[2];

    return [remainder, t - b/a * s, s];
}

export function chineseRemainder(remainderModuloPairs: [bigint, bigint][]): bigint | null{
    let currentSolution: bigint | null = remainderModuloPairs[0][0];
    let currentModulo: bigint = remainderModuloPairs[0][1];
    for(let index = 1; index < remainderModuloPairs.length; index++){
        let stepResult: [bigint, bigint] | null = chineseRemainderForTwo(currentSolution, currentModulo, remainderModuloPairs[index][0], remainderModuloPairs[index][1]);
        if(stepResult == null){
            return null;
        }
        currentSolution = stepResult[0];
        currentModulo = stepResult[1];
    }
    return currentSolution;
}

function chineseRemainderForTwo(remainder1: bigint, modulo1: bigint, remainder2: bigint, modulo2: bigint): [bigint, bigint] | null{
    const extendedEuclidResult: [bigint, bigint, bigint] = extendedEuclid(modulo1, modulo2);
    const gcd: bigint = extendedEuclidResult[0];
    const firstCoeffcient: bigint = remainder1 / gcd;
    const secondCoefficient: bigint = remainder2 / gcd;
    const offset: bigint = Util.modulo(remainder1, gcd);
    if(Util.modulo(remainder2, gcd) != offset){
        return null;
    }
    const solution: bigint = firstCoeffcient * extendedEuclidResult[2] * modulo2 + secondCoefficient * extendedEuclidResult[1] * modulo1 + offset;
    const nextModulo: bigint = (modulo1 * modulo2) / gcd;
    return [Util.modulo(solution, nextModulo), nextModulo];
}