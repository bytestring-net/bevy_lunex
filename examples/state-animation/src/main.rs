use bevy::{prelude::*, window::SystemCursorIcon};
use bevy_lunex::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            UiLunexPlugins,
            //UiLunexDebugPlugin::<0, 0>
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, spawn)
        .add_event::<SpawnEvent>()
        .run()
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        UiSourceCamera::<0>,
        Transform::from_translation(Vec3::Z * 1000.0),
    ));

    commands.spawn((
        Name::new("Root"),
        UiLayoutRoot::new_2d(),
        UiFetchFromCamera::<0>,
    )).with_children(|ui| {
        ui.spawn((
            Name::new("Mesh"),
            // spawn time animation (in this case fading out of 'collapsed' state)
            UiStateAnimation::new(vec![("collapsed", Anim::line(1., 0., 1.).with_end_trig())]),
            UiLayout::new(vec![
                ("base", UiLayout::window().pos(Rl(50.)).size(Rh(25.))),
                ("hover", UiLayout::window().pos(Rl(50.)).size(Rh((80., 25.)))),
                ("click", UiLayout::window().pos(Rl(50.)).size(Rh((80., 10.)))),
                ("collapsed", UiLayout::window().pos(Rl(10.)).size(Rh(100.))),
            ]),
            UiColor::new(vec![
                ("base", Color::hsla(50., 0., 1., 0.5)),
                ("hover", Color::hsla(200., 1., 0.5, 1.0)),
                ("click", Color::hsla(330., 1., 0.5, 1.0)),
                ("collapsed", Color::hsla(0., 0., 0., 0.3)),
            ]),
            UiMeshPlane2d,
            MeshMaterial2d(materials.add(ColorMaterial::default())),
            OnHoverSetCursor::new(SystemCursorIcon::Pointer),
            children![(
                Name::new("text"),
                Pickable::IGNORE,
                UiLayout::window().size(Rl(100.)).pack(),
                Text2d::new("hover me!"),
                TextFont::from_font_size(100.),
            )],
        ))
         // on Pointer<Over> events, insert an animation for 'hover' state
         // a single linear segment from 0 to 1 with a speed of 2 units/sec (in 0.5 secs) with a trigger at the end
        .observe(morphing!(Pointer<Over>, "hover", Anim::line(0., 1., 2.).with_end_trig()))
        // this will continue to 0 from the current weight (at a speed of 0.5 unit/sec) (1 unit in 2 seconds)
        .observe(morphing!(Pointer<Out>, "hover", Anim::line(1., 0., 1.0/2.0)))
        // replacing, because we always want to jump to 1 when clicked
        .observe(replacing!(Pointer<Pressed>, "click",
            // you can insert multiple stage animations
            // go to weight 0 in 1 sec, then back to weight 1 in 0.5 sec
            // starting with a weight of 1
            Anim::segs(vec![Seg::To(0., 1.), Seg::To(1.,2.)]).with_init(1.).looping()
        ))
        .observe(morphing!(Pointer<Released>, "click",
            Anim::segs(vec![
                // in 1 sec, go to weight 0
                Seg::To(0., 1.),
                // delay for 0.2 seconds
                Seg::Hold(0.2),
                // then go to 1 in 0.3 with this curve function
                Seg::Curved(1., 1.0/0.3, downarc),
                // and back to 0
                Seg::Curved(0., 1.0/0.3, uparc),
                // send a trigger
                Seg::Trig
            ])
        ))
        .observe(print)
        .observe(spawn_after_hover);
    });
}

fn print(
    trig: Trigger<AnimTrig>,
) {
    info!("reached Trig point in animation of '{}' for entity {}", trig.event().0, trig.target());
}

fn spawn_after_hover(
    trig: Trigger<AnimTrig>,
    mut writer: EventWriter<SpawnEvent>,
) {
    if trig.event().0 == "hover" {
        writer.write(SpawnEvent);
    }
}

fn despawn_after_click(
    trig: Trigger<AnimTrig>,
    mut commands: Commands,
) {
    if trig.event().0 == "click" {
        commands.entity(trig.target()).despawn();
    }
}

// test curve functions (stolen from fundsp)
fn uparc(x: f32) -> f32 {
    1. - (1. - x * x).max(0.).sqrt()
}

fn downarc(x: f32) -> f32 {
    ((2. - x) * x).max(0.).sqrt()
}


// this is a workaround to avoid spawning immediately after the trigger
// instead i want to spawn in Update
// since this entity starts with full 'collapsed' state (and fades to 'base')
// if we don't do this, then it's spawned after PostCompute
// and so it displays the 'base' state for one frame
#[derive(Event)]
struct SpawnEvent;

fn spawn(
    mut commands: Commands,
    mut reader: EventReader<SpawnEvent>,
    query: Query<Entity, With<UiLayoutRoot>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in reader.read() {
        commands.spawn((
            Name::new("Other Mesh"),
            UiStateAnimation::new(vec![("collapsed", Anim::line(1., 0., 1.).with_end_trig())]),
            UiLayout::new(vec![
                ("base", UiLayout::window().pos(Rl(0.)).size(Rl(25.))),
                ("hover", UiLayout::window().pos(Rl(0.)).size(Rl(30.))),
                ("click", UiLayout::window().pos(Rl(0.)).size(Rl(40.))),
                ("collapsed", UiLayout::window().pos(Rl((0., 50.))).size(Rl(10.))),
            ]),
            UiColor::new(vec![
                ("base", Color::hsla(200., 1., 0.5, 1.0)),
                ("hover", Color::hsla(146., 1., 0.8, 1.)),
                ("click", Color::hsla(245., 1., 0.5, 1.)),
                ("collapsed", Color::hsla(0., 1., 0.5, 1.0)),
            ]),
            UiMeshPlane2d,
            MeshMaterial2d(materials.add(ColorMaterial::default())),
            ChildOf(query.single().unwrap()),
            children![(
                Name::new("text"),
                Pickable::IGNORE,
                UiLayout::window().size(Rl(100.)).pack(),
                Text2d::new("click me!"),
                TextFont::from_font_size(100.),
            )],
        ))

        .observe(morphing!(Pointer<Over>, "hover", Anim::line(0., 1., 1.0/0.3).with_end_trig()))
        .observe(morphing!(Pointer<Out>, "hover", Anim::line(1., 0., 1.0/0.8)))
        .observe(replacing!(Pointer<Pressed>, "click",
            Anim::segs(vec![Seg::To(0.,1.), Seg::To(1.,2.)]).with_init(1.).looping()
        ))
        .observe(morphing!(Pointer<Released>, "click",
            Anim::segs(vec![
                Seg::To(0., 1.),
                Seg::Hold(0.2),
                Seg::Curved(1., 1.0/0.3, downarc),
                Seg::Curved(0., 1.0/0.3, uparc),
                Seg::Trig
            ])
        ))
        .observe(print)
        .observe(despawn_after_click);
    }
}

