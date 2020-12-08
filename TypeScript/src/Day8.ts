import Day from "./Day";

enum OpCode {
    noOp = 'nop',
    accumulate = 'acc',
    jump = 'jmp'
}

interface Instruction{
    action: OpCode;
    argument: number;
}

enum ExecutionResult{
    terminated,
    loopDetected,
    accessViolation
}

class Day8 implements Day<Instruction[]>{
    parseInput(text: string): Instruction[] {
        return text.split(/\r?\n/)
            .map((line: string) => this.parseInstruction(line))
            .filter(notEmpty);
    }

    private parseInstruction(instructionText: string): Instruction | null{
        const pattern: RegExp = /(nop|acc|jmp) ((\+|-)\d+)/;
        const match = instructionText.match(pattern);
        if(!match){
            return null;
        }

        const argument: number = parseInt(match[2]);
        const opCode: OpCode = match[1] as OpCode;

        return {action: opCode, argument: argument};
    }

    solvePart1(input: Instruction[]): string {
        const handheld: NonLoopingHandheld = new NonLoopingHandheld(input);
        const stoppingAccumulator: number = handheld.execute(0, 0)[1];
        return stoppingAccumulator.toString();
    }

    solvePart2(input: Instruction[]): string {
        const handheld: NonLoopingHandheld = new NonLoopingHandheld(input);

        for(let instruction of input){
            let executionResult: [ExecutionResult, number];
            switch(instruction.action){
                case OpCode.noOp:
                    instruction.action = OpCode.jump;
                    executionResult = handheld.execute(0, 0);
                    if(executionResult[0] == ExecutionResult.terminated){
                        return executionResult[1].toString();
                    }
                    instruction.action = OpCode.noOp;
                    break;
                case OpCode.jump:
                    instruction.action = OpCode.noOp;
                    executionResult = handheld.execute(0, 0);
                    if(executionResult[0] == ExecutionResult.terminated){
                        return executionResult[1].toString();
                    }
                    instruction.action = OpCode.jump;
                    break;
            }
        }

        return "Corruption not found!";
    }
}

class HandheldState{
    accumulator: number;
    instructionPointer: number;

    constructor(accumulator: number = 0, instructionPointer: number = 0){
        this.accumulator = accumulator;
        this.instructionPointer = instructionPointer;
    }

    noOp(input: number){
        this.instructionPointer++;
    }

    accumulate(input: number){
        this.accumulator += input;
        this.instructionPointer++;
    }

    jump(input: number){
        this.instructionPointer += input;
    }
}

class NonLoopingHandheld{
    private readonly code: Instruction[];

    constructor(code: Instruction[]){
        this.code = code;
    }

    execute(entryPoint: number = 0, initialAccumulator: number = 0): [ExecutionResult, number]{
        const state: HandheldState = new HandheldState(initialAccumulator, entryPoint);
        const visitedInstructions: Set<number> = new Set<number>();

        while(!visitedInstructions.has(state.instructionPointer)){
            visitedInstructions.add(state.instructionPointer);
            if(state.instructionPointer == this.code.length){
                return [ExecutionResult.terminated, state.accumulator];
            }
            if(state.instructionPointer < 0 || state.instructionPointer > this.code.length){
                return [ExecutionResult.accessViolation, state.accumulator];
            }
            const currentInstruction: Instruction = this.code[state.instructionPointer];
            this.executeInstruction(currentInstruction, state);
        }

        return [ExecutionResult.loopDetected, state.accumulator];
    }

    private executeInstruction(instruction: Instruction, state: HandheldState){
        switch(instruction.action){
            case OpCode.noOp:
                state.noOp(instruction.argument);
                return;
            case OpCode.accumulate:
                state.accumulate(instruction.argument);
                return;
            case OpCode.jump:
                state.jump(instruction.argument);
                return;
        }
    }
}

function notEmpty<TValue>(value: TValue | null | undefined): value is TValue {
    return value !== null && value !== undefined;
}

export default Day8;