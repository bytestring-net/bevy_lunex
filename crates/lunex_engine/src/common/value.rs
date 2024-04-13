use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;

use crate::import::*;

use super::NiceDisplay;


// #==========================#
// #=== DECLARATIVE MACROS ===#

macro_rules! uivalue_declare {
    ($($ufield:ident), *) => {
        /// **Ui value** - Represents collection of different units.
        /// They are computed at runtime when layout computation is happening.
        /// The supported units are:
        /// * [`Ab`] [`Rl`] [`Rw`] [`Rh`] [`Em`] [`Sp`]
        /// ## 📦 Types
        /// First class implementations for `(T)` are:
        /// * [`f32`] [`Vec2`] [`Vec3`] [`Vec4`]
        /// ## 🛠️ Example
        /// ```
        /// # use lunex_core::{UiValue, Ab, Em, Rl, Sp};
        /// let a: UiValue<f32> = Ab(4.0) + Em(1.0);  // -> 4px + 1em
        /// let b: UiValue<f32> = Ab(40.0) - Rl(5.0); // -> 40px - 5%
        /// let c: UiValue<f32> = Sp(5.0).into();     // -> 5 space
        /// ```
        #[derive(Debug, Default, Clone, Copy, PartialEq)]
        pub struct UiValue<T> {
            $(
                $ufield: Option<T>,
            )*
        }
        impl <T> UiValue<T> {
            /// Creates new empty [`UiValue`]
            pub const fn new() -> Self {
                UiValue {
                    $(
                        $ufield: None,
                    )*
                }
            }
        }
        impl <T: Add<Output = T> + Add> Add for UiValue<T> {
            type Output = Self;
            fn add(self, other: Self) -> Self::Output {
                UiValue {
                    $(
                        $ufield: if let Some(v1) = self.$ufield {
                            if let Some(v2) = other.$ufield { Some(v1 + v2) } else { Some(v1) }
                        } else { other.$ufield },
                    )*
                }
            }
        }
        impl <T: Add<Output = T> + Copy> AddAssign for UiValue<T> {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs
            }
        }
        impl <T: Neg<Output = T>> Neg for UiValue<T> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                UiValue {
                    $(
                        $ufield: if let Some(v) = self.$ufield { Some(-v) } else { None },
                    )*
                }
            }
        }
        impl <T: Sub<Output = T> + Sub + Neg<Output = T>> Sub for UiValue<T> {
            type Output = Self;
            fn sub(self, other: Self) -> Self::Output {
                UiValue {
                    $(
                        $ufield: if let Some(v1) = self.$ufield {
                            if let Some(v2) = other.$ufield { Some(v1 - v2) } else { Some(v1) }
                        } else { other.$ufield },
                    )*
                }
            }
        }
        impl <T: Sub<Output = T> + Copy + Neg<Output = T>> SubAssign for UiValue<T> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs
            }
        }
        impl <T: Mul<Output = T> + Mul> Mul for UiValue<T> {
            type Output = Self;
            fn mul(self, other: Self) -> Self::Output {
                let mut output = UiValue::new();
                $(
                    if let Some(v1) = self.$ufield {
                        if let Some(v2) = other.$ufield {
                            output.$ufield = Some(v1 * v2);
                        }
                    }
                )*
                output
            }
        }
        impl <T: Mul<Output = T> + Copy> MulAssign for UiValue<T> {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs
            }
        }    
    }
}
macro_rules! uivalue_implement {
    ($( ($unit:ident, $ufield:ident) ),* ) => {

        $(
            impl <T> Into<UiValue<T>> for $unit<T> {
                fn into(self) -> UiValue<T> {
                    let mut ret = UiValue::new();
                    ret.$ufield = Some(self.0);
                    ret
                }
            }
            impl <T: Add<Output = T> + Add> Add<$unit<T>> for UiValue<T> {
                type Output = Self;
                fn add(mut self, other: $unit<T>) -> Self::Output {
                    match self.$ufield {
                        Some(v) => {
                            self.$ufield = Some(v + other.0);
                            self
                        },
                        None => {
                            self.$ufield = Some(other.0);
                            self
                        },
                    }
                }
            }
            impl <T: Add<Output = T> + Copy> AddAssign<$unit<T>> for UiValue<T> {
                fn add_assign(&mut self, rhs: $unit<T>) {
                    match self.$ufield {
                        Some(v) => self.$ufield = Some(v + rhs.0),
                        None => self.$ufield = Some(rhs.0),
                    }
                }
            }
            impl <T: Sub<Output = T> + Sub> Sub<$unit<T>> for UiValue<T> {
                type Output = Self;
                fn sub(mut self, other: $unit<T>) -> Self::Output {
                    match self.$ufield {
                        Some(v) => {
                            self.$ufield = Some(v - other.0);
                            self
                        },
                        None => {
                            self.$ufield = Some(other.0);
                            self
                        },
                    }
                }
            }
            impl <T: Sub<Output = T> + Copy> SubAssign<$unit<T>> for UiValue<T> {
                fn sub_assign(&mut self, rhs: $unit<T>) {
                    match self.$ufield {
                        Some(v) => self.$ufield = Some(v - rhs.0),
                        None => self.$ufield = Some(rhs.0),
                    }
                }
            }
            impl <T: Mul<Output = T> + Mul> Mul<$unit<T>> for UiValue<T> {
                type Output = Self;
                fn mul(mut self, other: $unit<T>) -> Self::Output {
                    if let Some(v) = self.$ufield {
                        self.$ufield = Some(v * other.0);
                    }
                    self
                }
            }
            impl <T: Mul<Output = T> + Copy> MulAssign<$unit<T>> for UiValue<T> {
                fn mul_assign(&mut self, rhs: $unit<T>) {
                    if let Some(v) = self.$ufield {
                        self.$ufield = Some(v * rhs.0);
                    }
                }
            }
        )*

        impl UiValue<Vec2> {
            /// Gets the X value of all units.
            pub fn get_x(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$ufield { out += $unit(v.x) }
                )*
                out
            }
            /// Gets the Y value of all units.
            pub fn get_y(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$ufield { out += $unit(v.y) }
                )*
                out
            }

            /// Replaces the X value of appropriate units with the new value.
            pub fn with_x(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$ufield { if let Some(v1) = &mut self.$ufield { v1.x = v2 } else { self.$ufield = Some(Vec2::new(v2, 0.0)) } }
                )*
                self
            }
            /// Replaces the Y value of appropriate units with the new value.
            pub fn with_y(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$ufield { if let Some(v1) = &mut self.$ufield { v1.y = v2 } else { self.$ufield = Some(Vec2::new(0.0, v2)) } }
                )*
                self
            }

            /// Sets the X value of appropriate units with the new value.
            pub fn set_x(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$ufield { if let Some(v1) = &mut self.$ufield { v1.x = v2 } else { self.$ufield = Some(Vec2::new(v2, 0.0)) } }
                )*
            }
            /// Sets the Y value of appropriate units with the new value.
            pub fn set_y(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$ufield { if let Some(v1) = &mut self.$ufield { v1.y = v2 } else { self.$ufield = Some(Vec2::new(0.0, v2)) } }
                )*
            }

        }
        impl UiValue<Vec3> {
            /// Gets the X value of all units.
            pub fn get_x(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$ufield { out += $unit(v.x) }
                )*
                out
            }
            /// Gets the Y value of all units.
            pub fn get_y(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$ufield { out += $unit(v.y) }
                )*
                out
            }
            /// Gets the Z value of all units.
            pub fn get_z(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$ufield { out += $unit(v.z) }
                )*
                out
            }

            /// Replaces the X value of appropriate units with the new value.
            pub fn with_x(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$ufield { if let Some(v1) = &mut self.$ufield { v1.x = v2 } else { self.$ufield = Some(Vec3::new(v2, 0.0, 0.0)) } }
                )*
                self
            }
            /// Replaces the Y value of appropriate units with the new value.
            pub fn with_y(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$ufield { if let Some(v1) = &mut self.$ufield { v1.y = v2 } else { self.$ufield = Some(Vec3::new(0.0, v2, 0.0)) } }
                )*
                self
            }
            /// Replaces the Z value of appropriate units with the new value.
            pub fn with_z(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$ufield { if let Some(v1) = &mut self.$ufield { v1.z = v2 } else { self.$ufield = Some(Vec3::new(0.0, 0.0, v2)) } }
                )*
                self
            }

            /// Sets the X value of appropriate units with the new value.
            pub fn set_x(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$ufield { if let Some(v1) = &mut self.$ufield { v1.x = v2 } else { self.$ufield = Some(Vec3::new(v2, 0.0, 0.0)) } }
                )*
            }
            /// Sets the Y value of appropriate units with the new value.
            pub fn set_y(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$ufield { if let Some(v1) = &mut self.$ufield { v1.y = v2 } else { self.$ufield = Some(Vec3::new(0.0, v2, 0.0)) } }
                )*
            }
            /// Sets the Z value of appropriate units with the new value.
            pub fn set_z(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$ufield { if let Some(v1) = &mut self.$ufield { v1.z = v2 } else { self.$ufield = Some(Vec3::new(0.0, 0.0, v2)) } }
                )*
            }
        }
        impl UiValue<Vec4> {
            /// Gets the X value of all units.
            pub fn get_x(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$ufield { out += $unit(v.x) }
                )*
                out
            }
            /// Gets the Y value of all units.
            pub fn get_y(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$ufield { out += $unit(v.y) }
                )*
                out
            }
            /// Gets the Z value of all units.
            pub fn get_z(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$ufield { out += $unit(v.z) }
                )*
                out
            }
            /// Gets the W value of all units.
            pub fn get_w(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$ufield { out += $unit(v.w) }
                )*
                out
            }

            /// Replaces the X value of appropriate units with the new value.
            pub fn with_x(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$ufield { if let Some(v1) = &mut self.$ufield { v1.x = v2 } else { self.$ufield = Some(Vec4::new(v2, 0.0, 0.0, 0.0)) } }
                )*
                self
            }
            /// Replaces the Y value of appropriate units with the new value.
            pub fn with_y(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$ufield { if let Some(v1) = &mut self.$ufield { v1.y = v2 } else { self.$ufield = Some(Vec4::new(0.0, v2, 0.0, 0.0)) } }
                )*
                self
            }
            /// Replaces the Z value of appropriate units with the new value.
            pub fn with_z(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$ufield { if let Some(v1) = &mut self.$ufield { v1.z = v2 } else { self.$ufield = Some(Vec4::new(0.0, 0.0, v2, 0.0)) } }
                )*
                self
            }
            /// Replaces the W value of appropriate units with the new value.
            pub fn with_w(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$ufield { if let Some(v1) = &mut self.$ufield { v1.w = v2 } else { self.$ufield = Some(Vec4::new(0.0, 0.0, 0.0, v2)) } }
                )*
                self
            }
            
            /// Sets the X value of appropriate units with the new value.
            pub fn set_x(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$ufield { if let Some(v1) = &mut self.$ufield { v1.x = v2 } else { self.$ufield = Some(Vec4::new(v2, 0.0, 0.0, 0.0)) } }
                )*
            }
            /// Sets the Y value of appropriate units with the new value.
            pub fn set_y(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$ufield { if let Some(v1) = &mut self.$ufield { v1.y = v2 } else { self.$ufield = Some(Vec4::new(0.0, v2, 0.0, 0.0)) } }
                )*
            }
            /// Sets the Z value of appropriate units with the new value.
            pub fn set_z(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$ufield { if let Some(v1) = &mut self.$ufield { v1.z = v2 } else { self.$ufield = Some(Vec4::new(0.0, 0.0, v2, 0.0)) } }
                )*
            }
            /// Sets the W value of appropriate units with the new value.
            pub fn set_w(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$ufield { if let Some(v1) = &mut self.$ufield { v1.w = v2 } else { self.$ufield = Some(Vec4::new(0.0, 0.0, 0.0, v2)) } }
                )*
            }
        }
    }
}
macro_rules! unit_implement {
    ($($unit:ident), *) => {
        $(
            impl <T: Add<Output = T>> Add for $unit<T> {
                type Output = Self;
                fn add(self, other: Self) -> Self::Output {
                    $unit(self.0 + other.0)
                }
            }
            impl <T: AddAssign<T>> AddAssign for $unit<T> {
                fn add_assign(&mut self, rhs: Self) {
                    self.0 += rhs.0
                }
            }
            impl <T: Sub<Output = T>> Sub for $unit<T> {
                type Output = Self;
                fn sub(self, other: Self) -> Self::Output {
                    $unit(self.0 - other.0)
                }
            }
            impl <T: SubAssign<T>> SubAssign for $unit<T> {
                fn sub_assign(&mut self, rhs: Self) {
                    self.0 -= rhs.0
                }
            }
            impl <T: Neg<Output = T>> Neg for $unit<T> {
                type Output = Self;
                fn neg(self) -> Self::Output {
                    $unit(-self.0)
                }
            }
            impl <T: Mul<Output = T>> Mul for $unit<T> {
                type Output = Self;
                fn mul(self, other: Self) -> Self::Output {
                    $unit(self.0 * other.0)
                }
            }
            impl <T: MulAssign<T>> MulAssign for $unit<T> {
                fn mul_assign(&mut self, rhs: Self) {
                    self.0 *= rhs.0
                }
            }
            impl <T: Mul<f32, Output = T>> Mul<f32> for $unit<T> {
                type Output = $unit<T>;
                fn mul(self, rhs: f32) -> Self::Output {
                    $unit(self.0 * rhs)
                }
            }
            impl <T: MulAssign<f32>> MulAssign<f32> for $unit<T> {
                fn mul_assign(&mut self, rhs: f32) {
                    self.0 *= rhs
                }
            }
        
            impl Into<UiValue<Vec2>> for $unit<(f32, f32)> {
                fn into(self) -> UiValue<Vec2> {
                    $unit(Vec2::new(self.0.0, self.0.1)).into()
                }
            }
            impl Into<UiValue<Vec3>> for $unit<(f32, f32, f32)> {
                fn into(self) -> UiValue<Vec3> {
                    $unit(Vec3::new(self.0.0, self.0.1, self.0.2)).into()
                }
            }
            impl Into<UiValue<Vec4>> for $unit<(f32, f32, f32, f32)> {
                fn into(self) -> UiValue<Vec4> {
                    $unit(Vec4::new(self.0.0, self.0.1, self.0.2, self.0.3)).into()
                }
            }
            impl Into<UiValue<Vec2>> for $unit<f32> {
                fn into(self) -> UiValue<Vec2> {
                    $unit(Vec2::new(self.0, self.0)).into()
                }
            }
            impl Into<UiValue<Vec3>> for $unit<f32> {
                fn into(self) -> UiValue<Vec3> {
                    $unit(Vec3::new(self.0, self.0, self.0)).into()
                }
            }
            impl Into<UiValue<Vec4>> for $unit<f32> {
                fn into(self) -> UiValue<Vec4> {
                    $unit(Vec4::new(self.0, self.0, self.0, self.0)).into()
                }
            }
        )*
    };
}
macro_rules! unit_cross_operations {
    (($unit1:ident, $ufield1:ident), ($unit2:ident, $ufield2:ident)) => {
        impl<T: Add<Output = T>> Add<$unit2<T>> for $unit1<T> {
            type Output = UiValue<T>;
            fn add(self, other: $unit2<T>) -> Self::Output {
                let mut ret = UiValue::new();
                ret.$ufield1 = Some(self.0);
                ret.$ufield2 = Some(other.0);
                ret
            }
        }
        impl<T: Sub<Output = T>> Sub<$unit2<T>> for $unit1<T> where T: Neg<Output = T> {
            type Output = UiValue<T>;
            fn sub(self, other: $unit2<T>) -> Self::Output {
                let mut ret = UiValue::new();
                ret.$ufield1 = Some(self.0);
                ret.$ufield2 = Some(-other.0);
                ret
            }
        }
    }
}


