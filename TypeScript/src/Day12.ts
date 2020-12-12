import Day from "./Day";
import * as Util from "./Util";
import * as Plane from "./Plane";

enum InstructionType{
    east= 'E',
    north = 'N',
    west = 'W',
    south = 'S',
    right = 'R',
    left = 'L',
    forward = 'F',
}

interface NavigationInstruction{
    type: InstructionType;
    amplitude: number;
}

interface DirectionalShipState{
    position: Plane.Point;
    orientation: Plane.AngleDirection;
}

interface WaypointShipState{
    ship: Plane.Point;
    waypoint: Plane.Vector;
}

class Day12 implements Day<NavigationInstruction[]>{
    parseInput(text: string): NavigationInstruction[] {
        return text.split(/\r?\n/)
            .map((line: string) => this.parseInstruction(line))
            .filter(Util.notEmpty);
    }

    private parseInstruction(text: string): NavigationInstruction | null{
        const pattern: RegExp = /(\w)(\d+)/;
        const matches = text.match(pattern);
        if(!matches || !<any>Object.values(InstructionType).includes(matches[1] as InstructionType)){
            return null;
        }

        const potentialInstructionType: InstructionType = matches[1] as InstructionType;
        //check whether the string value exists on the enum
        if(!Object.values(InstructionType).includes(potentialInstructionType)){
            return null;
        }
        const amplitude: number = parseInt(matches[2]);
        return {type: potentialInstructionType, amplitude: amplitude};
    }

    solvePart1(input: NavigationInstruction[]): string {
        const initialPosition: Plane.Point = {x: 0, y: 0};
        const initialOrientation: Plane.AngleDirection = new Plane.AngleDirection(0);
        let shipState: DirectionalShipState = {position: initialPosition, orientation: initialOrientation};
        for(let instruction of input){
            shipState = this.executeDirectionalInstruction(shipState, instruction);
        }
        const finalDistance = Plane.manhattanDistance(initialPosition, shipState.position);
        return finalDistance.toString();
    }

    private executeDirectionalInstruction(shipState: DirectionalShipState, instruction: NavigationInstruction): DirectionalShipState{
        const newPosition: Plane.Point = Plane.move(shipState.position, this.movementDirection(instruction.type, shipState), instruction.amplitude);
        const newOrientation: Plane.AngleDirection = shipState.orientation.addAngle(this.angleChange(instruction));
        return {position: newPosition, orientation: newOrientation};
    }

    private movementDirection(moventType: InstructionType, shipState: DirectionalShipState): Plane.Direction{
        if(moventType == InstructionType.forward){
            return shipState.orientation;
        }

        return this.absoluteMovementDirection(moventType);
    }

    private absoluteMovementDirection(moventType: InstructionType): Plane.Direction{
        switch(moventType){
            case InstructionType.east:
                return {x: 1, y: 0};
            case InstructionType.north:
                return {x: 0, y: 1};
            case InstructionType.west:
                return {x: -1, y: 0};
            case InstructionType.south:
                return {x: 0, y: -1};
            default:
                return {x: 0, y: 0};
        }
    }

    private angleChange(instruction: NavigationInstruction): number{
        switch(instruction.type){
            case InstructionType.left:
                return instruction.amplitude;
            case InstructionType.right:
                return -instruction.amplitude;
            default:
                return 0;
        }
    }

    solvePart2(input: NavigationInstruction[]): string {
        const initialPosition: Plane.Point = {x: 0, y: 0};
        const initialWaypoint: Plane.Point = {x: 10, y: 1};
        let shipState: WaypointShipState = {ship: initialPosition, waypoint: initialWaypoint};
        for(let instruction of input){
            shipState = this.executeWaypointInstruction(shipState, instruction);
        }
        const finalDistance = Plane.manhattanDistance(initialPosition, shipState.ship);
        return finalDistance.toString();
    }

    private executeWaypointInstruction(shipState: WaypointShipState, instruction: NavigationInstruction): WaypointShipState{
        if(instruction.type == InstructionType.forward){
            const newShipPosition: Plane.Point = Plane.move(shipState.ship, shipState.waypoint, instruction.amplitude);
            return {ship: newShipPosition, waypoint: shipState.waypoint};
        }

        let newWaypoint: Plane.Vector;
        switch(instruction.type){
            case InstructionType.east:
            case InstructionType.north:
            case InstructionType.west:
            case InstructionType.south:
                newWaypoint = Plane.move(shipState.waypoint, this.absoluteMovementDirection(instruction.type), instruction.amplitude);
                break;
            case InstructionType.left:
            case InstructionType.right:
                newWaypoint = Plane.rotatedVector(shipState.waypoint, this.angleChange(instruction));
                break;
            default:
                newWaypoint = Plane.copyVector(shipState.waypoint);
                break;
        }

        return {ship: shipState.ship, waypoint: newWaypoint};
    }
}

export default Day12;
