import Day from "./Day";
import * as Util from "./Util";

interface Recipe{
    ingredients: Set<string>;
    allergens: Set<string>;
}


class Day21 implements Day<Recipe[]>{
    parseInput(text: string): Recipe[] {
        return text.split(/\r?\n/)
            .map((line: string) => this.parseRecipe(line))
            .filter(Util.notEmpty);
    }

    private parseRecipe(text: string): Recipe | null{
        const pattern: RegExp = /(\w+( \w+)*) \(contains (\w+(, \w+)*)\)/;
        const match: RegExpMatchArray | null = text.match(pattern);
        if(match == null){
            return null;
        }
        const ingredients: Set<string> = new Set<string>(match[1].split(' '));
        const allergens: Set<string> = new Set<string>(match[3].split(', '));
        return {ingredients: ingredients, allergens: allergens};
    }

    solvePart1(input: Recipe[]): string {
        const recipesByAllergen: Map<string, Recipe[]> = this.recipesByAllergen(input);
        const possibleIngredientsByAllergen: Map<string, Set<string>> = this.commonIngredientsForAllergen(recipesByAllergen);
        this.reduceByUniquenessOfIngredientForAllergen(possibleIngredientsByAllergen);
        const ingredientsWithAllergens: Set<string> = Util.union(possibleIngredientsByAllergen.values());
        const numberOfRecipesWithIngredient = this.numberOfRecipesWithIngredient(input);
        const result: number = Util.reduce(
            numberOfRecipesWithIngredient.entries(),
            ((sumOfCounts: number, ingredentCountPair: [string, number]) =>
                ingredientsWithAllergens.has(ingredentCountPair[0])
                ? sumOfCounts
                : sumOfCounts + ingredentCountPair[1]),
             0);
        return result.toString();
    }

    private recipesByAllergen(recipes: Recipe[]): Map<string, Recipe[]>{
        const recipesByAllergen: Map<string, Recipe[]> = new Map<string, Recipe[]>();
        for(let recipe of recipes){
            for(let allergen of recipe.allergens.keys()){
                Util.addToValueList(allergen, recipe, recipesByAllergen);
            }
        }
        return recipesByAllergen;
    }

    private commonIngredientsForAllergen(recipesByAllergen: Map<string, Recipe[]>): Map<string, Set<string>>{
        const commonIngredients: Map<string, Set<string>> = new Map<string, Set<string>>();
        recipesByAllergen.forEach((recipes: Recipe[], allergen: string) =>
            commonIngredients.set(allergen, this.commonIngredients(recipes)));
        return commonIngredients;
    }

    private commonIngredients(recipes: Recipe[]): Set<string>{
        if(recipes.length == 0){
            return new Set<string>();
        }
        //We want a copy since we will delete items.
        const commonIngredients: Set<string> = new Set<string>(recipes[0].ingredients.keys());
        for(let recipeIndex: number = 1; recipeIndex < recipes.length; recipeIndex++){
            const recipeIngrdients: Set<string> = recipes[recipeIndex].ingredients;
            const ingredientsToRemove: string[] = [];
            for(let ingredient of commonIngredients){
                if(!recipeIngrdients.has(ingredient)){
                    ingredientsToRemove.push(ingredient);
                }
            }
            for(let ingredient of ingredientsToRemove){
                commonIngredients.delete(ingredient);
            }
        }
        return commonIngredients;
    }

    private reduceByUniquenessOfIngredientForAllergen(possibleIngredientsByAllergen: Map<string, Set<string>>): void{
        const alreadyDeterminedAllergens: Set<string> = new Set<string>();
        let newAllergensWithOnePossibleIngredient: string[] = this.newSingleValueKeys(possibleIngredientsByAllergen, alreadyDeterminedAllergens);
        while(newAllergensWithOnePossibleIngredient.length > 0){
            for(let allergen of newAllergensWithOnePossibleIngredient){
                const ingredient: string = (possibleIngredientsByAllergen.get(allergen) as Set<string>).keys().next().value;
                this.removeFromValueSets(possibleIngredientsByAllergen, ingredient, allergen);
                alreadyDeterminedAllergens.add(allergen);
            }
            newAllergensWithOnePossibleIngredient = this.newSingleValueKeys(possibleIngredientsByAllergen, alreadyDeterminedAllergens);
        }
    }

    private newSingleValueKeys<T,U>(setMap: Map<T, Set<U>>, knownKeys: Set<T>): T[]{
        const newKeys: T[] = [];
        setMap.forEach((valueSet: Set<U>, key: T) => {
            if(valueSet.size == 1 && !knownKeys.has(key)){
                newKeys.push(key);
            }
        });
        return newKeys;
    }

    private removeFromValueSets<T,U>(setMap: Map<T, Set<U>>, valueToRemove: U, keyToSkip: T): void{
        setMap.forEach((valueSet: Set<U>, key: T) => {
            if(key != keyToSkip && valueSet.has(valueToRemove)){
                valueSet.delete(valueToRemove);
            }
        });
    }

    private numberOfRecipesWithIngredient(recipes: Recipe[]): Map<string, number>{
        const recipesCounts: Map<string, number> = new Map<string, number>();
        for(let recipe of recipes){
            for(let ingredient of recipe.ingredients.keys()){
                Util.addCount(recipesCounts, ingredient, 1);
            }
        }
        return recipesCounts;
    }

    solvePart2(input: Recipe[]): string {
        const recipesByAllergen: Map<string, Recipe[]> = this.recipesByAllergen(input);
        const possibleIngredientsByAllergen: Map<string, Set<string>> = this.commonIngredientsForAllergen(recipesByAllergen);
        this.reduceByUniquenessOfIngredientForAllergen(possibleIngredientsByAllergen);
        const allergens: string[] = Array.from(possibleIngredientsByAllergen.keys());
        allergens.sort();
        const ingredientsSortedByAllergen: string[] = allergens.map((allergen: string) => (possibleIngredientsByAllergen.get(allergen) as Set<string>).keys().next().value);
        const result: string = ingredientsSortedByAllergen.join(',');
        return result;
    }

}


export default Day21;