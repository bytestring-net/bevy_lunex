use bevy::prelude::*;
use bevy::sprite::Anchor;

use bevy_lunex_core::{UiTree, Widget, WindowLayout, SolidLayout, LunexError, LayoutPackage};

use super::element::text_compute_size_simple;

// ===========================================================
// === GRID DEFINITION ===

/// # Grid parameters
/// 
/// Struct that is passed to [`grid_generate`] or [`grid_generate_inside`] function and containing grid information.
/// The fields are used to define grid of widgets created inside the function.
/// 
/// # Fields
/// 
/// * `grid` = 2D Vector of String values, is used to determine rows and columns and to name the grid widgets. Use [`textgrid`] macro here.
/// * `anchor` = the origin of the grid, useful when you try to position the grid somewhere specific. [`grid_generate_inside`] ignores this field.
/// * `gap_relative` = width and height of the gaps between widgets in % relative to the parent widget.
/// * `width_relative` = width of one widget in % relative to the parent widget.
/// * `height_relative` = height of one widget in % relative to the parent widget.
/// * `width_border_gap` = if gaps should also be on the outside of the grid.
/// * `height_border_gap` = if gaps should also be on the outside of the grid.
/*#[derive(Clone, Debug)]
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
    /// let grid = GridParams::new(textgrid![["widget 1", "widget 2"], ["widget 3", "widget 4"]]);
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

    /// Grid parameters set to a custom anchor
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
impl AsRef<GridParams> for GridParams {
    fn as_ref(&self) -> &GridParams {
        &self
    }
}
impl AsMut<GridParams> for GridParams {
    fn as_mut(&mut self) -> &mut GridParams {
        self
    }
}
*/
// ===========================================================
// === GRID GENERATION ===

