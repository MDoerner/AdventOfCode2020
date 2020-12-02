const fs = require('fs');
const path = require('path');
import type Day from "./Day";

interface NewDay {
    new(): Day<any>
  }

interface PuzzleConfiguration{
    day: number
    part: number
}

function puzzleConfig(args: string[]): PuzzleConfiguration{
    if(args.length < 4){
        return null;
    }

    let argsDay: number = parseInt(args[2])
    let argsPart: number = parseInt(args[3])

    return {day: argsDay, part: argsPart}
}

function puzzleInput(day: number): string{
    let path = puzzlePath(day);
    return inputFromFile(path);
}

function inputFromFile(path: string): string {
    return fs.readFileSync(path, 'utf8');
}

function puzzlePath(day: number): string{
    let basePath = path.join(__dirname, '..', 'Input');
    let filename = "Day" + day + ".txt";
    return path.join(basePath, filename);
}

function puzzleSolver(day: number): Day<any>{
    let dayCreator: NewDay;
    try {
        let imported = require(`./Day${day}`) as any;
        dayCreator = imported.default as NewDay;
    } catch (e) {
        console.log(`Unable to load Day${day}`);
        console.log(e);
        return null;
    }

    return new dayCreator();
}

function puzzleOutput(config: PuzzleConfiguration): string{
    if(!config){
        return "No proper config!";
    }
    
    let input = puzzleInput(config.day);
    let solver = puzzleSolver(config.day);
    
    if(!solver || !input){
        return "";
    }

    let parsedInput = solver.parseInput(input);

    switch(config.part){
        case 1: return solver.solvePart1(parsedInput);
        case 2: return solver.solvePart2(parsedInput);
        default: return "";
    }
}

let config = puzzleConfig(process.argv);
let output = puzzleOutput(config);
console.log(output);