// #========================#
// #=== TYPE DEFINITIONS ===#

/// **Absolute** - Represents non-changing unit. Scale can be modified but by default `1Ab = 1Px`.
/// ## 🛠️ Example
/// ```
/// # use lunex_core::Ab;
/// let a: Ab<f32> = Ab(4.0) + Ab(6.0); // -> 10px
/// let b: Ab<f32> = Ab(4.0) * 2.0;     // -> 8px
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Ab<T>(pub T);

/// **Relative** - Represents scalable unit `0% to 100%`. `120%` is allowed.
/// ## 🛠️ Example
/// ```
/// # use lunex_core::Rl;
/// let a: Rl<f32> = Rl(25.0) + Rl(40.0); // -> 65%
/// let b: Rl<f32> = Rl(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rl<T>(pub T);

/// **Relative width** - Represents scalable unit `0% to 100%`. `120%` is allowed.
/// Proportional to width measure even when used in height field.
/// ## 🛠️ Example
/// ```
/// # use lunex_core::Rw;
/// let a: Rw<f32> = Rw(25.0) + Rw(40.0); // -> 65%
/// let b: Rw<f32> = Rw(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rw<T>(pub T);

/// **Relative height** - Represents scalable unit `0% to 100%`. `120%` is allowed.
/// Proportional to height measure even when used in width field.
/// ## 🛠️ Example
/// ```
/// # use lunex_core::Rh;
/// let a: Rh<f32> = Rh(25.0) + Rh(40.0); // -> 65%
/// let b: Rh<f32> = Rh(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rh<T>(pub T);

