use crate::*;


// #=======================#
// #=== THE HOVER STATE ===#

/// **Ui Hover** - A built in state that should be triggered manually when a pointer hovers over a Ui-Node.
/// This state first **needs to be enabled** for the entity by adding it as a component.
/// 
/// Then you can use the [`Self::id`] function to identify this state inside components
/// that allow you to specify per state properties like [`Uilayout`].
/// 
/// ```
///      UiLayout::new(vec![
///          (UiBase::id(), UiLayout::window().full()),
///          (UiHover::id(), UiLayout::window().x(Rl(10.0)).full())
///      ]),
/// ```
/// 
/// For more information check the documentation on [`UiState`].
///
/// ## 🛠️ Example
/// ```
/// # use bevy::prelude::*;
/// # use bevy_lunex::*;
/// # fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
/// # commands.spawn((
/// #     UiLayoutRoot,
/// # )).with_children(|ui| {
///       ui.spawn((
///           // ... Layout, Color, etc.
///           UiHover::new().forward_speed(20.0).backward_speed(4.0),   // Enable the state
///
///       // Add the observers
///       )).observe(hover_set::<Pointer<Over>, true>)
///         .observe(hover_set::<Pointer<Out>, false>);
/// # });
/// # }
/// ```
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
/// Method implementations
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
/// State implementation
impl UiStateTrait for UiHover {
    fn value(&self) -> f32 {
        (self.curve)(self.value)
    }
}

/// This system updates the hover transition value over time
pub fn system_state_hover_update(
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

/// Event that enables the hover transition
#[derive(Event, Clone, Copy)]
pub struct UiHoverSet(pub bool);

/// This observer enables the hover transition on trigger
fn observer_state_hover_set(
    trigger: Trigger<UiHoverSet>,
    mut query: Query<&mut UiHover>,
) {
    if let Ok(mut hover) = query.get_mut(trigger.entity()) {
        hover.enable = trigger.0;
    }
}

/// Utility observer that triggers the [`UiHoverSet`] event on triggered event.
pub fn hover_set<E: Event, const BOOL: bool>(trigger: Trigger<E>, mut commands: Commands) {
    commands.trigger_targets(UiHoverSet(BOOL), trigger.entity());
}


// #==========================#
// #=== THE SELECTED STATE ===#

/// # WORK IN PROGRESS
#[derive(Component, Deref, DerefMut, Clone, PartialEq, Debug)]
pub struct UiSelected(pub f32);
impl UiStateTrait for UiSelected {
    fn value(&self) -> f32 {
        self.0
    }
}


// #=========================#
// #=== THE CLICKED STATE ===#

/// # WORK IN PROGRESS
#[derive(Component, Deref, DerefMut, Clone, PartialEq, Debug)]
pub struct UiClicked(pub f32);
impl UiStateTrait for UiClicked {
    fn value(&self) -> f32 {
        self.0
    }
}


// #=======================#
// #=== THE INTRO STATE ===#

/// # WORK IN PROGRESS
#[derive(Component, Deref, DerefMut, Clone, PartialEq, Debug)]
pub struct UiIntro(pub f32);
impl UiStateTrait for UiIntro {
    fn value(&self) -> f32 {
        self.0
    }
}


// #=======================#
// #=== THE OUTRO STATE ===#

/// # WORK IN PROGRESS
#[derive(Component, Deref, DerefMut, Clone, PartialEq, Debug)]
pub struct UiOutro(pub f32);
impl UiStateTrait for UiOutro {
    fn value(&self) -> f32 {
        self.0
    }
}




// #========================#
// #=== THE STATE PLUGIN ===#

/// This observer will listen for said event and duplicate it to it's children
fn observer_event_duplicator<E: Event + Copy>(trigger: Trigger<E>, mut commands: Commands, mut query: Query<&Children>) {
    if let Ok(children) = query.get_mut(trigger.entity()) {
        let targets: Vec<Entity> = children.into_iter().map(|e| *e).collect();
        commands.trigger_targets(*trigger.event(), targets);
    }
}

/// This plugin is used for the main logic.
pub struct UiLunexStatePlugin;
impl Plugin for UiLunexStatePlugin {
    fn build(&self, app: &mut App) {

        // Add observers
        app.add_observer(observer_state_hover_set);

        // Add event child duplication
        app.add_observer(observer_event_duplicator::<UiHoverSet>);

        // PRE-COMPUTE SYSTEMS
        app.add_systems(Update, (

            system_state_hover_update,
            system_state_pipe_into_manager::<UiHover>,
            system_state_pipe_into_manager::<UiSelected>,
            system_state_pipe_into_manager::<UiClicked>,
            system_state_pipe_into_manager::<UiIntro>,
            system_state_pipe_into_manager::<UiOutro>,

        ).in_set(UiSystems::PreCompute));
    }
}