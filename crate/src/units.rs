use crate::*;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;


// #======================#
// #=== THE UNIT TYPES ===#

/// **Absolute** - Represents non-changing unit. Scale can be modified but by default `1Ab = 1Px`.
/// ## 🛠️ Example
/// ```
/// # use bevy_lunex::*;
/// let a: Ab<f32> = Ab(4.0) + Ab(6.0); // -> 10px
/// let b: Ab<f32> = Ab(4.0) * 2.0;     // -> 8px
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Ab<T>(pub T);

/// **Relative** - Represents scalable unit `0% to 100%`. `120%` is allowed.
/// ## 🛠️ Example
/// ```
/// # use bevy_lunex::*;
/// let a: Rl<f32> = Rl(25.0) + Rl(40.0); // -> 65%
/// let b: Rl<f32> = Rl(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Rl<T>(pub T);

/// **Relative width** - Represents scalable unit `0% to 100%`. `120%` is allowed.
/// Proportional to a width measure even when used in a height field.
/// ## 🛠️ Example
/// ```
/// # use bevy_lunex::*;
/// let a: Rw<f32> = Rw(25.0) + Rw(40.0); // -> 65%
/// let b: Rw<f32> = Rw(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Rw<T>(pub T);

/// **Relative height** - Represents scalable unit `0% to 100%`. `120%` is allowed.
/// Proportional to a height measure even when used in a width field.
/// ## 🛠️ Example
/// ```
/// # use bevy_lunex::*;
/// let a: Rh<f32> = Rh(25.0) + Rh(40.0); // -> 65%
/// let b: Rh<f32> = Rh(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Rh<T>(pub T);

/// **Size of M** - Represents unit that is the size of the symbol `M`. Which is `16px` with `font size 16px` and so on.
/// ## 🛠️ Example
/// ```
/// # use bevy_lunex::*;
/// let a: Em<f32> = Em(1.0) + Em(2.0); // -> 3em == 48px with font size 16px
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Em<T>(pub T);

/// **Viewport** - Represents scalable unit `0% to 100%` of the root container. `120%` is allowed.
/// ## 🛠️ Example
/// ```
/// # use bevy_lunex::*;
/// let a: Vp<f32> = Vp(25.0) + Vp(40.0); // -> 65%
/// let b: Vp<f32> = Vp(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Vp<T>(pub T);

/// **Viewport width** - Represents scalable unit `0% to 100%` of the root container. `120%` is allowed.
/// Proportional to a width measure even when used in a height field.
/// ## 🛠️ Example
/// ```
/// # use bevy_lunex::*;
/// let a: Vw<f32> = Vw(25.0) + Vw(40.0); // -> 65%
/// let b: Vw<f32> = Vw(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Vw<T>(pub T);

/// **Viewport Height** - Represents scalable unit `0% to 100%` of the root container. `120%` is allowed.
/// Proportional to a height measure even when used in a width field.
/// ## 🛠️ Example
/// ```
/// # use bevy_lunex::*;
/// let a: Vh<f32> = Vh(25.0) + Vh(40.0); // -> 65%
/// let b: Vh<f32> = Vh(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Vh<T>(pub T);


/// Implement basic math and conversions for a type
macro_rules! init_unit {
    ($($unit:ident), *) => {
        
        /* #[derive(Debug, Clone, Copy, PartialEq, Reflect)]
        pub enum UiValueType {
            $(
                $unit,
            )*
        } */

        $(
            // Implement negation of the same type
            impl <T: Neg<Output = T>> Neg for $unit<T> {
                type Output = Self;
                fn neg(self) -> Self::Output {
                    $unit(-self.0)
                }
            }

            // Implement addition of the same type
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

            // Implement subtraction of the same type
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

            // Implement multiplication of the same type
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

            // Implement multiplication with the f32 type
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
        )*
    };
}
init_unit!(Ab, Rl, Rw, Rh, Em, Vp, Vw, Vh);

/// Implement basic math and conversions for a type
macro_rules! impl_unit_operations {
    ($($unit:ident), *) => {
        $(
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
        )*
    };
}
impl_unit_operations!(Ab, Rl, Rw, Rh, Em, Vp, Vw, Vh);