//### RETURN (Widget, Vec<Vec<Widget>>)!!!!!!!!!!!!!
/*
/// # Grid generate
/// 
/// A complex function that will generate a grid of widgets. Can be used to make lists too.
///
/// This function uses a widget to hold the grid, meaning no matter how many columns or rows there are, the grid widgets will have the same size.
/// # Arguments
/// * `tree` = UiTree in which the grid should be made.
/// * `path` = Path to a new widget that will hold the grid.
/// * `relative` = Relative position of the grid in parenting widget.
/// * `grid_params` = A struct holding all necessary info about the grid.
pub fn grid_generate(
    tree: &mut UiTree,
    path: &String,
    relative: Vec2,
    grid_params: &GridParams,
) -> Result<Widget, LunexError> {
    let xx = grid_params.grid.len();
    let yy = grid_params.grid[0].len();

    for i in 0..grid_params.grid.len() {
        if grid_params.grid[i].len() != yy {
            return Err(LunexError::GridFormat { c1: i, len_c1: grid_params.grid[i].len(), len_c0: xx });
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
        tree,
        path,
        WindowLayout {
            relative: Vec2::new(
                relative.x - anchor_offset.x * container_width,
                relative.y - anchor_offset.y * container_height,
            ),
            width_relative: container_width,
            height_relative: container_height,
            ..Default::default()
        },
    ) {
        Ok(widget) => widget,
        Err(e) => return Err(e),
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
                tree,
                &widget.end(&grid_params.grid[x][y]),
                WindowLayout {
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
                },
            ) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }
    }
    Ok(widget)
}

/// # Grid generate Solid
/// 
/// A complex function that will generate a grid of widgets. Can be used to make lists too.
///
/// This function uses a widget to hold the grid, meaning no matter how many columns or rows there are, the grid widgets will have the same size.
/// # Arguments
/// * `tree` = UiTree in which the grid should be made.
/// * `path` = Path to a new widget that will hold the grid.
/// * `relative` = Relative position of the grid in parenting widget.
/// * `grid_params` = A struct holding all necessary info about the grid.
pub fn grid_generate_solid(
    tree: &mut UiTree,
    path: &String,
    grid_params: &GridParams,
) -> Result<Widget, LunexError> {
    let xx = grid_params.grid.len();
    let yy = grid_params.grid[0].len();

    for i in 0..grid_params.grid.len() {
        if grid_params.grid[i].len() != yy {
            return Err(LunexError::GridFormat { c1: i, len_c1: grid_params.grid[i].len(), len_c0: xx });
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
        Anchor::TopCenter => Vec2::new(0.0, -1.0),
        Anchor::TopLeft => Vec2::new(-1.0, -1.0),
        Anchor::TopRight => Vec2::new(1.0, -1.0),

        Anchor::Center => Vec2::new(0.0, 0.0),
        Anchor::CenterLeft => Vec2::new(-1.0, 0.0),
        Anchor::CenterRight => Vec2::new(1.0, 0.0),

        Anchor::BottomCenter => Vec2::new(0.0, 1.0),
        Anchor::BottomLeft => Vec2::new(-1.0, 1.0),
        Anchor::BottomRight => Vec2::new(1.0, 1.0),

        Anchor::Custom(point) => Vec2::new(point.x * 2.0, -point.y * 2.0),
    };
    println!("{} {}", container_width, container_height);
    let widget = match Widget::create(
        tree,
        path,
        SolidLayout {
            horizontal_anchor: anchor_offset.x,
            vertical_anchor: anchor_offset.y,
            
            width: container_width,
            height: container_height,
            ..Default::default()
        },
    ) {
        Ok(widget) => widget,
        Err(e) => return Err(e),
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
                tree,
                &widget.end(&grid_params.grid[x][y]),
                WindowLayout {
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
                },
            ) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }
    }
    Ok(widget)
}

/// # Grid generate inside
/// A complex function that will generate a grid of widgets. Can be used to make lists too.
///
/// This function generates the grid inside of given widget, meaning with more columns and rows, the size of grid widgets will decrease.
/// # Arguments
/// * `tree` = UITree in which the grid should be made.
/// * `widget` = The widget in which the grid should be made.
/// * `grid_params` = A struct holding all necessary info about the grid.
pub fn grid_generate_inside(
    tree: &mut UiTree,
    widget: &Widget,
    params: impl AsRef<GridParams>,
) -> Result<(), LunexError> {
    let grid_params = params.as_ref();
    let xx = grid_params.grid.len();
    let yy = grid_params.grid[0].len();

    for i in 0..grid_params.grid.len() {
        if grid_params.grid[i].len() != yy {
            return Err(LunexError::GridFormat { c1: i, len_c1: grid_params.grid[i].len(), len_c0: yy });
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
                tree,
                &widget.end(&grid_params.grid[x][y]),
                WindowLayout {
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
                },
            ) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }
    }
    Ok(())
}
*/

/// # Grid
/// It's made out of vector of [`GridSegment`]. Depending on the grid orientation, they are either columns next to each other or rows under each other.
/// 
/// You can specify the segments, the gap between each segment, the orientation and border gap.
/// 
/// Used for creating any kind of 2D grid like item inventories.
#[derive(Clone, Debug)]
pub struct Grid {
    /// If the grid orientation is along the X axis or Y axis
    pub orientation: GridOrientation,
    /// The vector with the grids segments to spawn
    pub segment: Vec<GridSegment>,
    /// The in-between gaps between the segments, should be n-1 of segments. Overflow is simply ignored
    pub gap: Vec<f32>,
    /// Border gaps on sides of the segment
    pub border: Option<[f32; 2]>,
}
impl Grid {
    /// Create a new grid from default
    pub fn new() -> Self {
        Grid::default()
    }

    /// Crate a new grid with this segment copied n times
    pub fn splat_segment(segment: impl AsRef<GridSegment>, n: usize) -> Self {
        let mut _segment = Vec::new();
        for _ in 0..n {
            _segment.push(segment.as_ref().to_owned());
        }
        Grid {
            segment: _segment,
            ..default()
        }
    }

