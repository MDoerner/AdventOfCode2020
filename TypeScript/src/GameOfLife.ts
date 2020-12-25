import * as Util from "./Util";


export abstract class GameOfLife<T>{
    abstract neighbours(item: T): Iterable<T>;
    abstract flipActive(activeNeighbourCount: number): boolean;
    abstract flipInactive(activeNeighbourCount: number): boolean;

    activeItemsAfterPlaying(initiallyActiveItems: Iterable<T>, roundsToPlay: number): Iterable<T>{
        if(roundsToPlay <= 0){
            return initiallyActiveItems;
        }

        const activeItems: Util.StructSet<T> = new Util.StructSet<T>(initiallyActiveItems);
        let changedItems: Iterable<T> = activeItems;
        for(let round: number = 0; round < roundsToPlay; round++){
            changedItems = this.playRound(activeItems, changedItems);
        }
        return activeItems;
    }

    /**
     * Play a round of game of life.
     *
     * @returns Returns the items that have changed state.
     */
    private playRound(activeItems: Util.StructSet<T>, itemsChangedLastRound: Iterable<T>): Iterable<T>{
        const relevantItems: Util.StructSet<T> = this.relevantItems(itemsChangedLastRound);
        //We need to meterialize before the we can change the states since the filter depends on it.
        const changingItems: T[] = [...Util.filter(relevantItems, (item: T) => this.itemChanges(item, activeItems))];
        this.changeItemStates(changingItems, activeItems);
        return changingItems;
    }

    private relevantItems(itemsChangedLastRound: Iterable<T>): Util.StructSet<T>{
        const items: Util.StructSet<T> = new Util.StructSet<T>();
        for(const item of itemsChangedLastRound){
            items.add(item);
            for(const neighbour of this.neighbours(item)){
                items.add(neighbour);
            }
        }
        return items;
    }

    private itemChanges(item: T, activeItems: Util.StructSet<T>): boolean{
        const activeNeighboursCount: number = this.activeNeighboursCount(item, activeItems);
        if(activeItems.has(item)){
            return this.flipActive(activeNeighboursCount);
        } else {
            return this.flipInactive(activeNeighboursCount);
        }
    }

    private activeNeighboursCount(item: T, activeItems: Util.StructSet<T>){
        return Util.count(
            Util.filter(this.neighbours(item),
                (neighbour: T) => activeItems.has(neighbour))
            );
    }

    private changeItemStates(changingItems: Iterable<T>, activeItems: Util.StructSet<T>): void{
        for(let changingItem of changingItems){
            if(activeItems.has(changingItem)){
                activeItems.delete(changingItem);
            } else {
                activeItems.add(changingItem);
            }
        }
    }
}