/// **Size of M** - Represents unit that is the size of the symbol `M`. Which is `16px` with `font size 16px` and so on.
/// ## 🛠️ Example
/// ```
/// # use lunex_core::Em;
/// let a: Em<f32> = Em(1.0) + Em(2.0); // -> 3em == 48px with font size 16px
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Em<T>(pub T);

/// **Space** - Represents proportional empty space left in the parent container. Requires to know space unit of surrounding
/// containers to know the exact value. Works on ratio basis ex. how much of empty space will be distributed to each container.
/// Used for context aware alignment. 
/// ## 🛠️ Example
/// ```
/// # use lunex_core::Prc;
/// let a: Sp<f32> = Sp(1.0) + Sp(2.0); // -> 3 space
/// let b: Sp<f32> = Sp(2.0) * 3.0;     // -> 6 space
/// ```
/// If container `a` and `b` were next to each other, they would split remaining space in **3:6** ratio.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Sp<T>(pub T);

/// **Viewport** - Represents scalable unit `0% to 100%` of the root container. `120%` is allowed.
/// ## 🛠️ Example
/// ```
/// # use lunex_core::Vp;
/// let a: Vp<f32> = Vp(25.0) + Vp(40.0); // -> 65%
/// let b: Vp<f32> = Vp(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vp<T>(pub T);

