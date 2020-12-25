import Day from "./Day";
import { add, Point, Vector } from "./Plane";
import * as Util from "./Util";

enum Border{
    Upper = 0,
    Right = 1,
    Lower = 2,
    Left = 3,
}

enum Pixel{
    Monster = '#',
    See = '.',
}

/**
 * Describes the a tile is flipped and rotated relative to its original state.
 *
 * Flipping on the vertical axis happens before rotation in the mathematical positive direction.
 */
interface Orientation{
    rotation: number;
    mirrorVertically: boolean;
}

class Tile{
    readonly id: number;
    readonly image: string[];
    readonly borderCodes: number[];
    readonly flipBorderCodes: number[];

    constructor(id: number, image: string[]){
        this.id = id;
        this.image = image;
        this.borderCodes = [];
        this.flipBorderCodes = [];
        const upperBorder: string = this.upperBorder();
        this.borderCodes.push(this.borderCode(upperBorder));
        this.flipBorderCodes.push(this.borderCode(Util.reverseString(upperBorder)));
        const rightBorder: string = this.rightBorder();
        this.borderCodes.push(this.borderCode(rightBorder));
        this.flipBorderCodes.push(this.borderCode(Util.reverseString(rightBorder)));
        const lowerBorder: string = this.lowerBorder();
        this.borderCodes.push(this.borderCode(lowerBorder));
        this.flipBorderCodes.push(this.borderCode(Util.reverseString(lowerBorder)));
        const leftBorder: string = this.leftBorder();
        this.borderCodes.push(this.borderCode(leftBorder));
        this.flipBorderCodes.push(this.borderCode(Util.reverseString(leftBorder)));
    }

    private borderCode(line: string){
        const monsterRegex: RegExp = new RegExp('\\' + Pixel.Monster,"g");
        const seeRegex: RegExp = new RegExp('\\' + Pixel.See,"g");
        const binaryRepresentation: string = line.replace(monsterRegex, "1").replace(seeRegex, "0");
        return parseInt(binaryRepresentation, 2);
    }

    //Borders are read as they appear when they have been reotated to the top.
    private upperBorder(): string{
        return this.image[0];
    }

    private lowerBorder(): string{
        return Util.reverseString(this.image[this.image.length - 1]);
    }

    private leftBorder(): string{
        return this.image.map((line: string) => Array.from(line)[0])
            .reverse()
            .join('');
    }

    private rightBorder(): string{
        return this.image.map((line: string) => Array.from(line)[line.length - 1])
            .join('');
    }

    imageContent(): string[]{
        return this.image
            .map((line: string) => this.lineContent(line))
            .slice(1, this.image.length - 1);
    }

    private lineContent(line: string) : string{
        return Array.from(line).slice(1, line.length - 1).join('');
    }
}

function verticallyMirroredImage(image: string[]): string[]{
    return image.map((line: string) => Util.reverseString(line));
}

function rotatedImage(image: string[], rotation: number): string[]{
    const fullRotationSteps: number = Math.floor(Util.moduloP(rotation, 4));
    let newImage: string[] = image;
    if(fullRotationSteps >= 2){
        newImage = verticallyMirroredImage(image).reverse();
    }
    if(fullRotationSteps % 2 == 0){
        return newImage;
    }
    const characterArray: string[][] = newImage.map((line: string) => Array.from(line));
    newImage = [];
    for(let y: number = 0; y < characterArray.length; y++){
        const newLine: string = characterArray
            .map((line: string[]) => line[characterArray.length - 1 - y])
            .join('');
        newImage.push(newLine);
    }
    return newImage;
}

function reorientedImage(image: string[], orientation: Orientation): string[]{
    const mirrorImage: string[] = orientation.mirrorVertically ? verticallyMirroredImage(image) : image;
    return rotatedImage(mirrorImage, orientation.rotation);
}

