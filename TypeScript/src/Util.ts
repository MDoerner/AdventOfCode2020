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

export function union<T>(sets: IterableIterator<Set<T>>): Set<T>{
    const newSet = new Set<T>();
    for(let set of sets){
        unionWith(newSet, set);
    }
    return newSet;
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

export class StructMap<T, U>{
    private readonly backingStore: Map<string, U> = new Map<string, U>();

    has(element: T): boolean{
        return this.backingStore.has(this.toKey(element));
    }

    private toKey(element: T): string{
        return JSON.stringify(element);
    }

    set(key: T, value: U): void {
        this.backingStore.set(this.toKey(key), value);
    }

    get(key: T): U | undefined {
        return this.backingStore.get(this.toKey(key));
    }

    delete(element: T): void {
        this.backingStore.delete(this.toKey(element));
    }

    toArray(): [T,U][]{
        let entries: [T,U][] = [];
        for(let entry of this.backingStore.entries()){
            entries.push([this.fromKey(entry[0]), entry[1]]);
        }
        return entries;
    }

    private fromKey(key: string): T{
        return JSON.parse(key);
    }

    get size(): number{
        return this.backingStore.size;
    }
}

export function addToValueSet<T, U>(key: T, value: U, map: Map<T, Set<U>>): void{
    if(map.has(key)){
        const valueSet: Set<U> = map.get(key) as Set<U>;
        valueSet.add(value);
    } else {
        map.set(key, new Set<U>([value]));
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

export function reduce<T, U>(iterator: IterableIterator<T>, reductionFunction:(previousValue: U, nextItem: T) => U, startValue: U): U{
    let currentValue: U = startValue;
    for(let item of iterator){
        currentValue = reductionFunction(currentValue, item);
    }
    return currentValue;
}

export class Queue<T>{
    private dequeueStore: T[] = [];
    private enqueueStore: T[] = [];
    private _size: number;

    constructor(initialContent: T[] = []){
        this.dequeueStore = Array.from(initialContent).reverse(); //copy
        this._size = this.dequeueStore.length;
    }

    [Symbol.iterator](): IterableIterator<T> {
        throw new Error("Method not implemented.");
    }

    get size(): number{
        return this._size;
    }

    enqueue(item: T): void{
        this.enqueueStore.push(item);
        this._size++;
    }

    dequeue(): T | undefined{
        if(this.size == 0){
            return undefined;
        }
        if(this.dequeueStore.length == 0){
            this.shiftToDequeue();
        }
        this._size--;
        return this.dequeueStore.pop();
    }

    private shiftToDequeue(): void{
        this.dequeueStore = this.enqueueStore.reverse().concat(this.dequeueStore);
        this.enqueueStore = [];
    }

    peek(numberOfElements: number): T[] | undefined{
        if(numberOfElements > this.size){
            return undefined;
        }
        if(numberOfElements > this.dequeueStore.length){
            this.shiftToDequeue();
        }
        return this.dequeueStore.slice(this.dequeueStore.length - numberOfElements).reverse();
    }
}