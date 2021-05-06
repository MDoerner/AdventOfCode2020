use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Add, Sub, Rem};

use crate::util;
use crate::space::Point;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct CoordinateRange<T>{
    pub lower_bound: T,
    pub upper_bound: T,
}

pub trait Grid<V, const N: usize>{
    type CoordinateType;
    fn coordinate_ranges(&self) -> &[CoordinateRange<Self::CoordinateType>; N];
    fn at_point<'a>(&'a self, point: &Point<Self::CoordinateType, N>) -> &'a V;
    fn is_on_main_grid(&self, point: &Point<Self::CoordinateType, N>) -> bool
            where <Self as Grid<V, N>>::CoordinateType: PartialOrd{
        let coordinate_ranges = self.coordinate_ranges();
        is_on_main_grid(point, coordinate_ranges)
    }
}

pub trait MutGrid<T, const N: usize>: Grid<T, N>{
    fn set_point(&mut self, point: &Point<Self::CoordinateType, N>, value: T);
}

pub struct LoopingGrid<T, V, const N: usize>{
    coordinate_ranges: [CoordinateRange<T>; N],
    map: HashMap<Point<T, N>,V>,
    default: V,
}

fn main_grid_point<T: Default + PartialOrd + Copy + Sub<Output=T> + Add<Output=T> + Rem<Output=T>, const N: usize>(point: &Point<T, N>, coordinate_ranges: &[CoordinateRange<T>; N]) -> Point<T, N>{
    if is_on_main_grid(point, coordinate_ranges){
        return point.to_owned();
    }
    let mut arr = [T::default(); N];
    for ind in 0..N{
        let main_grid_width = coordinate_ranges[ind].upper_bound - coordinate_ranges[ind].lower_bound;
        arr[ind] = util::extended_modulo(point[ind],  coordinate_ranges[ind].lower_bound, main_grid_width);
    }
    Point::new(arr)
}

impl<T: Hash + Eq + Default + PartialOrd + Copy + Sub<Output=T> + Add<Output=T> + Rem<Output=T>, V: PartialEq, const N: usize> LoopingGrid<T, V, N>{
    pub fn new(coordinate_ranges: [CoordinateRange<T>; N], default: V, non_default_values: impl Iterator<Item = (Point<T, N>,V)>) -> LoopingGrid<T, V, N>{
        let mut map: HashMap<Point<T, N>, V> = HashMap::new();
        for (point, value) in non_default_values{
            if value != default {
                map.insert(main_grid_point(&point, &coordinate_ranges), value);
            }
        }
        LoopingGrid {coordinate_ranges, map, default}
    }
}

impl<T: Hash + Eq + Default + PartialOrd + Copy + Sub<Output=T> + Add<Output=T> + Rem<Output=T>, V, const N: usize> Grid<V, N> for LoopingGrid<T, V, N>{
    type CoordinateType = T;

    fn coordinate_ranges(&self) -> &[CoordinateRange<Self::CoordinateType>; N] {
        &self.coordinate_ranges
    }

    fn at_point<>(&self, point: &Point<Self::CoordinateType, N>) -> &V {
        match self.map.get(&main_grid_point(point, self.coordinate_ranges())){
            Some(value) => value,
            None => &self.default,
        }
    }
}

impl<T: Hash + Eq + Default + PartialOrd + Copy + Sub<Output=T> + Add<Output=T> + Rem<Output=T>, V: PartialEq, const N: usize> MutGrid<V, N> for LoopingGrid<T, V, N>{
    fn set_point(&mut self, point: &Point<T, N>, value: V){
        let reference_point = main_grid_point(point, self.coordinate_ranges());
        if value != self.default{
            self.map.insert(reference_point, value);
        }else if self.map.contains_key(&reference_point){
            self.map.remove(&reference_point);
        }
    }
}

impl<T: Hash + Eq, V, const N: usize> IntoIterator for LoopingGrid<T, V, N>{
    type Item = (Point<T, N>, V);
    type IntoIter = std::collections::hash_map::IntoIter<Point<T, N>, V>;

    fn into_iter(self) -> Self::IntoIter{
        self.map.into_iter()
    }
}

pub struct OutsideDefaultGrid<T, V, const N: usize>{
    coordinate_ranges: [CoordinateRange<T>; N],
    map: HashMap<Point<T, N>,V>,
    default: V,
}

fn is_on_main_grid<T: PartialOrd, const N: usize>(point: &Point<T, N>, coordinate_ranges: &[CoordinateRange<T>; N]) -> bool{
    for ind in 0..N{
        if coordinate_ranges[ind].lower_bound > point[ind] || point[ind] >= coordinate_ranges[ind].upper_bound{
            return  false;
        }
    }
    true
}

impl<T: Hash + Eq + PartialOrd + Copy, V: PartialEq, const N: usize> OutsideDefaultGrid<T, V, N>{
    pub fn new(coordinate_ranges: [CoordinateRange<T>; N], default: V, non_default_values: impl Iterator<Item = (Point<T, N>,V)>) -> OutsideDefaultGrid<T, V, N>{
        let mut map: HashMap<Point<T, N>, V> = HashMap::new();
        for (point, value) in non_default_values{
            if value != default && is_on_main_grid(&point, &coordinate_ranges) {
                map.insert(point, value);
            }
        }
        OutsideDefaultGrid {coordinate_ranges, map, default}
    }
}

impl<T: Hash + Eq + PartialOrd + Copy, V: PartialEq, const N: usize> Grid<V, N> for OutsideDefaultGrid<T, V, N>{
    type CoordinateType = T;

    fn coordinate_ranges(&self) -> &[CoordinateRange<Self::CoordinateType>; N] { &self.coordinate_ranges }

    fn at_point<>(&self, point: &Point<Self::CoordinateType, N>) -> &V {
        if !is_on_main_grid(point, self.coordinate_ranges()){
            return &self.default;
        }
        match self.map.get(point){
            Some(value) => value,
            None => &self.default,
        }
    }
}

impl<T: Hash + Eq + PartialOrd + Copy, V: PartialEq, const N: usize> MutGrid<V, N> for OutsideDefaultGrid<T, V, N>{
    fn set_point<>(&mut self, point: &Point<T, N>, value: V){
        if !is_on_main_grid(point, self.coordinate_ranges()){
            return;
        }
        if value != self.default{
            self.map.insert(point.to_owned(), value);
        }else if self.map.contains_key(point){
            self.map.remove(point);
        }
    }
}

impl<T: Hash + Eq, V, const N: usize> IntoIterator for OutsideDefaultGrid<T, V, N>{
    type Item = (Point<T, N>, V);
    type IntoIter = std::collections::hash_map::IntoIter<Point<T, N>, V>;

    fn into_iter(self) -> Self::IntoIter{
        self.map.into_iter()
    }
}
