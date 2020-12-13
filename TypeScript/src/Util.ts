export function notEmpty<TValue>(value: TValue | null | undefined): value is TValue {
    return value !== null && value !== undefined;
}

export function addCount<T>(counts: Map<T, number>, key: T, count: number){
    if(!counts.has(key)){
        counts.set(key, count);
    } else {
        const oldCount: number = counts.get(key) as number;
        counts.set(key, oldCount + count);
    }
}

export function zeroIfInvalid(maybeNumber: number | null | undefined): number{
    return maybeNumber == undefined || maybeNumber == null ? 0 : maybeNumber;
}

export function unionWith<T>(set: Set<T>, otherSet: Set<T>): Set<T>{
    for(let item of otherSet){
        set.add(item);
    }
    return set;
}

export function union<T>(set: Set<T>, otherSet: Set<T>): Set<T>{
    const newSet = new Set<T>();
    return unionWith(unionWith(newSet, set), otherSet);
}

export function gcd(a: number, b:number): number{
    if(b == 0){
        return a;
    }

    return gcd(b, a % b);
}

export function extendedEuclid(a: bigint, b:bigint): [bigint, bigint, bigint]{
    if(a == BigInt(0)){
        return [b, BigInt(0), BigInt(1)];
    }

    const recursiveResult: [bigint, bigint, bigint] = extendedEuclid(b % a, a);
    const remainder: bigint = recursiveResult[0];
    const s: bigint = recursiveResult[1];
    const t: bigint = recursiveResult[2];

    return [remainder, t - b/a * s, s];
}

export function chineseRemainder(remainderModuloPairs: [bigint, bigint][]): bigint{
    let currentSolution: bigint = remainderModuloPairs[0][0];
    let currentModulo: bigint = remainderModuloPairs[0][1];
    for(let index = 1; index < remainderModuloPairs.length; index++){
        currentSolution = chineseRemainderForTwo(currentSolution, currentModulo, remainderModuloPairs[index][0], remainderModuloPairs[index][1]);
        currentModulo *= remainderModuloPairs[index][1];
    }
    return currentSolution;
}

function chineseRemainderForTwo(remainder1: bigint, modulo1: bigint, remainder2: bigint, modulo2: bigint): bigint{
    const extendedEuclidResult: [bigint, bigint, bigint] = extendedEuclid(modulo1, modulo2);
    const solution: bigint = remainder1 * extendedEuclidResult[2] * modulo2 + remainder2 * extendedEuclidResult[1] * modulo1;
    return modulo(solution, (modulo1 * modulo2));
}

export function directChineseRemainder(remainderModuloPairs: [bigint, bigint][]): bigint{
    const productOfModulos: bigint = remainderModuloPairs.reduce((product: bigint, pair: [bigint, bigint]) => product * pair[1], BigInt(1));
    const otherModulos: bigint[] = remainderModuloPairs.map((pair: [bigint, bigint]) => productOfModulos/pair[1]);
    let bezoutNumbers: bigint[] = [];
    for(let index in remainderModuloPairs){
        let euclidResult: [bigint, bigint, bigint] = extendedEuclid(otherModulos[index], remainderModuloPairs[index][1]);
        bezoutNumbers.push(euclidResult[1]);
    }

    let result: bigint = BigInt(0);
    for(let index in remainderModuloPairs){
        result = modulo(result + bezoutNumbers[index] * otherModulos[index] * remainderModuloPairs[index][0], productOfModulos);
    }

    return result;
}

export function modulo(a: bigint, b: bigint): bigint{
    const jsModulo: bigint = a % b;
    return jsModulo < 0 ? (b >= 0 ? jsModulo + b : jsModulo - b) : jsModulo;
}

export function parseBigInt(text: string): bigint{
    return BigInt(parseInt(text));
}