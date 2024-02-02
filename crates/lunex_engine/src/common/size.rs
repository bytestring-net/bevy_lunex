use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;

use crate::import::*;
use super::NiceDisplay;


// #===============#
// #=== TESTING ===#

#[cfg(test)]
mod test {
    use super::{Abs, Prc, Rem, NodeSize, Vec2};
    #[test]
    fn all () {
        assert_eq!(NodeSize::new().with_abs(Abs(5)) + Abs(5) + Abs(5), NodeSize::new().with_abs(Abs(15)));
        assert_eq!(NodeSize::new().with_prc(Prc(5)) + Prc(5) + Prc(5), NodeSize::new().with_prc(Prc(15)));
        assert_eq!(NodeSize::new().with_rem(Rem(5)) + Rem(5) + Rem(5), NodeSize::new().with_rem(Rem(15)));

        let amount = Abs(5) + Prc(10) + Rem(15);
        assert_eq!(amount, NodeSize::new().with_abs(Abs(5)).with_prc(Prc(10)).with_rem(Rem(15)));

        let mut new_amount = amount + Abs(20);
        assert_eq!(new_amount, NodeSize::new().with_abs(Abs(25)).with_prc(Prc(10)).with_rem(Rem(15)));

        new_amount += Prc(20);
        assert_eq!(new_amount, NodeSize::new().with_abs(Abs(25)).with_prc(Prc(30)).with_rem(Rem(15)));

        new_amount += amount;
        assert_eq!(new_amount, NodeSize::new().with_abs(Abs(30)).with_prc(Prc(40)).with_rem(Rem(30)));

        let node: NodeSize<Vec2> = Rem(Vec2::new(10.0, 12.0)).into();
        assert_eq!(node, NodeSize::<Vec2>::new().with_x(Rem(10.0)).with_y(Rem(12.0)));

        let _: NodeSize<Vec2> = NodeSize::from_standard((1.0, 2.0));

    }
}


// #========================#
// #=== TYPE DEFINITIONS ===#

/// Represents non-changing unit. Scale can be modified but by default `1Abs = 1Px`.
/// ## 🛠️ Example
/// ```
/// # use lunex_core::Abs;
/// let a: Abs<f32> = Abs(4.0) + Abs(6.0); // -> 10px
/// let b: Abs<f32> = Abs(4.0) * 2.0;      // -> 8px
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Abs<T>(pub T);

/// Represents scalable unit `0% to 100%`. `120%` is allowed.
/// ## 🛠️ Example
/// ```
/// # use lunex_core::Prc;
/// let a: Prc<f32> = Prc(25.0) + Prc(40.0); // -> 65%
/// let b: Prc<f32> = Prc(25.0) * 3.0;       // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Prc<T>(pub T);

/// Represents unit that is of size of the symbol `M`. Which is `16px` with `font size 16px` and so on.
/// ## 🛠️ Example
/// ```
/// # use lunex_core::Rem;
/// let a: Rem<f32> = Rem(1.0) + Rem(2.0); // -> 3rem == 48px with font size 16px
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rem<T>(pub T);


/// Represents collection of different units.
/// They are computed at runtime when layout computation is happening.
/// The supported units are:
/// * [`Abs`] [`Prc`] [`Rem`]
/// ## 📦 Types
/// First class implementations for `(T)` are:
/// * [`f32`] [`Vec2`] [`Vec3`] [`Vec4`]
/// ## 🛠️ Example
/// ```
/// # use lunex_core::{NodeSize, Abs, Rem};
/// let a: NodeSize<f32> = Abs(4.0) + Rem(1.0);  // -> 4px + 1rem
/// let b: NodeSize<f32> = Abs(40.0) - Prc(5.0); // -> 40px - 5%
/// let c: NodeSize<f32> = Prc(50.0).into();     // -> 50%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct NodeSize<T> {
    /// ## Absolute
    /// Represents non-changing unit. Scale can vary but by default `1Abs = 1Px`.
    pub abs: Option<T>,
    /// ## Percentage
    /// `0% to 100%`. Overflow allowed.
    pub prc: Option<T>,
    /// ## Rem
    /// Size of symbol `M` which is `16px` with `font size 16px` and so on.
    pub rem: Option<T>,
}


// #===============================#
// #=== GENERIC IMPLEMENTATIONS ===#

// # Impl `with_abs` and `set_abs` ...
impl<T> NodeSize<T> {
    /// ## With
    /// Replaces the value of appropriate units with the new value.
    pub fn with(mut self, other: NodeSize<T>) -> Self {
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { *v1 = v2 } else { self.abs = Some(v2) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { *v1 = v2 } else { self.prc = Some(v2) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { *v1 = v2 } else { self.rem = Some(v2) } }
        self
    }
    /// ## With Absolute
    /// Replaces the value with the new `absolute` value.
    pub fn with_abs(mut self, abs: Abs<T>) -> Self {
        self.abs = Some(abs.0);
        self
    }
    /// ## With Percentage
    /// Replaces the value with the new `percentage` value.
    pub fn with_prc(mut self, prc: Prc<T>) -> Self {
        self.prc = Some(prc.0);
        self
    }
    /// ## With Rem
    /// Replaces the value with the new `rem` value.
    pub fn with_rem(mut self, rem: Rem<T>) -> Self {
        self.rem = Some(rem.0);
        self
    }
    /// ## Set
    /// Sets the value of appropriate units to the new value.
    pub fn set(&mut self, other: NodeSize<T>) {
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { *v1 = v2 } else { self.abs = Some(v2) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { *v1 = v2 } else { self.prc = Some(v2) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { *v1 = v2 } else { self.rem = Some(v2) } }
    }
    /// ## Set Absolute
    /// Sets the value to the new `absolute` value.
    pub fn set_abs(&mut self, abs: Abs<T>) {
        self.abs = Some(abs.0);
    }
    /// ## Set Percentage
    /// Sets the value to the new `percentage` value.
    pub fn set_prc(&mut self, prc: Prc<T>) {
        self.prc = Some(prc.0);
    }
    /// ## Set Rem
    /// Sets the value to the new `rem` value.
    pub fn set_rem(&mut self, rem: Rem<T>) {
        self.rem = Some(rem.0);
    }
}

// # Impl `from_standard` Tailwind scale
impl<T: Mul<f32, Output = T>> NodeSize<T> {
    /// ## From Standard
    /// Creates new NodeSize from the standardized [TailwindCSS](https://tailwindcss.com/docs/customizing-spacing#default-spacing-scale) convention.
    /// * `0.5 == 0.125rem`
    /// * `1 == 0.25rem`
    /// * `2 == 0.5rem`
    /// * `3 == 0.75rem`
    /// * `4 == 1rem`
    /// * _and so on..._
    /// 
    pub fn from_standard(size: impl Into<T>) -> NodeSize<T> {
        Rem(size.into() * 0.25).into()
    }
}


// CONVERSION ======

// # Impl into `Abs(T) -> NodeSize(T)`
impl <T> Into<NodeSize<T>> for Abs<T> {
    fn into(self) -> NodeSize<T> {
        NodeSize::new().with_abs(self)
    }
}
// # Impl into `Prc(T) -> NodeSize(T)`
impl <T> Into<NodeSize<T>> for Prc<T> {
    fn into(self) -> NodeSize<T> {
        NodeSize::new().with_prc(self)
    }
}
// # Impl into `Rem(T) -> NodeSize(T)`
impl <T> Into<NodeSize<T>> for Rem<T> {
    fn into(self) -> NodeSize<T> {
        NodeSize::new().with_rem(self)
    }
}


// ADDITION ======

// # Impl `Abs(T) + Abs(T)`
impl<T: Add<Output = T>> Add for Abs<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Abs(self.0 + other.0)
    }
}
// # Impl `Abs(T) + Prc(T)`
impl<T: Add<Output = T>> Add<Prc<T>> for Abs<T> {
    type Output = NodeSize<T>;
    fn add(self, other: Prc<T>) -> Self::Output {
        NodeSize::from_abs_prc(self.0, other.0)
    }
}
// # Impl `Abs(T) + Rem(T)`
impl<T: Add<Output = T>> Add<Rem<T>> for Abs<T> {
    type Output = NodeSize<T>;
    fn add(self, other: Rem<T>) -> Self::Output {
        NodeSize::from_abs_rem(self.0, other.0)
    }
}

// # Impl `Prc(T) + Prc(T)`
impl<T: Add<Output = T>> Add for Prc<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Prc(self.0 + other.0)
    }
}
// # Impl `Prc(T) + Abs(T)`
impl<T: Add<Output = T>> Add<Abs<T>> for Prc<T> {
    type Output = NodeSize<T>;
    fn add(self, other: Abs<T>) -> Self::Output {
        NodeSize::from_abs_prc(other.0, self.0)
    }
}
// # Impl `Prc(T) + Rem(T)`
impl<T: Add<Output = T>> Add<Rem<T>> for Prc<T> {
    type Output = NodeSize<T>;
    fn add(self, other: Rem<T>) -> Self::Output {
        NodeSize::from_prc_rem(self.0, other.0)
    }
}

// # Impl `Rem(T) + Rem(T)`
impl<T: Add<Output = T>> Add for Rem<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Rem(self.0 + other.0)
    }
}
// # Impl `Rem(T) + Abs(T)`
impl<T: Add<Output = T>> Add<Abs<T>> for Rem<T> {
    type Output = NodeSize<T>;
    fn add(self, other: Abs<T>) -> Self::Output {
        NodeSize::from_abs_rem(other.0, self.0)
    }
}
// # Impl `Rem(T) + Prc(T)`
impl<T: Add<Output = T>> Add<Prc<T>> for Rem<T> {
    type Output = NodeSize<T>;
    fn add(self, other: Prc<T>) -> Self::Output {
        NodeSize::from_prc_rem(other.0, self.0)
    }
}

