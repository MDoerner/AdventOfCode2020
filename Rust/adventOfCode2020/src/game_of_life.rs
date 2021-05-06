use std::collections::HashSet;
use std::hash::Hash;

pub trait GameOfLifeRules {
    type ItemType;
    fn neighbours<'a>(&self, item: &'a Self::ItemType) -> Vec<Self::ItemType> where Self::ItemType: 'a;
    fn flip_active(&self, active_neighbour_count: usize) -> bool;
    fn flip_inactive(&self, active_neighbour_count: usize) -> bool;
}

pub struct GameOfLife<R: GameOfLifeRules>{
    rules: R,
}

impl<R: GameOfLifeRules> GameOfLife<R>
    where R::ItemType: Hash + Eq + Copy{
    pub fn new(game_rules: R) -> GameOfLife<R>{
        GameOfLife{rules: game_rules}
    }

    pub fn active_items_after_playing<'a>(&self, rounds_to_play: usize, initially_active_items: impl Iterator<Item=&'a R::ItemType>, initially_relevant_items: impl Iterator<Item=&'a R::ItemType>) -> impl Iterator<Item=R::ItemType> where R::ItemType:'a {
        let mut active_items: HashSet<R::ItemType> = initially_active_items
            .map(|item| item.to_owned())
            .collect();
        let mut relevant_items: HashSet<R::ItemType> = initially_relevant_items
            .map(|item| item.to_owned())
            .collect();
        for _ in 0..rounds_to_play{
            if relevant_items.is_empty(){
                break;
            }
            relevant_items = self.play_round(&mut active_items, relevant_items);
        }
        active_items.into_iter()
    }

    pub fn active_items_after_stabelizing<'a>(&self, initially_active_items: impl Iterator<Item=&'a R::ItemType>, initially_relevant_items: impl Iterator<Item=&'a R::ItemType>) -> impl Iterator<Item=R::ItemType> where R::ItemType:'a {
        let mut active_items: HashSet<R::ItemType> = initially_active_items
            .map(|item| item.to_owned())
            .collect();
        let mut relevant_items: HashSet<R::ItemType> = initially_relevant_items
            .map(|item| item.to_owned())
            .collect();
        loop{
            if relevant_items.is_empty(){
                break;
            }
            relevant_items = self.play_round(&mut active_items, relevant_items);
        }
        active_items.into_iter()
    }

    //Returns the items relevant for the next round.
    fn play_round(&self, active_items: &mut HashSet<R::ItemType>, relevant_items:HashSet<R::ItemType>) -> HashSet<R::ItemType>{
        let changing_items: Vec<R::ItemType> = relevant_items.into_iter()
            .filter(|item| self.item_changes(item, active_items))
            .collect();
        change_item_state(&changing_items, active_items);
        self.relevant_items_after_changes(changing_items)
    }

    fn item_changes(&self, item: &R::ItemType,  active_items: &HashSet<R::ItemType>) -> bool{
        let count_of_active_neighbours = self.active_neighbours_count(item, active_items);
        if active_items.contains(item){
            self.rules.flip_active(count_of_active_neighbours)
        } else {
            self.rules.flip_inactive(count_of_active_neighbours)
        }
    }

    fn relevant_items_after_changes(&self, changed_items: Vec<R::ItemType>) -> HashSet<R::ItemType>{
        let mut relevant_items: HashSet<R::ItemType> = HashSet::new();
        for item in changed_items.into_iter(){
            for neighbour in self.rules.neighbours(&item).into_iter(){
                relevant_items.insert(neighbour);
            }
            relevant_items.insert(item);
        }
        relevant_items
    }

    fn active_neighbours_count(&self, item: &R::ItemType, active_items: &HashSet<R::ItemType>) -> usize{
        self.rules
            .neighbours(item)
            .into_iter()
            .filter(|neighbour| active_items.contains(neighbour))
            .count()
    }
}

fn change_item_state<T: Hash + Eq + Copy>(items_to_change: &[T], active_items: &mut HashSet<T>){
    for item in items_to_change{
        if active_items.contains(item){
            active_items.remove(item);
        }else{
            active_items.insert(item.to_owned());
        }
    }
}

