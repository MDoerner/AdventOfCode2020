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