// # Impl `NodeSize(T) + NodeSize(T)`
impl<T: Add<Output = T> + Add> Add for NodeSize<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        NodeSize {
            abs: if let Some(v1) = self.abs {
                    if let Some(v2) = other.abs { Some(v1 + v2)} else { Some(v1) }
                } else {
                    if let Some(v2) = other.abs { Some(v2) } else { None }
                },
            prc: if let Some(v1) = self.prc {
                    if let Some(v2) = other.prc { Some(v1 + v2) } else { Some(v1) }
                } else {
                    if let Some(v2) = other.prc { Some(v2) } else { None }
                },
            rem: if let Some(v1) = self.rem {
                    if let Some(v2) = other.rem { Some(v1 + v2) } else { Some(v1) }
                } else {
                    if let Some(v2) = other.rem { Some(v2) } else { None }
                },
        }
    }
}
// # Impl `NodeSize(T) + Abs(T)`
impl<T: Add<Output = T> + Add> Add<Abs<T>> for NodeSize<T> {
    type Output = Self;
    fn add(mut self, other: Abs<T>) -> Self::Output {
        match self.abs {
            Some(v) => {
                self.abs = Some(v + other.0);
                self
            },
            None => self.with_abs(other),
        }
    }
}
// # Impl `NodeSize(T) + Prc(T)`
impl<T: Add<Output = T> + Add> Add<Prc<T>> for NodeSize<T> {
    type Output = Self;
    fn add(mut self, other: Prc<T>) -> Self::Output {
        match self.prc {
            Some(v) => {
                self.prc = Some(v + other.0);
                self
            },
            None => self.with_prc(other),
        }
    }
}
// # Impl `NodeSize(T) + Rem(T)`
impl<T: Add<Output = T> + Add> Add<Rem<T>> for NodeSize<T> {
    type Output = Self;
    fn add(mut self, other: Rem<T>) -> Self::Output {
        match self.rem {
            Some(v) => {
                self.rem = Some(v + other.0);
                self
            },
            None => self.with_rem(other),
        }
    }
}

// # Impl `NodeSize(T) += NodeSize(T)`
impl<T: Add<Output = T> + Copy> AddAssign for NodeSize<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
// # Impl `NodeSize(T) += Abs(T)`
impl<T: Add<Output = T> + Copy> AddAssign<Abs<T>> for NodeSize<T> {
    fn add_assign(&mut self, rhs: Abs<T>) {
        match self.abs {
            Some(v) => self.abs = Some(v + rhs.0),
            None => self.abs = Some(rhs.0),
        }
    }
}
// # Impl `NodeSize(T) += Prc(T)`
impl<T: Add<Output = T> + Copy> AddAssign<Prc<T>> for NodeSize<T> {
    fn add_assign(&mut self, rhs: Prc<T>) {
        match self.prc {
            Some(v) => self.prc = Some(v + rhs.0),
            None => self.prc = Some(rhs.0),
        }
    }
}
// # Impl `NodeSize(T) += Rem(T)`
impl<T: Add<Output = T> + Copy> AddAssign<Rem<T>> for NodeSize<T> {
    fn add_assign(&mut self, rhs: Rem<T>) {
        match self.rem {
            Some(v) => self.rem = Some(v + rhs.0),
            None => self.rem = Some(rhs.0),
        }
    }
}


// NEGATION ======

impl<T: Neg<Output = T>> Neg for Abs<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Abs(-self.0)
    }
}
impl<T: Neg<Output = T>> Neg for Prc<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Prc(-self.0)
    }
}
impl<T: Neg<Output = T>> Neg for Rem<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Rem(-self.0)
    }
}
impl<T: Neg<Output = T>> Neg for NodeSize<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        NodeSize {
            abs: if let Some(v) = self.abs { Some(-v) } else { None },
            prc: if let Some(v) = self.prc { Some(-v) } else { None },
            rem: if let Some(v) = self.rem { Some(-v) } else { None },
        }
    }
}


// SUBTRACTION ======

// # Impl `Abs(T) - Abs(T)`
impl<T: Sub<Output = T>> Sub for Abs<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Abs(self.0 - other.0)
    }
}
// # Impl `Abs(T) - Prc(T)`
impl<T: Sub<Output = T>> Sub<Prc<T>> for Abs<T> where T: Neg<Output = T> {
    type Output = NodeSize<T>;
    fn sub(self, other: Prc<T>) -> Self::Output {
        NodeSize::from_abs_prc(self.0, -other.0)
    }
}
// # Impl `Abs(T) - Rem(T)`
impl<T: Sub<Output = T>> Sub<Rem<T>> for Abs<T> where T: Neg<Output = T> {
    type Output = NodeSize<T>;
    fn sub(self, other: Rem<T>) -> Self::Output {
        NodeSize::from_abs_rem(self.0, -other.0)
    }
}

// # Impl `Prc(T) - Prc(T)`
impl<T: Sub<Output = T>> Sub for Prc<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Prc(self.0 - other.0)
    }
}
// # Impl `Prc(T) - Abs(T)`
impl<T: Sub<Output = T>> Sub<Abs<T>> for Prc<T> where T: Neg<Output = T> {
    type Output = NodeSize<T>;
    fn sub(self, other: Abs<T>) -> Self::Output {
        NodeSize::from_abs_prc(-other.0, self.0)
    }
}
// # Impl `Prc(T) - Rem(T)`
impl<T: Sub<Output = T>> Sub<Rem<T>> for Prc<T> where T: Neg<Output = T> {
    type Output = NodeSize<T>;
    fn sub(self, other: Rem<T>) -> Self::Output {
        NodeSize::from_prc_rem(self.0, -other.0)
    }
}

// # Impl `Rem(T) - Rem(T)`
impl<T: Sub<Output = T>> Sub for Rem<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Rem(self.0 - other.0)
    }
}
// # Impl `Rem(T) - Abs(T)`
impl<T: Sub<Output = T>> Sub<Abs<T>> for Rem<T> where T: Neg<Output = T> {
    type Output = NodeSize<T>;
    fn sub(self, other: Abs<T>) -> Self::Output {
        NodeSize::from_abs_rem(-other.0, self.0)
    }
}
// # Impl `Rem(T) - Prc(T)`
impl<T: Sub<Output = T>> Sub<Prc<T>> for Rem<T> where T: Neg<Output = T> {
    type Output = NodeSize<T>;
    fn sub(self, other: Prc<T>) -> Self::Output {
        NodeSize::from_prc_rem(-other.0, self.0)
    }
}

// # Impl `NodeSize(T) - NodeSize(T)`
impl<T: Sub<Output = T> + Sub + Neg<Output = T>> Sub for NodeSize<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        NodeSize {
            abs: if let Some(v1) = self.abs {
                    if let Some(v2) = other.abs { Some(v1 - v2)} else { Some(v1) }
                } else {
                    if let Some(v2) = other.abs { Some(-v2) } else { None }
                },
            prc: if let Some(v1) = self.prc {
                    if let Some(v2) = other.prc { Some(v1 - v2) } else { Some(v1) }
                } else {
                    if let Some(v2) = other.prc { Some(-v2) } else { None }
                },
            rem: if let Some(v1) = self.rem {
                    if let Some(v2) = other.rem { Some(v1 - v2) } else { Some(v1) }
                } else {
                    if let Some(v2) = other.rem { Some(-v2) } else { None }
                },
        }
    }
}
// # Impl `NodeSize(T) - Abs(T)`
impl<T: Sub<Output = T> + Sub> Sub<Abs<T>> for NodeSize<T> {
    type Output = Self;
    fn sub(mut self, other: Abs<T>) -> Self::Output {
        match self.abs {
            Some(v) => {
                self.abs = Some(v - other.0);
                self
            },
            None => self.with_abs(other),
        }
    }
}
// # Impl `NodeSize(T) - Prc(T)`
impl<T: Sub<Output = T> + Sub> Sub<Prc<T>> for NodeSize<T> {
    type Output = Self;
    fn sub(mut self, other: Prc<T>) -> Self::Output {
        match self.prc {
            Some(v) => {
                self.prc = Some(v - other.0);
                self
            },
            None => self.with_prc(other),
        }
    }
}
// # Impl `NodeSize(T) - Rem(T)`
impl<T: Sub<Output = T> + Sub> Sub<Rem<T>> for NodeSize<T> {
    type Output = Self;
    fn sub(mut self, other: Rem<T>) -> Self::Output {
        match self.rem {
            Some(v) => {
                self.rem = Some(v - other.0);
                self
            },
            None => self.with_rem(other),
        }
    }
}

// # Impl `NodeSize(T) -= NodeSize(T)`
impl<T: Sub<Output = T> + Copy + Neg<Output = T>> SubAssign for NodeSize<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
// # Impl `NodeSize(T) -= Abs(T)`
impl<T: Sub<Output = T> + Copy> SubAssign<Abs<T>> for NodeSize<T> {
    fn sub_assign(&mut self, rhs: Abs<T>) {
        match self.abs {
            Some(v) => self.abs = Some(v - rhs.0),
            None => self.abs = Some(rhs.0),
        }
    }
}
// # Impl `NodeSize(T) -= Prc(T)`
impl<T: Sub<Output = T> + Copy> SubAssign<Prc<T>> for NodeSize<T> {
    fn sub_assign(&mut self, rhs: Prc<T>) {
        match self.prc {
            Some(v) => self.prc = Some(v - rhs.0),
            None => self.prc = Some(rhs.0),
        }
    }
}
// # Impl `NodeSize(T) -= Rem(T)`
impl<T: Sub<Output = T> + Copy> SubAssign<Rem<T>> for NodeSize<T> {
    fn sub_assign(&mut self, rhs: Rem<T>) {
        match self.rem {
            Some(v) => self.rem = Some(v - rhs.0),
            None => self.rem = Some(rhs.0),
        }
    }
}