    /// Crate a new grid with this gap copied n times
    pub fn splat_gaps(gap: f32, n: usize) -> Self {
        let mut _gap = Vec::new();
        for _ in 0..n {
            _gap.push(gap);
        }
        Grid {
            gap: _gap,
            ..default()
        }
    }

    /// Adds as many segments of the same size as there are gaps
    pub fn add_segments(mut self, segment: impl AsRef<GridSegment>) -> Self {
        let mut _segment = Vec::new();
        for _ in 0..self.gap.len() {
            _segment.push(segment.as_ref().to_owned());
        }
        self.segment = _segment;
        self
    }

    /// Adds as many gaps of the same size as there are segments
    pub fn add_gaps(mut self, gap: f32) -> Self {
        let mut _gap = Vec::new();
        for _ in 0..self.segment.len() {
            _gap.push(gap);
        }
        self.gap = _gap;
        self
    }

    /// Set orientation of the grid to the value given
    pub fn with_orientation(mut self, orientation: GridOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set segments of the grid to the value given
    pub fn with_segments(mut self, segment: Vec<GridSegment>) -> Self {
        self.segment = segment;
        self
    }

    /// Set gaps of the grid to the value given
    pub fn with_gaps(mut self, gaps: Vec<f32>) -> Self {
        self.gap = gaps;
        self
    }

    /// Set border of the grid to the value given
    pub fn with_border(mut self, border: Option<[f32; 2]>) -> Self {
        self.border = border;
        self
    }


    /// Iterate over and find the longest segment and use it as size
    pub fn compute_size(&self) -> f32 {
        let mut segment_size = 0.0;
        for x in &self.segment {
            let size = x.compute_lenght(self.orientation);
            if size > segment_size { segment_size = size }
        }
        segment_size
    }

    /// Iterate over and sum all segment sizes and gaps to return lenght
    pub fn compute_lenght(&self) -> f32 {
        let mut segment_lenght = 0.0;
        for x in &self.segment {
            segment_lenght += x.compute_size(self.orientation);
        }
        //let gaps: f32 = self.gap.iter().sum();
        let mut gaps = 0.0;
        // Cannot have more gaps that cells - 1
        for x in 0..self.gap.len().min(self.segment.len()-1) {
            gaps += self.gap[x]
        }
        match self.border {
            Some (border) => segment_lenght + gaps + border[0] + border[1],
            None => segment_lenght + gaps
        }
    }

    /// # Build In
    /// Builds the grid in the selected widget and returns the widget grid
    pub fn build_in(&self, tree: &mut UiTree, widget: impl AsRef<Widget>) -> Result<Vec<Vec<Widget>>, LunexError> {
        let widget = widget.as_ref();

        let grid_size = self.compute_size();
        let grid_lenght = self.compute_lenght();

        let size_normalization = 100.0/grid_size;
        let length_normalization = 100.0/grid_lenght;

        let mut segment_length_so_far = 0.0;
        let mut gap_length_so_far = 0.0;
        if let Some(border) = self.border {
            gap_length_so_far += border[0] * length_normalization
        }

        let mut widget_return: Vec<Vec<Widget>> = Vec::new();

        for x in 0..self.segment.len() {

            if x != 0 && x <= self.gap.len(){ gap_length_so_far += self.gap[x-1] * length_normalization }

            let ll = self.segment[x].compute_size(self.orientation) * length_normalization;
            let ss = self.segment[x].compute_lenght(self.orientation) * size_normalization;

            widget_return.push(self.segment[x].build_in_part_grid(tree, &widget, self.orientation, x, segment_length_so_far + gap_length_so_far, ll, ss)?);

            segment_length_so_far += ll;
        }



        Ok(widget_return)
    }

    /// # Build In Solid
    /// Builds the grid in a new widget and returns a tuple containing the new widget and the widget grid
    /// 
    /// [`SolidLayout`] provided is used with overwritten width and heigh parameters
    pub fn build_in_solid(&self, tree: &mut UiTree, path: impl AsRef<str>, layout: SolidLayout) -> Result<(Widget, Vec<Vec<Widget>>), LunexError> {

        let grid_size = self.compute_size();
        let grid_lenght = self.compute_lenght();

        let size_normalization = 100.0/grid_size;
        let length_normalization = 100.0/grid_lenght;

        let mut segment_length_so_far = 0.0;
        let mut gap_length_so_far = 0.0;
        if let Some(border) = self.border {
            gap_length_so_far += border[0] * length_normalization
        }
        let widget = match self.orientation {
            GridOrientation::Horizontal => Widget::create(tree, path, layout.with_width(grid_size).with_height(grid_lenght))?,
            GridOrientation::Vertical => Widget::create(tree, path, layout.with_width(grid_lenght).with_height(grid_size))?,
        };

        let mut widget_return: Vec<Vec<Widget>> = Vec::new();

        for x in 0..self.segment.len() {

            if x != 0 && x <= self.gap.len(){ gap_length_so_far += self.gap[x-1] * length_normalization }

            let ll = self.segment[x].compute_size(self.orientation) * length_normalization;
            let ss = self.segment[x].compute_lenght(self.orientation) * size_normalization;

            widget_return.push(self.segment[x].build_in_part_grid(tree, &widget, self.orientation, x, segment_length_so_far + gap_length_so_far, ll, ss)?);

            segment_length_so_far += ll;
        }



        Ok((widget, widget_return))
    }

    /// # Build In Window
    /// Builds the grid in a new widget and returns a tuple containing the new widget and the widget grid
    /// 
    /// [`WindowLayout`] provided is used with overwritten width_relative and heigh_relative parameters
    pub fn build_in_window(&self, tree: &mut UiTree, path: impl AsRef<str>, layout: WindowLayout) -> Result<(Widget, Vec<Vec<Widget>>), LunexError> {

        let grid_size = self.compute_size();
        let grid_lenght = self.compute_lenght();

        let size_normalization = 100.0/grid_size;
        let length_normalization = 100.0/grid_lenght;

        let mut segment_length_so_far = 0.0;
        let mut gap_length_so_far = 0.0;
        if let Some(border) = self.border {
            gap_length_so_far += border[0] * length_normalization
        }
        let widget = match self.orientation {
            GridOrientation::Horizontal => Widget::create(tree, path, layout.with_width_rel(grid_size).with_height_rel(grid_lenght))?,
            GridOrientation::Vertical => Widget::create(tree, path, layout.with_width_rel(grid_lenght).with_height_rel(grid_size))?,
        };

        let mut widget_return: Vec<Vec<Widget>> = Vec::new();

        for x in 0..self.segment.len() {

            if x != 0 && x <= self.gap.len(){ gap_length_so_far += self.gap[x-1] * length_normalization }

            let ll = self.segment[x].compute_size(self.orientation) * length_normalization;
            let ss = self.segment[x].compute_lenght(self.orientation) * size_normalization;

            widget_return.push(self.segment[x].build_in_part_grid(tree, &widget, self.orientation, x, segment_length_so_far + gap_length_so_far, ll, ss)?);

            segment_length_so_far += ll;
        }



        Ok((widget, widget_return))
    }

}
impl Default for Grid {
    fn default() -> Self {
        Grid {
            orientation: GridOrientation::Horizontal,
            segment: Vec::new(),
            gap: Vec::new(),
            border: None,
        }
    }
}


/// # Grid Segment
/// Represents a row/column in [`Grid`]. It's made out of vector of [`GridCell`].
/// 
/// You can specify the size of each cell, the gap between each cell, the scale and border gap.
/// 
/// Can be spawned standalone without a grid.
/// 
/// Used for creating any kind of row or column like text tabs or file hierarchies.
#[derive(Clone, Debug)]
pub struct GridSegment {
    /// If the segment should scale in length to the % of the widget. Leave None for Auto.
    pub scale: Option<f32>,
    /// The vector with the segments cells to spawn
    pub cell: Vec<GridCell>,
    /// The in-between gaps between the cells, should be n-1 of cells. Overflow is simply ignored
    pub gap: Vec<f32>,
    /// Border gaps on sides of the segment
    pub border: Option<[f32; 2]>,
}
impl GridSegment {
    /// Create a new segment from default
    pub fn new() -> Self {
        GridSegment::default()
    }

