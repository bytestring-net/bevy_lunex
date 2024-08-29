use crate::import::*;
use crate::UiValue;


// #========================#
// #=== STACK PROPERTIES ===#

/// **Stack direction** - A type used to define in which direction should the subnodes be placed.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum StackDirection {
    #[default]
    Horizontal,
    Vertical,
}


/// **Stack margin** - A special type to define margin subnodes should inherit. Contains a set of presets.
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::StackMargin;
/// let margin = StackMargin::Start;  // -> Default, does nothing
/// let margin = StackMargin::Center; // -> Subnodes on sides will inherit 1sp on sides facing out
/// let margin = StackMargin::End;    // -> First subnode will inherit 1sp on left side
/// let margin = StackMargin::Between;// -> All subnodes except 1st will inherit 1sp on left side
/// let margin = StackMargin::Evenly; // -> All subnodes will inherit 1sp on both sides except 1st
/// let margin = StackMargin::Around; // -> All subnodes will inherit 1sp on both sides
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
pub enum StackMargin {
    /// Default, does nothing.
    #[default]
    Start,
    /// Subnodes on sides will inherit 1sp on sides facing out.
    Center,
    /// First subnode will inherit 1sp on left side.
    End,
    /// All subnodes except 1st will inherit 1sp on left side.
    Between,
    /// All subnodes will inherit 1sp on both sides except 1st (only left).
    Evenly,
    /// All subnodes will inherit 1sp on both sides.
    Around,
    /// Manually set margin for all subnodes.
    Manual(Box<UiValue<Vec4>>),
}


// #=================#
// #=== THE STACK ===#

/// **Ui stack** - A type used to define how should subnodes be placed within this node.
/// ## 🛠️ Example
/// ```
/// # use lunex_engine::UiStack;
/// let stack: UiStack = UiStack::new().flipped(true);
/// ```
#[derive(Debug, Default, Clone, PartialEq, Component)]
pub struct UiStack {
    /// Populating direction
    pub direction: StackDirection,
    /// Populating direction (flip around y axis) - `top-to-bottom` vs `bottom-to-top`.
    pub flipped: bool,
    /// Populating direction (flip around x axis) - `left-to-right` vs `right-to-left`.
    pub inverted: bool,
    /// Minimal gap between subnodes.
    pub gap: UiValue<Vec2>,
    /// The margin that subnodes should inherit.
    pub margin: StackMargin,
}
impl UiStack {
    /// Creates new empty Stack.
    pub fn new() -> Self {
        Default::default()
    }
    /// Replaces the direction with a new value.
    pub fn direction(mut self, direction: StackDirection) -> Self {
        self.direction = direction;
        self
    }
    /// Replaces the flipped value with a new value.
    pub fn flipped(mut self, value: bool) -> Self {
        self.flipped = value;
        self
    }
    /// Replaces the inversion value with a new value.
    pub fn inverted(mut self, value: bool) -> Self {
        self.inverted = value;
        self
    }
    /// Replaces the gap with a new value.
    pub fn gap(mut self, gap: impl Into<UiValue<Vec2>>) -> Self {
        self.gap = gap.into();
        self
    }
    /// Replaces the horizontal gap with a new value.
    pub fn gap_x(mut self, gap: impl Into<UiValue<f32>>) -> Self {
        self.gap.set_x(gap);
        self
    }
    /// Replaces the vertical gap with a new value.
    pub fn gap_y(mut self, gap: impl Into<UiValue<f32>>) -> Self {
        self.gap.set_y(gap);
        self
    }
    /// Replaces the margin with a new value.
    pub fn margin(mut self, margin: StackMargin) -> Self {
        self.margin = margin;
        self
    }
    /// Sets the direction to a new value.
    pub fn set_direction(&mut self, direction: StackDirection) {
        self.direction = direction;
    }
    /// Sets the flipped value to a new value.
    pub fn set_flipped(&mut self, value: bool) {
        self.flipped = value;
    }
    /// Sets the inversion value with a new value.
    pub fn set_inverted(&mut self, value: bool) {
        self.inverted = value;
    }
    /// Sets the gap to a new value.
    pub fn set_gap(&mut self, gap: impl Into<UiValue<Vec2>>) {
        self.gap = gap.into();
    }
    /// Sets the horizontal gap to a new value.
    pub fn set_gap_x(&mut self, gap: impl Into<UiValue<f32>>) {
        self.gap.set_x(gap);
    }
    /// Sets the vertical gap to a new value.
    pub fn set_gap_y(&mut self, gap: impl Into<UiValue<f32>>) {
        self.gap.set_y(gap);
    }
    /// Sets the margin to a new value.
    pub fn set_margin(&mut self, margin: StackMargin) {
        self.margin = margin;
    }
}