// MULTIPLICATION ======

// # Impl `Abs(T) * Abs(T)`
impl<T: Mul<Output = T>> Mul for Abs<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Abs(self.0 * other.0)
    }
}
// # Impl `Prc(T) * Prc(T)`
impl<T: Mul<Output = T>> Mul for Prc<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Prc(self.0 * other.0)
    }
}
// # Impl `Rem(T) * Rem(T)`
impl<T: Mul<Output = T>> Mul for Rem<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Rem(self.0 * other.0)
    }
}

// # Impl `NodeSize(T) * NodeSize(T)`
impl<T: Mul<Output = T> + Mul> Mul for NodeSize<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let mut output = NodeSize::new();
        if let Some(v1) = self.abs {
            if let Some(v2) = other.abs {
                output.abs = Some(v1 * v2);
            }
        }
        if let Some(v1) = self.prc {
            if let Some(v2) = other.prc {
                output.prc = Some(v1 * v2);
            }
        }
        if let Some(v1) = self.rem {
            if let Some(v2) = other.rem {
                output.rem = Some(v1 * v2);
            }
        }
        output
    }
}
// # Impl `NodeSize(T) * Abs(T)`
impl<T: Mul<Output = T> + Mul> Mul<Abs<T>> for NodeSize<T> {
    type Output = Self;
    fn mul(mut self, other: Abs<T>) -> Self::Output {
        if let Some(v) = self.abs {
            self.abs = Some(v * other.0);
        }
        self
    }
}
// # Impl `NodeSize(T) * Prc(T)`
impl<T: Mul<Output = T> + Mul> Mul<Prc<T>> for NodeSize<T> {
    type Output = Self;
    fn mul(mut self, other: Prc<T>) -> Self::Output {
        if let Some(v) = self.prc {
            self.prc = Some(v * other.0);
        }
        self
    }
}
// # Impl `NodeSize(T) * Rem(T)`
impl<T: Mul<Output = T> + Mul> Mul<Rem<T>> for NodeSize<T> {
    type Output = Self;
    fn mul(mut self, other: Rem<T>) -> Self::Output {
        if let Some(v) = self.rem {
            self.rem = Some(v * other.0);
        }
        self
    }
}

// # Impl `NodeSize(T) *= NodeSize(T)`
impl<T: Mul<Output = T> + Copy> MulAssign for NodeSize<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}
// # Impl `NodeSize(T) *= Abs(T)`
impl<T: Mul<Output = T> + Copy> MulAssign<Abs<T>> for NodeSize<T> {
    fn mul_assign(&mut self, rhs: Abs<T>) {
        if let Some(v) = self.abs {
            self.abs = Some(v * rhs.0);
        }
    }
}
// # Impl `NodeSize(T) *= Prc(T)`
impl<T: Mul<Output = T> + Copy> MulAssign<Prc<T>> for NodeSize<T> {
    fn mul_assign(&mut self, rhs: Prc<T>) {
        if let Some(v) = self.prc {
            self.prc = Some(v * rhs.0);
        }
    }
}
// # Impl `NodeSize(T) *= Rem(T)`
impl<T: Mul<Output = T> + Copy> MulAssign<Rem<T>> for NodeSize<T> {
    fn mul_assign(&mut self, rhs: Rem<T>) {
        if let Some(v) = self.rem {
            self.rem = Some(v * rhs.0);
        }
    }
}


// MULTIPLICATION with F32 ======

// # Impl `NodeSize(T) * f32 = NodeSize(T)`
impl<T: Mul<f32, Output = T>> Mul<f32> for NodeSize<T> {
    type Output = NodeSize<T>;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut output = NodeSize::new();
        if let Some(v) = self.abs {
            output.abs = Some(v * rhs);
        }
        if let Some(v) = self.prc {
            output.prc = Some(v * rhs);
        }
        if let Some(v) = self.rem {
            output.rem = Some(v * rhs);
        }
        output
    }
}
// # Impl `Abs(T) * f32 = Abs(T)`
impl<T: Mul<f32, Output = T>> Mul<f32> for Abs<T> {
    type Output = Abs<T>;
    fn mul(self, rhs: f32) -> Self::Output {
        Abs(self.0 * rhs)
    }
}
// # Impl `Prc(T) * f32 = Prc(T)`
impl<T: Mul<f32, Output = T>> Mul<f32> for Prc<T> {
    type Output = Prc<T>;
    fn mul(self, rhs: f32) -> Self::Output {
        Prc(self.0 * rhs)
    }
}
// # Impl `Rem(T) * f32 = Rem(T)`
impl<T: Mul<f32, Output = T>> Mul<f32> for Rem<T> {
    type Output = Rem<T>;
    fn mul(self, rhs: f32) -> Self::Output {
        Rem(self.0 * rhs)
    }
}

// # Impl `NodeSize(T) *= f32`
impl<T: Mul<f32, Output = T> + Copy> MulAssign<f32> for NodeSize<T> {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}
// # Impl `Abs(T) *= f32`
impl<T: Mul<f32, Output = T> + Copy> MulAssign<f32> for Abs<T> {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Abs(self.0 * rhs);
    }
}
// # Impl `Prc(T) *= f32`
impl<T: Mul<f32, Output = T> + Copy> MulAssign<f32> for Prc<T> {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Prc(self.0 * rhs);
    }
}
// # Impl `Rem(T) *= f32`
impl<T: Mul<f32, Output = T> + Copy> MulAssign<f32> for Rem<T> {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Rem(self.0 * rhs);
    }
}


// #===============================#
// #=== CASTING IMPLEMENTATIONS ===#

// # Impl ((x, x)) => NodeSize(Vec2)
impl Into<NodeSize<Vec2>> for Abs<(f32, f32)> {
    fn into(self) -> NodeSize<Vec2> {
        NodeSize::from_abs((self.0.0, self.0.1).into())
    }
}
impl Into<NodeSize<Vec2>> for Prc<(f32, f32)> {
    fn into(self) -> NodeSize<Vec2> {
        NodeSize::from_prc((self.0.0, self.0.1).into())
    }
}
impl Into<NodeSize<Vec2>> for Rem<(f32, f32)> {
    fn into(self) -> NodeSize<Vec2> {
        NodeSize::from_rem((self.0.0, self.0.1).into())
    }
}

// # Impl ((x, x, x)) => NodeSize(Vec3)
impl Into<NodeSize<Vec3>> for Abs<(f32, f32, f32)> {
    fn into(self) -> NodeSize<Vec3> {
        NodeSize::from_abs((self.0.0, self.0.1, self.0.2).into())
    }
}
impl Into<NodeSize<Vec3>> for Prc<(f32, f32, f32)> {
    fn into(self) -> NodeSize<Vec3> {
        NodeSize::from_prc((self.0.0, self.0.1, self.0.2).into())
    }
}
impl Into<NodeSize<Vec3>> for Rem<(f32, f32, f32)> {
    fn into(self) -> NodeSize<Vec3> {
        NodeSize::from_rem((self.0.0, self.0.1, self.0.2).into())
    }
}

// # Impl ((x, x, x, x)) => NodeSize(Vec4)
impl Into<NodeSize<Vec4>> for Abs<(f32, f32, f32, f32)> {
    fn into(self) -> NodeSize<Vec4> {
        NodeSize::from_abs((self.0.0, self.0.1, self.0.2, self.0.3).into())
    }
}
impl Into<NodeSize<Vec4>> for Prc<(f32, f32, f32, f32)> {
    fn into(self) -> NodeSize<Vec4> {
        NodeSize::from_prc((self.0.0, self.0.1, self.0.2, self.0.3).into())
    }
}
impl Into<NodeSize<Vec4>> for Rem<(f32, f32, f32, f32)> {
    fn into(self) -> NodeSize<Vec4> {
        NodeSize::from_rem((self.0.0, self.0.1, self.0.2, self.0.3).into())
    }
}

// # Impl (x) => NodeSize(Vec2)
impl Into<NodeSize<Vec2>> for Abs<f32> {
    fn into(self) -> NodeSize<Vec2> {
        NodeSize::from_abs((self.0, self.0).into())
    }
}
impl Into<NodeSize<Vec2>> for Prc<f32> {
    fn into(self) -> NodeSize<Vec2> {
        NodeSize::from_prc((self.0, self.0).into())
    }
}
impl Into<NodeSize<Vec2>> for Rem<f32> {
    fn into(self) -> NodeSize<Vec2> {
        NodeSize::from_rem((self.0, self.0).into())
    }
}
impl Into<NodeSize<Vec2>> for NodeSize<f32> {
    fn into(self) -> NodeSize<Vec2> {
        let mut out = NodeSize::<Vec2>::new();
        out.set_x(self);
        out.set_y(self);
        out
    }
}

