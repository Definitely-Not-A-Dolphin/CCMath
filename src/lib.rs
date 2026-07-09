/// CCMath: a crate for doing math with complex numbers
use num_traits::Float;
use std::fmt::{Debug, Display, Formatter, Result};

/// Struct representing a complex number
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Complex<T: Float> {
    real: T,
    imag: T,
}

/// Alias for [`Complex`]
pub type CC<T> = Complex<T>;
// Personal tip: use this alias when code is getting a little hard to read, it cleans things up!

trait Numbers: Float {
    fn two() -> Self;
    fn pi() -> Self;
}

impl<T: Float> Numbers for T {
    /// Returns the number two
    fn two() -> T {
        T::one() + T::one()
    }
    fn pi() -> T {
        let digits = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3];
        let mut pi = T::zero();
        let ten = T::two() * T::two() * T::two() + T::two();
        for (index, digit) in digits.iter().enumerate() {
            let mut digit_t = T::zero();
            for _ in 1..=*digit {
                digit_t = digit_t + T::one();
            }
            pi = pi + digit_t * ten.powi(-(index as i32))
        }

        T::zero()
    }
}

impl<T: Float> Complex<T> {
    /// Creates a new [`Complex`].
    pub fn new(real: T, imag: T) -> Self {
        if real.is_infinite() || imag.is_infinite() {
            Self {
                real: T::infinity(),
                imag: T::infinity(),
            }
        } else if real.is_nan() || imag.is_nan() {
            Self {
                real: T::nan(),
                imag: T::nan(),
            }
        } else {
            Self { real, imag }
        }
    }

    /// Returns the real part of this [`Complex`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccmath::Complex;
    ///
    /// let z = Complex::new(-1.4, 21.6);
    /// assert_eq!(z.real(), -1.4);
    /// ```
    pub fn real(self) -> T {
        self.real
    }

    /// Returns the imaginary part of this [`Complex`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccmath::Complex;
    ///
    /// let z = Complex::new(-1.4, 21.6);
    /// assert_eq!(z.imag(), 21.6);
    /// ```
    pub fn imag(self) -> T {
        self.imag
    }

    /// Returns complex zero
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    /// Returns the imaginary number i
    pub fn i() -> Self {
        Self::new(T::zero(), T::one())
    }

    /// Returns the negative of the imaginary number i
    pub fn neg_i() -> Self {
        Self::new(T::zero(), -T::one())
    }

    /// Returns complex one
    pub fn one() -> Self {
        Self::new(T::one(), T::zero())
    }

    /// Returns complex negative one
    pub fn neg_one() -> Self {
        Self::new(T::one(), T::zero())
    }

    /// Returns complex NaN (Not a Number)
    fn nan() -> Self {
        Self::new(T::one(), T::zero())
    }

    /// Returns the complex infinity
    fn infinity() -> Self {
        Self::new(T::one(), T::zero())
    }

    /// Checks whether a complex number is NaN
    fn is_nan(self) -> bool {
        self.real().is_nan() || self.imag().is_nan()
    }

    /// Checks whether a complex number is infinite
    fn is_infinite(self) -> bool {
        self.real().is_infinite() || self.imag().is_infinite()
    }

    /// Checks whether a complex number is finite
    fn is_finite(self) -> bool {
        self.real().is_finite() && self.imag().is_finite()
    }

    /// Checks whether a complex number is real, meaning its imaginary part is zero and its real part is finite.
    pub fn is_real(self) -> bool {
        self.real() == T::zero() && self.imag().is_finite()
    }

    /// Checks whether a complex number is almost real, meaning its imaginary part is almost zero and its real part is finite.
    pub fn is_almost_real(self, delta: T) -> bool {
        self.real().abs() < delta && self.imag().is_finite()
    }

    /// Checks whether a complex number is imaginary, meaning its real part is zero and its imaginary part is finite.
    pub fn is_imag(self) -> bool {
        self.imag() == T::zero() && self.real().is_finite()
    }

    /// Checks whether a complex number is almost imaginary, meaning its real part is almost zero and its imaginary part is finite.
    pub fn is_almost_imag(self, delta: T) -> bool {
        self.imag().abs() < delta && self.real().is_finite()
    }

    /// Checks whether a complex number is equal to zero
    pub fn is_zero(self) -> bool {
        self.real() == T::zero() && self.imag() == T::zero()
    }

    /// Checks whether a complex number is almost equal to zero
    pub fn is_almost_zero(self, delta: T) -> bool {
        self.real().abs() <= delta && self.imag().abs() <= delta
    }