/// Implement adding two types together
macro_rules! impl_unit_cross_operations {
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

impl_unit_cross_operations!((Ab, ab), (Rl, rl));
impl_unit_cross_operations!((Ab, ab), (Rw, rw));
impl_unit_cross_operations!((Ab, ab), (Rh, rh));
impl_unit_cross_operations!((Ab, ab), (Em, em));
impl_unit_cross_operations!((Ab, ab), (Vp, vp));
impl_unit_cross_operations!((Ab, ab), (Vw, vw));
impl_unit_cross_operations!((Ab, ab), (Vh, vh));

impl_unit_cross_operations!((Rl, rl), (Ab, ab));
impl_unit_cross_operations!((Rl, rl), (Rw, rw));
impl_unit_cross_operations!((Rl, rl), (Rh, rh));
impl_unit_cross_operations!((Rl, rl), (Em, em));
impl_unit_cross_operations!((Rl, rl), (Vp, vp));
impl_unit_cross_operations!((Rl, rl), (Vw, vw));
impl_unit_cross_operations!((Rl, rl), (Vh, vh));

impl_unit_cross_operations!((Rw, rw), (Ab, ab));
impl_unit_cross_operations!((Rw, rw), (Rl, rl));
impl_unit_cross_operations!((Rw, rw), (Rh, rh));
impl_unit_cross_operations!((Rw, rw), (Em, em));
impl_unit_cross_operations!((Rw, rw), (Vp, vp));
impl_unit_cross_operations!((Rw, rw), (Vw, vw));
impl_unit_cross_operations!((Rw, rw), (Vh, vh));

impl_unit_cross_operations!((Rh, rh), (Ab, ab));
impl_unit_cross_operations!((Rh, rh), (Rl, rl));
impl_unit_cross_operations!((Rh, rh), (Rw, rw));
impl_unit_cross_operations!((Rh, rh), (Em, em));
impl_unit_cross_operations!((Rh, rh), (Vp, vp));
impl_unit_cross_operations!((Rh, rh), (Vw, vw));
impl_unit_cross_operations!((Rh, rh), (Vh, vh));

impl_unit_cross_operations!((Em, em), (Ab, ab));
impl_unit_cross_operations!((Em, em), (Rl, rl));
impl_unit_cross_operations!((Em, em), (Rw, rw));
impl_unit_cross_operations!((Em, em), (Rh, rh));
impl_unit_cross_operations!((Em, em), (Vp, vp));
impl_unit_cross_operations!((Em, em), (Vw, vw));
impl_unit_cross_operations!((Em, em), (Vh, vh));

impl_unit_cross_operations!((Vp, vp), (Ab, ab));
impl_unit_cross_operations!((Vp, vp), (Rl, rl));
impl_unit_cross_operations!((Vp, vp), (Rw, rw));
impl_unit_cross_operations!((Vp, vp), (Rh, rh));
impl_unit_cross_operations!((Vp, vp), (Em, em));
impl_unit_cross_operations!((Vp, vp), (Vw, vw));
impl_unit_cross_operations!((Vp, vp), (Vh, vh));

impl_unit_cross_operations!((Vw, vw), (Ab, ab));
impl_unit_cross_operations!((Vw, vw), (Rl, rl));
impl_unit_cross_operations!((Vw, vw), (Rw, rw));
impl_unit_cross_operations!((Vw, vw), (Rh, rh));
impl_unit_cross_operations!((Vw, vw), (Em, em));
impl_unit_cross_operations!((Vw, vw), (Vp, vp));
impl_unit_cross_operations!((Vw, vw), (Vh, vh));

impl_unit_cross_operations!((Vh, vh), (Ab, ab));
impl_unit_cross_operations!((Vh, vh), (Rl, rl));
impl_unit_cross_operations!((Vh, vh), (Rw, rw));
impl_unit_cross_operations!((Vh, vh), (Rh, rh));
impl_unit_cross_operations!((Vh, vh), (Em, em));
impl_unit_cross_operations!((Vh, vh), (Vp, vp));
impl_unit_cross_operations!((Vh, vh), (Vw, vw));


// #================================#
// #=== THE VALUE IMPLEMENTATION ===#

/// Declare [`UiValue`] struct with these fields
macro_rules! init_value {
    ($($struct_field:ident), *) => {
        /// **Ui value** - A collection of different units used for UI.
        /// They are computed at runtime when layout is being calculated (context-aware).
        /// The supported units that implement `Into<UiValue>` are:
        /// * [`Ab`] [`Rl`] [`Rw`] [`Rh`] [`Em`] [`Vw`] [`Vh`]
        /// ## 📦 Types
        /// First class implementations for `(T)` are:
        /// * [`f32`] [`Vec2`] [`Vec3`] [`Vec4`]
        /// ## 🛠️ Example
        /// ```
        /// # use bevy_lunex::*;
        /// # use bevy::prelude::*;
        /// let a: UiValue<f32> = Ab(4.0) + Em(1.0);  // -> 4px + 1em
        /// let b: UiValue<f32> = Ab(40.0) - Rl(5.0); // -> 40px - 5%
        /// let c: UiValue<Vec2> = (Ab(20.0), Em(2.0)).into(); // -> [20px, 2em]
        /// ```
        #[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
        pub struct UiValue<T> {
            $(
                $struct_field: Option<T>,
            )*
        }
        impl <T> UiValue<T> {
            /// Creates new empty [`UiValue`]
            pub const fn new() -> Self {
                UiValue {
                    $(
                        $struct_field: None,
                    )*
                }
            }
        }

        // Implement negation of the same type
        impl <T: Neg<Output = T>> Neg for UiValue<T> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                UiValue {
                    $(
                        $struct_field: if let Some(v) = self.$struct_field { Some(-v) } else { None },
                    )*
                }
            }
        }

        // Implement addition of the same type
        impl <T: Add<Output = T> + Add> Add for UiValue<T> {
            type Output = Self;
            fn add(self, other: Self) -> Self::Output {
                UiValue {
                    $(
                        $struct_field: if let Some(v1) = self.$struct_field {
                            if let Some(v2) = other.$struct_field { Some(v1 + v2) } else { Some(v1) }
                        } else { other.$struct_field },
                    )*
                }
            }
        }
        impl <T: Add<Output = T> + Copy> AddAssign for UiValue<T> {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs
            }
        }

        // Implement subtraction of the same type
        impl <T: Sub<Output = T> + Sub + Neg<Output = T>> Sub for UiValue<T> {
            type Output = Self;
            fn sub(self, other: Self) -> Self::Output {
                UiValue {
                    $(
                        $struct_field: if let Some(v1) = self.$struct_field {
                            if let Some(v2) = other.$struct_field { Some(v1 - v2) } else { Some(v1) }
                        } else { other.$struct_field },
                    )*
                }
            }
        }
        impl <T: Sub<Output = T> + Copy + Neg<Output = T>> SubAssign for UiValue<T> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs
            }
        }

        // Implement multiplication of the same type
        impl <T: Mul<Output = T> + Mul> Mul for UiValue<T> {
            type Output = Self;
            fn mul(self, other: Self) -> Self::Output {
                let mut output = UiValue::new();
                $(
                    if let Some(v1) = self.$struct_field {
                        if let Some(v2) = other.$struct_field {
                            output.$struct_field = Some(v1 * v2);
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

        // Implement multiplication with the f32 type
        impl <T: Mul<f32, Output = T>> Mul<f32> for UiValue<T> {
            type Output = Self;
            fn mul(self, other: f32) -> Self::Output {
                let mut output = UiValue::new();
                $(
                    if let Some(v1) = self.$struct_field {
                        output.$struct_field = Some(v1 * other);
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
    }
}
init_value!(ab, rl, rw, rh, em, vp, vw, vh);

/// Bind these structs to appropriate [`UiValue`] fields and implement math operations
macro_rules! bind_value {
    ($( ($unit:ident, $struct_field:ident) ),* ) => {

        $(
            // Bind conversion of the type to the field
            impl <T> From<$unit<T>> for UiValue<T> {
                fn from(val: $unit<T>) -> UiValue<T> {
                    let mut ret = UiValue::new();
                    ret.$struct_field = Some(val.0);
                    ret
                }
            }

            // Bind addition of the type to the field
            impl <T: Add<Output = T> + Add> Add<$unit<T>> for UiValue<T> {
                type Output = Self;
                fn add(mut self, other: $unit<T>) -> Self::Output {
                    match self.$struct_field {
                        Some(v) => {
                            self.$struct_field = Some(v + other.0);
                            self
                        },
                        None => {
                            self.$struct_field = Some(other.0);
                            self
                        },
                    }
                }
            }
            impl <T: Add<Output = T> + Copy> AddAssign<$unit<T>> for UiValue<T> {
                fn add_assign(&mut self, rhs: $unit<T>) {
                    match self.$struct_field {
                        Some(v) => self.$struct_field = Some(v + rhs.0),
                        None => self.$struct_field = Some(rhs.0),
                    }
                }
            }

            // Bind subtraction of the type to the field
            impl <T: Sub<Output = T> + Sub> Sub<$unit<T>> for UiValue<T> {
                type Output = Self;
                fn sub(mut self, other: $unit<T>) -> Self::Output {
                    match self.$struct_field {
                        Some(v) => {
                            self.$struct_field = Some(v - other.0);
                            self
                        },
                        None => {
                            self.$struct_field = Some(other.0);
                            self
                        },
                    }
                }
            }
            impl <T: Sub<Output = T> + Copy> SubAssign<$unit<T>> for UiValue<T> {
                fn sub_assign(&mut self, rhs: $unit<T>) {
                    match self.$struct_field {
                        Some(v) => self.$struct_field = Some(v - rhs.0),
                        None => self.$struct_field = Some(rhs.0),
                    }
                }
            }

            // Bind multiplication of the type to the field
            impl <T: Mul<Output = T> + Mul> Mul<$unit<T>> for UiValue<T> {
                type Output = Self;
                fn mul(mut self, other: $unit<T>) -> Self::Output {
                    if let Some(v) = self.$struct_field {
                        self.$struct_field = Some(v * other.0);
                    }
                    self
                }
            }
            impl <T: Mul<Output = T> + Copy> MulAssign<$unit<T>> for UiValue<T> {
                fn mul_assign(&mut self, rhs: $unit<T>) {
                    if let Some(v) = self.$struct_field {
                        self.$struct_field = Some(v * rhs.0);
                    }
                }
            }
        )*

        impl UiValue<Vec2> {
            /// Gets the X value of all units.
            pub fn get_x(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$struct_field { out += $unit(v.x) }
                )*
                out
            }
            /// Gets the Y value of all units.
            pub fn get_y(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$struct_field { out += $unit(v.y) }
                )*
                out
            }

            /// Replaces the X value of appropriate units with the new value.
            pub fn with_x(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.x = v2 } else { self.$struct_field = Some(Vec2::new(v2, 0.0)) } }
                )*
                self
            }
            /// Replaces the Y value of appropriate units with the new value.
            pub fn with_y(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.y = v2 } else { self.$struct_field = Some(Vec2::new(0.0, v2)) } }
                )*
                self
            }

            /// Sets the X value of appropriate units with the new value.
            pub fn set_x(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.x = v2 } else { self.$struct_field = Some(Vec2::new(v2, 0.0)) } }
                )*
            }
            /// Sets the Y value of appropriate units with the new value.
            pub fn set_y(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.y = v2 } else { self.$struct_field = Some(Vec2::new(0.0, v2)) } }
                )*
            }

        }
        impl UiValue<Vec3> {
            /// Gets the X value of all units.
            pub fn get_x(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$struct_field { out += $unit(v.x) }
                )*
                out
            }
            /// Gets the Y value of all units.
            pub fn get_y(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$struct_field { out += $unit(v.y) }
                )*
                out
            }
            /// Gets the Z value of all units.
            pub fn get_z(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$struct_field { out += $unit(v.z) }
                )*
                out
            }

            /// Replaces the X value of appropriate units with the new value.
            pub fn with_x(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.x = v2 } else { self.$struct_field = Some(Vec3::new(v2, 0.0, 0.0)) } }
                )*
                self
            }
            /// Replaces the Y value of appropriate units with the new value.
            pub fn with_y(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.y = v2 } else { self.$struct_field = Some(Vec3::new(0.0, v2, 0.0)) } }
                )*
                self
            }
            /// Replaces the Z value of appropriate units with the new value.
            pub fn with_z(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.z = v2 } else { self.$struct_field = Some(Vec3::new(0.0, 0.0, v2)) } }
                )*
                self
            }

            /// Sets the X value of appropriate units with the new value.
            pub fn set_x(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.x = v2 } else { self.$struct_field = Some(Vec3::new(v2, 0.0, 0.0)) } }
                )*
            }
            /// Sets the Y value of appropriate units with the new value.
            pub fn set_y(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.y = v2 } else { self.$struct_field = Some(Vec3::new(0.0, v2, 0.0)) } }
                )*
            }
            /// Sets the Z value of appropriate units with the new value.
            pub fn set_z(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.z = v2 } else { self.$struct_field = Some(Vec3::new(0.0, 0.0, v2)) } }
                )*
            }
        }
        impl UiValue<Vec4> {
            /// Gets the X value of all units.
            pub fn get_x(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$struct_field { out += $unit(v.x) }
                )*
                out
            }
            /// Gets the Y value of all units.
            pub fn get_y(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$struct_field { out += $unit(v.y) }
                )*
                out
            }
            /// Gets the Z value of all units.
            pub fn get_z(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$struct_field { out += $unit(v.z) }
                )*
                out
            }
            /// Gets the W value of all units.
            pub fn get_w(&self) -> UiValue<f32> {
                let mut out = UiValue::<f32>::new();
                $(
                    if let Some(v) = self.$struct_field { out += $unit(v.w) }
                )*
                out
            }

            /// Replaces the X value of appropriate units with the new value.
            pub fn with_x(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.x = v2 } else { self.$struct_field = Some(Vec4::new(v2, 0.0, 0.0, 0.0)) } }
                )*
                self
            }
            /// Replaces the Y value of appropriate units with the new value.
            pub fn with_y(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.y = v2 } else { self.$struct_field = Some(Vec4::new(0.0, v2, 0.0, 0.0)) } }
                )*
                self
            }
            /// Replaces the Z value of appropriate units with the new value.
            pub fn with_z(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.z = v2 } else { self.$struct_field = Some(Vec4::new(0.0, 0.0, v2, 0.0)) } }
                )*
                self
            }
            /// Replaces the W value of appropriate units with the new value.
            pub fn with_w(mut self, other: impl Into<UiValue<f32>>) -> Self {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.w = v2 } else { self.$struct_field = Some(Vec4::new(0.0, 0.0, 0.0, v2)) } }
                )*
                self
            }

            /// Sets the X value of appropriate units with the new value.
            pub fn set_x(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.x = v2 } else { self.$struct_field = Some(Vec4::new(v2, 0.0, 0.0, 0.0)) } }
                )*
            }
            /// Sets the Y value of appropriate units with the new value.
            pub fn set_y(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.y = v2 } else { self.$struct_field = Some(Vec4::new(0.0, v2, 0.0, 0.0)) } }
                )*
            }
            /// Sets the Z value of appropriate units with the new value.
            pub fn set_z(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.z = v2 } else { self.$struct_field = Some(Vec4::new(0.0, 0.0, v2, 0.0)) } }
                )*
            }
            /// Sets the W value of appropriate units with the new value.
            pub fn set_w(&mut self, other: impl Into<UiValue<f32>>) {
                let other = other.into();
                $(
                    if let Some(v2) = other.$struct_field { if let Some(v1) = &mut self.$struct_field { v1.w = v2 } else { self.$struct_field = Some(Vec4::new(0.0, 0.0, 0.0, v2)) } }
                )*
            }
        }
    }
}
bind_value!((Ab, ab), (Rl, rl), (Rw, rw), (Rh, rh), (Em, em), (Vp, vp), (Vw, vw), (Vh, vh));

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