/// **Viewport width** - Represents scalable unit `0% to 100%` of the root container. `120%` is allowed.
/// Proportional to width measure even when used in height field.
/// ## 🛠️ Example
/// ```
/// # use lunex_core::Vw;
/// let a: Vw<f32> = Vw(25.0) + Vw(40.0); // -> 65%
/// let b: Vw<f32> = Vw(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vw<T>(pub T);

/// **Viewport Height** - Represents scalable unit `0% to 100%` of the root container. `120%` is allowed.
/// Proportional to height measure even when used in width field.
/// ## 🛠️ Example
/// ```
/// # use lunex_core::Vh;
/// let a: Vh<f32> = Vh(25.0) + Vh(40.0); // -> 65%
/// let b: Vh<f32> = Vh(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vh<T>(pub T);

// #===================#
// #=== MACRO CALLS ===#

uivalue_declare!(ab, rl, rw, rh, em, sp, vp, vw, vh);
unit_implement!(Ab, Rl, Rw, Rh, Em, Sp, Vp, Vw, Vh);
uivalue_implement!((Ab, ab), (Rl, rl), (Rw, rw), (Rh, rh), (Em, em), (Sp, sp), (Vp, vp), (Vw, vw), (Vh, vh));

unit_cross_operations!((Ab, ab), (Rl, rl));
unit_cross_operations!((Ab, ab), (Rw, rw));
unit_cross_operations!((Ab, ab), (Rh, rh));
unit_cross_operations!((Ab, ab), (Em, em));
unit_cross_operations!((Ab, ab), (Sp, sp));
unit_cross_operations!((Ab, ab), (Vp, vp));
unit_cross_operations!((Ab, ab), (Vw, vw));
unit_cross_operations!((Ab, ab), (Vh, vh));

unit_cross_operations!((Rl, rl), (Ab, ab));
unit_cross_operations!((Rl, rl), (Rw, rw));
unit_cross_operations!((Rl, rl), (Rh, rh));
unit_cross_operations!((Rl, rl), (Em, em));
unit_cross_operations!((Rl, rl), (Sp, sp));
unit_cross_operations!((Rl, rl), (Vp, vp));
unit_cross_operations!((Rl, rl), (Vw, vw));
unit_cross_operations!((Rl, rl), (Vh, vh));