function reverseOrientation(orientation: Orientation): Orientation{
    const reverseRotation: number = orientation.mirrorVertically
        ? orientation.rotation
        : Util.moduloP(-orientation.rotation, 4);
    return {rotation: reverseRotation, mirrorVertically: orientation.mirrorVertically};
}

function reorientedCoordinantes(point: Point, orientation: Orientation, width: number, height: number): Point{
    const fullRotationSteps: number = Math.floor(Util.moduloP(orientation.rotation, 4));
    let newX: number = point.x;
    let newY: number = point.y;
    if(orientation.mirrorVertically){
        newX = width - newX - 1;
    }
    if(fullRotationSteps >= 2){
        newX = width - newX - 1;
        newY = height - newY - 1;
    }
    if(fullRotationSteps % 2 == 1){
        let oldX = newX;
        newX = newY;
        newY = height - oldX - 1;
    }
    return {x: newX, y: newY};
}

interface SeeMonsterMask{
    readonly width: number;
    readonly height: number;
    readonly monsterPoints: Util.StructSet<Vector>;
}

class Day20 implements Day<[Map<number,Tile>, SeeMonsterMask]>{
    parseInput(text: string): [Map<number,Tile>, SeeMonsterMask] {
        const parts: string[] = text.split(/\r?\n\r?\n/);
        const seeMonsterMask: SeeMonsterMask = this.parseSeeMonster(parts[0]);
        const tiles: Tile[] = parts.slice(1)
            .map((section: string) => this.parseTile(section));
        const tileMap: Map<number,Tile> = new Map<number,Tile>();
        for(let tile of tiles){
            tileMap.set(tile.id, tile);
        }
        return [tileMap, seeMonsterMask];
    }

    private parseTile(text: string): Tile{
        const lines: string[] = text.split(/\r?\n/);
        const id: number = this.parseTileId(lines[0]);
        const image: string[] = lines.slice(1);
        return new Tile(id, image);
    }

    parseTileId(titleHeader: string): number{
        const pattern: RegExp = /^Tile (\d+):$/;
        const match: RegExpMatchArray | null = titleHeader.match(pattern);
        if(match == null){
            throw Error("Illegal tile header: " + titleHeader);
        }
        return parseInt(match[1]);
    }

    parseSeeMonster(text: string): SeeMonsterMask{
        const pixels: string[][] = text.split(/\r?\n/).map((line: string) => Array.from(line));
        const height: number = pixels.length;
        const width: number = height == 0 ? 0 : pixels[0].length;
        const monsterPoints: Util.StructSet<Vector> = new Util.StructSet<Vector>();
        for(let y: number = 0; y < pixels.length; y++){
            for(let x: number = 0; x < pixels[y].length; x++){
                if(pixels[y][x] == Pixel.Monster){
                    monsterPoints.add({x: x, y: y});
                }
            }
        }
        return {height: height, width: width, monsterPoints: monsterPoints};
    }

    solvePart1(input: [Map<number,Tile>, SeeMonsterMask]): string {
        const tilesByBorderCode: Map<number, number[]> = this.tilesByBorderCode(input[0]);
        const cornerIds: number[] = this.cornerIds(tilesByBorderCode);
        const result: number = cornerIds.reduce((product: number, id: number) => product * id, 1);
        return result.toString();
    }

    private tilesByBorderCode(tilesById: Map<number,Tile>): Map<number, number[]>{
        const tilesByBorderCode: Map<number, number[]> = new Map<number, number[]>();
        for(let tile of tilesById.values()){
            for(let borderCode of tile.borderCodes){
                Util.addToValueList(borderCode, tile.id, tilesByBorderCode);
            }
            for(let borderCode of tile.flipBorderCodes){
                Util.addToValueList(borderCode, tile.id, tilesByBorderCode);
            }
        }
        return tilesByBorderCode;
    }

    private cornerIds(tilesByBorderCode: Map<number, number[]>): number[]{
        const borderTilesWithBorderBoderCodes: Map<number, number[]> = this.borderTilesWithBorderBoderCodes(tilesByBorderCode);
        const cornerIds: number[] = [];
        borderTilesWithBorderBoderCodes.forEach((borderCodes: number[], tileId: number) => {
            if(borderCodes.length == 4){
                cornerIds.push(tileId);
            }
        });
        return cornerIds;
    }

