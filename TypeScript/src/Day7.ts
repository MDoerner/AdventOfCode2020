import Day from "./Day";
import * as Util from "./Util";

interface BagType{
    appearance: string;
    color: string;
}

interface LuggageRule{
    containingBag: BagType;
    containedBags: Map<BagType, number>;
}

function equalBagTypes(bagType: BagType, otherBag: BagType): boolean {
    return otherBag.appearance == bagType.appearance
        && otherBag.color == bagType.color;
}

class Day7 implements Day<LuggageRule[]>{
    parseInput(text: string): LuggageRule[] {
        return text.split(/\r?\n/)
            .map((line: string) => this.parseLuggageRule(line))
            .filter(Util.notEmpty);
    }

    private parseLuggageRule(ruleText: string): LuggageRule | null{
        const pattern: RegExp = /(\w+) (\w+) bags contain(( no other bags)|(,? \d+ \w+ \w+ bags?)+)/;
        const match = ruleText.match(pattern);

        if(!match){
            return null;
        }

        const containingBag: BagType = {appearance: match[1], color: match[2]}
        const containedBags: Map<BagType,number> = this.parseContainedBags(match[3]);
        return {containingBag: containingBag, containedBags: containedBags};
    }

    private parseContainedBags(containedBagsText: string): Map<BagType,number>{
        let containedBags: Map<BagType,number> = new Map<BagType,number>();

        if(containedBagsText == " no other bags"){
            return containedBags;
        }

        const containingBagDefinitions: string[] = containedBagsText.split(',');
        containingBagDefinitions
            .map((definition: string) => this.parseContainingBagSpecification(definition))
            .filter(Util.notEmpty)
            .forEach((bagSpecification: [BagType, number]) => containedBags.set(bagSpecification[0], bagSpecification[1]));

            return containedBags;
    }

    private parseContainingBagSpecification(definition: string): [BagType, number] | null{
        const pattern: RegExp = /(\d+) (\w+) (\w+) bags?/;
        const match = definition.match(pattern);

        if(!match){
            return null;
        }

        const containedBag: BagType = {appearance: match[2], color: match[3]};
        const numberOfBags: number = parseInt(match[1]);

        return [containedBag, numberOfBags];
    }

    solvePart1(input: LuggageRule[]): string {
        const myBag: BagType = {appearance: "shiny", color: "gold"};
        const containedBagsGraph: Map<string, Set<string>> = this.toContainedGraph(input);
        const containingBags: Set<string> = this.descendants(containedBagsGraph, JSON.stringify(myBag));
        return containingBags.size.toString();
    }

    //string because javascript does not allow to redefine equality for the map.
    private toContainedGraph(rules: LuggageRule[]): Map<string, Set<string>>{
        let graph = new Map<string, Set<string>>();
        for (let rule of rules){
            for (let containedBag of rule.containedBags.keys()){
                this.addDirectedEdge(graph, JSON.stringify(containedBag), JSON.stringify(rule.containingBag));
            }
        }
        return graph;
    }

    private addDirectedEdge<TVertex>(graph: Map<TVertex, Set<TVertex>>, startVertex: TVertex, endVertex: TVertex){
        if(!graph.has(startVertex)){
            graph.set(startVertex, new Set<TVertex>([endVertex]));
        }

        let currentChildren: Set<TVertex> = graph.get(startVertex) as Set<TVertex>;
        currentChildren.add(endVertex);
    }

    private descendants<TValue>(dac: Map<TValue, Set<TValue>>, vertex: TValue): Set<TValue>{
        const descendants: Set<TValue> = new Set<TValue>();

        if(!dac.has(vertex)){
            return descendants;
        }

        const children: Set<TValue> = dac.get(vertex) as Set<TValue>;

        for(let child of children){
            this.addWithDescendants(descendants, dac, child);
        }

        return descendants;
    }

    private addWithDescendants<TValue>(set: Set<TValue>, dac: Map<TValue, Set<TValue>>, vertex: TValue){
        if(set.has(vertex)){
            return;
        }
        set.add(vertex);

        if(!dac.has(vertex)){
            return;
        }

        const children: Set<TValue> = dac.get(vertex) as Set<TValue>;

        for(let child of children){
            this.addWithDescendants(set, dac, child);
        }
    }

    solvePart2(input: LuggageRule[]): string {
        const myBag: BagType = {appearance: "shiny", color: "gold"};
        const myBagKey: string = JSON.stringify(myBag);
        const allContainedBags: Map<string,Map<string,number>> = this.totalContainedBags(input);
        if(!allContainedBags.has(myBagKey)){
            return "404 Bag not Found!"
        }
        const containedBags: Map<string, number> = allContainedBags.get(myBagKey) as Map<string, number>;
        const totalBagCount: number = Array.from(containedBags.values()).reduce((sum: number, item: number) => sum + item, 0);
        return totalBagCount.toString();
    }

    private totalContainedBags(rules: LuggageRule[]): Map<string,Map<string,number>>{
        const ruleDac: Map<string,Map<string,number>> = this.toContainingGraph(rules);
        const containedBags: Map<string,Map<string,number>> = new Map<string,Map<string,number>>();
        for (let bagType of ruleDac.keys()){
            this.addTotalContainedBags(containedBags, ruleDac, bagType);
        }
        return containedBags;
    }

    private toContainingGraph(rules: LuggageRule[]): Map<string,Map<string,number>>{
        const ruleDac: Map<string,Map<string,number>> = new Map<string,Map<string,number>>();
        for(let rule of rules){
            ruleDac.set(JSON.stringify(rule.containingBag), this.toStringMap(rule.containedBags));
        }
        return ruleDac;
    }

    private toStringMap<TKey, TValue>(objectMap: Map<TKey, TValue>): Map<string, TValue>{
        const stringMap: Map<string, TValue> = new Map<string, TValue>();
        for(let element of objectMap){
            stringMap.set(JSON.stringify(element[0]), element[1]);
        }
        return stringMap;
    }

    private addTotalContainedBags(containedBags: Map<string,Map<string,number>>, ruleDac: Map<string,Map<string,number>>, bagType: string){
        if(containedBags.has(bagType) || !ruleDac.has(bagType)){
            return;
        }

        const totalBagsInBagType: Map<string, number> = new Map<string, number>();
        const bagsInBagType: Map<string, number> = ruleDac.get(bagType) as Map<string, number>;

        for (let element of bagsInBagType){
            let containedBag: string = element[0];
            let numberOfBags: number = element[1];

            if(!containedBags.has(containedBag)){
                this.addTotalContainedBags(containedBags, ruleDac, containedBag);
            }

            this.addBagTotals(totalBagsInBagType, containedBag, numberOfBags, containedBags);
        }

        containedBags.set(bagType, totalBagsInBagType);
    }

    private addBagTotals(totals: Map<string, number>, bagType: string, count: number, containedBags: Map<string,Map<string,number>>){
        Util.addCount(totals, bagType, count);
        let containedBagTotals: Map<string, number> = containedBags.get(bagType) as Map<string, number>;
        for(let element of containedBagTotals){
            let containedBag: string = element[0];
            let numberOfBags: number = element[1];
            Util.addCount(totals, containedBag, numberOfBags * count);
        }
    }
}



export default Day7;