use std::ops::{Add,Mul,Sub,Neg,Div,Index};
use std::convert::TryFrom;
use std::num::TryFromIntError;
use std::iter::IntoIterator;
use std::slice::Iter;
use crate::util;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Vector<T: num::Num, const N: usize>{
    arr : [T; N],
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Point<T: num::Num, const N: usize>{
    arr : [T; N],
}

impl<T: num::Num, const N: usize> Vector<T, N>{
    pub fn new(arr: [T; N]) -> Vector<T, N>{
        Vector{arr}
    }
}

impl<T: num::Num, const N: usize> Point<T, N>{
    pub fn new(arr: [T; N]) -> Point<T, N>{
        Point{arr}
    }
}

impl<T: num::Num, I, const N: usize>Index<I> for Vector<T, N> where [T]: Index<I>{
    type Output = <[T] as std::ops::Index<I>>::Output;

    fn index(&self, indx: I) -> &Self::Output{
        &self.arr[indx]
    }
}

impl<T: num::Num, I, const N: usize>Index<I> for Point<T, N> where [T]: Index<I>{
    type Output = <[T] as std::ops::Index<I>>::Output;

    fn index(&self, indx: I) -> &Self::Output{
        &self.arr[indx]
    }
}

impl<T: Add<Output = T> + num::Num + num::Zero + Copy, const N: usize> Add for Vector<T, N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output{
        //Use array_zip and array_map when they stabelize.
        let mut arr = [T::zero(); N];
        for ind in 0..N{
            arr[ind] = self[ind] + other[ind]
        }
        Self {arr}
    }
}

impl<T: Add<Output = T> + num::Num + num::Zero + Copy, const N: usize> Add<Vector<T,N>> for Point<T,N> {
    type Output = Self;

    fn add(self, other: Vector<T, N>) -> Self::Output{
        //Use array_zip and array_map when they stabelize.
        let mut arr = [T::zero(); N];
        for ind in 0..N{
            arr[ind] = self[ind] + other[ind]
        }
        Self {arr}
    }
}

impl<T: Sub<Output = T> + num::Num + num::Zero + Copy, const N: usize> Sub for Vector<T, N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output{
        //Use array_zip and array_map when they stabelize.
        let mut arr = [T::zero(); N];
        for ind in 0..N{
            arr[ind] = self[ind] - other[ind]
        }
        Self {arr}
    }
}

impl<T: Sub<Output = T> + num::Num + num::Zero + Copy, const N: usize> Sub<Vector<T, N>> for Point<T, N> {
    type Output = Self;

    fn sub(self, other: Vector<T, N>) -> Self::Output{
        //Use array_zip and array_map when they stabelize.
        let mut arr = [T::zero(); N];
        for ind in 0..N{
            arr[ind] = self[ind] - other[ind]
        }
        Self {arr}
    }
}

impl<T: Sub<Output = T> + num::Num + num::Zero + Copy, const N: usize> Sub for Point<T, N> {
    type Output = Vector<T, N>;

    fn sub(self, other: Self) -> Self::Output{
        //Use array_zip and array_map when they stabelize.
        let mut arr = [T::zero(); N];
        for ind in 0..N{
            arr[ind] = self[ind] - other[ind]
        }
        Vector {arr}
    }
}

impl<T: Mul<Output = T> + num::Num + Copy + num::One, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output{
        //Use array_map when it stabelizes.
        let mut arr = [T::one(); N];
        for ind in 0..N{
            arr[ind] = self[ind] * rhs
        }
        Self {arr}
    }
}

impl<T: Div<Output = T> + num::Num + Copy + num::One, const N: usize> Div<T> for Vector<T, N> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output{
        //Use array_map when it stabelizes.
        let mut arr = [T::one(); N];
        for ind in 0..N{
            arr[ind] = self[ind] / rhs
        }
        Self {arr}
    }
}

impl<T: Neg<Output = T> + num::Num + num::Zero + Copy, const N: usize> Neg for Vector<T, N> {
    type Output = Self;

    fn neg(self) -> Self::Output{
        //Use array_map when it stabelizes.
        let mut arr = [T::zero(); N];
        for ind in 0..N{
            arr[ind] = -self[ind]
        }
        Self {arr}
    }
}


impl<'a, T: num::Num + Copy, const N: usize> IntoIterator for &'a Vector<T, N>{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter{
        self.arr.iter()
    }
}

impl<'a, T: num::Num + Copy, const N:usize> IntoIterator for &'a Point<T, N>{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter{
        self.arr.iter()
    }
}


impl<T: num::Integer + Neg<Output = T> + Copy> Vector<T, 2>{
    pub fn rotate(self, angle: i32) -> Vector<T, 2>{
        let rotation_steps: i32 = util::modulo(angle, 360) / 90;
        match rotation_steps{
            0 => self,
            1 => Vector{
                arr: [-self[1], self[0]],
            },
            2 => -self,
            3 => Vector{
                arr: [self[1], -self[0]],
            },
            _ => unreachable!(), //This cannot happen because of how modulo works.
        }
    }
}

impl<T: num::Integer + Div<Output = T> + Copy> Vector<T, 2>{
    pub fn to_direction(self) -> Vector<T, 2>{
        let gcd = self[0].gcd(&self[1]);
        self / gcd
    }
}


impl<T: num::Signed + Copy, const N: usize> Vector<T, N>{
    pub fn l1_norm(self) -> T{
        self.into_iter()
            .map(|coordinate| coordinate.abs())
            .fold(T::zero(), |sum, item| sum + item)
    }
}

pub fn manhattan_metric<T: num::Signed + Copy, const N:usize>(a: Point<T, N>, b: Point<T, N>) -> T{
    (a - b).l1_norm()
}

pub fn neighbour_offsets_unidirectional<T: num::One + num::Zero + num::Num + Neg<Output = T> + Copy, const N: usize>() -> Vec<Vector<T, N>>{
    let mut offsets: Vec<Vector<T, N>> = vec![];
    for ind in 0..N{
        for x in [T::one(), -(T::one())].iter(){
            let mut arr = [T::zero(); N];
            arr[ind] = x.to_owned();
            let offset = Vector {arr};
            offsets.push(offset);
        }
    }
    offsets
}

pub fn neighbour_offsets_full<T: num::One + num::Zero + num::Num + Neg<Output = T> + Copy, const N: usize>() -> Result<Vec<Vector<T, N>>, TryFromIntError>{
    let n_u32 = u32::try_from(N)?;
    let mut offsets: Vec<Vector<T, N>> = vec![];
    let coordinate_values = [T::one(), -(T::one()), T::zero()];
    for offset_index in 0..(3usize.pow(n_u32)-1){
        let mut arr = [T::zero(); N];
        let mut normalized_offset = offset_index;
        for vector_coordinate in arr.iter_mut(){
            *vector_coordinate = coordinate_values[normalized_offset % 3];
            normalized_offset /= 3;
        }
        let offset = Vector {arr};
        offsets.push(offset);
    }
    Ok(offsets)
}