    private borderTilesWithBorderBoderCodes(tilesByBorderCode: Map<number, number[]>): Map<number, number[]>{
        const borderBorderCodesWithTiles: Map<number, number> = this.borderBorderCodesWithTiles(tilesByBorderCode);
        const bordersBorderCodesByTiles: Map<number, number[]> = new Map<number, number[]>();
        borderBorderCodesWithTiles.forEach((tileId: number, borderCode: number) => {
            Util.addToValueList(tileId, borderCode, bordersBorderCodesByTiles);
        });
        return bordersBorderCodesByTiles;
    }

    private borderBorderCodesWithTiles(tilesByBorderCode: Map<number, number[]>): Map<number, number>{
        const uniqueBorderTiles: Map<number, number> = new Map<number, number>();
        tilesByBorderCode.forEach((tileIds: number[], borderCode: number) => {
            if(tileIds.length == 1){
                uniqueBorderTiles.set(borderCode, tileIds[0]);
            }
        });
        return uniqueBorderTiles;
    }

    solvePart2(input: [Map<number,Tile>, SeeMonsterMask]): string {
        const satteliteImage: string[] = this.assembledImage(input[0]);
        const satteliteImageMonsterPixels: boolean[][] = satteliteImage.map((line: string) => Array.from(line).map((character: string) => character == Pixel.Monster));
        const monsterPixelCount: number = satteliteImageMonsterPixels.reduce((count: number, nextLine: boolean[]) =>
            count + nextLine.reduce((count: number, nextPixel: boolean) =>
                count + (nextPixel ? 1 : 0),
                0),
            0);
        const pointsWithMonsters: Util.StructSet<Point> = this.pointsWithMonsters(satteliteImageMonsterPixels, input[1]);
        const result: number = monsterPixelCount - pointsWithMonsters.size;

        //This is primarily a debug measure.
        const imageWithSeeMonsters = this.imageWithSeeMonsters(satteliteImageMonsterPixels, pointsWithMonsters);
        const displayImage: string = verticallyMirroredImage(rotatedImage(satteliteImage, 3)).join('\n');
        const displayImageWithMonsters: string = verticallyMirroredImage(rotatedImage(imageWithSeeMonsters, 3)).join('\n');
        console.log(displayImage);
        console.log('\n');
        console.log(displayImageWithMonsters);

        return result.toString();
    }

    private imageWithSeeMonsters(imagePixels: boolean[][], pointsWithMonsters: Util.StructSet<Point>): string[]{
        return imagePixels.map((line: boolean[], y: number) => line.map((isMonsterPixes: boolean, x: number) => {
            if(!isMonsterPixes){
                return '.';
            }
            if(pointsWithMonsters.has({x: x, y:y})){
                return 'O';
            }
            return '#';
        }).join(''));
    }

    private pointsWithMonsters(imagePixels: boolean[][], monsterMask: SeeMonsterMask): Util.StructSet<Point>{
        const monsterPoints: Util.StructSet<Point> = new Util.StructSet<Point>();
        const possibleOrientations: Orientation[] = this.possibleOrientations();
        for(let orientation of possibleOrientations){
            this.addMonstersForOrientation(orientation, imagePixels, monsterMask, monsterPoints);
        }
        return monsterPoints;
    }

    private possibleOrientations(): Orientation[]{
        const orientations: Orientation[] = [];
        for(let rotation = 0; rotation < 4; rotation++){
            orientations.push({rotation: rotation, mirrorVertically: false});
            orientations.push({rotation: rotation, mirrorVertically: true});
        }
        return orientations;
    }

