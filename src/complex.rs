use num_traits::*;
use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Complex<T: Num + Copy> (pub T, pub T);



impl<T: Num + Copy> Add for Complex<T> {
    type Output = Complex<T>;

    fn add(self, other: Self) -> Self {
        return Complex(
            self.0 + other.0,
            self.1 + other.1
        );
    }
}

impl<T: Num + Copy> Mul for Complex<T> {
    type Output = Complex<T>;

    fn mul(self, other: Self) -> Self {
        Complex(
            (self.0 * other.0) - (self.1 * other.1),
            (self.0 * other.1) + (self.1 * other.0)
        )
    }
}


impl<T: Num + Copy> Zero for Complex<T> {
    fn zero() -> Self {
        Complex(
            T::zero(),
            T::zero()
        )
    }

    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}


impl<T: Num + Copy> One for Complex<T> {
    fn one() -> Self {
        Complex(
            T::one(),
            T::zero()
        )
    }
}


impl<T: Num + Copy> Complex<T> {
    pub fn abs_sq(self) -> T {
        (self.0 * self.0) + (self.1 * self.1)
    }

    pub fn pow(self, i: u16) -> Complex<T> {
        (0..i).fold(Complex(T::one(), T::zero()), |c, _| c*self)
    }
}