unit_cross_operations!((Rw, rw), (Ab, ab));
unit_cross_operations!((Rw, rw), (Rl, rl));
unit_cross_operations!((Rw, rw), (Rh, rh));
unit_cross_operations!((Rw, rw), (Em, em));
unit_cross_operations!((Rw, rw), (Sp, sp));
unit_cross_operations!((Rw, rw), (Vp, vp));
unit_cross_operations!((Rw, rw), (Vw, vw));
unit_cross_operations!((Rw, rw), (Vh, vh));

unit_cross_operations!((Rh, rh), (Ab, ab));
unit_cross_operations!((Rh, rh), (Rl, rl));
unit_cross_operations!((Rh, rh), (Rw, rw));
unit_cross_operations!((Rh, rh), (Em, em));
unit_cross_operations!((Rh, rh), (Sp, sp));
unit_cross_operations!((Rh, rh), (Vp, vp));
unit_cross_operations!((Rh, rh), (Vw, vw));
unit_cross_operations!((Rh, rh), (Vh, vh));

unit_cross_operations!((Em, em), (Ab, ab));
unit_cross_operations!((Em, em), (Rl, rl));
unit_cross_operations!((Em, em), (Rw, rw));
unit_cross_operations!((Em, em), (Rh, rh));
unit_cross_operations!((Em, em), (Sp, sp));
unit_cross_operations!((Em, em), (Vp, vp));
unit_cross_operations!((Em, em), (Vw, vw));
unit_cross_operations!((Em, em), (Vh, vh));

unit_cross_operations!((Sp, sp), (Ab, ab));
unit_cross_operations!((Sp, sp), (Rl, rl));
unit_cross_operations!((Sp, sp), (Rw, rw));
unit_cross_operations!((Sp, sp), (Rh, rh));
unit_cross_operations!((Sp, sp), (Em, em));
unit_cross_operations!((Sp, sp), (Vp, vp));
unit_cross_operations!((Sp, sp), (Vw, vw));
unit_cross_operations!((Sp, sp), (Vh, vh));

unit_cross_operations!((Vp, vp), (Ab, ab));
unit_cross_operations!((Vp, vp), (Rl, rl));
unit_cross_operations!((Vp, vp), (Rw, rw));
unit_cross_operations!((Vp, vp), (Rh, rh));
unit_cross_operations!((Vp, vp), (Em, em));
unit_cross_operations!((Vp, vp), (Sp, sp));
unit_cross_operations!((Vp, vp), (Vw, vw));
unit_cross_operations!((Vp, vp), (Vh, vh));

unit_cross_operations!((Vw, vw), (Ab, ab));
unit_cross_operations!((Vw, vw), (Rl, rl));
unit_cross_operations!((Vw, vw), (Rw, rw));
unit_cross_operations!((Vw, vw), (Rh, rh));
unit_cross_operations!((Vw, vw), (Em, em));
unit_cross_operations!((Vw, vw), (Sp, sp));
unit_cross_operations!((Vw, vw), (Vp, vp));
unit_cross_operations!((Vw, vw), (Vh, vh));

unit_cross_operations!((Vh, vh), (Ab, ab));
unit_cross_operations!((Vh, vh), (Rl, rl));
unit_cross_operations!((Vh, vh), (Rw, rw));
unit_cross_operations!((Vh, vh), (Rh, rh));
unit_cross_operations!((Vh, vh), (Em, em));
unit_cross_operations!((Vh, vh), (Sp, sp));
unit_cross_operations!((Vh, vh), (Vp, vp));
unit_cross_operations!((Vh, vh), (Vw, vw));

// #==============================#
// #=== CUSTOM IMPLEMENTATIONS ===#

// # Impl (x) => UiValue(f32)
impl Into<UiValue<f32>> for f32 {
    fn into(self) -> UiValue<f32> {
        Ab(self).into()
    }
}

impl Into<UiValue<Vec2>> for UiValue<f32> {
    fn into(self) -> UiValue<Vec2> {
        let mut out = UiValue::<Vec2>::new();
        out.set_x(self);
        out.set_y(self);
        out
    }
}
// # Impl (x) => UiValue(Vec2)
impl Into<UiValue<Vec2>> for f32 {
    fn into(self) -> UiValue<Vec2> {
        Ab(Vec2::new(self, self)).into()
    }
}
// # Impl ((x, x)) => UiValue(Vec2)
impl Into<UiValue<Vec2>> for (f32, f32) {
    fn into(self) -> UiValue<Vec2> {
        Ab(Vec2::new(self.0, self.1)).into()
    }
}

impl Into<UiValue<Vec3>> for UiValue<f32> {
    fn into(self) -> UiValue<Vec3> {
        let mut out = UiValue::<Vec3>::new();
        out.set_x(self);
        out.set_y(self);
        out.set_z(self);
        out
    }
}
// # Impl (x) => UiValue(Vec3)
impl Into<UiValue<Vec3>> for f32 {
    fn into(self) -> UiValue<Vec3> {
        Ab(Vec3::new(self, self, self)).into()
    }
}
// # Impl ((x, x, x)) => UiValue(Vec3)
impl Into<UiValue<Vec3>> for (f32, f32, f32) {
    fn into(self) -> UiValue<Vec3> {
        Ab(Vec3::new(self.0, self.1, self.2)).into()
    }
}