// # Impl (x) => NodeSize(Vec3)
impl Into<NodeSize<Vec3>> for Abs<f32> {
    fn into(self) -> NodeSize<Vec3> {
        NodeSize::from_abs((self.0, self.0, self.0).into())
    }
}
impl Into<NodeSize<Vec3>> for Prc<f32> {
    fn into(self) -> NodeSize<Vec3> {
        NodeSize::from_prc((self.0, self.0, self.0).into())
    }
}
impl Into<NodeSize<Vec3>> for Rem<f32> {
    fn into(self) -> NodeSize<Vec3> {
        NodeSize::from_rem((self.0, self.0, self.0).into())
    }
}
impl Into<NodeSize<Vec3>> for NodeSize<f32> {
    fn into(self) -> NodeSize<Vec3> {
        let mut out = NodeSize::<Vec3>::new();
        out.set_x(self);
        out.set_y(self);
        out.set_z(self);
        out
    }
}

// # Impl (x) => NodeSize(Vec4)
impl Into<NodeSize<Vec4>> for Abs<f32> {
    fn into(self) -> NodeSize<Vec4> {
        NodeSize::from_abs((self.0, self.0, self.0, self.0).into())
    }
}
impl Into<NodeSize<Vec4>> for Prc<f32> {
    fn into(self) -> NodeSize<Vec4> {
        NodeSize::from_prc((self.0, self.0, self.0, self.0).into())
    }
}
impl Into<NodeSize<Vec4>> for Rem<f32> {
    fn into(self) -> NodeSize<Vec4> {
        NodeSize::from_rem((self.0, self.0, self.0, self.0).into())
    }
}
impl Into<NodeSize<Vec4>> for NodeSize<f32> {
    fn into(self) -> NodeSize<Vec4> {
        let mut out = NodeSize::<Vec4>::new();
        out.set_x(self);
        out.set_y(self);
        out.set_z(self);
        out.set_w(self);
        out
    }
}

// #================================#
// #=== SPECIFIC IMPLEMENTATIONS ===#

// # Impl `splat2`
impl Abs<Vec2> {
    /// Is equal to writing:
    /// ```no_run
    /// Abs(Vec2::splat(v))
    /// ```
    pub const fn splat2(v: f32) -> Self {
        Abs(Vec2::splat(v))
    }
}
// # Impl `splat3`
impl Abs<Vec3> {
    /// Is equal to writing:
    /// ```no_run
    /// Abs(Vec3::splat(v))
    /// ```
    pub const fn splat3(v: f32) -> Self {
        Abs(Vec3::splat(v))
    }
}
// # Impl `splat4`
impl Abs<Vec4> {
    /// Is equal to writing:
    /// ```no_run
    /// Abs(Vec4::splat(v))
    /// ```
    pub const fn splat4(v: f32) -> Self {
        Abs(Vec4::splat(v))
    }
}

// # Impl `splat2`
impl Prc<Vec2> {
    /// Is equal to writing:
    /// ```no_run
    /// Prc(Vec2::splat(v))
    /// ```
    pub const fn splat2(v: f32) -> Self {
        Prc(Vec2::splat(v))
    }
}
// # Impl `splat3`
impl Prc<Vec3> {
    /// Is equal to writing:
    /// ```no_run
    /// Prc(Vec3::splat(v))
    /// ```
    pub const fn splat3(v: f32) -> Self {
        Prc(Vec3::splat(v))
    }
}
// # Impl `splat4`
impl Prc<Vec4> {
    /// Is equal to writing:
    /// ```no_run
    /// Prc(Vec4::splat(v))
    /// ```
    pub const fn splat4(v: f32) -> Self {
        Prc(Vec4::splat(v))
    }
}

// # Impl `splat2`
impl Rem<Vec2> {
    /// Is equal to writing:
    /// ```no_run
    /// Rem(Vec2::splat(v))
    /// ```
    pub const fn splat2(v: f32) -> Self {
        Rem(Vec2::splat(v))
    }
}
// # Impl `splat3`
impl Rem<Vec3> {
    /// Is equal to writing:
    /// ```no_run
    /// Rem(Vec3::splat(v))
    /// ```
    pub const fn splat3(v: f32) -> Self {
        Rem(Vec3::splat(v))
    }
}
// # Impl `splat4`
impl Rem<Vec4> {
    /// Is equal to writing:
    /// ```no_run
    /// Rem(Vec4::splat(v))
    /// ```
    pub const fn splat4(v: f32) -> Self {
        Rem(Vec4::splat(v))
    }
}


