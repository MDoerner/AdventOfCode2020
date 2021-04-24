use crate::space::Point;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Rem;
use crate::util;

pub trait Grid<T, const N: usize>{
    type CoordinateType;
    fn width(&self) -> &[Self::CoordinateType; N];
    fn at_point<'a>(&'a self, point: &Point<Self::CoordinateType, N>) -> &'a T
        where <Self as Grid<T, N>>::CoordinateType: num::Num;
}

pub trait MutGrid<T, const N: usize>: Grid<T, N>{
    fn set_point(&mut self, point: &Point<Self::CoordinateType, N>, value: T)
        where <Self as Grid<T, N>>::CoordinateType: num::Num;
}

pub struct LoopingGrid<T, V, const N: usize> where T: num::Num, V:PartialEq{
    width: [T; N],
    map: HashMap<Point<T, N>,V>,
    default: V,
}

fn main_grid_point<T: num::Num + num::Zero + Rem + PartialOrd + Copy, const N: usize>(point: &Point<T, N>, width: &[T; N]) -> Point<T, N>{
    let mut arr = [T::zero(); N];
    for ind in 0..N{
        arr[ind] = util::modulo(point[ind], width[ind]);
    }
    Point::new(arr)
}

impl<T: num::Num + Hash + Eq + num::Zero + Rem + PartialOrd + Copy, V: PartialEq, const N: usize> LoopingGrid<T, V, N>{
    pub fn new(width: [T; N], default: V, non_default_values: impl Iterator<Item = (Point<T, N>,V)>) -> LoopingGrid<T, V, N>{
        let mut map: HashMap<Point<T, N>, V> = HashMap::new();
        for (point, value) in non_default_values{
            if value != default {
                map.insert(main_grid_point(&point, &width), value);
            }
        }
        LoopingGrid{width, map, default}
    }
}

impl<T: num::Num + Hash + Eq + PartialOrd + Copy, V: PartialEq, const N: usize> Grid<V, N> for LoopingGrid<T, V, N>{
    type CoordinateType = T;

    fn width(&self) -> &[Self::CoordinateType; N] { &self.width }

    fn at_point<>(&self, point: &Point<Self::CoordinateType, N>) -> &V {
        match self.map.get(&main_grid_point(point, self.width())){
            Some(value) => value,
            None => &self.default,
        }
    }
}

impl<T: num::Num + Hash + Eq + PartialOrd + Copy, V: PartialEq, const N: usize> MutGrid<V, N> for LoopingGrid<T, V, N>{
    fn set_point<>(&mut self, point: &Point<T, N>, value: V){
        let reference_point = main_grid_point(point, self.width());
        if value != self.default{
            self.map.insert(reference_point, value);
        }else if self.map.contains_key(&reference_point){
            self.map.remove(&reference_point);
        }
    }
}

impl<T: num::Num + Hash + Eq, V: PartialEq, const N: usize> IntoIterator for LoopingGrid<T, V, N>{
    type Item = (Point<T, N>, V);
    type IntoIter = std::collections::hash_map::IntoIter<Point<T, N>, V>;

    fn into_iter(self) -> Self::IntoIter{
        self.map.into_iter()
    }
}

pub struct OutsideDefaultGrid<T, V, const N: usize> where T: num::Num, V:PartialEq{
    width: [T; N],
    map: HashMap<Point<T, N>,V>,
    default: V,
}

fn is_on_main_grid<T: num::Num + num::Zero + PartialOrd + Copy, const N: usize>(point: &Point<T, N>, width: &[T; N]) -> bool{
    for ind in 0..N{
        if T::zero() >= point[ind] || point[ind] > width[ind]{
            return  false;
        }
    }
    true
}

impl<T: num::Num + Hash + Eq + PartialOrd + Copy, V: PartialEq, const N: usize> OutsideDefaultGrid<T, V, N>{
    pub fn new(width: [T;N], default: V, non_default_values: impl Iterator<Item = (Point<T, N>,V)>) -> OutsideDefaultGrid<T, V, N>{
        let mut map: HashMap<Point<T, N>, V> = HashMap::new();
        for (point, value) in non_default_values{
            if value != default && is_on_main_grid(&point, &width) {
                map.insert(point, value);
            }
        }
        OutsideDefaultGrid{width, map, default}
    }
}

impl<T: num::Num + Hash + Eq + PartialOrd + Copy, V: PartialEq, const N: usize> Grid<V, N> for OutsideDefaultGrid<T, V, N>{
    type CoordinateType = T;

    fn width(&self) -> &[Self::CoordinateType; N] { &self.width }

    fn at_point<>(&self, point: &Point<Self::CoordinateType, N>) -> &V {
        if !is_on_main_grid(point, self.width()){
            return &self.default;
        }
        match self.map.get(point){
            Some(value) => value,
            None => &self.default,
        }
    }
}

impl<T: num::Num + Hash + Eq + PartialOrd + Copy, V: PartialEq, const N: usize> MutGrid<V, N> for OutsideDefaultGrid<T, V, N>{
    fn set_point<>(&mut self, point: &Point<T, N>, value: V){
        if !is_on_main_grid(point, self.width()){
            return;
        }
        if value != self.default{
            self.map.insert(point.to_owned(), value);
        }else if self.map.contains_key(point){
            self.map.remove(point);
        }
    }
}

impl<T: num::Num + Hash + Eq, V: PartialEq, const N: usize> IntoIterator for OutsideDefaultGrid<T, V, N>{
    type Item = (Point<T, N>, V);
    type IntoIter = std::collections::hash_map::IntoIter<Point<T, N>, V>;

    fn into_iter(self) -> Self::IntoIter{
        self.map.into_iter()
    }
}