    private addMonstersForOrientation(orientation: Orientation, imagePixels: boolean[][], monsterMask: SeeMonsterMask, monsterPoints: Util.StructSet<Point>){
        if(imagePixels.length == 0){
            return;
        }
        const transformedWidth: number = orientation.rotation % 2 == 0 ? imagePixels[0].length : imagePixels.length;
        const transformedHeight: number = orientation.rotation % 2 == 0 ? imagePixels.length : imagePixels[0].length;
        for(let y: number = 0; y <= transformedHeight - monsterMask.height; y++){
            for(let x: number = 0; x <= transformedWidth - monsterMask.width; x++){
                const basePoint: Point = {x: x, y: y};
                if(this.monsterMaskMatches(basePoint, monsterMask, orientation, transformedHeight, transformedWidth, imagePixels)){
                    this.addPoints(basePoint, monsterMask, orientation, transformedHeight, transformedWidth, monsterPoints);
                }
            }
        }
    }

    private monsterMaskMatches(basePoint: Point, mask: SeeMonsterMask, orientation: Orientation, height: number, width: number, imagePixels: boolean[][]): boolean{
        for(let offset of mask.monsterPoints){
            const point: Point = reorientedCoordinantes(add(basePoint, offset), orientation, imagePixels[0].length, imagePixels.length);
            if(!imagePixels[point.y][point.x]){
                return false;
            }
        }
        return true;
    }

    private addPoints(basePoint: Point, mask: SeeMonsterMask, orientation: Orientation, height: number, width: number, monsterPoints: Util.StructSet<Point>): void{
        for(let offset of mask.monsterPoints){
            const point: Point = reorientedCoordinantes(add(basePoint, offset), orientation, width, height);
            monsterPoints.add(point);
        }
    }

    private assembledImage(tilesById: Map<number,Tile>): string[]{
        const tiles: [Tile, Orientation][][] = this.arrangedTiles(tilesById);
        const baseImages: [string[], Orientation][][] = tiles.map((line: [Tile, Orientation][]) =>
            line.map((tileOrientationPair: [Tile, Orientation]) =>
                [tileOrientationPair[0].imageContent(), tileOrientationPair[1]]));
        const arrangedImages: string[][][] = baseImages.map((line: [string[], Orientation][]) =>
            line.map((imageOrientationPair: [string[], Orientation]) =>
                reorientedImage(imageOrientationPair[0], imageOrientationPair[1])));
        return arrangedImages
            .map((images: string[][]) => this.joinLines(images))
            .reduce((image: string[], nextImagePart: string[]) => image.concat(nextImagePart), []);
    }

    /**
     * @param images All images have to have the same height.
     */
    private joinLines(images: string[][]): string[]{
        if(images.length == 0){
            return [];
        }
        const newImage: string[] = [];
        for(let rowIndex in images[0]){
            const nextRow: string = images.map((image: string[]) => image[rowIndex]).join('');
            newImage.push(nextRow);
        }
        return newImage;
    }

    //Always mirror verticall first, then rotate counter-clockwise.
    private arrangedTiles(tilesById: Map<number,Tile>): [Tile, Orientation][][]{
        const tilesByBorderCode: Map<number, number[]> = this.tilesByBorderCode(tilesById);
        const borderTilesWithBorderBoderCodes: Map<number, number[]> = this.borderTilesWithBorderBoderCodes(tilesByBorderCode);
        const cornerIds: number[] = this.cornerIds(tilesByBorderCode);
        const firstRow: [Tile, Orientation][] = this.arrangedFirstRow(cornerIds[0], tilesById, borderTilesWithBorderBoderCodes, tilesByBorderCode);
        const rows: [Tile, Orientation][][] = [firstRow];
        let currentRow: [Tile, Orientation][] | null = firstRow;
        while(currentRow != null){
            const currentLowerBorderCodes: number[] = currentRow.map((tileOrientationPair: [Tile, Orientation]) => this.borderCode(Border.Lower, tileOrientationPair[0], tileOrientationPair[1]));
            const startTile: [Tile, Orientation] | null = this.borderTileInDirection(currentRow[0][0], currentRow[0][1], Border.Lower, Border.Left, tilesByBorderCode, tilesById, borderTilesWithBorderBoderCodes);
            if(startTile == null){
                return rows;
            }
            currentRow = this.arrangedSubsequentRow(startTile, currentLowerBorderCodes, tilesById, tilesByBorderCode);
            if(currentRow == null){
                return rows;
            }
            rows.push(currentRow);
        }
        return rows;
    }