impl Into<UiValue<Vec4>> for UiValue<f32> {
    fn into(self) -> UiValue<Vec4> {
        let mut out = UiValue::<Vec4>::new();
        out.set_x(self);
        out.set_y(self);
        out.set_z(self);
        out.set_w(self);
        out
    }
}
// # Impl (x) => UiValue(Vec4)
impl Into<UiValue<Vec4>> for f32 {
    fn into(self) -> UiValue<Vec4> {
        Ab(Vec4::new(self, self, self, self)).into()
    }
}
// # Impl ((x, x, x, x)) => UiValue(Vec4)
impl Into<UiValue<Vec4>> for (f32, f32, f32, f32) {
    fn into(self) -> UiValue<Vec4> {
        Ab(Vec4::new(self.0, self.1, self.2, self.3)).into()
    }
}


impl Ab<f32> {
    /// ## Zero
    pub const ZERO: Ab<f32> = Ab(0.0);
    /// ## One
    pub const ONE: Ab<f32> = Ab(1.0);
    /// ## Extra-small
    pub const XS: Ab<f32> = Ab(1.0 * 16.0);
    /// ## Small
    pub const SM: Ab<f32> = Ab(2.0 * 16.0);
    /// ## Medium
    pub const MD: Ab<f32> = Ab(3.0 * 16.0);
    /// ## Large
    pub const LG: Ab<f32> = Ab(4.0 * 16.0);
    /// ## Extra-large
    pub const XL: Ab<f32> = Ab(6.0 * 16.0);
    /// ## Extra-large 2
    pub const XL2: Ab<f32> = Ab(8.0 * 16.0);
    /// ## Extra-large 3
    pub const XL3: Ab<f32> = Ab(10.0 * 16.0);
    /// ## Extra-large 4
    pub const XL4: Ab<f32> = Ab(12.0 * 16.0);
    /// ## Extra-large 5
    pub const XL5: Ab<f32> = Ab(14.0 * 16.0);
    /// ## Extra-large 6
    pub const XL6: Ab<f32> = Ab(16.0 * 16.0);
    /// ## Extra-large 7
    pub const XL7: Ab<f32> = Ab(18.0 * 16.0);
}
impl Ab<Vec2> {
    /// ## Zero - Vec2
    pub const ZERO_2: Ab<Vec2> = Ab(Vec2::splat(0.0));
    /// ## One - Vec2
    pub const ONE_2: Ab<Vec2> = Ab(Vec2::splat(1.0));
    /// ## Extra-small - Vec2
    pub const XS_2: Ab<Vec2> = Ab(Vec2::splat(1.0 * 16.0));
    /// ## Small - Vec2
    pub const SM_2: Ab<Vec2> = Ab(Vec2::splat(2.0 * 16.0));
    /// ## Medium - Vec2
    pub const MD_2: Ab<Vec2> = Ab(Vec2::splat(3.0 * 16.0));
    /// ## Large - Vec2
    pub const LG_2: Ab<Vec2> = Ab(Vec2::splat(4.0 * 16.0));
    /// ## Extra-large - Vec2
    pub const XL_2: Ab<Vec2> = Ab(Vec2::splat(6.0 * 16.0));
    /// ## Extra-large 2 - Vec2
    pub const XL2_2: Ab<Vec2> = Ab(Vec2::splat(8.0 * 16.0));
    /// ## Extra-large 3 - Vec2
    pub const XL3_2: Ab<Vec2> = Ab(Vec2::splat(10.0 * 16.0));
    /// ## Extra-large 4 - Vec2
    pub const XL4_2: Ab<Vec2> = Ab(Vec2::splat(12.0 * 16.0));
    /// ## Extra-large 5 - Vec2
    pub const XL5_2: Ab<Vec2> = Ab(Vec2::splat(14.0 * 16.0));
    /// ## Extra-large 6 - Vec2
    pub const XL6_2: Ab<Vec2> = Ab(Vec2::splat(16.0 * 16.0));
    /// ## Extra-large 7 - Vec2
    pub const XL7_2: Ab<Vec2> = Ab(Vec2::splat(18.0 * 16.0));
}
impl Ab<Vec3> {
    /// ## Zero - Vec3
    pub const ZERO_3: Ab<Vec3> = Ab(Vec3::splat(0.0));
    /// ## One - Vec3
    pub const ONE_3: Ab<Vec3> = Ab(Vec3::splat(1.0));
    /// ## Extra-small - Vec3
    pub const XS_3: Ab<Vec3> = Ab(Vec3::splat(1.0 * 16.0));
    /// ## Small - Vec3
    pub const SM_3: Ab<Vec3> = Ab(Vec3::splat(2.0 * 16.0));
    /// ## Medium - Vec3
    pub const MD_3: Ab<Vec3> = Ab(Vec3::splat(3.0 * 16.0));
    /// ## Large - Vec3
    pub const LG_3: Ab<Vec3> = Ab(Vec3::splat(4.0 * 16.0));
    /// ## Extra-large - Vec3
    pub const XL_3: Ab<Vec3> = Ab(Vec3::splat(6.0 * 16.0));
    /// ## Extra-large 2 - Vec3
    pub const XL2_3: Ab<Vec3> = Ab(Vec3::splat(8.0 * 16.0));
    /// ## Extra-large 3 - Vec3
    pub const XL3_3: Ab<Vec3> = Ab(Vec3::splat(10.0 * 16.0));
    /// ## Extra-large 4 - Vec3
    pub const XL4_3: Ab<Vec3> = Ab(Vec3::splat(12.0 * 16.0));
    /// ## Extra-large 5 - Vec3
    pub const XL5_3: Ab<Vec3> = Ab(Vec3::splat(14.0 * 16.0));
    /// ## Extra-large 6 - Vec3
    pub const XL6_3: Ab<Vec3> = Ab(Vec3::splat(16.0 * 16.0));
    /// ## Extra-large 7 - Vec3
    pub const XL7_3: Ab<Vec3> = Ab(Vec3::splat(18.0 * 16.0));
}
impl Ab<Vec4> {
    /// ## Zero - Vec4
    pub const ZERO_4: Ab<Vec4> = Ab(Vec4::splat(0.0));
    /// ## One - Vec4
    pub const ONE_4: Ab<Vec4> = Ab(Vec4::splat(1.0));
    /// ## Extra-small - Vec4
    pub const XS_4: Ab<Vec4> = Ab(Vec4::splat(1.0 * 16.0));
    /// ## Small - Vec4
    pub const SM_4: Ab<Vec4> = Ab(Vec4::splat(2.0 * 16.0));
    /// ## Medium - Vec4
    pub const MD_4: Ab<Vec4> = Ab(Vec4::splat(3.0 * 16.0));
    /// ## Large - Vec4
    pub const LG_4: Ab<Vec4> = Ab(Vec4::splat(4.0 * 16.0));
    /// ## Extra-large - Vec4
    pub const XL_4: Ab<Vec4> = Ab(Vec4::splat(6.0 * 16.0));
    /// ## Extra-large 2 - Vec4
    pub const XL2_4: Ab<Vec4> = Ab(Vec4::splat(8.0 * 16.0));
    /// ## Extra-large 3 - Vec4
    pub const XL3_4: Ab<Vec4> = Ab(Vec4::splat(10.0 * 16.0));
    /// ## Extra-large 4 - Vec4
    pub const XL4_4: Ab<Vec4> = Ab(Vec4::splat(12.0 * 16.0));
    /// ## Extra-large 5 - Vec4
    pub const XL5_4: Ab<Vec4> = Ab(Vec4::splat(14.0 * 16.0));
    /// ## Extra-large 6 - Vec4
    pub const XL6_4: Ab<Vec4> = Ab(Vec4::splat(16.0 * 16.0));
    /// ## Extra-large 7 - Vec4
    pub const XL7_4: Ab<Vec4> = Ab(Vec4::splat(18.0 * 16.0));
}


