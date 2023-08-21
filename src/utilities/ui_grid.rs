use bevy::{prelude::*, utils::thiserror::Error};
use bevy::sprite::Anchor;

use crate::{UiTree, Widget, layout, BranchError};

// ===========================================================
// === GRID GENERATION ===

/// ### Grid parameters
/// 
/// Struct that is passed to [`grid_generate`] or [`grid_generate_inside`] function containing grid information.
/// The fields are used to define grid of widgets created inside the function.
/// 
/// ### Fields
/// 
/// * `grid` = 2D Vector of String values, is used to determine rows and columns and to name the grid widgets. Use [`textgrid`] macro here.
/// * `anchor` = the origin of the grid, useful when you try to position the grid somewhere specific. [`grid_generate_inside`] ignores this field.
/// * `gap_relative` = width and height of the gaps between widgets in % relative to the parent widget.
/// * `width_relative` = width of one widget in % relative to the parent widget.
/// * `height_relative` = height of one widget in % relative to the parent widget.
/// * `width_border_gap` = if gaps should also be on the outside of the grid.
/// * `height_border_gap` = if gaps should also be on the outside of the grid.
#[derive(Clone, Debug)]
pub struct GridParams {
    pub grid: Vec<Vec<String>>,
    pub anchor: Anchor,
    pub gap_relative: Vec2,
    pub width_relative: f32,
    pub height_relative: f32,
    pub width_border_gap: bool,
    pub height_border_gap: bool,
}

impl Default for GridParams {
    fn default() -> Self {
        GridParams {
            grid: vec![Vec::new()],
            anchor: Anchor::TopLeft,
            gap_relative: Vec2::new(2.0, 2.0),
            width_relative: 10.0,
            height_relative: 10.0,
            width_border_gap: false,
            height_border_gap: false,
        }
    }
}

impl GridParams {
    /// Blank new grid parameters from 2D Vector. Use [`textgrid`] macro here.
    ///```
    /// let grid: = GridParams::new(textgrid![["widget 1", "widget 2"], ["widget 3", "widget 4"]]);
    /// ```
    /// This will create layout like this:
    ///
    ///|   Grid    |     1      |     2      |
    ///|-----------|------------|------------|
    ///|   **1**   | -widget 1- | -widget 3- |
    ///|   **2**   | -widget 2- | -widget 4- |
    pub fn new(grid: &Vec<Vec<String>>) -> GridParams {
        GridParams {
            grid: grid.clone(),
            ..Default::default()
        }
    }

    /// Grid parameters set to a custom relative height
    pub fn with_anchor(mut self, anchor: Anchor) -> GridParams {
        self.anchor = anchor;
        self
    }

    /// Grid parameters set to a custom relative width
    pub fn with_width(mut self, width: f32) -> GridParams {
        self.width_relative = width;
        self
    }
    /// Grid parameters set to a custom relative height
    pub fn with_height(mut self, height: f32) -> GridParams {
        self.height_relative = height;
        self
    }

    /// Grid parameters set to a custom relative gap width
    pub fn with_width_gap(mut self, width: f32) -> GridParams {
        self.gap_relative.x = width;
        self
    }
    /// Grid parameters set to a custom relative gap height
    pub fn with_height_gap(mut self, height: f32) -> GridParams {
        self.gap_relative.y = height;
        self
    }

    /// Grid parameters with enabled/disabled width border gap
    pub fn with_width_gap_border(mut self, enable: bool) -> GridParams {
        self.width_border_gap = enable;
        self
    }
    /// Grid parameters with enabled/disabled height border gap
    pub fn with_height_gap_border(mut self, enable: bool) -> GridParams {
        self.height_border_gap = enable;
        self
    }
}

#[derive(Debug, Error)]
pub enum GridError {
    #[error("Grid column {c1:} (len: {len_c1:}) has a different length to column 0 (len: {len_c0:})")]
    Format {
        c1: usize,
        len_c1: usize,
        len_c0: usize,
    },
    #[error("branch error: {0:}")]
    Branch(BranchError),
}

