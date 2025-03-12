![Logo](assets/promo/bevy_lunex.png)

Blazingly fast retained ***layout engine*** for Bevy entities, built around vanilla **Bevy ECS**. It gives you the ability to make ***your own custom UI*** using regular ECS like every other part of your app.

> [!IMPORTANT]
> This book is made for version `0.3.X` of `Bevy_Lunex`

> [!NOTE]
> This crate is being maintained by a university student. Don't expect updates during the semester.

> [!WARNING]
> This crate is opinionated and thus you must decide if it is a good fit for what you want to achieve.
>
> This is mainly because Lunex provides you with only capability to position entities, leaving everything
> else in your hands. The current version also lacks any kind of flexbox-like layout.
>
> #### Good fit 👍
> - Worldspace 3D UI
> - Spritebased 2D UI
> - Custom rendering hook
> - Very customizable
> - Low-level interactivity
>
> #### Not so good 👎
> - Development speed & iteration
> - Using prebuilt input components
> - Making desktop application UI
