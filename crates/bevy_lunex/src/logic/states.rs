use crate::*;


// #==============#
// #=== EVENTS ===#

/// This event will override hover transition state of targetted entity
#[derive(Event, PartialEq, Clone, Copy)]
pub struct SetUiStateTransition<S: UiState> {
    pub target: Entity,
    pub transition: f32,
    phantom: PhantomData<S>,
}
impl <S: UiState> SetUiStateTransition<S> {
    pub fn new(target: Entity, transition: f32) -> Self {
        Self { target, transition, phantom: PhantomData }
    }
}
fn set_ui_state_transition<S: UiState>(mut events: EventReader<SetUiStateTransition<S>>, mut query: Query<&mut UiAnimator<S>>) {
    for event in events.read() {
        if let Ok(mut hover) = query.get_mut(event.target) {
            if hover.animation_transition != event.transition {
                hover.animation_transition = event.transition
            }
        }
    }
}


// #=====================#
// #=== STATE STRUCTS ===#

/// Control struct for the button state
#[derive(Component, Debug, Clone, PartialEq)]
pub struct UiAnimator<S: UiState> {
    marker: PhantomData<S>,
    /// -1.0 backwards, 1.0 forward
    pub (crate) animation_direction: f32,
    /// Range from `0.0` to `1.0`
    pub (crate) animation_transition: f32,
    /// Setting this to true will disable logic with intention that something else will pipe the control data instead
    pub receiver: bool,
    /// Hover animation speed when transitioning to state
    pub animation_speed_forward: f32,
    /// Hover animation speed when transitioning back to default
    pub animation_speed_backward: f32,
}
impl <S: UiState> UiAnimator<S> {
    /// Creates new struct
    pub fn new() -> Self {
        UiAnimator {
            marker: PhantomData,
            animation_direction: 0.0,
            animation_transition: 0.0,
            receiver: false,
            animation_speed_backward: 8.0,
            animation_speed_forward: 8.0,
        }
    }
    /// Marks this hover as receiver
    pub fn receiver(mut self, receiver: bool) -> Self {
        self.receiver = receiver;
        self
    }
    /// Replaces the forward_speed with a new value.
    pub fn forward_speed(mut self, speed: f32) -> Self {
        self.animation_speed_forward = speed;
        self
    }
    /// Replaces the backward_speed with a new value.
    pub fn backward_speed(mut self, speed: f32) -> Self {
        self.animation_speed_backward = speed;
        self
    }
    /// Checks if animation is moving forward
    pub fn is_forward(&self) -> bool {
        self.animation_direction == 1.0
    }
}
fn ui_animation<S: UiState>(time: Res<Time>, mut query: Query<&mut UiAnimator<S>>) {
    for mut control in &mut query {
        if control.receiver { continue }
        if !((control.animation_transition == 0.0 && control.animation_direction.is_sign_negative()) && (control.animation_transition == 1.0 && control.animation_direction.is_sign_positive())) {
            control.animation_transition += time.delta_seconds() * control.animation_direction * if control.animation_direction == 1.0 { control.animation_speed_forward } else { control.animation_speed_backward };
            control.animation_transition = control.animation_transition.clamp(0.0, 1.0);
        }
    }
}
fn ui_animation_state<S: UiState>(mut query: Query<(&UiAnimator<S>, &mut UiLayoutController), Changed<UiAnimator<S>>>) {
    for (animator, mut controller) in &mut query {
        controller.index[1] = Hover::INDEX;
        controller.tween = animator.animation_transition;
    }
}


/// This struct synchronizes different entities hover state.
/// It takes corresponding [`Hover`] and pipes it into specified entities.
#[derive(Component, Clone, PartialEq)]
pub struct UiAnimatorPipe<S: UiState> {
    /// All entities to to pipe hover state control data to
    pub entity: Vec<Entity>,
    marker: PhantomData<S>
}
impl <S: UiState> UiAnimatorPipe<S> {
    /// Creates new struct
    pub fn new(entity: Vec<Entity>) -> Self {
        UiAnimatorPipe {
            entity,
            marker: PhantomData
        }
    }
}
fn ui_state_pipe_system<S: UiState>(query: Query<(&UiAnimator<S>, &UiAnimatorPipe<S>), Changed<UiAnimator<S>>>, mut event: EventWriter<SetUiStateTransition<S>>) {
    for (state, pipe) in &query {
        for e in &pipe.entity {
            event.send(SetUiStateTransition::new(*e, state.animation_transition));
        }
    }
}


