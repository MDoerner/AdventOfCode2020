import Day from "./Day";

interface TravelGroup{
    customsForms: string[]
}

class Day6 implements Day<TravelGroup[]>{
    parseInput(text: string): TravelGroup[] {
        return text.split(/\r?\n\r?\n/)
            .map((groupSection: string) => {return {customsForms: groupSection.split(/\r?\n/)}});
    }

    solvePart1(input: TravelGroup[]): string {
        let groupCustomsFormItems: Set<string>[] = input.map((travelGroup: TravelGroup) => this.uniqueItems(travelGroup.customsForms));
        let groupItemsSum: number = groupCustomsFormItems
            .map((uniqueItems: Set<string>) => uniqueItems.size)
            .reduce((result: number, currentSize: number) => result + currentSize);
        return groupItemsSum.toString();
    }

    private uniqueItems(customsForms: string[]): Set<string>{
        let items = new Set<string>();
        customsForms.forEach((customsForm: string) => this.addItems(customsForm, items));
        return items;
    }

    private addItems(customsForm: string, set: Set<string>){
        Array.from(customsForm.trim()).forEach((character: string) => set.add(character));
    }

    solvePart2(input: TravelGroup[]): string {
        let groupCustomsFormItems: Set<string>[] = input.map((travelGroup: TravelGroup) => this.commonItems(travelGroup));
        let groupItemsSum: number = groupCustomsFormItems
            .map((uniqueItems: Set<string>) => uniqueItems.size)
            .reduce((result: number, currentSize: number) => result + currentSize);
        return groupItemsSum.toString();
    }

    private commonItems(group: TravelGroup): Set<string>{
        let groupSize: number = group.customsForms.length;
        let itemsWithCounts: Map<string, number> = this.uniqueItemsWithCount(group.customsForms);
        let commonCustomsItems = Array.from(itemsWithCounts.entries())
            .filter((kvp: [string, number]) => kvp[1] == groupSize)
            .map((kvp: [string, number]) => kvp[0]);
        return new Set<string>(commonCustomsItems);
    }

    private uniqueItemsWithCount(customsForms: string[]): Map<string, number>{
        let counts = new Map<string, number>();
        customsForms.forEach((customsForm: string) => this.increaseItemCounts(customsForm, counts));
        return counts;
    }

    private increaseItemCounts(customsForm: string, counts: Map<string, number>){
        Array.from(customsForm.trim()).forEach((character: string) => this.increaseItemCount(character, counts));
    }

    private increaseItemCount(customsItem: string, counts: Map<string, number>){
        if(counts.has(customsItem)) {
            counts.set(customsItem, counts.get(customsItem) as number + 1);
        } else {
            counts.set(customsItem, 1);
        }
    }
}


export default Day6;