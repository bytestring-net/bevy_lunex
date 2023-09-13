use std::borrow::Borrow;

use bevy::prelude::*;
use bevy_lunex::prelude::*;
use bevy_vector_shapes::prelude::*;


/// Renders the widget
#[derive(Component)]
pub struct VectorElementRectangle {
    color: Color,
    corner_radii: Vec4
}
pub fn vector_rectangle_update (mut painter: ShapePainter, query: Query<(&Transform, &VectorElementRectangle)>) {
    for (transform, color) in &query {

        painter.set_translation(transform.translation);
        painter.set_scale(Vec3::splat(1.0));

        let ww = transform.scale.x;
        let hh = transform.scale.y;

        painter.color = color.color;
        painter.thickness = 1.0;
        painter.corner_radii = color.corner_radii;
        painter.rect(Vec2::new(ww, hh));
    }
}





pub struct BoxStyle {
    text_style: TextStyle,
    painter: Box<dyn Fn(ShapePainter<'_, '_>, f32, f32)>,
}
impl BoxStyle {
    pub fn new(text_style: impl Borrow<TextStyle>) -> Self {
        let gg = |mut painter: ShapePainter, w: f32, h: f32| {
            painter.color = Color::VIOLET;
            painter.thickness = 1.0;
            painter.corner_radii = Vec4::splat(10.0);
            painter.rect(Vec2::new(w, h));
        };

        BoxStyle {
            text_style: text_style.borrow().to_owned(),
            painter: Box::new(gg),
        }
    }
}


#[derive(Component)]
pub struct DropDownBox {
    text_style: TextStyle,
    options: Vec<String>,
    selected: String,
    selected_id: usize,
}
impl DropDownBox {
    pub fn new(options: Vec<String>, selected_id: usize, text_style: impl Borrow<TextStyle>) -> Self {
        DropDownBox {
            text_style: text_style.borrow().to_owned(),
            selected: options[selected_id].clone(),
            selected_id,
            options: options,
        }

    }

    pub fn build_list(&self, commands: &mut Commands, tree: &mut UiTree, widget: &Widget) -> Result<(), LunexError>{
        
        let segment = GridSegment::text_cells(&self.options, 50.0, 60.0).add_gaps(1.0);
        let (_, wlist) = segment.build_in_window_absolute(tree, widget.end("Droplist"), WindowLayout::empty().with_rel(Vec2::new(0.0, 100.0)), GridOrientation::Vertical)?;

        for x in 0..wlist.len() {
            commands.spawn((
                ElementBundle::new(&wlist[x], Element::fullfill()),
                VectorElementRectangle {
                    color: Color::rgb_u8(148, 52, 70),
                    corner_radii: Vec4::splat(4.0)
                },
            ));
            commands.spawn(
                TextElementBundle::new(&wlist[x], &TextParams::centerleft().at(10.0, 50.0).with_style(&self.text_style).with_height(Some(60.0)), &self.options[x])
            );
        }

        Ok(())
    }
}



pub fn dropdown_element_update (mut commands: Commands, mut trees: Query<&mut UiTree>, cursors: Query<&Cursor>, mut query: Query<(&Widget, &DropDownElement)>) {
    for mut tree in &mut trees {
        for (widget, dropdown) in &mut query {
            let mut trigger = false;
            for cursor in &cursors {
                if widget.contains_position(&tree, &cursor.position_world().as_lunex(tree.offset)).unwrap() {
                    trigger = true;
                    break;
                }
            }

            if trigger {
                match widget.fetch_ext(&tree, "Droplist") {
                    Err(..) => {
                        //println!("Building list");
                        dropdown.build_list(&mut commands, &mut tree, widget).unwrap();
                    },
                    Ok (..) => {},
                }
            } else {
                match widget.fetch_ext(&tree, "Droplist") {
                    Err(..) => {},
                    Ok (..) => {
                        //println!("Dropping list");
                        let mut trigger = false;
                        for cursor in &cursors {
                            if widget.contains_position_ext(&tree, "Droplist", &cursor.position_world().as_lunex(tree.offset)).unwrap() {
                                trigger = true;
                                break;
                            }
                        }
                        if trigger == false {
                            widget.remove(&mut tree, "Droplist").unwrap();
                        }
                    },
                }
            }
        }
    }
}