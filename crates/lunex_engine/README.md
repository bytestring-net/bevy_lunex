# Lunex engine

> [!CAUTION]
> This crate is not inteded to be used standalone. It is a dependency of `Bevy_Lunex`.

This crate contains ***Bevy agnostic code***, that is dependant only on Bevy re-exports of `Glam` and `Thiserror`. The other dependency is `Colored` for nice formating of terminal output and `IndexMap` for consistently iterable hashmaps.

Even though the code is not coupled with Bevy's internals, it is still ***assuming you will use Bevy***. There are `Component` derives present purely for the purpuse to be used in `Bevy_Lunex` up the stack. The naming of the structs and docs are also targetted for Bevy.

But if you really need to use this crate outside of Bevy ecosystem, it is very easy to do so.