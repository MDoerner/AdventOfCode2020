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

export function defaultIfInvalid<T>(maybeValue: T | null | undefined, defaultValue: T): T{
    return maybeValue == undefined || maybeValue == null ? defaultValue: maybeValue;
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

//Modulo with the guarantee to return a positive value.
export function modulo(a: bigint, b: bigint): bigint{
    const jsModulo: bigint = a % b;
    return jsModulo < 0n ? (b >= 0n ? jsModulo + b : jsModulo - b) : jsModulo;
}