/// Trait for implementing evaluation logic for `(T)`.
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
        if let Some(v) = self.vw { out += (v/100.0) * viewport_size }
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
        if let Some(v) = self.vp { out += (v/100.0) * viewport_size }
        if let Some(v) = self.vw { out += (v/100.0) * viewport_size.x }
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
        if let Some(v) = self.vp { out += (v/100.0) * viewport_size }
        if let Some(v) = self.vw { out += (v/100.0) * viewport_size.x }
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
        if let Some(v) = self.vp { out += (v/100.0) * viewport_size }
        if let Some(v) = self.vw { out += (v/100.0) * viewport_size.x }
        if let Some(v) = self.vh { out += (v/100.0) * viewport_size.y }
        out
    }
}


/// **Nice display** - Special trait for formatted console debug output with colors.
pub trait NiceDisplay {
    /// Output the data in a formatted string using the `colorise` crate.
    fn to_nicestr(&self) -> String;
}

impl NiceDisplay for Anchor {
    fn to_nicestr(&self) -> String {
        match self {
            Anchor::Center => "Center".green().to_string(),
            Anchor::BottomLeft => "BottomLeft".green().to_string(),
            Anchor::BottomCenter => "BottomCenter".green().to_string(),
            Anchor::BottomRight => "BottomRight".green().to_string(),
            Anchor::CenterLeft => "CenterLeft".green().to_string(),
            Anchor::CenterRight => "CenterRight".green().to_string(),
            Anchor::TopLeft => "TopLeft".green().to_string(),
            Anchor::TopCenter => "TopCenter".green().to_string(),
            Anchor::TopRight => "TopRight".green().to_string(),
            Anchor::Custom(point) => format!("({}, {})", format!("{}", point.x).green(), format!("{}", point.y).green()),
        }
    }
}