// #=====================#
// #=== FUNCTIONALITY ===#

impl NiceDisplay for UiValue<f32> {
    fn to_nicestr(&self) -> String {
        let mut t = String::new();
        if let Some(v) = self.ab {
            if v != 0.0 {
                t = format!("{}", v.to_string().bright_blue());
            }
        }
        if let Some(v) = self.rl {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, v.to_string().bright_green(), "%".bright_green());
            }
        }
        if let Some(v) = self.rw {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, v.to_string().bright_green(), "%w".bright_green());
            }
        }
        if let Some(v) = self.rh {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, v.to_string().bright_green(), "%h".bright_green());
            }
        }
        if let Some(v) = self.em {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, v.to_string().bright_red(), "m".bright_red());
            }
        }
        if let Some(v) = self.sp {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, v.to_string().bright_red(), "sp".bright_red());
            }
        }
        if let Some(v) = self.vp {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, v.to_string().bright_green(), "v%".bright_green());
            }
        }
        if let Some(v) = self.vw {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, v.to_string().bright_green(), "v%w".bright_green());
            }
        }
        if let Some(v) = self.vh {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, v.to_string().bright_green(), "v%h".bright_green());
            }
        }
        if t.is_empty() { t = format!("{}", "0".bright_blue()); };
        format!("{}", t.black())
    }
}
impl NiceDisplay for UiValue<Vec2> {
    fn to_nicestr(&self) -> String {
        let mut tx = String::new();
        let mut ty = String::new();
        if let Some(v) = self.ab {
            if v.x != 0.0 {
                tx = format!("{}", v.x.to_string().bright_blue());
            }
            if v.y != 0.0 {
                ty = format!("{}", v.y.to_string().bright_blue());
            }
        }
        if let Some(v) = self.rl {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "%".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "%".bright_green());
            }
        }
        if let Some(v) = self.rw {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "%w".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "%w".bright_green());
            }
        }
        if let Some(v) = self.rh {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "%h".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "%h".bright_green());
            }
        }
        if let Some(v) = self.em {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_red(), "m".bright_red());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_red(), "m".bright_red());
            }
        }
        if let Some(v) = self.sp {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_red(), "sp".bright_red());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_red(), "sp".bright_red());
            }
        }
        if let Some(v) = self.vp {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "v%".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "v%".bright_green());
            }
        }
        if let Some(v) = self.vw {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "v%w".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "v%w".bright_green());
            }
        }
        if let Some(v) = self.vh {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "v%h".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "v%h".bright_green());
            }
        }
        if tx.is_empty() { tx = format!("{}", "0".bright_blue()); };
        if ty.is_empty() { ty = format!("{}", "0".bright_blue()); };
        format!("x: {}, y: {}", tx.black(), ty.black())
    }
}
impl NiceDisplay for UiValue<Vec3> {
    fn to_nicestr(&self) -> String {
        let mut tx = String::new();
        let mut ty = String::new();
        let mut tz = String::new();
        if let Some(v) = self.ab {
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
        if let Some(v) = self.rl {
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
        if let Some(v) = self.rw {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "%w".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "%w".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_green(), "%w".bright_green());
            }
        }
        if let Some(v) = self.rh {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "%h".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "%h".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_green(), "%h".bright_green());
            }
        }
        if let Some(v) = self.em {
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
        if let Some(v) = self.sp {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_red(), "sp".bright_red());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_red(), "sp".bright_red());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_red(), "sp".bright_red());
            }
        }
        if let Some(v) = self.vp {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "v%".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "v%".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_green(), "v%".bright_green());
            }
        }
        if let Some(v) = self.vw {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "v%w".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "v%w".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_green(), "v%w".bright_green());
            }
        }
        if let Some(v) = self.vh {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "v%h".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "v%h".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_green(), "v%h".bright_green());
            }
        }
        if tx.is_empty() { tx = format!("{}", "0".bright_blue()); };
        if ty.is_empty() { ty = format!("{}", "0".bright_blue()); };
        if tz.is_empty() { tz = format!("{}", "0".bright_blue()); };
        format!("x: {}, y: {} z:{}", tx.black(), ty.black(), tz.black())
    }
}
impl NiceDisplay for UiValue<Vec4> {
    fn to_nicestr(&self) -> String {
        let mut tx = String::new();
        let mut ty = String::new();
        let mut tz = String::new();
        let mut tw = String::new();
        if let Some(v) = self.ab {
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
        if let Some(v) = self.rl {
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
        if let Some(v) = self.rw {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "%w".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "%w".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_green(), "%w".bright_green());
            }
            if v.w != 0.0 {
                if !tw.is_empty() { tw += " + " };
                tw = format!("{}{}{}", tw, v.w.to_string().bright_green(), "%w".bright_green());
            }
        }
        if let Some(v) = self.rh {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "%h".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "%h".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_green(), "%h".bright_green());
            }
            if v.w != 0.0 {
                if !tw.is_empty() { tw += " + " };
                tw = format!("{}{}{}", tw, v.w.to_string().bright_green(), "%h".bright_green());
            }
        }
        if let Some(v) = self.em {
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
        if let Some(v) = self.sp {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_red(), "sp".bright_red());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_red(), "sp".bright_red());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_red(), "sp".bright_red());
            }
            if v.w != 0.0 {
                if !tw.is_empty() { tw += " + " };
                tw = format!("{}{}{}", tw, v.w.to_string().bright_red(), "sp".bright_red());
            }
        }
        if let Some(v) = self.vp {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "v%".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "v%".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_green(), "v%".bright_green());
            }
            if v.w != 0.0 {
                if !tw.is_empty() { tw += " + " };
                tw = format!("{}{}{}", tw, v.w.to_string().bright_green(), "v%".bright_green());
            }
        }
        if let Some(v) = self.vw {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "v%w".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "v%w".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_green(), "v%w".bright_green());
            }
            if v.w != 0.0 {
                if !tw.is_empty() { tw += " + " };
                tw = format!("{}{}{}", tw, v.w.to_string().bright_green(), "v%w".bright_green());
            }
        }
        if let Some(v) = self.vh {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, v.x.to_string().bright_green(), "v%h".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, v.y.to_string().bright_green(), "v%h".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, v.z.to_string().bright_green(), "v%h".bright_green());
            }
            if v.w != 0.0 {
                if !tw.is_empty() { tw += " + " };
                tw = format!("{}{}{}", tw, v.w.to_string().bright_green(), "v%h".bright_green());
            }
        }
        if tx.is_empty() { tx = format!("{}", "0".bright_blue()); };
        if ty.is_empty() { ty = format!("{}", "0".bright_blue()); };
        if tz.is_empty() { tz = format!("{}", "0".bright_blue()); };
        if tw.is_empty() { tw = format!("{}", "0".bright_blue()); };
        format!("x: {}, y: {} z:{} w:{}", tx.black(), ty.black(), tz.black(), tw.black())
    }
}

// #=============#
// #=== TESTS ===#

#[cfg(test)]
mod test {
    use crate::NiceDisplay;

    use super::{Ab, Rl, Rw, Rh, Em, Sp, UiValue, Vec2};
    #[test]
    fn all () {
        let _: UiValue<f32> = Ab(5.0) + Rl(5.0);
        let _: UiValue<f32> = Rw(5.0) + Rh(5.0);
        let _: UiValue<f32> = Em(5.0) + Sp(5.0);

        let size: UiValue<Vec2> = Ab(Vec2::splat(5.0)) + Rl(Vec2::splat(5.0));
        println!("{}", size.to_nicestr());
    }
}

