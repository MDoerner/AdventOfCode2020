import Day from "./Day";
import * as Util from "./Util";

enum InstructionType{
    setMask = 'mask',
    setMemory = 'mem',
}

interface Instruction{
    type: InstructionType;
    address: bigint;
    value: bigint;
}

class Day14 implements Day<Instruction[]>{
    parseInput(text: string): Instruction[] {
        return text.split(/\r?\n/)
            .map((line: string) => this.parseInstruction(line))
            .filter(Util.notEmpty);
    }

    private parseInstruction(text: string): Instruction | null{
        const pattern: RegExp = /^(mask = ((X|\d)+)|mem\[(\d+)\] = (\d+))$/;
        const match: RegExpMatchArray | null = text.match(pattern);
        if(!match){
            return null;
        }

        if(match[2] != undefined){
            const mask: [bigint, bigint] = this.parseBitMask(match[2]);
            return {type: InstructionType.setMask, address: mask[0], value: mask[1]};
        }

        if(match[4] != undefined){
            const address: bigint = BigInt(match[4]);
            const value: bigint = BigInt(match[5]);
            return {type: InstructionType.setMemory, address: address, value: value};
        }

        return null;
    }

    private parseBitMask(maskText: string): [bigint, bigint]{
        return [this.parseZeroMask(maskText), this.parseOneMask(maskText)];
    }

    private parseOneMask(maskText: string): bigint{
        return BigInt('0b' + maskText.replace(/X/g, '1'));
    }

    private parseZeroMask(maskText: string): bigint{
        return BigInt('0b' + maskText.replace(/X/g, '0'));
    }

    solvePart1(input: Instruction[]): string {
        const computer: DockingComputer = new DockingComputer();
        computer.execute(input);
        const finalMemory: [bigint, bigint][] = computer.nonZeroMemoryContent();
        const result: bigint = finalMemory.reduce((valueSum: bigint, addressMemoryPair: [bigint, bigint]) => valueSum + addressMemoryPair[1], 0n);
        return result.toString();
    }

    solvePart2(input: Instruction[]): string {
        const computer: DockingComputer = new DockingComputerMark2();
        computer.execute(input);
        const finalMemory: [bigint, bigint][] = computer.nonZeroMemoryContent();
        const result: bigint = finalMemory.reduce((valueSum: bigint, addressMemoryPair: [bigint, bigint]) => valueSum + addressMemoryPair[1], 0n);
        return result.toString();
    }
}

class DockingComputer{
    private _mask: [bigint, bigint];
    protected _memory: Map<bigint,bigint> = new Map<bigint, bigint>();

    get zeroMask(): bigint{
        return this._mask[0];
    }

    get oneMask(): bigint{
        return this._mask[1];
    }

    constructor(initialZeroMask : bigint = 0n, initialOneMask: bigint = 0n){
        this._mask = [initialZeroMask, initialOneMask];
    }

    memory(address: bigint){
        if(!this._memory.has(address)){
            return 0;
        }
        return this._memory.get(address) as bigint;
    }

    protected setMemory(address: bigint, value: bigint): void{
        const valueToStore: bigint = (this.oneMask | value) & this.zeroMask;
        this.storeValueInMemory(address, valueToStore);
    }

    protected storeValueInMemory(address: bigint, value: bigint){
        if(value == 0n){
            if(this._memory.has(address))
            {
                this._memory.delete(address);
            }
        }
        this._memory.set(address, value);
    }

    nonZeroMemoryContent(): [bigint, bigint][]{
        let memoryContents: [bigint, bigint][] = [];
        for(let memoryItem of this._memory.entries()){
            memoryContents.push(memoryItem);
        }
        return memoryContents;
    }

    execute(code: Instruction[]): void{
        for(let instruction of code){
            this.executeInstruction(instruction);
        }
    }

    private executeInstruction(instruction: Instruction): void{
        switch(instruction.type){
            case InstructionType.setMask:
                this.setMask(instruction.address, instruction.value);
                break;
            case InstructionType.setMemory:
                this.setMemory(instruction.address, instruction.value);
                break;
        }
    }

    private setMask(zeroMask:bigint, oneMask: bigint): void{
        this._mask = [zeroMask, oneMask];
    }
}

class DockingComputerMark2 extends DockingComputer{
    private readonly NUMBER_OF_VALID_BITS: number = 36;

    get floatingMask(): bigint{
        return this.zeroMask ^ this.oneMask;
    }

    protected setMemory(address: bigint, value: bigint): void{
        const fixedAddress: bigint = (this.oneMask | address) & ~this.floatingMask;
        const floatingAddressParts: bigint[] = this.bitCombinations(this.floatingMask);
        for(let floatingAddress of floatingAddressParts){
            this.storeValueInMemory(fixedAddress | floatingAddress, value);
        }
    }

    private bitCombinations(mask: bigint): bigint[]{
        const bits: bigint[] = this.bits(mask);
        let combinations: bigint[] = [0n];
        for(let bit of bits){
            const newCombinations: bigint[] = combinations.map((combination: bigint) => combination | bit);
            combinations = combinations.concat(newCombinations);
        }
        return combinations
    }

    private bits(mask: bigint): bigint[]{
        let maskBits: bigint[] = [];
        let testBit: bigint = 1n;
        for(let powerOfTwo: number = 0; powerOfTwo < this.NUMBER_OF_VALID_BITS; powerOfTwo++){
            if(testBit & mask){
                maskBits.push(testBit);
            }
            testBit = testBit << 1n;
        }
        return maskBits;
    }
}


export default Day14;