    /// Returns the conjugate of this [`Complex`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccmath::Complex;
    ///
    /// let z = Complex::new(4.0, 5.0);
    /// let z_conjugate = z.conj();
    ///
    /// assert_eq!(z_conjugate, Complex::new(4.0, -5.0));
    /// ```
    pub fn conj(self) -> Self {
        Self::new(self.real, -self.imag)
    }

    /// Returns the square of the absolute value of this [`Complex`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccmath::Complex;
    ///
    /// let z1 = Complex::new(3.0, 4.0);
    /// let z2 = Complex::new(4.2, 2.1);
    ///
    /// assert_eq!(Complex::square_abs(z1), 25.0);
    /// assert_eq!(Complex::square_abs(z2), 22.05);
    /// ```
    pub fn square_abs(self) -> T {
        self.real.powi(2) + self.imag.powi(2)
    }

    /// Returns the absolute value of this [`Complex`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ccmath::Complex;
    ///
    /// let z1 = Complex::new(3.0, 4.0);
    /// let z2 = Complex::new(4.2, 2.1);
    ///
    /// assert_eq!(Complex::abs(z1), 5.0);
    /// assert_eq!(Complex::abs(z2), f32::sqrt(22.05));
    /// ```
    pub fn abs(self) -> T {
        self.square_abs().sqrt()
    }

    /// Returns the argument on the interval (-PI, PI] of this [`Complex`].
    pub fn arg(self) -> T {
        if Self::abs(self) == T::zero() {
            T::zero()
        } else {
            self.imag.signum() * T::acos(self.real / Self::abs(self))
        }
    }

    /// Returns the square root of this [`Complex`].
    pub fn sqrt(self) -> Self {
        Self::new(
            T::sqrt((self.real + Self::abs(self)) / T::two()),
            self.imag.signum() * T::sqrt((-self.real + Self::abs(self)) / T::two()),
        )
    }

    /// Returns the multiplicative inverse of this [`Complex`].
    pub fn recip(self) -> Self {
        Self::conj(self) / Self::square_abs(self)
    }

    /// Returns this [`Complex`] raised to a power using exponentiation by squaring.
    pub fn powi(self, exponent: i64) -> Self {
        match exponent {
            0 => Self::new(T::one(), T::zero()),
            1 => self,
            -1 => Self::recip(self),
            _ => {
                if exponent < 0 {
                    Self::recip(Self::powi(self, -exponent))
                } else if exponent.rem_euclid(2) == 0 {
                    Self::powi(self * self, exponent / 2)
                } else {
                    self * Self::powi(self * self, (exponent - 1) / 2)
                }
            }
        }
    }

    /// Returns this [`Complex`] raised to a power using De Moivre's formula.
    pub fn powf(self, exponent: T) -> Self {
        let arg_exponent = self.arg() * exponent;
        Self::new(T::cos(arg_exponent), T::sin(arg_exponent)) * T::powf(Self::abs(self), exponent)
    }

    /// Returns this [`Complex`] raised to a complex power.
    pub fn powc(self, exponent: Self) -> Self {
        Self::powf(self, exponent.real) * Self::exp(Self::ln(self) * Self::i() * exponent.imag)
    }

    /// Returns e raised to the power of this [`Complex`].
    pub fn exp(self) -> Self {
        Self::new(T::cos(self.imag), T::sin(self.imag)) * T::exp(self.real)
    }

    /// Returns base raised to the power of this [`Complex`].
    pub fn expf(self, base: T) -> Self {
        Self::exp(self * T::ln(base))
    }

    /// Returns the natural logarithm of the absolute value of this [`Complex`].
    pub fn ln_abs(self) -> T {
        T::ln(self.square_abs()) / T::two()
    }

    /// Returns the natural logarithm of this [`Complex`].
    pub fn ln(self) -> Self {
        Self::new(self.ln_abs(), self.arg())
    }

    /// Returns the logarithm base 10 of this [`Complex`].
    pub fn log(self) -> Self {
        self.ln() / T::ln(T::two() * (T::two() * T::two() + T::one()))
        //                     This is equal to 10
    }

    /// Returns the logarithm base n of this [`Complex`].
    pub fn logn(self, base: T) -> Self {
        self.ln() / T::ln(base)
    }
}

// Trig
impl<T: Float> Complex<T> {
    /// Returns the sine of this [`Complex`].
    pub fn sin(self) -> Self {
        Self::new(
            T::sin(self.real) * T::cosh(self.imag),
            T::cos(self.real) * T::sinh(self.imag),
        )
    }