    private arrangedFirstRow(startCornerId: number, tilesById: Map<number,Tile>, borderTilesWithBorderBoderCodes: Map<number, number[]>, tilesByBorderCode: Map<number, number[]>): [Tile, Orientation][]{
        const leftUpperCorner: Tile = tilesById.get(startCornerId) as Tile;
        const leftUpperRotation: number = this.upperLeftCornerRotation(leftUpperCorner, borderTilesWithBorderBoderCodes);
        const leftUpperOrientation: Orientation = {rotation: leftUpperRotation, mirrorVertically: false};
        const rowTiles: [Tile, Orientation][] = [[leftUpperCorner, leftUpperOrientation]];
        let currentTile: [Tile, Orientation] | null = rowTiles[0];
        while(currentTile != null){
            currentTile = this.borderTileInDirection(currentTile[0], currentTile[1], Border.Right, Border.Upper, tilesByBorderCode, tilesById, borderTilesWithBorderBoderCodes);
            if(currentTile != null){
                rowTiles.push(currentTile);
            }
        }
        return rowTiles;
    }

    private upperLeftCornerRotation(upperLeftCorner: Tile, borderTilesWithBorderBoderCodes: Map<number, number[]>): number{
        const borderBorderCodes: number[] = borderTilesWithBorderBoderCodes.get(upperLeftCorner.id) as number[];
        const borderIndeces: number[] = borderBorderCodes.map((code: number) => upperLeftCorner.borderCodes.indexOf(code))
            .filter((index: number) => index >= 0)
            .sort();
        if(borderIndeces[0] == Border.Upper && borderIndeces[1] == Border.Left){
            return 0;
        }
        return borderIndeces[0] + 1;
    }

    private borderTileInDirection(tile: Tile, orientation: Orientation, direction: Border, borderDirection: Border, tilesByBorderCode: Map<number, number[]>, tilesById: Map<number,Tile>, borderTilesWithBorderBoderCodes: Map<number, number[]>): [Tile, Orientation] | null{
        const borderCodeInDirection: number = this.borderCode(direction, tile, orientation);
        const otherTilesWithBorderCode: number[] = (tilesByBorderCode.get(borderCodeInDirection) as number[]).filter((id: number) => id != tile.id);
        if(otherTilesWithBorderCode.length == 0){
            return null;
        }
        const borderTileInDirection: Tile = tilesById.get(otherTilesWithBorderCode[0]) as Tile;
        if(!borderTilesWithBorderBoderCodes.has(borderTileInDirection.id)){
            //Not a border tile.
            return null;
        }
        const borderBorderCodes: number[] = borderTilesWithBorderBoderCodes.get(borderTileInDirection.id) as number[];
        const borderTileOrientation: Orientation | null = this.borderTileOrientation(borderTileInDirection, borderCodeInDirection, Util.moduloP(direction + 2, 4), borderDirection, borderBorderCodes);
        if(borderTileOrientation == null){
            return null;
        }
        return [borderTileInDirection, borderTileOrientation];
    }

    private borderCode(border: Border, tile: Tile, orientation: Orientation){
        const borderIndexBeforeRotation: number = (border + orientation.rotation) % 4;
        if(orientation.mirrorVertically){
            if(borderIndexBeforeRotation % 2 == 0){
                return tile.flipBorderCodes[borderIndexBeforeRotation];
            } else {
                return tile.flipBorderCodes[(borderIndexBeforeRotation + 2) % 4];
            }
        } else {
            return tile.borderCodes[borderIndexBeforeRotation];
        }
    }

