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

/// Declare UiValue struct and add fields to it
macro_rules! uivalue_declare {
    ($($ufield:ident), *) => {
        /// **Ui value** - A collection of different units used for UI.
        /// They are computed at runtime when layout is being calculated (context-aware).
        /// The supported units that implement `Into<UiValue>` are:
        /// * [`Ab`] [`Rl`] [`Rw`] [`Rh`] [`Em`] [`Sp`] [`Vw`] [`Vh`]
        /// ## 📦 Types
        /// First class implementations for `(T)` are:
        /// * [`f32`] [`Vec2`] [`Vec3`] [`Vec4`]
        /// ## 🛠️ Example
        /// ```
        /// # use lunex_core::{UiValue, Ab, Em, Rl, Sp};
        /// let a: UiValue<f32> = Ab(4.0) + Em(1.0);  // -> 4px + 1em
        /// let b: UiValue<f32> = Ab(40.0) - Rl(5.0); // -> 40px - 5%
        /// let c: UiValue<f32> = Sp(5.0).into();     // -> 5 space
        /// let d: UiValue<Vec2> = (Ab(20.0), Em(2.0)).into() // -> [20px, 2em]
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

/// Implement adding to UiValue struct
macro_rules! uivalue_implement {
    ($( ($unit:ident, $ufield:ident) ),* ) => {

        $(
            impl <T> From<$unit<T>> for UiValue<T> {
                fn from(val: $unit<T>) -> UiValue<T> {
                    let mut ret = UiValue::new();
                    ret.$ufield = Some(val.0);
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

        impl <T: Mul<f32, Output = T>> Mul<f32> for UiValue<T> {
            type Output = Self;
            fn mul(self, other: f32) -> Self::Output {
                let mut output = UiValue::new();
                $(
                    if let Some(v1) = self.$ufield {
                        output.$ufield = Some(v1 * other);
                    }
                )*
                output
            }
        }
        impl <T: Mul<f32, Output = T> + Copy> MulAssign<f32> for UiValue<T> {
            fn mul_assign(&mut self, rhs: f32) {
                *self = *self * rhs
            }
        }

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

/// Implement basic math and conversions for a type
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
        
            impl From<$unit<(f32, f32)>> for UiValue<Vec2> {
                fn from(val: $unit<(f32, f32)>) -> UiValue<Vec2> {
                    $unit(Vec2::new(val.0.0, val.0.1)).into()
                }
            }
            impl From<$unit<(f32, f32, f32)>> for UiValue<Vec3> {
                fn from(val: $unit<(f32, f32, f32)>) -> UiValue<Vec3> {
                    $unit(Vec3::new(val.0.0, val.0.1, val.0.2)).into()
                }
            }
            impl From<$unit<(f32, f32, f32, f32)>> for UiValue<Vec4> {
                fn from(val: $unit<(f32, f32, f32, f32)>) -> UiValue<Vec4> {
                    $unit(Vec4::new(val.0.0, val.0.1, val.0.2, val.0.3)).into()
                }
            }

            impl From<$unit<f32>> for UiValue<Vec2> {
                fn from(val: $unit<f32>) -> UiValue<Vec2> {
                    $unit(Vec2::new(val.0, val.0)).into()
                }
            }
            impl From<$unit<f32>> for UiValue<Vec3> {
                fn from(val: $unit<f32>) -> UiValue<Vec3> {
                    $unit(Vec3::new(val.0, val.0, val.0)).into()
                }
            }
            impl From<$unit<f32>> for UiValue<Vec4> {
                fn from(val: $unit<f32>) -> UiValue<Vec4> {
                    $unit(Vec4::new(val.0, val.0, val.0, val.0)).into()
                }
            }

            impl From<$unit<f32>> for UiValueType<f32> {
                fn from(val: $unit<f32>) -> UiValueType<f32> {
                    UiValueType::$unit(val)
                }
            }
        )*
    };
}

/// Implement adding two types together
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
/// # use lunex_engine::Ab;
/// let a: Ab<f32> = Ab(4.0) + Ab(6.0); // -> 10px
/// let b: Ab<f32> = Ab(4.0) * 2.0;     // -> 8px
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Ab<T>(pub T);

/// **Relative** - Represents scalable unit `0% to 100%`. `120%` is allowed.
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::Rl;
/// let a: Rl<f32> = Rl(25.0) + Rl(40.0); // -> 65%
/// let b: Rl<f32> = Rl(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rl<T>(pub T);

/// **Relative width** - Represents scalable unit `0% to 100%`. `120%` is allowed.
/// Proportional to a width measure even when used in a height field.
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::Rw;
/// let a: Rw<f32> = Rw(25.0) + Rw(40.0); // -> 65%
/// let b: Rw<f32> = Rw(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rw<T>(pub T);

/// **Relative height** - Represents scalable unit `0% to 100%`. `120%` is allowed.
/// Proportional to a height measure even when used in a width field.
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::Rh;
/// let a: Rh<f32> = Rh(25.0) + Rh(40.0); // -> 65%
/// let b: Rh<f32> = Rh(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rh<T>(pub T);

/// **Size of M** - Represents unit that is the size of the symbol `M`. Which is `16px` with `font size 16px` and so on.
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::Em;
/// let a: Em<f32> = Em(1.0) + Em(2.0); // -> 3em == 48px with font size 16px
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Em<T>(pub T);

/// **Space** - Represents proportional empty space left in the parent container. Requires to know space unit of surrounding
/// containers to know the exact value. Works on ratio basis ex. how much of empty space will be distributed to each container.
/// Used for context aware alignment. 
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::Sp;
/// let a: Sp<f32> = Sp(1.0) + Sp(2.0); // -> 3 space
/// let b: Sp<f32> = Sp(2.0) * 3.0;     // -> 6 space
/// ```
/// If container `a` and `b` were next to each other, they would split remaining space in **3:6** ratio.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Sp<T>(pub T);

/// **Viewport** - Represents scalable unit `0% to 100%` of the root container. `120%` is allowed.
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::Vp;
/// let a: Vp<f32> = Vp(25.0) + Vp(40.0); // -> 65%
/// let b: Vp<f32> = Vp(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vp<T>(pub T);

/// **Viewport width** - Represents scalable unit `0% to 100%` of the root container. `120%` is allowed.
/// Proportional to a width measure even when used in a height field.
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::Vw;
/// let a: Vw<f32> = Vw(25.0) + Vw(40.0); // -> 65%
/// let b: Vw<f32> = Vw(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vw<T>(pub T);

/// **Viewport Height** - Represents scalable unit `0% to 100%` of the root container. `120%` is allowed.
/// Proportional to a height measure even when used in a width field.
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::Vh;
/// let a: Vh<f32> = Vh(25.0) + Vh(40.0); // -> 65%
/// let b: Vh<f32> = Vh(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vh<T>(pub T);

/// **Unit type** - Enum with all possible ui unit types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UiValueType<T> {
    Ab(Ab<T>),
    Rl(Rl<T>),
    Rw(Rw<T>),
    Rh(Rh<T>),
    Em(Em<T>),
    Sp(Sp<T>),
    Vp(Vp<T>),
    Vw(Vw<T>),
    Vh(Vh<T>),
}

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

// # Impl (A, B) => UiValue(Vec2)
impl <A, B> From<(A, B)> for UiValue<Vec2> where 
    A: Into<UiValue<f32>>, 
    B: Into<UiValue<f32>>
{
    fn from(val: (A, B)) -> Self {
        UiValue::<Vec2>::new().with_x(val.0).with_y(val.1)
    }
}

// # Impl (A, B, C) => UiValue(Vec3)
impl <A, B, C> From<(A, B, C)> for UiValue<Vec3> where 
    A: Into<UiValue<f32>>, 
    B: Into<UiValue<f32>>,
    C: Into<UiValue<f32>>
{
    fn from(val: (A, B, C)) -> Self {
        UiValue::<Vec3>::new().with_x(val.0).with_y(val.1).with_z(val.2)
    }
}

// # Impl (A, B, C, D) => UiValue(Vec4)
impl <A, B, C, D> From<(A, B, C, D)> for UiValue<Vec4> where 
    A: Into<UiValue<f32>>, 
    B: Into<UiValue<f32>>,
    C: Into<UiValue<f32>>,
    D: Into<UiValue<f32>>
{
    fn from(val: (A, B, C, D)) -> Self {
        UiValue::<Vec4>::new().with_x(val.0).with_y(val.1).with_z(val.2).with_w(val.3)
    }
}

// # Impl f32 => UiValue(f32)
impl From<f32> for UiValue<f32> {
    fn from(val: f32) -> Self {
        Ab(val).into()
    }
}
// # Impl f32 => UiValue(Vec2)
impl From<f32> for UiValue<Vec2> {
    fn from(val: f32) -> Self {
        Ab(Vec2::new(val, val)).into()
    }
}
// # Impl f32 => UiValue(Vec3)
impl From<f32> for UiValue<Vec3> {
    fn from(val: f32) -> Self {
        Ab(Vec3::new(val, val, val)).into()
    }
}
// # Impl f32 => UiValue(Vec4)
impl From<f32> for UiValue<Vec4> {
    fn from(val: f32) -> Self {
        Ab(Vec4::new(val, val, val, val)).into()
    }
}

// # Impl UiValue(f32) => UiValue(Vec2)
impl From<UiValue<f32>> for UiValue<Vec2> {
    fn from(val: UiValue<f32>) -> Self {
        let mut out = UiValue::<Vec2>::new();
        out.set_x(val);
        out.set_y(val);
        out
    }
}
// # Impl UiValue(f32) => UiValue(Vec3)
impl From<UiValue<f32>> for UiValue<Vec3> {
    fn from(val: UiValue<f32>) -> Self {
        let mut out = UiValue::<Vec3>::new();
        out.set_x(val);
        out.set_y(val);
        out.set_z(val);
        out
    }
}
// # Impl UiValue(f32) => UiValue(Vec4)
impl From<UiValue<f32>> for UiValue<Vec4> {
    fn from(val: UiValue<f32>) -> Self {
        let mut out = UiValue::<Vec4>::new();
        out.set_x(val);
        out.set_y(val);
        out.set_z(val);
        out.set_w(val);
        out
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

/// ## UiValue Evaluate
/// Trait for implementing evaluation logic for `(TT)`.
/// `(T)` should be 1 vector unit version of `(TT)`.
/// ## 📦 Types
pub trait UiValueEvaluate<T> {
    /// Evaluates the NodeSize for `(T)`
    fn evaluate(&self, absolute_scale: T, parent_size: T, viewport_size: T, font_size: T) -> T;
}

// # Impl evaluate
impl UiValueEvaluate<f32> for UiValue<f32> {
    fn evaluate(&self, absolute_scale: f32, parent_size: f32, viewport_size: f32, font_size: f32) -> f32 {
        let mut out = 0.0;
        if let Some(v) = self.ab { out += v * absolute_scale }
        if let Some(v) = self.rl { out += (v/100.0) * parent_size }
        if let Some(v) = self.rw { out += (v/100.0) * parent_size }
        if let Some(v) = self.rh { out += (v/100.0) * parent_size }
        if let Some(v) = self.em { out += v * font_size }
        if let Some(v) = self.vp { out += (v/100.0) * viewport_size }
        if let Some(v) = self.vh { out += (v/100.0) * viewport_size }
        out
    }
}
impl UiValueEvaluate<Vec2> for UiValue<Vec2> {
    fn evaluate(&self, absolute_scale: Vec2, parent_size: Vec2, viewport_size: Vec2, font_size: Vec2) -> Vec2 {
        let mut out = Vec2::ZERO;
        if let Some(v) = self.ab { out += v * absolute_scale }
        if let Some(v) = self.rl { out += (v/100.0) * parent_size }
        if let Some(v) = self.rw { out += (v/100.0) * parent_size.x }
        if let Some(v) = self.rh { out += (v/100.0) * parent_size.y }
        if let Some(v) = self.em { out += v * font_size }
        if let Some(v) = self.vp { out += (v/100.0) * viewport_size.x }
        if let Some(v) = self.vh { out += (v/100.0) * viewport_size.y }
        out
    }
}
impl UiValueEvaluate<Vec3> for UiValue<Vec3> {
    fn evaluate(&self, absolute_scale: Vec3, parent_size: Vec3, viewport_size: Vec3, font_size: Vec3) -> Vec3 {
        let mut out = Vec3::ZERO;
        if let Some(v) = self.ab { out += v * absolute_scale }
        if let Some(v) = self.rl { out += (v/100.0) * parent_size }
        if let Some(v) = self.rw { out += (v/100.0) * parent_size.x }
        if let Some(v) = self.rh { out += (v/100.0) * parent_size.y }
        if let Some(v) = self.em { out += v * font_size }
        if let Some(v) = self.vp { out += (v/100.0) * viewport_size.x }
        if let Some(v) = self.vh { out += (v/100.0) * viewport_size.y }
        out
    }
}
impl UiValueEvaluate<Vec4> for UiValue<Vec4> {
    fn evaluate(&self, absolute_scale: Vec4, parent_size: Vec4, viewport_size: Vec4, font_size: Vec4) -> Vec4 {
        let mut out = Vec4::ZERO;
        if let Some(v) = self.ab { out += v * absolute_scale }
        if let Some(v) = self.rl { out += (v/100.0) * parent_size }
        if let Some(v) = self.rw { out += (v/100.0) * parent_size.x }
        if let Some(v) = self.rh { out += (v/100.0) * parent_size.y }
        if let Some(v) = self.em { out += v * font_size }
        if let Some(v) = self.vp { out += (v/100.0) * viewport_size.x }
        if let Some(v) = self.vh { out += (v/100.0) * viewport_size.y }
        out
    }
}


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

