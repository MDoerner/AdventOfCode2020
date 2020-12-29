use num;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
use std::ops::Neg;
use std::ops::Div;
use std::iter::Iterator;
use std::iter::IntoIterator;
use crate::util;

#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Copy)]
pub struct Vector2d<T: num::Num>{
    pub x: T,
    pub y: T,
}

#[derive(PartialEq, Eq, Hash, Default, Debug, Clone, Copy)]
pub struct Point2d<T: num::Num>{
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T> + num::Num> Add for Vector2d<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output{
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Add<Output = T> + num::Num> Add<Vector2d<T>> for Point2d<T> {
    type Output = Self;

    fn add(self, other: Vector2d<T>) -> Self::Output{
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T> + num::Num> Sub for Vector2d<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output{
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Sub<Output = T> + num::Num> Sub<Vector2d<T>> for Point2d<T> {
    type Output = Self;

    fn sub(self, other: Vector2d<T>) -> Self::Output{
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Sub<Output = T> + num::Num> Sub for Point2d<T> {
    type Output = Vector2d<T>;

    fn sub(self, other: Self) -> Self::Output{
        Vector2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Mul<Output = T> + num::Num + Copy> Mul<T> for Vector2d<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output{
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Div<Output = T> + num::Num + Copy> Div<T> for Vector2d<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output{
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Neg<Output = T> + num::Num> Neg for Vector2d<T> {
    type Output = Self;

    fn neg(self) -> Self::Output{
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}


pub struct Vector2dIterator<T: num::Num>{
    vector: Vector2d<T>,
    index: usize,
}

pub struct Point2dIterator<T: num::Num>{
    point: Point2d<T>,
    index: usize,
}

impl<T: num::Num + Copy> Iterator for Vector2dIterator<T>{
    type Item = T;
    fn next(&mut self) -> Option<T>{
        let result = match self.index {
            0 => self.vector.x,
            1 => self.vector.y,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

impl<T: num::Num + Copy> Iterator for Point2dIterator<T>{
    type Item = T;
    fn next(&mut self) -> Option<T>{
        let result = match self.index {
            0 => self.point.x,
            1 => self.point.y,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

impl<T: num::Num + Copy> IntoIterator for Vector2d<T>{
    type Item = T;
    type IntoIter = Vector2dIterator<T>;

    fn into_iter(self) -> Self::IntoIter{
        Vector2dIterator {
            vector: self,
            index: 0,
        }
    }
}

impl<T: num::Num + Copy> IntoIterator for Point2d<T>{
    type Item = T;
    type IntoIter = Point2dIterator<T>;

    fn into_iter(self) -> Self::IntoIter{
        Point2dIterator {
            point: self,
            index: 0,
        }
    }
}


impl<T: num::Integer + Neg<Output = T>> Vector2d<T>{
    pub fn rotate(self, angle: i32) -> Vector2d<T>{
        let rotation_steps: i32 = util::modulo(angle, 360) / 90;
        match rotation_steps{
            0 => self,
            1 => Vector2d{
                x: -self.y,
                y: self.x,
            },
            2 => -self,
            3 => Vector2d{
                x: self.y,
                y: -self.x,
            },
            _ => unreachable!(), //This cannot happen because of how modulo works.
        }
    }
}

impl<T: num::Integer + Div<Output = T> + Copy> Vector2d<T>{
    pub fn to_direction(self) -> Vector2d<T>{
        let gcd = self.x.gcd(&self.y);
        self / gcd
    }
}


impl<T: num::Signed + Copy> Vector2d<T>{
    pub fn l1_norm(self) -> T{
        self.into_iter()
            .map(|coordinate| coordinate.abs())
            .fold(T::zero(), |sum, item| sum + item)
    }
}

pub fn manhattan_metric<T: num::Signed + Copy>(a: Point2d<T>, b: Point2d<T>) -> T{
    (a - b).l1_norm()
}


impl<T: num::One + num::Zero + num::Num + Neg<Output = T> + Copy> Point2d<T>{
    pub fn neighbours(self) -> Vec<Point2d<T>>{
        let offsets = neighbour_offsets();
        offsets.into_iter()
            .map(|offset| self + offset)
            .collect()
    }
}

pub fn neighbour_offsets<T: num::One + num::Zero + num::Num + Neg<Output = T> + Copy>() -> Vec<Vector2d<T>>{
    let mut offsets: Vec<Vector2d<T>> = vec![];
    for x in [T::one(), T::zero(), (T::one())].iter(){
        for y in [T::one(), T::zero(), -(T::one())].iter(){
            if x != &T::zero() || y != &T::zero(){
                let offset: Vector2d<T> = Vector2d { x: *x, y: *y};
                offsets.push(offset);
            }
        }
    }
    offsets
}