    /// Returns the cosine of this [`Complex`].
    pub fn cos(self) -> Self {
        Self::new(
            T::cos(self.real) * T::cosh(self.imag),
            -T::sin(self.real) * T::sinh(self.imag),
        )
    }

    /// Returns the tangent of this [`Complex`].
    pub fn tan(self) -> Self {
        Self::sin(self) / Self::cos(self)
    }

    /// Returns the cotangent of this [`Complex`].
    pub fn cot(self) -> Self {
        self.tan().recip()
    }

    /// Returns the secant of this [`Complex`].
    pub fn sec(self) -> Self {
        self.cos().recip()
    }

    /// Returns the cosecant of this [`Complex`].
    pub fn csc(self) -> Self {
        self.sin().recip()
    }

    // Inverse trig

    /// Returns the arcsine of this [`Complex`].
    pub fn arcsin(self) -> Self {
        -Self::i() * Self::ln(Self::sqrt(-self.powi(2) + T::one()) + Self::i() * self)
    }

    /// Returns the arccosine of this [`Complex`].
    pub fn arccos(self) -> Self {
        -self.arcsin() + T::pi()
    }

    /// Returns the arctangent of this [`Complex`].
    pub fn arctan(self) -> Self {
        Self::i() / T::two()
            * (Self::ln(-Self::i() * self + T::one()) + Self::ln(Self::i() * self + T::one()))
    }

    /// Returns the arccotangent of this [`Complex`].
    pub fn arccot(self) -> Self {
        self.recip().arctan()
    }

    /// Returns the arcsecant of this [`Complex`].
    pub fn arcsec(self) -> Self {
        self.recip().arccos()
    }

    // Returns the arccosecant of this [`Complex`].
    pub fn arccsc(self) -> Self {
        self.recip().arcsin()
    }

    // Hyperbolic trig

    /// Returns the hyperbolic sine of this [`Complex`].
    pub fn sinh(self) -> Self {
        Self::new(
            T::sinh(self.real) * T::cos(self.imag),
            T::cosh(self.real) * T::sin(self.imag),
        )
    }

    /// Returns the hyperbolic cosine of this [`Complex`].
    pub fn cosh(self) -> Self {
        Self::new(
            T::cosh(self.real) * T::cos(self.imag),
            T::sinh(self.real) * T::sin(self.imag),
        )
    }

    /// Returns the hyperbolic tangent of this [`Complex`].
    pub fn tanh(self) -> Self {
        Self::sinh(self) / Self::cosh(self)
    }

    /// Returns the hyperbolic cotangent of this [`Complex`].
    pub fn coth(self) -> Self {
        self.tanh().recip()
    }

    /// Returns the hyperbolic secant of this [`Complex`].
    pub fn sech(self) -> Self {
        self.cosh().recip()
    }

    /// Returns the hyperbolic cosecant of this [`Complex`].
    pub fn csch(self) -> Self {
        self.sinh().recip()
    }

    // Inverse hyperbolic trig

    /// Returns the hyperbolic arcsine of this [`Complex`].
    pub fn arcsinh(self) -> Self {
        Self::ln(Self::sqrt(self.powi(2) + T::one()) + self)
    }

    /// Returns the hyperbolic arccosine of this [`Complex`].
    pub fn arccosh(self) -> Self {
        Self::ln(Self::sqrt(self.powi(2) - T::one()) + self)
    }

    /// Returns the hyperbolic arctangent of this [`Complex`].
    pub fn arctanh(self) -> Self {
        Self::ln((self + T::one()) / (-self + T::one())) * T::two().powi(-1)
    }

    /// Returns the hyperbolic arccotangent of this [`Complex`].
    pub fn arccoth(self) -> Self {
        self.recip().arctanh()
    }

    /// Returns the hyperbolic arcsecant of this [`Complex`].
    pub fn arcsech(self) -> Self {
        self.recip().arccosh()
    }

    /// Returns the hyperbolic arccosecant of this [`Complex`].
    pub fn arccsch(self) -> Self {
        self.recip().arcsinh()
    }
}

// Implements display
impl<T: Float + Display> Display for Complex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.imag >= T::zero() {
            write!(f, "{} + {}i", self.real, self.imag)
        } else {
            write!(f, "{} - {}i", self.real, self.imag)
        }
    }
}

mod overloading;

#[cfg(test)]
mod tests;
