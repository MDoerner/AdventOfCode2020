import Day from "./Day";
import * as Util from "./Util";


class Day22 implements Day<Util.Queue<number>[]>{
    parseInput(text: string): Util.Queue<number>[] {
        return text.split(/\r?\n\r?\n/)
            .map((deckText: string) => this.parseDeck(deckText));
    }

    parseDeck(text: string): Util.Queue<number>{
        const cards: number[] = text.split(/\r?\n/)
            .slice(1)
            .map((cardText: string) => parseInt(cardText));
        return new Util.Queue<number>(cards);
    }

    solvePart1(input: Util.Queue<number>[]): string {
        this.playGame(input);
        const scores: number[] = input.map((deck: Util.Queue<number>) => this.deckScore(deck));
        const result: number = Math.max(...scores);
        return result.toString();
    }

    private playGame(decks: Util.Queue<number>[]): void{
        while(!this.gameHasEnded(decks)){
            this.playRound(decks);
        }
    }

    private gameHasEnded(decks: Util.Queue<number>[]): boolean{
        return decks.reduce((aDeckIsEmpty: boolean, nextDeck: Util.Queue<number>) => aDeckIsEmpty || (nextDeck.size == 0), false);
    }

    /**
     * @param decks None of the decks may be empty, i.e. the game must not have ended.
     */
    private playRound(decks: Util.Queue<number>[]): void{
        const cardsPlayed: number[] = decks.map((deck: Util.Queue<number>) => (deck.dequeue() as number));
        const gameResult: [number, number[]] = this.evaluateRound(cardsPlayed);
        const winningDeck: Util.Queue<number> = decks[gameResult[0]];
        const cardsToAdd: number[] = gameResult[1];
        for(let card of cardsToAdd){
            winningDeck.enqueue(card);
        }
    }

    /**
     * @returns Returns a pair consisting of the winning player's index and the cards to add to his deck.
     */
    private evaluateRound(cardsPlayed: number[]): [number, number[]]{
        const maxCard: number = Math.max(...cardsPlayed);
        const winningPlayerIndex: number = cardsPlayed.indexOf(maxCard);
        const cardsToAdd: number[] = cardsPlayed.sort((a: number, b: number) => b - a);
        return [winningPlayerIndex, cardsToAdd];
    }

    /**
     * Scored dacks are empty afterwards.
     */
    private deckScore(deck: Util.Queue<number>){
        let score: number = 0;
        for(let scoreMultiplier: number = deck.size; scoreMultiplier > 0; scoreMultiplier--){
            score += scoreMultiplier * (deck.dequeue() as number);
        }
        return score;
    }

    solvePart2(input: Util.Queue<number>[]): string {
        this.playRecursiveGame(input);
        const scores: number[] = input.map((deck: Util.Queue<number>) => this.deckScore(deck));
        const result: number = Math.max(...scores);
        return result.toString();
    }

    /**
     * @returns Returns whether the game has been played to the end.
     */
    private playRecursiveGame(decks: Util.Queue<number>[]): boolean{
        const alreadyEncounteredConfigurations: Set<string> = new Set<string>();
        while(!this.gameHasEnded(decks)){
            const curentConfiguration: string = this.configurationKey(decks);
            if(alreadyEncounteredConfigurations.has(curentConfiguration)){
                return false;
            }
            alreadyEncounteredConfigurations.add(curentConfiguration);
            this.playRecursiveRound(decks);
        }
        return true;
    }

    private configurationKey(decks: Util.Queue<number>[]): string{
        return decks.map((deck: Util.Queue<number>) => this.deckKey(deck))
            .join('|');
    }

    private deckKey(deck: Util.Queue<number>): string{
        return (deck.peek(deck.size) as number[])
            .map((card: number) => card.toString())
            .join(',');
    }

    private playRecursiveRound(decks: Util.Queue<number>[]): void{
        const cardsPlayed: number[] = decks.map((deck: Util.Queue<number>) => (deck.dequeue() as number));
        const gameResult: [number, number[]] = this.evaluateRecursiveRound(cardsPlayed, decks);
        const winningDeck: Util.Queue<number> = decks[gameResult[0]];
        const cardsToAdd: number[] = gameResult[1];
        for(let card of cardsToAdd){
            winningDeck.enqueue(card);
        }
    }

    private canPlayRecursiveGame(cardsPlayed: number[], deckSizes: number[]): boolean{
        for(let index in cardsPlayed){
            const deckSize: number | undefined = deckSizes[index];
            if(deckSizes == undefined || cardsPlayed[index] > deckSize){
                return false;
            }
        }
        return true;
    }

    private recursiveDecks(cardsPlayed: number[], decks: Util.Queue<number>[]): Util.Queue<number>[]{
        return decks.map((deck: Util.Queue<number>, index: number) => new Util.Queue<number>(deck.peek(cardsPlayed[index])));
    }

    private evaluateRecursiveRound(cardsPlayed: number[], decks: Util.Queue<number>[]): [number, number[]]{
        const deckSizes: number[] = decks.map((deck: Util.Queue<number>) => deck.size);
        if(this.canPlayRecursiveGame(cardsPlayed, deckSizes)){
            const recursiveDecks: Util.Queue<number>[] = this.recursiveDecks(cardsPlayed, decks);
            const gamePlayedToTheEndWithoutrepetition: boolean = this.playRecursiveGame(recursiveDecks);
            if(gamePlayedToTheEndWithoutrepetition){
                return this.evaluateRecursiveGame(cardsPlayed, recursiveDecks);
            } else {
                return [0, cardsPlayed];
            }
        } else {
            return this.evaluateRound(cardsPlayed);
        }
    }

    /**
     * @param recursiveDecks Consumes the recursive decks.
    *  @returns Returns a pair consisting of the winning player's index and the cards to add to his deck.
    */
   private evaluateRecursiveGame(cardsPlayed: number[], recursiveDecks: Util.Queue<number>[]): [number, number[]]{
        const scores: number[] = recursiveDecks.map((deck: Util.Queue<number>) => this.deckScore(deck));
        const winningScore: number = Math.max(...scores);
        const winningPlayerIndex: number = scores.indexOf(winningScore);
        const cardsToAdd: number[] = cardsPlayed.map((card: number, index: number) => [card, scores[index]])
            .sort((a: number[], b: number[]) => b[1] - a[1])
            .map((cardScorePair: number[]) => cardScorePair[0]);
       return [winningPlayerIndex, cardsToAdd];
   }
}


export default Day22;