    /// Crate a new segment from the textrow, each cell matching the length of the text
    pub fn text_cells(textrow: impl AsRef<Vec<String>>) -> Self {
        let text = textrow.as_ref();
        let mut _cell = Vec::new();
        for i in 0..text.len() {
            _cell.push(
                GridCell::named(text_compute_size_simple(&text[i], 10.0), &text[i])
            );
        }
        GridSegment {
            cell: _cell,
            ..default()
        }
    }

    /// Crate a new segment with this segment copied n times
    pub fn splat_cells(cell: impl AsRef<GridCell>, n: usize) -> Self {
        let mut _cell = Vec::new();
        for _ in 0..n {
            _cell.push(cell.as_ref().to_owned());
        }
        GridSegment {
            cell: _cell,
            ..default()
        }
    }

    /// Crate a new segment with this gap copied n times
    pub fn splat_gaps(gap: f32, n: usize) -> Self {
        let mut _gap = Vec::new();
        for _ in 0..n {
            _gap.push(gap);
        }
        GridSegment {
            gap: _gap,
            ..default()
        }
    }

    /// Adds as many cells of the same size as there are gaps
    pub fn add_cells(mut self, cell: impl AsRef<GridCell>) -> Self {
        let mut _cell = Vec::new();
        for _ in 0..self.gap.len() {
            _cell.push(cell.as_ref().to_owned());
        }
        self.cell = _cell;
        self
    }