    private borderTileOrientation(tile: Tile, flipBorderCode: number, codeDirection: Border, borderDirection: Border, flipBorderBorderCodes: number[]): Orientation | null{
        for(let code of flipBorderBorderCodes){
            const orientation = this.tileOrientation(tile, [flipBorderCode, code], [codeDirection, borderDirection]);
            if(orientation != null){
                return orientation;
            }
        }
        return null;
    }

    /**
     * The directions must have a common corner.
     */
    private tileOrientation(tile: Tile, flipBorderCodes: [number, number], codeDirections: [Border, Border]): Orientation | null{
        //Same or opposite directions.
        if(codeDirections[0] % 2 == codeDirections[1] % 2){
            return null;
        }
        const flipCodeDirections = flipBorderCodes.map((code: number) => tile.flipBorderCodes.indexOf(code));
        if(flipCodeDirections[0] < 0 && flipCodeDirections[1] >= 0
            || flipCodeDirections[0] >= 0 && flipCodeDirections[1] < 0){
            //We can only flip all borders together.
            return null;
        }
        if(flipCodeDirections[0] >= 0){
            if(Util.moduloP(flipCodeDirections[0] - flipCodeDirections[1], 4) != Util.moduloP(codeDirections[0] - codeDirections[1], 4)){
                //Switching the orientation reqiures flipping the borders.
                return null;
            }
            const rotation: number = Util.moduloP(flipCodeDirections[0] - codeDirections[0], 4);
            return {rotation: rotation, mirrorVertically: false};
        }
        const directions = flipBorderCodes.map((code: number) => tile.borderCodes.indexOf(code));
        if(directions[0] < 0 || directions[1] < 0){
            //One of the codes does not exist on the tile at all.
            return null;
        }
        if(Util.moduloP(directions[1] - directions[0], 4) != Util.moduloP(codeDirections[0] - codeDirections[1], 4)){
            //Flipping reverses the orientation.
            return null;
        }
        const rotation: number =   Util.moduloP(Util.moduloP(-directions[0], 4) - codeDirections[0], 4);
        return {rotation: rotation, mirrorVertically: true};
    }

    private arrangedSubsequentRow(
        startTile: [Tile, Orientation],
        previousRowLowerBorderCodes: number[],
        tilesById: Map<number,Tile>,
        tilesByBorderCode: Map<number, number[]>
    ): [Tile, Orientation][] | null{
        const rowTiles: [Tile, Orientation][] = [startTile];
        let currentTile: [Tile, Orientation] | null = startTile;
        for(let upperBorderIndex: number = 1; upperBorderIndex < previousRowLowerBorderCodes.length; upperBorderIndex++){
            currentTile = this.tileInDirection(currentTile[0], currentTile[1], Border.Right, Border.Upper, previousRowLowerBorderCodes[upperBorderIndex], tilesByBorderCode, tilesById);
            if(currentTile == null){
                return null;
            }
            rowTiles.push(currentTile);
        }
        return rowTiles;
    }

    private tileInDirection(
        tile: Tile,
        orientation: Orientation,
        direction: Border,
        referenceDirection: Border,
        referenceBorderCode: number,
        tilesByBorderCode: Map<number, number[]>,
        tilesById: Map<number,Tile>
    ): [Tile, Orientation] | null{
        const borderCodeInDirection: number = this.borderCode(direction, tile, orientation);
        const otherTilesWithBorderCode: number[] = (tilesByBorderCode.get(borderCodeInDirection) as number[]).filter((id: number) => id != tile.id);
        if(otherTilesWithBorderCode.length == 0){
            return null;
        }
        const tileInDirection: Tile = tilesById.get(otherTilesWithBorderCode[0]) as Tile;
        const tileOrientation: Orientation | null = this.tileOrientation(tileInDirection, [borderCodeInDirection, referenceBorderCode], [Util.moduloP(direction + 2, 4), referenceDirection]);
        if(tileOrientation == null){
            return null;
        }
        return [tileInDirection, tileOrientation];
    }

}


export default Day20;