/// ### Grid generate
/// 
/// A complex function that will generate a grid of widgets. Can be used to make lists too.
///
/// This function uses a widget to hold the grid, meaning no matter how many columns or rows there are, the grid widgets will have the same size.
/// ### Arguments
/// * `system` = UiTree in which the grid should be made.
/// * `path` = Path to a new widget that will hold the grid.
/// * `relative` = Relative position of the grid in parenting widget.
/// * `grid_params` = A struct holding all necessary info about the grid.
pub fn grid_generate(
    system: &mut UiTree,
    path: &String,
    relative: Vec2,
    grid_params: &GridParams,
) -> Result<Widget, GridError> {
    let xx = grid_params.grid.len();
    let yy = grid_params.grid[0].len();

    for i in 0..grid_params.grid.len() {
        if grid_params.grid[i].len() != yy {
            return Err(GridError::Format { c1: i, len_c1: grid_params.grid[i].len(), len_c0: xx });
        }
    }

    let total_width = grid_params.width_relative * xx as f32;
    let total_height = grid_params.height_relative * yy as f32;

    let total_wgap = grid_params.gap_relative.x
        * (xx as f32
            + if grid_params.width_border_gap == true {
                1.0
            } else {
                -1.0
            });
    let total_hgap = grid_params.gap_relative.y
        * (yy as f32
            + if grid_params.height_border_gap == true {
                1.0
            } else {
                -1.0
            });

    let container_width = total_width + total_wgap;
    let container_height = total_height + total_hgap;

    let anchor_offset = match grid_params.anchor {
        Anchor::TopCenter => Vec2::new(0.5, 0.0),
        Anchor::TopLeft => Vec2::new(0.0, 0.0),
        Anchor::TopRight => Vec2::new(1.0, 0.0),

        Anchor::Center => Vec2::new(0.5, 0.5),
        Anchor::CenterLeft => Vec2::new(0.0, 0.5),
        Anchor::CenterRight => Vec2::new(1.0, 0.5),

        Anchor::BottomCenter => Vec2::new(0.5, 1.0),
        Anchor::BottomLeft => Vec2::new(0.0, 1.0),
        Anchor::BottomRight => Vec2::new(1.0, 1.0),

        Anchor::Custom(point) => Vec2::new(point.x + 0.5, -point.y + 0.5),
    };

    let widget = match Widget::create(
        system,
        path,
        layout::Window {
            relative: Vec2::new(
                relative.x - anchor_offset.x * container_width,
                relative.y - anchor_offset.y * container_height,
            ),
            width_relative: container_width,
            height_relative: container_height,
            ..Default::default()
        }
        .pack(),
    ) {
        Result::Ok(widget) => widget,
        Result::Err(e) => return Result::Err(GridError::Branch(e)),
    };

    let width = (100.0 * total_width / container_width) / xx as f32;
    let height = (100.0 * total_height / container_height) / yy as f32;

    let wgap = (100.0 * total_wgap / container_width)
        / (xx as f32
            + if grid_params.width_border_gap == true {
                1.0
            } else {
                0.0
            });
    let hgap = (100.0 * total_hgap / container_height)
        / (yy as f32
            + if grid_params.height_border_gap == true {
                1.0
            } else {
                0.0
            });

    for x in 0..xx {
        for y in 0..yy {
            match Widget::create(
                system,
                &widget.end(&grid_params.grid[x][y]),
                layout::Window {
                    relative: Vec2::new(
                        width * x as f32
                            + wgap * x as f32
                            + if grid_params.width_border_gap == true {
                                wgap
                            } else {
                                0.0
                            },
                        height * y as f32
                            + hgap * y as f32
                            + if grid_params.height_border_gap == true {
                                hgap
                            } else {
                                0.0
                            },
                    ),
                    width_relative: width,
                    height_relative: height,
                    ..Default::default()
                }
                .pack(),
            ) {
                Result::Ok(_) => (),
                Result::Err(e) => return Result::Err(GridError::Branch(e)),
            };
        }
    }
    Result::Ok(widget)
}