/// Default base color component
#[derive(Component, Debug, Clone, PartialEq)]
pub struct UiColor<S: UiState> {
    /// The base color
    pub color: Color,
    /// Phantom data
    phantom: PhantomData<S>
}
impl <S: UiState> UiColor<S> {
    /// Creates new struct
    pub fn new(color: Color) -> Self {
        UiColor {
            color,
            phantom: PhantomData,
        }
    }
}
fn set_ui_color<S: UiState>(query: Query<(&UiAnimator<S>, &UiColor<Base>, &UiColor<S>, Entity), Changed<UiAnimator<S>>>, mut set_color: EventWriter<actions::SetColor>) {
    for (hover, basecolor, hovercolor, entity) in &query {
        set_color.send(actions::SetColor {
            target: entity,
            color: basecolor.color.lerp(hovercolor.color, hover.animation_transition),
        });
    }
}


// #=============#
// #=== HOVER ===#

#[derive(Resource)]
struct UiSoundPlayer {
    entity: Option<Entity>,
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct OnHoverPlaySound {
    pub sound: Handle<AudioSource>,
}
impl OnHoverPlaySound {
    /// Specify the entity you want to create events for.
    pub fn new(sound: Handle<AudioSource>) -> Self {
        OnHoverPlaySound {
            sound,
        }
    }
}
fn on_hover_play_sound_system(mut events: EventReader<Pointer<Over>>, mut commands: Commands, query: Query<&OnHoverPlaySound>, mut player: ResMut<UiSoundPlayer>) {
    for event in events.read() {
        if let Ok(listener) = query.get(event.target) {

            if let Some(entity) = player.entity {
                if let Some(cmd) = commands.get_entity(entity) {
                    cmd.despawn_recursive();
                }
            }

            let entity = commands.spawn(AudioBundle { source: listener.sound.clone(), settings: PlaybackSettings::DESPAWN.with_volume(bevy::audio::Volume::new(0.3)) }).id();
            player.entity = Some(entity);
        }
    }
}

/// System that changes animation direction on hover
fn hover_enter_system(mut events: EventReader<Pointer<Over>>, mut query: Query<&mut UiAnimator<Hover>>) {
    for event in events.read() {
        if let Ok(mut hover) = query.get_mut(event.target) {
            hover.animation_direction = 1.0;
        }
    }
}

/// System that changes animation direction on hover
fn hover_leave_system(mut events: EventReader<Pointer<Out>>, mut query: Query<&mut UiAnimator<Hover>>) {
    for event in events.read() {
        if let Ok(mut hover) = query.get_mut(event.target) {
            hover.animation_direction = -1.0;
        }
    }
}


// #===============#
// #=== PLUGINS ===#

pub struct StatePlugin<T,N,S>(pub PhantomData<T>, pub PhantomData<N>, pub PhantomData<S>);
impl <T:Component, N:Default + Component, S: UiState> StatePlugin<T,N,S> {
    pub fn new() -> Self {
        StatePlugin::<T,N,S>(PhantomData, PhantomData, PhantomData)
    }
}
impl <T:Component, N:Default + Component, S: UiState> Plugin for StatePlugin<T,N,S> {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SetUiStateTransition<S>>()
            .add_systems(Update, set_ui_state_transition::<S>.run_if(on_event::<SetUiStateTransition<S>>()))

            .add_systems(Update, ui_state_pipe_system::<S>)

            .add_systems(Update, ui_animation_state::<S>)

            .add_systems(Update, (ui_animation::<S>, set_ui_color::<S>.after(UiSystems::Process)).chain())

            .add_systems(Update, send_layout_to_node::<T, N, S>.in_set(UiSystems::Send).before(send_content_size_to_node::<T, N>));
    }
}

pub struct DefaultStatesPlugin;
impl Plugin for DefaultStatesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(UiSoundPlayer { entity: None })
            .add_systems(Update, on_hover_play_sound_system.run_if(on_event::<Pointer<Over>>()))
            .add_systems(Update, hover_enter_system.run_if(on_event::<Pointer<Over>>()))
            .add_systems(Update, hover_leave_system.run_if(on_event::<Pointer<Out>>()));
    }
}