    /// Adds as many gaps of the same size as there are cells
    pub fn add_gaps(mut self, gap: f32) -> Self {
        let mut _gap = Vec::new();
        for _ in 0..self.cell.len() {
            _gap.push(gap);
        }
        self.gap = _gap;
        self
    }

    /// Set segments of the segment to the value given
    pub fn with_cells(mut self, cell: Vec<GridCell>) -> Self {
        self.cell = cell;
        self
    }

    /// Set gaps of the segment to the value given
    pub fn with_gaps(mut self, gaps: Vec<f32>) -> Self {
        self.gap = gaps;
        self
    }

    /// Set border of the segment to the value given
    pub fn with_border(mut self, border: Option<[f32; 2]>) -> Self {
        self.border = border;
        self
    }

    /// Set scale of the segment to the value given
    pub fn with_scale(mut self, scale: Option<f32>) -> Self {
        self.scale = scale;
        self
    }


    /// Iterate over and find the biggest cell and use it as size
    pub fn compute_size(&self, orientation: GridOrientation) -> f32 {
        //let orientation = GridOrientation::Horizontal;
        let mut segment_size = 0.0;
        match orientation {
            GridOrientation::Horizontal => for x in &self.cell {
                if x.size.y > segment_size { segment_size = x.size.y }
            },
            GridOrientation::Vertical => for x in &self.cell {
                if x.size.x > segment_size { segment_size = x.size.x }
            },
        }
        segment_size
    }

    /// Iterate over and sum all cells and gaps to return lenght
    pub fn compute_lenght(&self, orientation: GridOrientation) -> f32 {
        //let orientation = GridOrientation::Horizontal;
        let mut segment_lenght = 0.0;
        match orientation {
            GridOrientation::Horizontal => for x in &self.cell {
                segment_lenght += x.size.x;
            },
            GridOrientation::Vertical => for x in &self.cell {
                segment_lenght += x.size.y;
            },
        }
        //let gaps: f32 = self.gap.iter().sum();
        let mut gaps = 0.0;
        // Cannot have more gaps that cells - 1
        for x in 0..self.gap.len().min(self.cell.len()-1) {
            gaps += self.gap[x]
        }
        match self.border {
            Some (border) => segment_lenght + gaps + border[0] + border[1],
            None => segment_lenght + gaps
        }
    }

