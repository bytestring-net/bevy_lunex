//! A showcase of the flow layout model: stack ordering, the `Sp` space unit,
//! justification/alignment margins, line wrapping and grid tracks.
//!
//! Hold the left mouse button and drag: the UI root resizes to span from the
//! window's top-left corner to your cursor - a live demo of the layout
//! recomputing without resizing (and thrashing) the actual OS window.

use bevy::prelude::*;
use bevy_lunex::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, UiLunexPlugins))
        .add_systems(Startup, setup)
        .add_systems(Update, resize_root_by_drag)
        .run()
}

fn setup(mut commands: Commands, window: Single<&Window>) {
    commands.spawn((
        Camera2d, UiSourceCamera::<0>,
        Transform::from_translation(Vec3::Z * 1000.0),
    ));

    let window_size = window.size();

    // The UI root acts as an implicit left-to-right flow container, so the
    // three showcase columns split the root width evenly. Its `Dimension` is
    // driven manually by the drag system instead of the camera viewport.
    let anchor = top_left_anchor(window_size, window_size);
    commands.spawn((
        Name::new("Root"),
        UiLayoutRoot::new_2d(),
        Dimension(window_size),
        Transform::from_xyz(anchor.x, anchor.y, 0.0),
        Sprite::from_color(Color::srgb(0.03, 0.04, 0.05), Vec2::ONE),
    ))
    .with_children(|ui| {
        ordering_panel(ui);
        justify_panel(ui);
        wrap_grid_panel(ui);
    });
}

/// The smallest root size the drag allows.
const ROOT_MIN: Vec2 = Vec2::new(200.0, 100.0);

/// Returns the transform position that anchors a root box of `size` at the
/// window's top-left corner (the root box is centered on its transform).
fn top_left_anchor(size: Vec2, window_size: Vec2) -> Vec2 {
    Vec2::new(-window_size.x / 2.0 + size.x / 2.0, window_size.y / 2.0 - size.y / 2.0)
}

/// While the left mouse button is held, resizes the UI root so it spans from
/// the window's top-left corner to the cursor. On release, the size stays.
fn resize_root_by_drag(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    mut roots: Query<(&mut Dimension, &mut Transform), With<UiLayoutRoot>>,
) {
    if !mouse.pressed(MouseButton::Left) { return }
    let Some(cursor) = window.cursor_position() else { return };
    let window_size = window.size();
    let size = cursor.clamp(ROOT_MIN, window_size);
    let anchor = top_left_anchor(size, window_size);
    for (mut dimension, mut transform) in &mut roots {
        **dimension = size;
        transform.translation.x = anchor.x;
        transform.translation.y = anchor.y;
    }
}

/// Spawns a dark panel filling its share of the window and runs the children spawner inside.
fn panel(ui: &mut ChildSpawnerCommands, name: &str, spawn_children: impl FnOnce(&mut ChildSpawnerCommands)) {
    ui.spawn((
        Name::new(name.to_string()),
        UiLayout::flow()
            .direction(UiFlowDirection::TopToBottom)
            .gap(Ab(8.0))
            .padding_all(Ab(12.0))
            .width(UiFlowSize::Grow)
            .height(UiFlowSize::Grow)
            .pack(),
        Sprite::from_color(Color::srgb(0.09, 0.10, 0.13), Vec2::ONE),
    ))
    .with_children(spawn_children);
}

fn palette(i: usize) -> Color {
    const PALETTE: [Color; 4] = [
        Color::srgb(0.85, 0.35, 0.35),
        Color::srgb(0.90, 0.65, 0.25),
        Color::srgb(0.40, 0.75, 0.45),
        Color::srgb(0.35, 0.60, 0.90),
    ];
    PALETTE[i % PALETTE.len()]
}

/// Panel 1: the four stack orderings.
fn ordering_panel(ui: &mut ChildSpawnerCommands) {
    panel(ui, "Ordering", |panel| {
        let orderings = [
            (UiFlowDirection::LeftToRight, "LTR"),
            (UiFlowDirection::RightToLeft, "RTL"),
            (UiFlowDirection::TopToBottom, "TTB"),
            (UiFlowDirection::BottomToTop, "BTT"),
        ];
        for (direction, name) in orderings {
            panel.spawn((
                Name::new(name.to_string()),
                UiLayout::flow()
                    .direction(direction)
                    .gap(Ab(6.0))
                    .padding_all(Ab(8.0))
                    .width(UiFlowSize::Grow)
                    .height(UiFlowSize::Grow)
                    .pack(),
                Sprite::from_color(Color::srgb(0.15, 0.17, 0.22), Vec2::ONE),
            ))
            .with_children(|row| {
                for i in 0..4 {
                    row.spawn((
                        Name::new(format!("{name} {i}")),
                        UiLayout::flow().width(Ab(56.0)).height(Ab(24.0)).pack(),
                        Sprite::from_color(palette(i), Vec2::ONE),
                    ));
                }
            });
        }
    });
}

