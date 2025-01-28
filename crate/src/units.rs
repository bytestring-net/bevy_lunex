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
/// # use crate::Ab;
/// let a: Ab<f32> = Ab(4.0) + Ab(6.0); // -> 10px
/// let b: Ab<f32> = Ab(4.0) * 2.0;     // -> 8px
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Ab<T>(pub T);

/// **Relative** - Represents scalable unit `0% to 100%`. `120%` is allowed.
/// ## 🛠️ Example
/// ```
/// # use crate::Rl;
/// let a: Rl<f32> = Rl(25.0) + Rl(40.0); // -> 65%
/// let b: Rl<f32> = Rl(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Rl<T>(pub T);

/// **Relative width** - Represents scalable unit `0% to 100%`. `120%` is allowed.
/// Proportional to a width measure even when used in a height field.
/// ## 🛠️ Example
/// ```
/// # use crate::Rw;
/// let a: Rw<f32> = Rw(25.0) + Rw(40.0); // -> 65%
/// let b: Rw<f32> = Rw(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Rw<T>(pub T);

/// **Relative height** - Represents scalable unit `0% to 100%`. `120%` is allowed.
/// Proportional to a height measure even when used in a width field.
/// ## 🛠️ Example
/// ```
/// # use crate::Rh;
/// let a: Rh<f32> = Rh(25.0) + Rh(40.0); // -> 65%
/// let b: Rh<f32> = Rh(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Rh<T>(pub T);

/// **Size of M** - Represents unit that is the size of the symbol `M`. Which is `16px` with `font size 16px` and so on.
/// ## 🛠️ Example
/// ```
/// # use crate::Em;
/// let a: Em<f32> = Em(1.0) + Em(2.0); // -> 3em == 48px with font size 16px
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Em<T>(pub T);

/// **Viewport** - Represents scalable unit `0% to 100%` of the root container. `120%` is allowed.
/// ## 🛠️ Example
/// ```
/// # use crate::Vp;
/// let a: Vp<f32> = Vp(25.0) + Vp(40.0); // -> 65%
/// let b: Vp<f32> = Vp(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Vp<T>(pub T);

/// **Viewport width** - Represents scalable unit `0% to 100%` of the root container. `120%` is allowed.
/// Proportional to a width measure even when used in a height field.
/// ## 🛠️ Example
/// ```
/// # use crate::Vw;
/// let a: Vw<f32> = Vw(25.0) + Vw(40.0); // -> 65%
/// let b: Vw<f32> = Vw(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Vw<T>(pub T);

/// **Viewport Height** - Represents scalable unit `0% to 100%` of the root container. `120%` is allowed.
/// Proportional to a height measure even when used in a width field.
/// ## 🛠️ Example
/// ```
/// # use crate::Vh;
/// let a: Vh<f32> = Vh(25.0) + Vh(40.0); // -> 65%
/// let b: Vh<f32> = Vh(25.0) * 3.0;      // -> 75%
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Reflect)]
pub struct Vh<T>(pub T);


/// Implement basic math and conversions for a type
macro_rules! init_uiunit {
    ($($unit:ident), *) => {
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
init_uiunit!(Ab, Rl, Rw, Rh, Em, Vp, Vw, Vh);


// #==================================#
// #=== THE UIVALUE IMPLEMENTATION ===#

/// Declare [`UiValue`] struct with these fields
macro_rules! init_uivalue {
    ($($struct_field:ident), *) => {
        /// **Ui value** - A collection of different units used for UI.
        /// They are computed at runtime when layout is being calculated (context-aware).
        /// The supported units that implement `Into<UiValue>` are:
        /// * [`Ab`] [`Rl`] [`Rw`] [`Rh`] [`Em`] [`Sp`] [`Vw`] [`Vh`]
        /// ## 📦 Types
        /// First class implementations for `(T)` are:
        /// * [`f32`] [`Vec2`] [`Vec3`] [`Vec4`]
        /// ## 🛠️ Example
        /// ```
        /// # use lunex_engine::{UiValue, Ab, Em, Rl, Sp};
        /// # use bevy::prelude::Vec2;
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
init_uivalue!(ab, rl, rw, rh, em, vp, vw, vh);

/// Bind these structs to appropriate [`UiValue`] fields and implement math operations
macro_rules! bind_uivalue {
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
bind_uivalue!((Ab, ab), (Rl, rl), (Rw, rw), (Rh, rh), (Em, em), (Vp, vp), (Vw, vw), (Vh, vh));

/// Implement basic math and conversions for a type
macro_rules! impl_uiunit {
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
impl_uiunit!(Ab, Rl, Rw, Rh, Em, Vp, Vw, Vh);


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