    /// Builds the grid segment in the selected widget
    pub fn build_in(&self, tree: &mut UiTree, widget: impl AsRef<Widget>, orientation: GridOrientation) -> Result<Vec<Widget>, LunexError> {
        let widget = widget.as_ref();

        let segment_size = self.compute_size(orientation);
        let segment_lenght = self.compute_lenght(orientation);

        let size_normalization = 100.0/segment_size;
        let length_normalization = if let Some(scale) = self.scale {scale } else { 100.0 }/segment_lenght;

        let mut cell_length_so_far = 0.0;
        let mut gap_length_so_far = 0.0;
        if let Some(border) = self.border {
            gap_length_so_far += border[0] * length_normalization
        }

        let mut widget_return: Vec<Widget> = Vec::new();

        match orientation {
            GridOrientation::Horizontal => for x in 0..self.cell.len() {

                if x != 0 && x <= self.gap.len(){ gap_length_so_far += self.gap[x-1] * length_normalization }

                let name = match &self.cell[x].name {
                    Some (str) => str.clone(),
                    None => format!("|{}|", x),
                };

                let ll = self.cell[x].size.x * length_normalization;

                widget_return.push(Widget::create(tree, widget.end(name), WindowLayout {
                    relative: Vec2::new(
                        cell_length_so_far + gap_length_so_far,
                        (segment_size/2.0 - self.cell[x].size.y/2.0) * size_normalization,
                    ),
                    width_relative: ll,
                    height_relative: self.cell[x].size.y * size_normalization,
                    ..default()
                })?);

                cell_length_so_far += ll;

            },

            GridOrientation::Vertical => for x in 0..self.cell.len() {

                if x != 0 && x <= self.gap.len(){ gap_length_so_far += self.gap[x-1] * length_normalization }
    
                let name = match &self.cell[x].name {
                    Some (str) => str.clone(),
                    None => format!("|{}|", x),
                };
    
                let ll = self.cell[x].size.y * length_normalization;
    
                widget_return.push(Widget::create(tree, widget.end(name), WindowLayout {
                    relative: Vec2::new(
                        (segment_size/2.0 - self.cell[x].size.x/2.0) * size_normalization,
                        cell_length_so_far + gap_length_so_far,
                    ),
                    width_relative: self.cell[x].size.x * size_normalization,
                    height_relative: ll,
                    ..default()
                })?);
    
                cell_length_so_far += ll;
    
            }
        }


        Ok(widget_return)
    }