// # Impl `get_x`, `with_x` and `set_x` ...
impl NodeSize<Vec2> {
    /// Gets the X value of all units.
    pub fn get_x(&self) -> NodeSize<f32> {
        let mut out = NodeSize::<f32>::new();
        if let Some(v) = self.abs { out += Abs(v.x) }
        if let Some(v) = self.prc { out += Prc(v.x) }
        if let Some(v) = self.rem { out += Rem(v.x) }
        out
    }
    /// Replaces the X value of appropriate units with the new value.
    pub fn with_x(mut self, other: impl Into<NodeSize<f32>>) -> Self {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.x = v2 } else { self.abs = Some(Vec2::new(v2, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.x = v2 } else { self.prc = Some(Vec2::new(v2, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.x = v2 } else { self.rem = Some(Vec2::new(v2, 0.0)) } }
        self
    }
    /// Sets the X value of appropriate units with the new value.
    pub fn set_x(&mut self, other: impl Into<NodeSize<f32>>) {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.x = v2 } else { self.abs = Some(Vec2::new(v2, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.x = v2 } else { self.prc = Some(Vec2::new(v2, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.x = v2 } else { self.rem = Some(Vec2::new(v2, 0.0)) } }
    }
    /// Gets the Y value of all units.
    pub fn get_y(&self) -> NodeSize<f32> {
        let mut out = NodeSize::<f32>::new();
        if let Some(v) = self.abs { out += Abs(v.y) }
        if let Some(v) = self.prc { out += Prc(v.y) }
        if let Some(v) = self.rem { out += Rem(v.y) }
        out
    }
    /// Replaces the Y value of appropriate units with the new value.
    pub fn with_y(mut self, other: impl Into<NodeSize<f32>>) -> Self {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.y = v2 } else { self.abs = Some(Vec2::new(0.0, v2)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.y = v2 } else { self.prc = Some(Vec2::new(0.0, v2)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.y = v2 } else { self.rem = Some(Vec2::new(0.0, v2)) } }
        self
    }
    /// Sets the Y value of appropriate units with the new value.
    pub fn set_y(&mut self, other: impl Into<NodeSize<f32>>) {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.y = v2 } else { self.abs = Some(Vec2::new(0.0, v2)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.y = v2 } else { self.prc = Some(Vec2::new(0.0, v2)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.y = v2 } else { self.rem = Some(Vec2::new(0.0, v2)) } }
    }
}
// # Impl `get_x`, `with_x` and `set_x` ...
impl NodeSize<Vec3> {
    /// Gets the X value of all units.
    pub fn get_x(&self) -> NodeSize<f32> {
        let mut out = NodeSize::<f32>::new();
        if let Some(v) = self.abs { out += Abs(v.x) }
        if let Some(v) = self.prc { out += Prc(v.x) }
        if let Some(v) = self.rem { out += Rem(v.x) }
        out
    }
    /// Replaces the X value of appropriate units with the new value.
    pub fn with_x(mut self, other: impl Into<NodeSize<f32>>) -> Self {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.x = v2 } else { self.abs = Some(Vec3::new(v2, 0.0, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.x = v2 } else { self.prc = Some(Vec3::new(v2, 0.0, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.x = v2 } else { self.rem = Some(Vec3::new(v2, 0.0, 0.0)) } }
        self
    }
    /// Sets the X value of appropriate units with the new value.
    pub fn set_x(&mut self, other: impl Into<NodeSize<f32>>) {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.x = v2 } else { self.abs = Some(Vec3::new(v2, 0.0, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.x = v2 } else { self.prc = Some(Vec3::new(v2, 0.0, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.x = v2 } else { self.rem = Some(Vec3::new(v2, 0.0, 0.0)) } }
    }
    /// Gets the Y value of all units.
    pub fn get_y(&self) -> NodeSize<f32> {
        let mut out = NodeSize::<f32>::new();
        if let Some(v) = self.abs { out += Abs(v.y) }
        if let Some(v) = self.prc { out += Prc(v.y) }
        if let Some(v) = self.rem { out += Rem(v.y) }
        out
    }
    /// Replaces the Y value of appropriate units with the new value.
    pub fn with_y(mut self, other: impl Into<NodeSize<f32>>) -> Self {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.y = v2 } else { self.abs = Some(Vec3::new(0.0, v2, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.y = v2 } else { self.prc = Some(Vec3::new(0.0, v2, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.y = v2 } else { self.rem = Some(Vec3::new(0.0, v2, 0.0)) } }
        self
    }
    /// Sets the Y value of appropriate units with the new value.
    pub fn set_y(&mut self, other: impl Into<NodeSize<f32>>) {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.y = v2 } else { self.abs = Some(Vec3::new(0.0, v2, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.y = v2 } else { self.prc = Some(Vec3::new(0.0, v2, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.y = v2 } else { self.rem = Some(Vec3::new(0.0, v2, 0.0)) } }
    }
    /// Gets the Z value of all units.
    pub fn get_z(&self) -> NodeSize<f32> {
        let mut out = NodeSize::<f32>::new();
        if let Some(v) = self.abs { out += Abs(v.z) }
        if let Some(v) = self.prc { out += Prc(v.z) }
        if let Some(v) = self.rem { out += Rem(v.z) }
        out
    }
    /// Replaces the Z value of appropriate units with the new value.
    pub fn with_z(mut self, other: impl Into<NodeSize<f32>>) -> Self {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.z = v2 } else { self.abs = Some(Vec3::new(0.0, 0.0, v2)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.z = v2 } else { self.prc = Some(Vec3::new(0.0, 0.0, v2)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.z = v2 } else { self.rem = Some(Vec3::new(0.0, 0.0, v2)) } }
        self
    }
    /// Sets the Z value of appropriate units with the new value.
    pub fn set_z(&mut self, other: impl Into<NodeSize<f32>>) {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.z = v2 } else { self.abs = Some(Vec3::new(0.0, 0.0, v2)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.z = v2 } else { self.prc = Some(Vec3::new(0.0, 0.0, v2)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.z = v2 } else { self.rem = Some(Vec3::new(0.0, 0.0, v2)) } }
    }
}
// # Impl `get_x`, `with_x` and `set_x` ...
impl NodeSize<Vec4> {
    /// Gets the X value of all units.
    pub fn get_x(&self) -> NodeSize<f32> {
        let mut out = NodeSize::<f32>::new();
        if let Some(v) = self.abs { out += Abs(v.x) }
        if let Some(v) = self.prc { out += Prc(v.x) }
        if let Some(v) = self.rem { out += Rem(v.x) }
        out
    }
    /// Replaces the X value of appropriate units with the new value.
    pub fn with_x(mut self, other: impl Into<NodeSize<f32>>) -> Self {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.x = v2 } else { self.abs = Some(Vec4::new(v2, 0.0, 0.0, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.x = v2 } else { self.prc = Some(Vec4::new(v2, 0.0, 0.0, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.x = v2 } else { self.rem = Some(Vec4::new(v2, 0.0, 0.0, 0.0)) } }
        self
    }
    /// Sets the X value of appropriate units with the new value.
    pub fn set_x(&mut self, other: impl Into<NodeSize<f32>>) {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.x = v2 } else { self.abs = Some(Vec4::new(v2, 0.0, 0.0, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.x = v2 } else { self.prc = Some(Vec4::new(v2, 0.0, 0.0, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.x = v2 } else { self.rem = Some(Vec4::new(v2, 0.0, 0.0, 0.0)) } }
    }
    /// Gets the Y value of all units.
    pub fn get_y(&self) -> NodeSize<f32> {
        let mut out = NodeSize::<f32>::new();
        if let Some(v) = self.abs { out += Abs(v.y) }
        if let Some(v) = self.prc { out += Prc(v.y) }
        if let Some(v) = self.rem { out += Rem(v.y) }
        out
    }
    /// Replaces the Y value of appropriate units with the new value.
    pub fn with_y(mut self, other: impl Into<NodeSize<f32>>) -> Self {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.y = v2 } else { self.abs = Some(Vec4::new(0.0, v2, 0.0, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.y = v2 } else { self.prc = Some(Vec4::new(0.0, v2, 0.0, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.y = v2 } else { self.rem = Some(Vec4::new(0.0, v2, 0.0, 0.0)) } }
        self
    }
    /// Sets the Y value of appropriate units with the new value.
    pub fn set_y(&mut self, other: impl Into<NodeSize<f32>>) {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.y = v2 } else { self.abs = Some(Vec4::new(0.0, v2, 0.0, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.y = v2 } else { self.prc = Some(Vec4::new(0.0, v2, 0.0, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.y = v2 } else { self.rem = Some(Vec4::new(0.0, v2, 0.0, 0.0)) } }
    }
    /// Gets the Z value of all units.
    pub fn get_z(&self) -> NodeSize<f32> {
        let mut out = NodeSize::<f32>::new();
        if let Some(v) = self.abs { out += Abs(v.z) }
        if let Some(v) = self.prc { out += Prc(v.z) }
        if let Some(v) = self.rem { out += Rem(v.z) }
        out
    }
    /// Replaces the Z value of appropriate units with the new value.
    pub fn with_z(mut self, other: impl Into<NodeSize<f32>>) -> Self {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.z = v2 } else { self.abs = Some(Vec4::new(0.0, 0.0, v2, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.z = v2 } else { self.prc = Some(Vec4::new(0.0, 0.0, v2, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.z = v2 } else { self.rem = Some(Vec4::new(0.0, 0.0, v2, 0.0)) } }
        self
    }
    /// Sets the Z value of appropriate units with the new value.
    pub fn set_z(&mut self, other: impl Into<NodeSize<f32>>) {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.z = v2 } else { self.abs = Some(Vec4::new(0.0, 0.0, v2, 0.0)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.z = v2 } else { self.prc = Some(Vec4::new(0.0, 0.0, v2, 0.0)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.z = v2 } else { self.rem = Some(Vec4::new(0.0, 0.0, v2, 0.0)) } }
    }
    /// Gets the W value of all units.
    pub fn get_w(&self) -> NodeSize<f32> {
        let mut out = NodeSize::<f32>::new();
        if let Some(v) = self.abs { out += Abs(v.w) }
        if let Some(v) = self.prc { out += Prc(v.w) }
        if let Some(v) = self.rem { out += Rem(v.w) }
        out
    }
    /// Replaces the W value of appropriate units with the new value.
    pub fn with_w(mut self, other: impl Into<NodeSize<f32>>) -> Self {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.w = v2 } else { self.abs = Some(Vec4::new(0.0, 0.0, 0.0, v2)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.w = v2 } else { self.prc = Some(Vec4::new(0.0, 0.0, 0.0, v2)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.w = v2 } else { self.rem = Some(Vec4::new(0.0, 0.0, 0.0, v2)) } }
        self
    }
    /// Sets the W value of appropriate units with the new value.
    pub fn set_w(&mut self, other: impl Into<NodeSize<f32>>) {
        let other = other.into();
        if let Some(v2) = other.abs { if let Some(v1) = &mut self.abs { v1.w = v2 } else { self.abs = Some(Vec4::new(0.0, 0.0, 0.0, v2)) } }
        if let Some(v2) = other.prc { if let Some(v1) = &mut self.prc { v1.w = v2 } else { self.prc = Some(Vec4::new(0.0, 0.0, 0.0, v2)) } }
        if let Some(v2) = other.rem { if let Some(v1) = &mut self.rem { v1.w = v2 } else { self.rem = Some(Vec4::new(0.0, 0.0, 0.0, v2)) } }
    }
}



/// ## NodeSize Evaluate
/// Trait for implementing evaluation logic for `(TT)`.
/// `(T)` should be 1 vector unit version of `(TT)`.
/// ## 📦 Types
/// * `(f32, f32)` = `(TT, T)`
/// * `(Vec2, f32)` = `(TT, T)`
/// * `(Vec3, f32)` = `(TT, T)`
/// * `(Vec4, f32)` = `(TT, T)`
pub trait NodeSizeEvaluate<T, TT> {
    /// Evaluates the NodeSize for `(T)`
    fn evaluate(&self, abs_scale: TT, parent_size: T, font_size: TT) -> T;
    /// Evaluates the NodeSize abs only for `(T)`
    fn evaluate_abs(&self, abs_scale: TT) -> T;
    /// Evaluates the NodeSize prc only for `(T)`
    fn evaluate_prc(&self, parent_size: T) -> T;
    /// Evaluates the NodeSize rem only for `(T)`
    fn evaluate_rem(&self, parent_size: TT) -> T;
    /// Evaluates the NodeSize abs + rem only for `(T)`
    fn evaluate_abs_rem(&self, abs_scale: TT, font_size: TT) -> T;
}

// # Impl evaluate
impl NodeSizeEvaluate<f32, f32> for NodeSize<f32> {
    fn evaluate(&self, abs_scale: f32, parent_size: f32, font_size: f32) -> f32 {
        let mut out = 0.0;
        if let Some(v) = self.abs { out += v * abs_scale }
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
    fn evaluate_abs(&self, abs_scale: f32) -> f32 {
        let mut out = 0.0;
        if let Some(v) = self.abs { out += v * abs_scale }
        out
    }
    fn evaluate_prc(&self, parent_size: f32) -> f32 {
        let mut out = 0.0;
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        out
    }
    fn evaluate_rem(&self, font_size: f32) -> f32 {
        let mut out = 0.0;
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
    fn evaluate_abs_rem(&self, abs_scale: f32, font_size: f32) -> f32 {
        let mut out = 0.0;
        if let Some(v) = self.abs { out += v * abs_scale }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
}
impl NodeSizeEvaluate<Vec2, f32> for NodeSize<Vec2> {
    fn evaluate(&self, abs_scale: f32, parent_size: Vec2, font_size: f32) -> Vec2 {
        let mut out = Vec2::ZERO;
        if let Some(v) = self.abs { out += v * abs_scale }
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
    fn evaluate_abs(&self, abs_scale: f32) -> Vec2 {
        let mut out = Vec2::ZERO;
        if let Some(v) = self.abs { out += v * abs_scale }
        out
    }
    fn evaluate_prc(&self, parent_size: Vec2) -> Vec2 {
        let mut out = Vec2::ZERO;
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        out
    }
    fn evaluate_rem(&self, font_size: f32) -> Vec2 {
        let mut out = Vec2::ZERO;
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
    fn evaluate_abs_rem(&self, abs_scale: f32, font_size: f32) -> Vec2 {
        let mut out = Vec2::ZERO;
        if let Some(v) = self.abs { out += v * abs_scale }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
}
impl NodeSizeEvaluate<Vec3, f32> for NodeSize<Vec3> {
    fn evaluate(&self, abs_scale: f32, parent_size: Vec3, font_size: f32) -> Vec3 {
        let mut out = Vec3::ZERO;
        if let Some(v) = self.abs { out += v * abs_scale }
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
    fn evaluate_abs(&self, abs_scale: f32) -> Vec3 {
        let mut out = Vec3::ZERO;
        if let Some(v) = self.abs { out += v * abs_scale }
        out
    }
    fn evaluate_prc(&self, parent_size: Vec3) -> Vec3 {
        let mut out = Vec3::ZERO;
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        out
    }
    fn evaluate_rem(&self, font_size: f32) -> Vec3 {
        let mut out = Vec3::ZERO;
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
    fn evaluate_abs_rem(&self, abs_scale: f32, font_size: f32) -> Vec3 {
        let mut out = Vec3::ZERO;
        if let Some(v) = self.abs { out += v * abs_scale }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
}
impl NodeSizeEvaluate<Vec4, f32> for NodeSize<Vec4> {
    fn evaluate(&self, abs_scale: f32, parent_size: Vec4, font_size: f32) -> Vec4 {
        let mut out = Vec4::ZERO;
        if let Some(v) = self.abs { out += v * abs_scale }
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
    fn evaluate_abs(&self, abs_scale: f32) -> Vec4 {
        let mut out = Vec4::ZERO;
        if let Some(v) = self.abs { out += v * abs_scale }
        out
    }
    fn evaluate_prc(&self, parent_size: Vec4) -> Vec4 {
        let mut out = Vec4::ZERO;
        if let Some(v) = self.prc { out += (v/100.0) * parent_size }
        out
    }
    fn evaluate_rem(&self, font_size: f32) -> Vec4 {
        let mut out = Vec4::ZERO;
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
    fn evaluate_abs_rem(&self, abs_scale: f32, font_size: f32) -> Vec4 {
        let mut out = Vec4::ZERO;
        if let Some(v) = self.abs { out += v * abs_scale }
        if let Some(v) = self.rem { out += v * font_size }
        out
    }
}

// # Impl to_nicestr
impl NiceDisplay for NodeSize<f32> {
    fn to_nicestr(&self) -> String {
        let mut t = String::new();
        if let Some(v) = self.abs {
            if v != 0.0 {
                t = format!("{}", v.to_string().bright_blue());
            }
        }
        if let Some(v) = self.prc {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, v.to_string().bright_green(), "%".bright_green());
            }
        }
        if let Some(v) = self.rem {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, v.to_string().bright_red(), "m".bright_red());
            }
        }
        if t.is_empty() { t = format!("{}", "0".bright_blue()); };
        format!("{}", t.black())
    }
}
impl NiceDisplay for NodeSize<Vec2> {
    fn to_nicestr(&self) -> String {
        let mut tx = String::new();
        let mut ty = String::new();
        if let Some(v) = self.abs {
            if v.x != 0.0 {
                tx = format!("{}", v.x.to_string().bright_blue());
            }
            if v.y != 0.0 {
                ty = format!("{}", v.y.to_string().bright_blue());
            }
        }
        if let Some(v) = self.prc {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "%".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "%".bright_green());
            }
        }
        if let Some(v) = self.rem {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_red(), "m".bright_red());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_red(), "m".bright_red());
            }
        }
        if tx.is_empty() { tx = format!("{}", "0".bright_blue()); };
        if ty.is_empty() { ty = format!("{}", "0".bright_blue()); };
        format!("x: {}, y: {}", tx.black(), ty.black())
    }
}
impl NiceDisplay for NodeSize<Vec3> {
    fn to_nicestr(&self) -> String {
        let mut tx = String::new();
        let mut ty = String::new();
        let mut tz = String::new();
        if let Some(v) = self.abs {
            if v.x != 0.0 {
                tx = format!("{}", v.x.to_string().bright_blue());
            }
            if v.y != 0.0 {
                ty = format!("{}", v.y.to_string().bright_blue());
            }
            if v.z != 0.0 {
                tz = format!("{}", v.z.to_string().bright_blue());
            }
        }
        if let Some(v) = self.prc {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "%".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "%".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_green(), "%".bright_green());
            }
        }
        if let Some(v) = self.rem {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_red(), "m".bright_red());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_red(), "m".bright_red());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_red(), "m".bright_red());
            }
        }
        if tx.is_empty() { tx = format!("{}", "0".bright_blue()); };
        if ty.is_empty() { ty = format!("{}", "0".bright_blue()); };
        if tz.is_empty() { tz = format!("{}", "0".bright_blue()); };
        format!("x: {}, y: {} z:{}", tx.black(), ty.black(), tz.black())
    }
}
impl NiceDisplay for NodeSize<Vec4> {
    fn to_nicestr(&self) -> String {
        let mut tx = String::new();
        let mut ty = String::new();
        let mut tz = String::new();
        let mut tw = String::new();
        if let Some(v) = self.abs {
            if v.x != 0.0 {
                tx = format!("{}", v.x.to_string().bright_blue());
            }
            if v.y != 0.0 {
                ty = format!("{}", v.y.to_string().bright_blue());
            }
            if v.z != 0.0 {
                tz = format!("{}", v.z.to_string().bright_blue());
            }
            if v.w != 0.0 {
                tw = format!("{}", v.w.to_string().bright_blue());
            }
        }
        if let Some(v) = self.prc {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "%".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "%".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_green(), "%".bright_green());
            }
            if v.w != 0.0 {
                if !tw.is_empty() { tw += " + " };
                tw = format!("{}{}{}", tw, v.w.to_string().bright_green(), "%".bright_green());
            }
        }
        if let Some(v) = self.rem {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_red(), "m".bright_red());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_red(), "m".bright_red());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_red(), "m".bright_red());
            }
            if v.w != 0.0 {
                if !tw.is_empty() { tw += " + " };
                tw = format!("{}{}{}", tw, v.w.to_string().bright_red(), "m".bright_red());
            }
        }
        if tx.is_empty() { tx = format!("{}", "0".bright_blue()); };
        if ty.is_empty() { ty = format!("{}", "0".bright_blue()); };
        if tz.is_empty() { tz = format!("{}", "0".bright_blue()); };
        if tw.is_empty() { tw = format!("{}", "0".bright_blue()); };
        format!("x: {}, y: {} z:{} w:{}", tx.black(), ty.black(), tz.black(), tw.black())
    }
}

// #================================#
// #=== CONSTANT IMPLEMENTATIONS ===#

// # Impl Constructors
impl <T> NodeSize<T> {
    /// Creates new empty NodeSize
    pub const fn new() -> Self {
        NodeSize {
            abs: None,
            prc: None,
            rem: None,
        }
    }
    /// Creates new empty NodeSize from absolute
    pub const fn from_abs(abs: T) -> NodeSize<T> {
        NodeSize {
            abs: Some(abs),
            prc: None,
            rem: None,
        }
    }
    /// Creates new empty NodeSize from percentage
    pub const fn from_prc(prc: T) -> NodeSize<T> {
        NodeSize {
            abs: None,
            prc: Some(prc),
            rem: None,
        }
    }
    /// Creates new empty NodeSize from rem
    pub const fn from_rem(rem: T) -> NodeSize<T> {
        NodeSize {
            abs: None,
            prc: None,
            rem: Some(rem),
        }
    }
    /// Creates new empty NodeSize from absolute & percentage
    pub const fn from_abs_prc(abs: T, prc: T) -> NodeSize<T> {
        NodeSize {
            abs: Some(abs),
            prc: Some(prc),
            rem: None,
        }
    }
    /// Creates new empty NodeSize from absolute & rem
    pub const fn from_abs_rem(abs: T, rem: T) -> NodeSize<T> {
        NodeSize {
            abs: Some(abs),
            prc: None,
            rem: Some(rem),
        }
    }
    /// Creates new empty NodeSize from percentage & rem
    pub const fn from_prc_rem(prc: T, rem: T) -> NodeSize<T> {
        NodeSize {
            abs: None,
            prc: Some(prc),
            rem: Some(rem),
        }
    }
    /// Creates new empty NodeSize from absolute & percentage & rem
    pub const fn from_abs_prc_rem(abs: T, prc: T, rem: T) -> NodeSize<T> {
        NodeSize {
            abs: Some(abs),
            prc: Some(prc),
            rem: Some(rem),
        }
    }
}

// # Impl CONSTS
impl Abs<f32> {
    /// ## Zero
    pub const ZERO: Abs<f32> = Abs(0.0);
    /// ## One
    pub const ONE: Abs<f32> = Abs(1.0);
    /// ## Extra-small
    pub const XS: Abs<f32> = Abs(1.0 * 16.0);
    /// ## Small
    pub const SM: Abs<f32> = Abs(2.0 * 16.0);
    /// ## Medium
    pub const MD: Abs<f32> = Abs(3.0 * 16.0);
    /// ## Large
    pub const LG: Abs<f32> = Abs(4.0 * 16.0);
    /// ## Extra-large
    pub const XL: Abs<f32> = Abs(6.0 * 16.0);
    /// ## Extra-large 2
    pub const XL2: Abs<f32> = Abs(8.0 * 16.0);
    /// ## Extra-large 3
    pub const XL3: Abs<f32> = Abs(10.0 * 16.0);
    /// ## Extra-large 4
    pub const XL4: Abs<f32> = Abs(12.0 * 16.0);
    /// ## Extra-large 5
    pub const XL5: Abs<f32> = Abs(14.0 * 16.0);
    /// ## Extra-large 6
    pub const XL6: Abs<f32> = Abs(16.0 * 16.0);
    /// ## Extra-large 7
    pub const XL7: Abs<f32> = Abs(18.0 * 16.0);
}
impl Prc<f32> {
    /// ## Zero
    pub const ZERO: Prc<f32> = Prc(0.0);
    /// ## One
    pub const ONE: Prc<f32> = Prc(1.0);
    /// ## Full
    pub const FULL: Prc<f32> = Prc(100.0);
    /// ## Half
    pub const HALF: Prc<f32> = Prc(100.0 / 2.0);
    /// ## Third
    pub const THIRD: Prc<f32> = Prc(100.0 / 3.0);
    /// ## Quarter
    pub const QUARTER: Prc<f32> = Prc(100.0 / 4.0);
    /// ## Fifth
    pub const FIFTH: Prc<f32> = Prc(100.0 / 5.0);
    /// ## Sixth
    pub const SIXTH: Prc<f32> = Prc(100.0 / 6.0);
    /// ## Seventh
    pub const SEVENTH: Prc<f32> = Prc(100.0 / 7.0);
    /// ## Eighth
    pub const EIGHTH: Prc<f32> = Prc(100.0 / 8.0);
    /// ## Ninth
    pub const NINTH: Prc<f32> = Prc(100.0 / 9.0);
    /// ## Tenth
    pub const TENTH: Prc<f32> = Prc(100.0 / 10.0);
    /// ## Eleventh
    pub const ELEVENTH: Prc<f32> = Prc(100.0 / 11.0);
    /// ## Twelfth
    pub const TWELFTH: Prc<f32> = Prc(100.0 / 12.0);
    /// ## Thirteenth
    pub const THIRTEENTH: Prc<f32> = Prc(100.0 / 13.0);
    /// ## Fourteenth
    pub const FOURTEENTH: Prc<f32> = Prc(100.0 / 14.0);
    /// ## Fifteenth
    pub const FIFTEENTH: Prc<f32> = Prc(100.0 / 15.0);
    /// ## Sixteenth
    pub const SIXTEENTH: Prc<f32> = Prc(100.0 / 16.0);
    /// ## Seventeenth
    pub const SEVENTEENTH: Prc<f32> = Prc(100.0 / 17.0);
    /// ## Eighteenth
    pub const EIGHTEENTH: Prc<f32> = Prc(100.0 / 18.0);
    /// ## Nineteenth
    pub const NINETEENTH: Prc<f32> = Prc(100.0 / 19.0);
    /// ## Twentieth
    pub const TWENTIETH: Prc<f32> = Prc(100.0 / 20.0);
}
impl Rem<f32> {
    /// ## Zero
    pub const ZERO: Rem<f32> = Rem(0.0);
    /// ## One
    pub const ONE: Rem<f32> = Rem(1.0);
    /// ## Extra-small
    pub const XS: Rem<f32> = Rem(1.0);
    /// ## Small
    pub const SM: Rem<f32> = Rem(2.0);
    /// ## Medium
    pub const MD: Rem<f32> = Rem(3.0);
    /// ## Large
    pub const LG: Rem<f32> = Rem(4.0);
    /// ## Extra-large
    pub const XL: Rem<f32> = Rem(6.0);
    /// ## Extra-large 2
    pub const XL2: Rem<f32> = Rem(8.0);
    /// ## Extra-large 3
    pub const XL3: Rem<f32> = Rem(10.0);
    /// ## Extra-large 4
    pub const XL4: Rem<f32> = Rem(12.0);
    /// ## Extra-large 5
    pub const XL5: Rem<f32> = Rem(14.0);
    /// ## Extra-large 6
    pub const XL6: Rem<f32> = Rem(16.0);
    /// ## Extra-large 7
    pub const XL7: Rem<f32> = Rem(18.0);
}

// # Impl CONSTS VEC2
impl Abs<Vec2> {
    /// ## Zero
    pub const ZERO_VEC2: Abs<Vec2> = Abs(Vec2::splat(0.0));
    /// ## One
    pub const ONE_VEC2: Abs<Vec2> = Abs(Vec2::splat(1.0));
    /// ## Extra-small
    pub const XS_VEC2: Abs<Vec2> = Abs(Vec2::splat(1.0 * 16.0));
    /// ## Small
    pub const SM_VEC2: Abs<Vec2> = Abs(Vec2::splat(2.0 * 16.0));
    /// ## Medium
    pub const MD_VEC2: Abs<Vec2> = Abs(Vec2::splat(3.0 * 16.0));
    /// ## Large
    pub const LG_VEC2: Abs<Vec2> = Abs(Vec2::splat(4.0 * 16.0));
    /// ## Extra-large
    pub const XL_VEC2: Abs<Vec2> = Abs(Vec2::splat(6.0 * 16.0));
    /// ## Extra-large 2
    pub const XL2_VEC2: Abs<Vec2> = Abs(Vec2::splat(8.0 * 16.0));
    /// ## Extra-large 3
    pub const XL3_VEC2: Abs<Vec2> = Abs(Vec2::splat(10.0 * 16.0));
    /// ## Extra-large 4
    pub const XL4_VEC2: Abs<Vec2> = Abs(Vec2::splat(12.0 * 16.0));
    /// ## Extra-large 5
    pub const XL5_VEC2: Abs<Vec2> = Abs(Vec2::splat(14.0 * 16.0));
    /// ## Extra-large 6
    pub const XL6_VEC2: Abs<Vec2> = Abs(Vec2::splat(16.0 * 16.0));
    /// ## Extra-large 7
    pub const XL7_VEC2: Abs<Vec2> = Abs(Vec2::splat(18.0 * 16.0));
}
impl Prc<Vec2> {
    /// ## Zero
    pub const ZERO_VEC2: Prc<Vec2> = Prc(Vec2::splat(0.0));
    /// ## One
    pub const ONE_VEC2: Prc<Vec2> = Prc(Vec2::splat(1.0));
    /// ## Full
    pub const FULL_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0));
    /// ## Half
    pub const HALF_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 2.0));
    /// ## Third
    pub const THIRD_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 3.0));
    /// ## Quarter
    pub const QUARTER_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 4.0));
    /// ## Fifth
    pub const FIFTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 5.0));
    /// ## Sixth
    pub const SIXTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 6.0));
    /// ## Seventh
    pub const SEVENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 7.0));
    /// ## Eighth
    pub const EIGHTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 8.0));
    /// ## Ninth
    pub const NINTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 9.0));
    /// ## Tenth
    pub const TENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 10.0));
    /// ## Eleventh
    pub const ELEVENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 11.0));
    /// ## Twelfth
    pub const TWELFTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 12.0));
    /// ## Thirteenth
    pub const THIRTEENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 13.0));
    /// ## Fourteenth
    pub const FOURTEENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 14.0));
    /// ## Fifteenth
    pub const FIFTEENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 15.0));
    /// ## Sixteenth
    pub const SIXTEENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 16.0));
    /// ## Seventeenth
    pub const SEVENTEENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 17.0));
    /// ## Eighteenth
    pub const EIGHTEENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 18.0));
    /// ## Nineteenth
    pub const NINETEENTH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 19.0));
    /// ## Twentieth
    pub const TWENTIETH_VEC2: Prc<Vec2> = Prc(Vec2::splat(100.0 / 20.0));
}
impl Rem<Vec2> {
    /// ## Zero
    pub const ZERO_VEC2: Rem<Vec2> = Rem(Vec2::splat(0.0));
    /// ## One
    pub const ONE_VEC2: Rem<Vec2> = Rem(Vec2::splat(1.0));
    /// ## Extra-small
    pub const XS_VEC2: Rem<Vec2> = Rem(Vec2::splat(1.0));
    /// ## Small
    pub const SM_VEC2: Rem<Vec2> = Rem(Vec2::splat(2.0));
    /// ## Medium
    pub const MD_VEC2: Rem<Vec2> = Rem(Vec2::splat(3.0));
    /// ## Large
    pub const LG_VEC2: Rem<Vec2> = Rem(Vec2::splat(4.0));
    /// ## Extra-large
    pub const XL_VEC2: Rem<Vec2> = Rem(Vec2::splat(6.0));
    /// ## Extra-large 2
    pub const XL2_VEC2: Rem<Vec2> = Rem(Vec2::splat(8.0));
    /// ## Extra-large 3
    pub const XL3_VEC2: Rem<Vec2> = Rem(Vec2::splat(10.0));
    /// ## Extra-large 4
    pub const XL4_VEC2: Rem<Vec2> = Rem(Vec2::splat(12.0));
    /// ## Extra-large 5
    pub const XL5_VEC2: Rem<Vec2> = Rem(Vec2::splat(14.0));
    /// ## Extra-large 6
    pub const XL6_VEC2: Rem<Vec2> = Rem(Vec2::splat(16.0));
    /// ## Extra-large 7
    pub const XL7_VEC2: Rem<Vec2> = Rem(Vec2::splat(18.0));
}

