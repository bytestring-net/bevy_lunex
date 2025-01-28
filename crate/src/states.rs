use crate::*;


// #===============================#
// #=== THE SPECIAL STATE LOGIC ===#

#[derive(Component, Clone, PartialEq, Debug)]
pub struct UiHover {
    value: f32,
    /// If the state is enabled
    pub enable: bool,
    /// The function to smooth the transition
    pub curve: fn(f32) -> f32,
    /// The speed of transition forwards
    pub forward_speed: f32,
    /// The speed of transition backwards
    pub backward_speed: f32,
}
impl UiHover {
    /// Create new instance
    pub fn new() -> Self {
        Self {
            value: 0.0,
            enable: false,
            curve: |v| {v},
            forward_speed: 1.0,
            backward_speed: 1.0,
        }
    }
    /// Replaces the curve function.
    pub fn curve(mut self, curve: fn(f32) -> f32) -> Self {
        self.curve = curve;
        self
    }
    /// Replaces the speed with a new value.
    pub fn forward_speed(mut self, forward_speed: f32) -> Self {
        self.forward_speed = forward_speed;
        self
    }
    /// Replaces the speed with a new value.
    pub fn backward_speed(mut self, backward_speed: f32) -> Self {
        self.backward_speed = backward_speed;
        self
    }
}
impl UiStateTrait for UiHover {
    fn value(&self) -> f32 {
        (self.curve)(self.value)
    }
}

#[derive(Component, Deref, DerefMut, Clone, PartialEq, Debug)]
pub struct UiSelected(pub f32);
impl UiStateTrait for UiSelected {
    fn value(&self) -> f32 {
        self.0
    }
}

#[derive(Component, Deref, DerefMut, Clone, PartialEq, Debug)]
pub struct UiClicked(pub f32);
impl UiStateTrait for UiClicked {
    fn value(&self) -> f32 {
        self.0
    }
}

#[derive(Component, Deref, DerefMut, Clone, PartialEq, Debug)]
pub struct UiIntro(pub f32);
impl UiStateTrait for UiIntro {
    fn value(&self) -> f32 {
        self.0
    }
}

#[derive(Component, Deref, DerefMut, Clone, PartialEq, Debug)]
pub struct UiOutro(pub f32);
impl UiStateTrait for UiOutro {
    fn value(&self) -> f32 {
        self.0
    }
}


pub fn update_state(
    time: Res<Time>,
    mut query: Query<&mut UiHover>,
) {
    for mut hover in &mut query {
        if hover.enable == true && hover.value < 1.0 {
            hover.value = (hover.value + hover.forward_speed * time.delta_secs()).min(1.0);
        }
        if hover.enable == false && hover.value > 0.0 {
            hover.value = (hover.value - hover.backward_speed * time.delta_secs()).max(0.0);
        }
    }
}