/// Panel 2: justification modes - all of them are injected `Sp` margins.
fn justify_panel(ui: &mut ChildSpawnerCommands) {
    panel(ui, "Justify", |panel| {
        let modes = [
            (UiJustify::Start, "Start"),
            (UiJustify::Center, "Center"),
            (UiJustify::End, "End"),
            (UiJustify::SpaceBetween, "Between"),
            (UiJustify::SpaceEvenly, "Evenly"),
            (UiJustify::SpaceAround, "Around"),
        ];
        for (justify, name) in modes {
            panel.spawn((
                Name::new(name.to_string()),
                UiLayout::flow()
                    .justify(justify)
                    .align(Align::CENTER)
                    .gap(Ab(6.0))
                    .padding_all(Ab(8.0))
                    .width(UiFlowSize::Grow)
                    .height(UiFlowSize::Grow)
                    .pack(),
                Sprite::from_color(Color::srgb(0.15, 0.17, 0.22), Vec2::ONE),
            ))
            .with_children(|row| {
                for i in 0..3 {
                    row.spawn((
                        Name::new(format!("{name} {i}")),
                        UiLayout::flow().width(Ab(64.0)).height(Ab(24.0)).pack(),
                        Sprite::from_color(palette(i), Vec2::ONE),
                    ));
                }
            });
        }
    });
}

/// Panel 3: line wrapping (plain and flipped) and grid tracks.
fn wrap_grid_panel(ui: &mut ChildSpawnerCommands) {
    panel(ui, "Wrap & Grid", |panel| {
        // === Wrapping: items flow onto new lines when the width runs out. ===
        panel.spawn((
            Name::new("Wrap"),
            UiLayout::flow()
                .wrapping()
                .align(Align::END)
                .gap(Ab(6.0))
                .padding_all(Ab(8.0))
                .width(UiFlowSize::Grow)
                .pack(),
            Sprite::from_color(Color::srgb(0.15, 0.17, 0.22), Vec2::ONE),
        ))
        .with_children(|wrap| {
            for i in 0..7 {
                wrap.spawn((
                    Name::new(format!("Wrap {i}")),
                    UiLayout::flow().width(Ab(64.0)).height(Ab(24.0)).pack(),
                    Sprite::from_color(palette(i), Vec2::ONE),
                ));
            }
        });

        // === Flipped wrapping: the first line sits at the bottom edge. ===
        // The fixed height is the exact worst case (4 items one per line:
        // 4*24 + 3*6 gaps + 2*8 padding = 130) - narrower states exactly fill it
        // and wider states show the flipped bottom-stacking with space above.
        panel.spawn((
            Name::new("Wrap flipped"),
            UiLayout::flow()
                .wrapping()
                .flipped()
                .align(Align::END)
                .gap(Ab(6.0))
                .padding_all(Ab(8.0))
                .width(UiFlowSize::Grow)
                .height(Ab(130.0))
                .pack(),
            Sprite::from_color(Color::srgb(0.15, 0.17, 0.22), Vec2::ONE),
        ))
        .with_children(|wrap| {
            for i in 0..4 {
                wrap.spawn((
                    Name::new(format!("Flipped {i}")),
                    UiLayout::flow().width(Ab(64.0)).height(Ab(24.0)).pack(),
                    Sprite::from_color(palette(i), Vec2::ONE),
                ));
            }
        });

        // === Grid: three track definitions, items fill their track. ===
        panel.spawn((
            Name::new("Grid"),
            UiLayout::flow()
                .gap(Ab(6.0))
                .padding_all(Ab(8.0))
                .width(UiFlowSize::Grow)
                .grid([
                    UiFlowSize::Fixed(Ab(48.0).into()),
                    UiFlowSize::Grow,
                    Sp(1.0).into(),
                ])
                .pack(),
            Sprite::from_color(Color::srgb(0.15, 0.17, 0.22), Vec2::ONE),
        ))
        .with_children(|grid| {
            for i in 0..7 {
                grid.spawn((
                    Name::new(format!("Cell {i}")),
                    UiLayout::flow().height(Ab(28.0)).pack(),
                    Sprite::from_color(palette(i), Vec2::ONE),
                ));
            }
        });
    });
}