// # Impl CONSTS VEC3
impl Abs<Vec3> {
    /// ## Zero
    pub const ZERO_VEC3: Abs<Vec3> = Abs(Vec3::splat(0.0));
    /// ## One
    pub const ONE_VEC3: Abs<Vec3> = Abs(Vec3::splat(1.0));
    /// ## Extra-small
    pub const XS_VEC3: Abs<Vec3> = Abs(Vec3::splat(1.0 * 16.0));
    /// ## Small
    pub const SM_VEC3: Abs<Vec3> = Abs(Vec3::splat(2.0 * 16.0));
    /// ## Medium
    pub const MD_VEC3: Abs<Vec3> = Abs(Vec3::splat(3.0 * 16.0));
    /// ## Large
    pub const LG_VEC3: Abs<Vec3> = Abs(Vec3::splat(4.0 * 16.0));
    /// ## Extra-large
    pub const XL_VEC3: Abs<Vec3> = Abs(Vec3::splat(6.0 * 16.0));
    /// ## Extra-large 2
    pub const XL2_VEC3: Abs<Vec3> = Abs(Vec3::splat(8.0 * 16.0));
    /// ## Extra-large 3
    pub const XL3_VEC3: Abs<Vec3> = Abs(Vec3::splat(10.0 * 16.0));
    /// ## Extra-large 4
    pub const XL4_VEC3: Abs<Vec3> = Abs(Vec3::splat(12.0 * 16.0));
    /// ## Extra-large 5
    pub const XL5_VEC3: Abs<Vec3> = Abs(Vec3::splat(14.0 * 16.0));
    /// ## Extra-large 6
    pub const XL6_VEC3: Abs<Vec3> = Abs(Vec3::splat(16.0 * 16.0));
    /// ## Extra-large 7
    pub const XL7_VEC3: Abs<Vec3> = Abs(Vec3::splat(18.0 * 16.0));
}
impl Prc<Vec3> {
    /// ## Zero
    pub const ZERO_VEC3: Prc<Vec3> = Prc(Vec3::splat(0.0));
    /// ## One
    pub const ONE_VEC3: Prc<Vec3> = Prc(Vec3::splat(1.0));
    /// ## Full
    pub const FULL_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0));
    /// ## Half
    pub const HALF_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 2.0));
    /// ## Third
    pub const THIRD_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 3.0));
    /// ## Quarter
    pub const QUARTER_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 4.0));
    /// ## Fifth
    pub const FIFTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 5.0));
    /// ## Sixth
    pub const SIXTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 6.0));
    /// ## Seventh
    pub const SEVENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 7.0));
    /// ## Eighth
    pub const EIGHTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 8.0));
    /// ## Ninth
    pub const NINTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 9.0));
    /// ## Tenth
    pub const TENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 10.0));
    /// ## Eleventh
    pub const ELEVENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 11.0));
    /// ## Twelfth
    pub const TWELFTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 12.0));
    /// ## Thirteenth
    pub const THIRTEENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 13.0));
    /// ## Fourteenth
    pub const FOURTEENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 14.0));
    /// ## Fifteenth
    pub const FIFTEENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 15.0));
    /// ## Sixteenth
    pub const SIXTEENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 16.0));
    /// ## Seventeenth
    pub const SEVENTEENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 17.0));
    /// ## Eighteenth
    pub const EIGHTEENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 18.0));
    /// ## Nineteenth
    pub const NINETEENTH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 19.0));
    /// ## Twentieth
    pub const TWENTIETH_VEC3: Prc<Vec3> = Prc(Vec3::splat(100.0 / 20.0));
}
impl Rem<Vec3> {
    /// ## Zero
    pub const ZERO_VEC3: Rem<Vec3> = Rem(Vec3::splat(0.0));
    /// ## One
    pub const ONE_VEC3: Rem<Vec3> = Rem(Vec3::splat(1.0));
    /// ## Extra-small
    pub const XS_VEC3: Rem<Vec3> = Rem(Vec3::splat(1.0));
    /// ## Small
    pub const SM_VEC3: Rem<Vec3> = Rem(Vec3::splat(2.0));
    /// ## Medium
    pub const MD_VEC3: Rem<Vec3> = Rem(Vec3::splat(3.0));
    /// ## Large
    pub const LG_VEC3: Rem<Vec3> = Rem(Vec3::splat(4.0));
    /// ## Extra-large
    pub const XL_VEC3: Rem<Vec3> = Rem(Vec3::splat(6.0));
    /// ## Extra-large 2
    pub const XL2_VEC3: Rem<Vec3> = Rem(Vec3::splat(8.0));
    /// ## Extra-large 3
    pub const XL3_VEC3: Rem<Vec3> = Rem(Vec3::splat(10.0));
    /// ## Extra-large 4
    pub const XL4_VEC3: Rem<Vec3> = Rem(Vec3::splat(12.0));
    /// ## Extra-large 5
    pub const XL5_VEC3: Rem<Vec3> = Rem(Vec3::splat(14.0));
    /// ## Extra-large 6
    pub const XL6_VEC3: Rem<Vec3> = Rem(Vec3::splat(16.0));
    /// ## Extra-large 7
    pub const XL7_VEC3: Rem<Vec3> = Rem(Vec3::splat(18.0));
}