/// ### Grid generate inside
/// A complex function that will generate a grid of widgets. Can be used to make lists too.
///
/// This function generates the grid inside of given widget, meaning with more columns and rows, the size of grid widgets will decrease.
/// ### Arguments
/// * `system` = UITree in which the grid should be made.
/// * `widget` = The widget in which the grid should be made.
/// * `grid_params` = A struct holding all necessary info about the grid.
pub fn grid_generate_inside(
    system: &mut UiTree,
    widget: &Widget,
    grid_params: &GridParams,
) -> Result<(), GridError> {
    let xx = grid_params.grid.len();
    let yy = grid_params.grid[0].len();

    for i in 0..grid_params.grid.len() {
        if grid_params.grid[i].len() != yy {
            return Result::Err(GridError::Format { c1: i, len_c1: grid_params.grid[i].len(), len_c0: yy });
        }
    }

    let total_width = grid_params.width_relative * xx as f32;
    let total_height = grid_params.height_relative * yy as f32;

    let total_wgap = grid_params.gap_relative.x
        * (xx as f32
            + if grid_params.width_border_gap == true {
                1.0
            } else {
                -1.0
            });
    let total_hgap = grid_params.gap_relative.y
        * (yy as f32
            + if grid_params.height_border_gap == true {
                1.0
            } else {
                -1.0
            });

    let container_width = total_width + total_wgap;
    let container_height = total_height + total_hgap;

    let width = (100.0 * total_width / container_width) / xx as f32;
    let height = (100.0 * total_height / container_height) / yy as f32;

    let wgap = (100.0 * total_wgap / container_width)
        / (xx as f32
            + if grid_params.width_border_gap == true {
                1.0
            } else {
                0.0
            });
    let hgap = (100.0 * total_hgap / container_height)
        / (yy as f32
            + if grid_params.height_border_gap == true {
                1.0
            } else {
                0.0
            });

    for x in 0..xx {
        for y in 0..yy {
            match Widget::create(
                system,
                &widget.end(&grid_params.grid[x][y]),
                layout::Window {
                    relative: Vec2::new(
                        width * x as f32
                            + wgap * x as f32
                            + if grid_params.width_border_gap == true {
                                wgap
                            } else {
                                0.0
                            },
                        height * y as f32
                            + hgap * y as f32
                            + if grid_params.height_border_gap == true {
                                hgap
                            } else {
                                0.0
                            },
                    ),
                    width_relative: width,
                    height_relative: height,
                    ..Default::default()
                }
                .pack(),
            ) {
                Result::Ok(_) => (),
                Result::Err(e) => return Result::Err(GridError::Branch(e)),
            };
        }
    }
    Result::Ok(())
}

/// ## Text Row
/// Attempts to construct 1D vector from given elements, useful when you don't want to type .to_string() every time.
/// ```
/// let row: Vec<String> = textrow!["item 1", "item 2", "item 3"];
/// ```
#[macro_export]
macro_rules! textrow {
    [$($element:expr),*] => {{
        vec![$($element.to_string()),*]
    }};
}

/// ## Text Grid
/// Attempts to construct 2D vector from given elements, useful when you don't want to type .to_string() every time.
/// ```
/// let grid: Vec<Vec<String>> = textgrid![["item 1", "item 2"], ["item 3", "item 4"]];
/// ```
#[macro_export]
macro_rules! textgrid {
    ($([$($element:expr),*]),*) => {
        vec![
            $(
                vec![
                    $($element.to_string()),*
                ]
            ),*
        ]
    };
}
