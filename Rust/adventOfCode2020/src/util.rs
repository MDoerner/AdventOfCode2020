use num;
use std::ops::Rem;

pub fn modulo<T: num::Num + num::Zero + Rem + PartialOrd + Copy>(n: T, d: T) -> T{
    if n >= T::zero() {
        n % d
    } else {
        (n % d) + d
    }
}