// # Impl CONSTS VEC4
impl Abs<Vec4> {
    /// ## Zero
    pub const ZERO_VEC4: Abs<Vec4> = Abs(Vec4::splat(0.0));
    /// ## One
    pub const ONE_VEC4: Abs<Vec4> = Abs(Vec4::splat(1.0));
    /// ## Extra-small
    pub const XS_VEC4: Abs<Vec4> = Abs(Vec4::splat(1.0 * 16.0));
    /// ## Small
    pub const SM_VEC4: Abs<Vec4> = Abs(Vec4::splat(2.0 * 16.0));
    /// ## Medium
    pub const MD_VEC4: Abs<Vec4> = Abs(Vec4::splat(3.0 * 16.0));
    /// ## Large
    pub const LG_VEC4: Abs<Vec4> = Abs(Vec4::splat(4.0 * 16.0));
    /// ## Extra-large
    pub const XL_VEC4: Abs<Vec4> = Abs(Vec4::splat(6.0 * 16.0));
    /// ## Extra-large 2
    pub const XL2_VEC4: Abs<Vec4> = Abs(Vec4::splat(8.0 * 16.0));
    /// ## Extra-large 3
    pub const XL3_VEC4: Abs<Vec4> = Abs(Vec4::splat(10.0 * 16.0));
    /// ## Extra-large 4
    pub const XL4_VEC4: Abs<Vec4> = Abs(Vec4::splat(12.0 * 16.0));
    /// ## Extra-large 5
    pub const XL5_VEC4: Abs<Vec4> = Abs(Vec4::splat(14.0 * 16.0));
    /// ## Extra-large 6
    pub const XL6_VEC4: Abs<Vec4> = Abs(Vec4::splat(16.0 * 16.0));
    /// ## Extra-large 7
    pub const XL7_VEC4: Abs<Vec4> = Abs(Vec4::splat(18.0 * 16.0));
}
impl Prc<Vec4> {
    /// ## Zero
    pub const ZERO_VEC4: Prc<Vec4> = Prc(Vec4::splat(0.0));
    /// ## One
    pub const ONE_VEC4: Prc<Vec4> = Prc(Vec4::splat(1.0));
    /// ## Full
    pub const FULL_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0));
    /// ## Half
    pub const HALF_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 2.0));
    /// ## Third
    pub const THIRD_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 3.0));
    /// ## Quarter
    pub const QUARTER_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 4.0));
    /// ## Fifth
    pub const FIFTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 5.0));
    /// ## Sixth
    pub const SIXTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 6.0));
    /// ## Seventh
    pub const SEVENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 7.0));
    /// ## Eighth
    pub const EIGHTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 8.0));
    /// ## Ninth
    pub const NINTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 9.0));
    /// ## Tenth
    pub const TENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 10.0));
    /// ## Eleventh
    pub const ELEVENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 11.0));
    /// ## Twelfth
    pub const TWELFTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 12.0));
    /// ## Thirteenth
    pub const THIRTEENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 13.0));
    /// ## Fourteenth
    pub const FOURTEENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 14.0));
    /// ## Fifteenth
    pub const FIFTEENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 15.0));
    /// ## Sixteenth
    pub const SIXTEENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 16.0));
    /// ## Seventeenth
    pub const SEVENTEENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 17.0));
    /// ## Eighteenth
    pub const EIGHTEENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 18.0));
    /// ## Nineteenth
    pub const NINETEENTH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 19.0));
    /// ## Twentieth
    pub const TWENTIETH_VEC4: Prc<Vec4> = Prc(Vec4::splat(100.0 / 20.0));
}
impl Rem<Vec4> {
    /// ## Zero
    pub const ZERO_VEC4: Rem<Vec4> = Rem(Vec4::splat(0.0));
    /// ## One
    pub const ONE_VEC4: Rem<Vec4> = Rem(Vec4::splat(1.0));
    /// ## Extra-small
    pub const XS_VEC4: Rem<Vec4> = Rem(Vec4::splat(1.0));
    /// ## Small
    pub const SM_VEC4: Rem<Vec4> = Rem(Vec4::splat(2.0));
    /// ## Medium
    pub const MD_VEC4: Rem<Vec4> = Rem(Vec4::splat(3.0));
    /// ## Large
    pub const LG_VEC4: Rem<Vec4> = Rem(Vec4::splat(4.0));
    /// ## Extra-large
    pub const XL_VEC4: Rem<Vec4> = Rem(Vec4::splat(6.0));
    /// ## Extra-large 2
    pub const XL2_VEC4: Rem<Vec4> = Rem(Vec4::splat(8.0));
    /// ## Extra-large 3
    pub const XL3_VEC4: Rem<Vec4> = Rem(Vec4::splat(10.0));
    /// ## Extra-large 4
    pub const XL4_VEC4: Rem<Vec4> = Rem(Vec4::splat(12.0));
    /// ## Extra-large 5
    pub const XL5_VEC4: Rem<Vec4> = Rem(Vec4::splat(14.0));
    /// ## Extra-large 6
    pub const XL6_VEC4: Rem<Vec4> = Rem(Vec4::splat(16.0));
    /// ## Extra-large 7
    pub const XL7_VEC4: Rem<Vec4> = Rem(Vec4::splat(18.0));
}
