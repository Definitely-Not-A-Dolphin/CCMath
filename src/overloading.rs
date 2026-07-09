use crate::Complex;
use num_traits::{self, Float};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// Addition

/// Complex<T> + T
impl<T: Float> Add<T> for Complex<T> {
    type Output = Complex<T>;

    fn add(self, rhs: T) -> Complex<T> {
        Complex::new(self.real + rhs, self.imag)
    }
}

/// Complex<T> += T
impl<T: Float> AddAssign<T> for Complex<T> {
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

/// Complex<T> + Complex<T>
impl<T: Float> Add<Complex<T>> for Complex<T> {
    type Output = Complex<T>;

    fn add(self, rhs: Complex<T>) -> Complex<T> {
        Complex::new(self.real + rhs.real, self.imag + rhs.imag)
    }
}

/// Complex<T> += Complex<T>
impl<T: Float> AddAssign<Complex<T>> for Complex<T> {
    fn add_assign(&mut self, rhs: Complex<T>) {
        *self = *self + rhs;
    }
}

// Subtraction

/// Complex<T> - T
impl<T: Float> Sub<T> for Complex<T> {
    type Output = Complex<T>;

    fn sub(self, rhs: T) -> Complex<T> {
        Complex::new(self.real - rhs, self.imag)
    }
}

/// Complex<T> -= T
impl<T: Float> SubAssign<T> for Complex<T> {
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

/// Complex<T> - Complex<T>
impl<T: Float> Sub<Complex<T>> for Complex<T> {
    type Output = Complex<T>;

    fn sub(self, rhs: Complex<T>) -> Complex<T> {
        Complex::new(self.real - rhs.real, self.imag - rhs.imag)
    }
}

/// Complex<T> -= Complex<T>
impl<T: Float> SubAssign<Complex<T>> for Complex<T> {
    fn sub_assign(&mut self, rhs: Complex<T>) {
        *self = *self - rhs;
    }
}

// Negation

/// -Complex<T>
impl<T: Float> Neg for Complex<T> {
    type Output = Complex<T>;

    fn neg(self) -> Complex<T> {
        Complex::new(-self.real, -self.imag)
    }
}

// Multiplication

/// Complex<T> * T
impl<T: Float> Mul<T> for Complex<T> {
    type Output = Complex<T>;

    fn mul(self, rhs: T) -> Complex<T> {
        Complex::new(self.real * rhs, self.imag * rhs)
    }
}

/// Complex<T> *= T
impl<T: Float> MulAssign<T> for Complex<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

/// Complex<T> * Complex<T>
impl<T: Float> Mul<Complex<T>> for Complex<T> {
    type Output = Complex<T>;

    fn mul(self, rhs: Complex<T>) -> Complex<T> {
        Complex::new(
            self.real * rhs.real - self.imag * rhs.imag,
            self.real * rhs.imag + self.imag * rhs.real,
        )
    }
}

/// Complex<T> *= Complex<T>
impl<T: Float> MulAssign<Complex<T>> for Complex<T> {
    fn mul_assign(&mut self, rhs: Complex<T>) {
        *self = *self * rhs
    }
}

// Division

/// Complex<T> / T
impl<T: Float> Div<T> for Complex<T> {
    type Output = Complex<T>;

    fn div(self, rhs: T) -> Complex<T> {
        Complex::new(self.real / rhs, self.imag / rhs)
    }
}

/// Complex<T> /= T
impl<T: Float> DivAssign<T> for Complex<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

/// Complex<T> / Complex<T>
impl<T: Float> Div<Complex<T>> for Complex<T> {
    type Output = Complex<T>;

    fn div(self, rhs: Complex<T>) -> Complex<T> {
        let rhsrecip = rhs.recip();
        Complex::new(
            self.real * rhsrecip.real - self.imag * rhsrecip.imag,
            self.real * rhsrecip.imag + self.imag * rhsrecip.real,
        )
    }
}

/// Complex<T> /= Complex<T>
impl<T: Float> DivAssign<Complex<T>> for Complex<T> {
    fn div_assign(&mut self, rhs: Complex<T>) {
        *self = *self / rhs
    }
}

/// T [operation] Complex<T>
macro_rules! impl_float_lhs {
    ($t:ty) => {
        impl Add<Complex<$t>> for $t {
            type Output = Complex<$t>;

            fn add(self, rhs: Complex<$t>) -> Complex<$t> {
                Complex::new(self + rhs.real, rhs.imag)
            }
        }
        impl Sub<Complex<$t>> for $t {
            type Output = Complex<$t>;

            fn sub(self, rhs: Complex<$t>) -> Complex<$t> {
                Complex::new(self - rhs.real, -rhs.imag)
            }
        }
        impl Mul<Complex<$t>> for $t {
            type Output = Complex<$t>;

            fn mul(self, rhs: Complex<$t>) -> Complex<$t> {
                Complex::new(rhs.real * self, rhs.imag * self)
            }
        }
        impl Div<Complex<$t>> for $t {
            type Output = Complex<$t>;

            fn div(self, rhs: Complex<$t>) -> Complex<$t> {
                Complex::conj(rhs) / Complex::square_abs(rhs) * self
            }
        }
    };
}

impl_float_lhs!(f32);
impl_float_lhs!(f64);
