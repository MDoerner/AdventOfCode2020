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

export function moduloP(a: number, b: number): number{
    const jsModulo: number = a % b;
    return jsModulo < 0 ? (b >= 0 ? jsModulo + b : jsModulo - b) : jsModulo;
}

export function concatSet<T>(array: T[], set: Set<T>): T[]{
    set.forEach((element: T) => array.push(element));
    return array;
}


export class StructSet<T>{
    private readonly backingStore: Set<string> = new Set<string>();

    has(element: T): boolean{
        return this.backingStore.has(this.toKey(element));
    }

    private toKey(element: T): string{
        return JSON.stringify(element);
    }

    add(element: T): void {
        this.backingStore.add(this.toKey(element));
    }

    delete(element: T): void {
        this.backingStore.delete(this.toKey(element));
    }

    toArray(): T[]{
        let points: T[] = [];
        for(let point of this.backingStore.keys()){
            points.push(this.fromKey(point));
        }
        return points;
    }

    private fromKey(key: string): T{
        return JSON.parse(key);
    }

    get size(): number{
        return this.backingStore.size;
    }
}

export function reverseString(str: string): string{
    return Array.from(str).reverse().join('');
}

export function addToValueList<T, U>(key: T, value: U, map: Map<T, U[]>): void{
    if(map.has(key)){
        const valueList: U[] = map.get(key) as U[];
        valueList.push(value);
    } else {
        map.set(key, [value]);
    }
}