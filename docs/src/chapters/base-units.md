# Base Units

Lunex features 9 different UI units, which are used as arguments for `UiValue<T>`. The `T` is expected to be `f32`, `Vec2`, `Vec3` or `Vec4`. They are used in layout functions where `impl Into<UiValue<T>>` is specified as argument.

* `Ab` - Stands for absolute, usually `Ab(1)` = **1px**
* `Rl` - Stands for relative, it means `Rl(1.0)` == **1%**
* `Rw` - Stands for relative width, it means `Rw(1.0)` == **1%w**, but when used in *height* field, it will use *width* as source
* `Rh` - Stands for relative height, it means `Rh(1.0)` == **1%h**, but when used in *width* field, it will use *height* as source
* `Em` - Stands for size of symbol M, it means `Em(1.0)` == **1em**, so size **16px** if font size is **16px**
* `Vp` - Stands for viewport, it means `Vp(1.0)` == **1v%** of the `UiTree` original size
* `Vw` - Stands for viewport width, it means `Vw(1.0)` == **1v%w** of the `UiTree` original size, but when used in *height* field, it will use *width* as source
* `Vh` - Stands for viewport height, it means `Vh(1.0)` == **1v%h** of the `UiTree` original size, but when used in *width* field, it will use *height* as source

## Basic Operations

All unit types implement basic mathematical operations:

```rust, noplayground
let a: Ab<f32> = Ab(4.0) + Ab(6.0); // -> 10px
let b: Ab<f32> = Ab(4.0) * 2.0;     // -> 8px
```

You can also combine different unit types:

```rust, noplayground
let a: UiValue<f32> = Ab(4.0) + Rl(6.0); // -> 4px + 6%
```

If a unit is unspecified, the `f32` value is considered to be in `Ab` unit:

```rust, noplayground
let a: Ab<f32> = 5.0.into(); // -> 5px
```

## Vector Definitions

You can easily define vectors using these units:

```rust, noplayground
let a: UiValue<Vec2> = Ab(10.0).into();             // -> [10px, 10px]
let b: UiValue<Vec2> = Ab((10.0, 15.0)).into();     // -> [10px, 15px]
let c: UiValue<Vec2> = (Ab(10.0), Rl(5.0)).into();  // -> [10px, 5%]
```

Works for larger vectors like `Vec3` and `Vec4` the same.
> [!TIP]
> If you put them as arguments to `impl Into<UiValue<T>>`, you don't have to call `.into()`.