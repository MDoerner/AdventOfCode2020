use std::ops::{Add,Sub,Rem};

pub fn modulo<T: num::Num + num::Zero + Rem + PartialOrd + Copy>(n: T, d: T) -> T{
    if n >= T::zero() {
        n % d
    } else {
        (n % d) + d
    }
}

pub fn extended_modulo<T: Sub<Output=T> + Add<Output=T> + Rem<Output=T> + PartialOrd + Copy>(n: T, base_value: T, width: T) -> T{
    let offset = n - base_value;
    let maybe_modulo = base_value + (offset % width);
    if maybe_modulo >= base_value {
        maybe_modulo
    } else {
        maybe_modulo + width
    }
}