impl NiceDisplay for UiValue<f32> {
    fn to_nicestr(&self) -> String {
        let mut t = String::new();
        if let Some(v) = self.ab {
            if v != 0.0 {
                t = format!("{}", format!("{v:.00}").bright_blue());
            }
        }
        if let Some(v) = self.rl {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, format!("{v:.00}").bright_green(), "%".bright_green());
            }
        }
        if let Some(v) = self.rw {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, format!("{v:.00}").bright_green(), "%w".bright_green());
            }
        }
        if let Some(v) = self.rh {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, format!("{v:.00}").bright_green(), "%h".bright_green());
            }
        }
        if let Some(v) = self.em {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, format!("{v:.00}").bright_red(), "m".bright_red());
            }
        }
        if let Some(v) = self.vp {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, format!("{v:.00}").bright_green(), "v%".bright_green());
            }
        }
        if let Some(v) = self.vw {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, format!("{v:.00}").bright_green(), "v%w".bright_green());
            }
        }
        if let Some(v) = self.vh {
            if v != 0.0 {
                if !t.is_empty() { t += " + " };
                t = format!("{}{}{}", t, format!("{v:.00}").bright_green(), "v%h".bright_green());
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
                tx = format!("{}", format!("{:.00}", v.x).bright_blue());
            }
            if v.y != 0.0 {
                ty = format!("{}", format!("{:.00}", v.y).bright_blue());
            }
        }
        if let Some(v) = self.rl {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_green(), "%".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_green(), "%".bright_green());
            }
        }
        if let Some(v) = self.rw {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_green(), "%w".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_green(), "%w".bright_green());
            }
        }
        if let Some(v) = self.rh {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_green(), "%h".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_green(), "%h".bright_green());
            }
        }
        if let Some(v) = self.em {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_red(), "m".bright_red());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_red(), "m".bright_red());
            }
        }
        if let Some(v) = self.vp {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_green(), "v%".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_green(), "v%".bright_green());
            }
        }
        if let Some(v) = self.vw {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_green(), "v%w".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_green(), "v%w".bright_green());
            }
        }
        if let Some(v) = self.vh {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_green(), "v%h".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_green(), "v%h".bright_green());
            }
        }
        if tx.is_empty() { tx = format!("{}", "0".bright_blue()); };
        if ty.is_empty() { ty = format!("{}", "0".bright_blue()); };
        format!("({tx}, {ty})")
    }
}
impl NiceDisplay for UiValue<Vec3> {
    fn to_nicestr(&self) -> String {
        let mut tx = String::new();
        let mut ty = String::new();
        let mut tz = String::new();
        if let Some(v) = self.ab {
            if v.x != 0.0 {
                tx = format!("{}", format!("{:.00}", v.x).bright_blue());
            }
            if v.y != 0.0 {
                ty = format!("{}", format!("{:.00}", v.y).bright_blue());
            }
            if v.z != 0.0 {
                tz = format!("{}", format!("{:.00}", v.z).bright_blue());
            }
        }
        if let Some(v) = self.rl {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_green(), "%".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_green(), "%".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, format!("{:.00}", v.z).bright_green(), "%".bright_green());
            }
        }
        if let Some(v) = self.rw {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_green(), "%w".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_green(), "%w".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, format!("{:.00}", v.z).bright_green(), "%w".bright_green());
            }
        }
        if let Some(v) = self.rh {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_green(), "%h".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_green(), "%h".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, format!("{:.00}", v.z).bright_green(), "%h".bright_green());
            }
        }
        if let Some(v) = self.em {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_red(), "m".bright_red());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_red(), "m".bright_red());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, format!("{:.00}", v.z).bright_red(), "m".bright_red());
            }
        }
        if let Some(v) = self.vp {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_green(), "v%".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_green(), "v%".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, format!("{:.00}", v.z).bright_green(), "v%".bright_green());
            }
        }
        if let Some(v) = self.vw {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_green(), "v%w".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_green(), "v%w".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, format!("{:.00}", v.z).bright_green(), "v%w".bright_green());
            }
        }
        if let Some(v) = self.vh {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_green(), "v%h".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_green(), "v%h".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, format!("{:.00}", v.z).bright_green(), "v%h".bright_green());
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
                tx = format!("{}", format!("{:.00}", v.x).bright_blue());
            }
            if v.y != 0.0 {
                ty = format!("{}", format!("{:.00}", v.y).bright_blue());
            }
            if v.z != 0.0 {
                tz = format!("{}", format!("{:.00}", v.z).bright_blue());
            }
            if v.w != 0.0 {
                tw = format!("{}", format!("{:.00}", v.w).bright_blue());
            }
        }
        if let Some(v) = self.rl {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_green(), "%".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_green(), "%".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, format!("{:.00}", v.z).bright_green(), "%".bright_green());
            }
            if v.w != 0.0 {
                if !tw.is_empty() { tw += " + " };
                tw = format!("{}{}{}", tw, format!("{:.00}", v.w).bright_green(), "%".bright_green());
            }
        }
        if let Some(v) = self.rw {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_green(), "%w".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_green(), "%w".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, format!("{:.00}", v.z).bright_green(), "%w".bright_green());
            }
            if v.w != 0.0 {
                if !tw.is_empty() { tw += " + " };
                tw = format!("{}{}{}", tw, format!("{:.00}", v.w).bright_green(), "%w".bright_green());
            }
        }
        if let Some(v) = self.rh {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_green(), "%h".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_green(), "%h".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, format!("{:.00}", v.z).bright_green(), "%h".bright_green());
            }
            if v.w != 0.0 {
                if !tw.is_empty() { tw += " + " };
                tw = format!("{}{}{}", tw, format!("{:.00}", v.w).bright_green(), "%h".bright_green());
            }
        }
        if let Some(v) = self.em {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_red(), "m".bright_red());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_red(), "m".bright_red());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, format!("{:.00}", v.z).bright_red(), "m".bright_red());
            }
            if v.w != 0.0 {
                if !tw.is_empty() { tw += " + " };
                tw = format!("{}{}{}", tw, format!("{:.00}", v.w).bright_red(), "m".bright_red());
            }
        }
        if let Some(v) = self.vp {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_green(), "v%".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_green(), "v%".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, format!("{:.00}", v.z).bright_green(), "v%".bright_green());
            }
            if v.w != 0.0 {
                if !tw.is_empty() { tw += " + " };
                tw = format!("{}{}{}", tw, format!("{:.00}", v.w).bright_green(), "v%".bright_green());
            }
        }
        if let Some(v) = self.vw {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_green(), "v%w".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_green(), "v%w".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, format!("{:.00}", v.z).bright_green(), "v%w".bright_green());
            }
            if v.w != 0.0 {
                if !tw.is_empty() { tw += " + " };
                tw = format!("{}{}{}", tw, format!("{:.00}", v.w).bright_green(), "v%w".bright_green());
            }
        }
        if let Some(v) = self.vh {
            if v.x != 0.0 {
                if !tx.is_empty() { tx += " + " };
                tx = format!("{}{}{}", tx, format!("{:.00}", v.x).bright_green(), "v%h".bright_green());
            }
            if v.y != 0.0 {
                if !ty.is_empty() { ty += " + " };
                ty = format!("{}{}{}", ty, format!("{:.00}", v.y).bright_green(), "v%h".bright_green());
            }
            if v.z != 0.0 {
                if !tz.is_empty() { tz += " + " };
                tz = format!("{}{}{}", tz, format!("{:.00}", v.z).bright_green(), "v%h".bright_green());
            }
            if v.w != 0.0 {
                if !tw.is_empty() { tw += " + " };
                tw = format!("{}{}{}", tw, format!("{:.00}", v.w).bright_green(), "v%h".bright_green());
            }
        }
        if tx.is_empty() { tx = format!("{}", "0".bright_blue()); };
        if ty.is_empty() { ty = format!("{}", "0".bright_blue()); };
        if tz.is_empty() { tz = format!("{}", "0".bright_blue()); };
        if tw.is_empty() { tw = format!("{}", "0".bright_blue()); };
        format!("x: {}, y: {} z:{} w:{}", tx.black(), ty.black(), tz.black(), tw.black())
    }
}
