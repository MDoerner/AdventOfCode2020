use num;
use crate::plane::Point2d;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Rem;
use crate::util;

pub trait Grid2d<T>{
    type CoordinateType;
    fn height(&self) -> Self::CoordinateType;
    fn width(&self) -> Self::CoordinateType;
    fn at_point<'a>(&'a self, point: &Point2d<Self::CoordinateType>) -> &'a T
        where <Self as Grid2d<T>>::CoordinateType: num::Num;
}

pub trait MutGrid2d<T>: Grid2d<T>{
    fn set_point(&mut self, point: Point2d<Self::CoordinateType>, value: T) -> ()
        where <Self as Grid2d<T>>::CoordinateType: num::Num;
}

pub struct LoopingGrid<T, V> where T: num::Num, V:PartialEq{
    height: T,
    width: T,
    map: HashMap<Point2d<T>,V>,
    default: V,
}

fn main_grid_point<T: num::Num + num::Zero + Rem + PartialOrd + Copy>(point: &Point2d<T>, height: T, width: T) -> Point2d<T>{
    Point2d{
        x: util::modulo(point.x, width),
        y: util::modulo(point.y, height),
    }
}

impl<T: num::Num + Hash + Eq + num::Zero + Rem + PartialOrd + Copy, V: PartialEq> LoopingGrid<T, V>{
    pub fn new(height: T, width: T, default: V, non_default_values: impl Iterator<Item = (Point2d<T>,V)>) -> LoopingGrid<T, V>{
        let mut map: HashMap<Point2d<T>, V> = HashMap::new();
        for (point, value) in non_default_values{
            if value != default {
                map.insert(main_grid_point(&point, height, width), value);
            }
        }
        LoopingGrid{
            height: height,
            width: width,
            map: map,
            default: default,
        }
    }
}

impl<T: num::Num + Hash + Eq + PartialOrd + Copy, V: PartialEq> Grid2d<V> for LoopingGrid<T, V>{
    type CoordinateType = T;

    fn height(&self) -> Self::CoordinateType { self.height }
    fn width(&self) -> Self::CoordinateType { self.width }

    fn at_point<>(&self, point: &Point2d<Self::CoordinateType>) -> &V {
        match self.map.get(&main_grid_point(point, self.height, self.width)){
            Some(value) => value,
            None => &self.default,
        }
    }
}

impl<T: num::Num + Hash + Eq + PartialOrd + Copy, V: PartialEq> MutGrid2d<V> for LoopingGrid<T, V>{
    fn set_point<>(&mut self, point: Point2d<T>, value: V) -> (){
        let reference_point = main_grid_point(&point, self.height, self.width);
        if value != self.default{
            self.map.insert(reference_point, value);
        }else{
            if self.map.contains_key(&reference_point){
                self.map.remove(&reference_point);
            }
        }
    }
}

impl<T: num::Num + Hash + Eq, V: PartialEq> IntoIterator for LoopingGrid<T, V>{
    type Item = (Point2d<T>, V);
    type IntoIter = std::collections::hash_map::IntoIter<Point2d<T>, V>;

    fn into_iter(self) -> Self::IntoIter{
        self.map.into_iter()
    }
}

pub struct OutsideDefaultGrid<T, V> where T: num::Num, V:PartialEq{
    height: T,
    width: T,
    map: HashMap<Point2d<T>,V>,
    default: V,
}

fn is_on_main_grid<T: num::Num + num::Zero + PartialOrd + Copy>(point: &Point2d<T>, height: T, width: T) -> bool{
    T::zero() <= point.x && point.x < width
        && T::zero() <= point.y && point.y < height
}

impl<T: num::Num + Hash + Eq + PartialOrd + Copy, V: PartialEq> OutsideDefaultGrid<T, V>{
    pub fn new(height: T, width: T, default: V, non_default_values: impl Iterator<Item = (Point2d<T>,V)>) -> LoopingGrid<T, V>{
        let mut map: HashMap<Point2d<T>, V> = HashMap::new();
        for (point, value) in non_default_values{
            if value != default && is_on_main_grid(&point, height, width) {
                map.insert(point, value);
            }
        }
        LoopingGrid{
            height: height,
            width: width,
            map: map,
            default: default,
        }
    }
}

impl<T: num::Num + Hash + Eq + PartialOrd + Copy, V: PartialEq> Grid2d<V> for OutsideDefaultGrid<T, V>{
    type CoordinateType = T;

    fn height(&self) -> Self::CoordinateType { self.height }
    fn width(&self) -> Self::CoordinateType { self.width }

    fn at_point<>(&self, point: &Point2d<Self::CoordinateType>) -> &V {
        if !is_on_main_grid(point, self.height, self.width){
            return &self.default;
        }
        match self.map.get(point){
            Some(value) => value,
            None => &self.default,
        }
    }
}

impl<T: num::Num + Hash + Eq + PartialOrd + Copy, V: PartialEq> MutGrid2d<V> for OutsideDefaultGrid<T, V>{
    fn set_point<>(&mut self, point: Point2d<T>, value: V) -> (){
        if !is_on_main_grid(&point, self.height, self.width){
            return;
        }
        if value != self.default{
            self.map.insert(point, value);
        }else{
            if self.map.contains_key(&point){
                self.map.remove(&point);
            }
        }
    }
}

impl<T: num::Num + Hash + Eq, V: PartialEq> IntoIterator for OutsideDefaultGrid<T, V>{
    type Item = (Point2d<T>, V);
    type IntoIter = std::collections::hash_map::IntoIter<Point2d<T>, V>;

    fn into_iter(self) -> Self::IntoIter{
        self.map.into_iter()
    }
}