    /// Builds the grid segment in the selected widget, but you can specify in which part
    fn build_in_part_grid(&self, tree: &mut UiTree, widget: impl AsRef<Widget>, orientation: GridOrientation, step: usize, length_pos: f32, size: f32, lenght: f32) -> Result<Vec<Widget>, LunexError> {
        let widget = widget.as_ref();

        let segment_size = self.compute_size(orientation);
        let segment_lenght = self.compute_lenght(orientation);

        let size_normalization = size/segment_size;
        let length_normalization = if let Some(scale) = self.scale {scale } else { lenght }/segment_lenght;

        let mut cell_length_so_far = 0.0;
        let mut gap_length_so_far = 0.0;
        if let Some(border) = self.border {
            gap_length_so_far += border[0] * length_normalization
        }

        let mut widget_return: Vec<Widget> = Vec::new();

        match orientation {
            GridOrientation::Horizontal => for x in 0..self.cell.len() {

                if x != 0 && x <= self.gap.len(){ gap_length_so_far += self.gap[x-1] * length_normalization }

                let name = match &self.cell[x].name {
                    Some (str) => str.clone(),
                    None => format!("|{}-{}|", step, x),
                };

                let ll = self.cell[x].size.x * length_normalization;

                widget_return.push(Widget::create(tree, widget.end(name), WindowLayout {
                    relative: Vec2::new(
                        cell_length_so_far + gap_length_so_far,
                        length_pos + (segment_size/2.0 - self.cell[x].size.y/2.0) * size_normalization,
                    ),
                    width_relative: ll,
                    height_relative: self.cell[x].size.y * size_normalization,
                    ..default()
                })?);

                cell_length_so_far += ll;

            },

            GridOrientation::Vertical => for x in 0..self.cell.len() {

                if x != 0 && x <= self.gap.len(){ gap_length_so_far += self.gap[x-1] * length_normalization }
    
                let name = match &self.cell[x].name {
                    Some (str) => str.clone(),
                    None => format!("|{}-{}|", step, x),
                };
    
                let ll = self.cell[x].size.y * length_normalization;
    
                widget_return.push(Widget::create(tree, widget.end(name), WindowLayout {
                    relative: Vec2::new(
                        length_pos + (segment_size/2.0 - self.cell[x].size.x/2.0) * size_normalization,
                        cell_length_so_far + gap_length_so_far,
                    ),
                    width_relative: self.cell[x].size.x * size_normalization,
                    height_relative: ll,
                    ..default()
                })?);
    
                cell_length_so_far += ll;
    
            }
        }


        Ok(widget_return)
    }
}
impl Default for GridSegment {
    fn default() -> Self {
        GridSegment {
            scale: None,
            cell: Vec::new(),
            gap: Vec::new(),
            border: None,
        }
    }
}
impl AsRef<GridSegment> for GridSegment {
    fn as_ref(&self) -> &GridSegment {
        &self
    }
}
impl AsMut<GridSegment> for GridSegment {
    fn as_mut(&mut self) -> &mut GridSegment {
        self
    }
}


/// # Grid Cell
/// Represents a cell in [`GridSegment`]. You can define it's size, name and alignment (WIP).
#[derive(Clone, Debug)]
pub struct GridCell {
    /// The relative size of the cell, X is always width no matter the orientation. Same for Y
    pub size: Vec2,
    /// Optional name of the cell to be used to name the grid, otherwise the index is used.
    pub name: Option<String>,
    /// Cell alignment within the segment. Shows only when some cells are bigger than other.
    pub align: GridAlign,
}
impl GridCell {
    pub fn new() -> GridCell {
        GridCell::default()
    }
    pub fn sized(size: Vec2) -> GridCell {
        GridCell {
            size,
            ..default()
        }
    }
    pub fn named(size: Vec2, name: impl AsRef<str>) -> GridCell {
        GridCell {
            size,
            name: Some(name.as_ref().into()),
            ..default()
        }
    }
}
impl Default for GridCell{
    fn default() -> GridCell {
        GridCell {
            size: Vec2::splat(10.0),
            name: None,
            align: GridAlign::Middle,
        }
    }
}
impl AsRef<GridCell> for GridCell {
    fn as_ref(&self) -> &GridCell {
        &self
    }
}
impl AsMut<GridCell> for GridCell {
    fn as_mut(&mut self) -> &mut GridCell {
        self
    }
}

/// Decides if [`GridSegment`] is row or column
#[derive(Clone, Debug, Copy)]
pub enum GridOrientation {
    Horizontal,
    Vertical,
}

/// Alignment of [`GridCell`] in a [`GridSegment`]. Currently WIP.
#[derive(Clone, Debug, Copy)]
pub enum GridAlign {
    Start,
    Middle,
    End,
}





// ===========================================================
// === GRID MACROS ===

/// # Text Row
/// Attempts to construct 1D vector from given elements, useful when you don't want to type `.to_string()` every time.
/// ```
/// let row: Vec<String> = textrow!["item 1", "item 2", "item 3"];
/// ```
#[macro_export]
macro_rules! textrow {
    [$($element:expr),*] => {{
        vec![$($element.to_string()),*]
    }};
}

/// # Text Grid
/// Attempts to construct 2D vector from given elements, useful when you don't want to type `.to_string()` every time.
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
