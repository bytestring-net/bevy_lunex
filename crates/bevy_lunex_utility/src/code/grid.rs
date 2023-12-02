use std::borrow::Borrow;

use bevy::prelude::*;

use bevy_lunex_core::{UiTree, Widget, WindowLayout, SolidLayout, LunexError};

use super::element::text_compute_size_simple;

// ===========================================================
// === GRID DEFINITION ===

/// # Grid
/// It's made out of vector of [`GridSegment`]. Depending on the grid orientation, they are either columns next to each other or rows under each other.
/// 
/// You can specify the segments, the gap between each segment, the orientation and border gap.
/// 
/// Used for creating any kind of 2D grid like item inventories.
/// 
/// The grid is defined in `units` and is scaled depending on the build function you use.
/// 
/// Grid can be for example 40x30 units depending on the number of cells, their size and gaps.
/// * If you use `build_in` the 40x30 is stretched to 100% of the widget.
/// * If you use `build_in_window` the 40x30 corresponds to 40% x 30% of the parenting widget.
/// * If you use `build_in_solid` the 40x30 grid is created inside [`SolidLayout`] widget of the same proportions.
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
    pub fn splat_segment(segment: impl Borrow<GridSegment>, n: usize) -> Self {
        let mut _segment = Vec::new();
        for _ in 0..n {
            _segment.push(segment.borrow().to_owned());
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
    pub fn add_segments(mut self, segment: impl Borrow<GridSegment>) -> Self {
        let mut _segment = Vec::new();
        for _ in 0..self.gap.len() {
            _segment.push(segment.borrow().to_owned());
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
    pub fn build_in<T:Default>(&self, tree: &mut UiTree<T>, widget: impl Borrow<Widget>) -> Result<Vec<Vec<Widget>>, LunexError> {
        let widget = widget.borrow();

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

            widget_return.push(self.segment[x].build_in_part_grid(tree, widget, self.orientation, x, segment_length_so_far + gap_length_so_far, ll, ss)?);

            segment_length_so_far += ll;
        }



        Ok(widget_return)
    }

    /// # Build In Solid
    /// Builds the grid in a new widget and returns a tuple containing the new widget and the widget grid
    /// 
    /// [`SolidLayout`] provided is used with overwritten width and heigh parameters
    pub fn build_in_solid<T:Default>(&self, tree: &mut UiTree<T>, path: impl Borrow<str>, layout: SolidLayout) -> Result<(Widget, Vec<Vec<Widget>>), LunexError> {

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
    pub fn build_in_window<T:Default>(&self, tree: &mut UiTree<T>, path: impl Borrow<str>, layout: WindowLayout) -> Result<(Widget, Vec<Vec<Widget>>), LunexError> {

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
            GridOrientation::Horizontal => Widget::create(tree, path, layout.size_rel((grid_size, grid_lenght)))?,
            GridOrientation::Vertical => Widget::create(tree, path, layout.size_rel((grid_lenght, grid_size)))?,
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
///
/// The segment is defined in `units` and is scaled depending on the build function you use.
/// 
/// Segment can be for example 40 units long depending on the number of cells, their size and gaps.
/// * If you use `build_in` the 40 units segment is stretched to 100% of the widget.
/// * If you use `build_in_window` the 40 units corresponds to 40% of the parenting widget.
/// * If you use `build_in_solid` the 40 units segment is created inside [`SolidLayout`] widget of the same proportions.
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
    pub fn text_cells(textrow: impl Borrow<Vec<String>>, text_size: f32, text_scale: f32) -> Self {
        let text = textrow.borrow();
        let mut _cell = Vec::new();
        for i in 0..text.len() {
            let mut estimate = text_compute_size_simple(&text[i], text_size);
            estimate.x /= 100.0/text_scale;
            estimate.x += 2.0 * estimate.y/(100.0/text_scale);

            _cell.push(
                GridCell::named(estimate, text[i].clone())
            );
        }
        GridSegment {
            cell: _cell,
            ..default()
        }
    }

    /// Crate a new segment with this segment copied n times
    pub fn splat_cells(cell: impl Borrow<GridCell>, n: usize) -> Self {
        let mut _cell = Vec::new();
        for _ in 0..n {
            _cell.push(cell.borrow().to_owned());
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

    /// Add cell to the segment
    pub fn add_cell(&mut self, cell: impl Borrow<GridCell>) {
        self.cell.push(cell.borrow().to_owned());
    }

    /// Adds as many cells of the same size as there are gaps
    pub fn add_cells(mut self, cell: impl Borrow<GridCell>) -> Self {
        let mut _cell = Vec::new();
        for _ in 0..self.gap.len() {
            _cell.push(cell.borrow().to_owned());
        }
        self.cell = _cell;
        self
    }

    /// Add cell to the segment
    pub fn add_gap(&mut self, gap: f32) {
        self.gap.push(gap);
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

    /// # Build In
    /// Builds the grid segment in the selected widget and returns the widget list
    pub fn build_in<T:Default>(&self, tree: &mut UiTree<T>, widget: impl Borrow<Widget>, orientation: GridOrientation) -> Result<Vec<Widget>, LunexError> {
        let widget = widget.borrow();

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
                    pos_relative: Vec2::new(
                        cell_length_so_far + gap_length_so_far,
                        (segment_size/2.0 - self.cell[x].size.y/2.0) * size_normalization,
                    ),
                    size_relative: (ll, self.cell[x].size.y * size_normalization).into(),
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
                    pos_relative: Vec2::new(
                        (segment_size/2.0 - self.cell[x].size.x/2.0) * size_normalization,
                        cell_length_so_far + gap_length_so_far,
                    ),
                    size_relative: (self.cell[x].size.x * size_normalization, ll).into(),
                    ..default()
                })?);
    
                cell_length_so_far += ll;
    
            }
        }


        Ok(widget_return)
    }

    /// # Build In Solid
    /// Builds the grid segment in a new widget and returns a tuple containing the new widget and the widget list
    /// 
    /// [`SolidLayout`] provided is used with overwritten width and heigh parameters
    pub fn build_in_solid<T:Default>(&self, tree: &mut UiTree<T>, path: impl Borrow<str>, layout: SolidLayout, orientation: GridOrientation) -> Result<(Widget, Vec<Widget>), LunexError> {

        let segment_size = self.compute_size(orientation);
        let segment_lenght = self.compute_lenght(orientation);

        let size_normalization = 100.0/segment_size;
        let length_normalization = if let Some(scale) = self.scale {scale } else { 100.0 }/segment_lenght;

        let mut cell_length_so_far = 0.0;
        let mut gap_length_so_far = 0.0;
        if let Some(border) = self.border {
            gap_length_so_far += border[0] * length_normalization
        }

        let widget = match orientation {
            GridOrientation::Horizontal => Widget::create(tree, path, layout.with_width(segment_lenght).with_height(segment_size))?,
            GridOrientation::Vertical => Widget::create(tree, path, layout.with_width(segment_size).with_height(segment_lenght))?,
        };

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
                    pos_relative: Vec2::new(
                        cell_length_so_far + gap_length_so_far,
                        (segment_size/2.0 - self.cell[x].size.y/2.0) * size_normalization,
                    ),
                    size_relative: (ll, self.cell[x].size.y * size_normalization).into(),
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
                    pos_relative: Vec2::new(
                        (segment_size/2.0 - self.cell[x].size.x/2.0) * size_normalization,
                        cell_length_so_far + gap_length_so_far,
                    ),
                    size_relative: (self.cell[x].size.x * size_normalization, ll).into(),
                    ..default()
                })?);
    
                cell_length_so_far += ll;
    
            }
        }


        Ok((widget, widget_return))
    }

    /// # Build In Window
    /// Builds the grid segment in a new widget and returns a tuple containing the new widget and the widget list
    /// 
    /// [`WindowLayout`] provided is used with overwritten width_relative and heigh_relative parameters
    pub fn build_in_window<T:Default>(&self, tree: &mut UiTree<T>, path: impl Borrow<str>, layout: WindowLayout, orientation: GridOrientation) -> Result<(Widget, Vec<Widget>), LunexError> {

        let segment_size = self.compute_size(orientation);
        let segment_lenght = self.compute_lenght(orientation);

        let size_normalization = 100.0/segment_size;
        let length_normalization = if let Some(scale) = self.scale {scale } else { 100.0 }/segment_lenght;

        let mut cell_length_so_far = 0.0;
        let mut gap_length_so_far = 0.0;
        if let Some(border) = self.border {
            gap_length_so_far += border[0] * length_normalization
        }

        let widget = match orientation {
            GridOrientation::Horizontal => Widget::create(tree, path, layout.size_rel((segment_lenght, segment_size)))?,
            GridOrientation::Vertical => Widget::create(tree, path, layout.size_rel((segment_size, segment_lenght)))?,
        };

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
                    pos_relative: Vec2::new(
                        cell_length_so_far + gap_length_so_far,
                        (segment_size/2.0 - self.cell[x].size.y/2.0) * size_normalization,
                    ),
                    size_relative: (ll, self.cell[x].size.x * size_normalization).into(),
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
                    pos_relative: Vec2::new(
                        (segment_size/2.0 - self.cell[x].size.x/2.0) * size_normalization,
                        cell_length_so_far + gap_length_so_far,
                    ),
                    size_relative: (self.cell[x].size.x * size_normalization, ll).into(),
                    ..default()
                })?);
    
                cell_length_so_far += ll;
    
            }
        }


        Ok((widget, widget_return))
    }

    /// # Build In Window Absolute
    /// Builds the grid segment in a new widget and returns a tuple containing the new widget and the widget list
    /// 
    /// [`WindowLayout`] provided is used with overwritten width_absolute and heigh_relative parameters
    pub fn build_in_window_absolute<T:Default>(&self, tree: &mut UiTree<T>, path: impl Borrow<str>, layout: WindowLayout, orientation: GridOrientation) -> Result<(Widget, Vec<Widget>), LunexError> {

        let segment_size = self.compute_size(orientation);
        let segment_lenght = self.compute_lenght(orientation);

        let size_normalization = 100.0/segment_size;
        let length_normalization = if let Some(scale) = self.scale {scale } else { 100.0 }/segment_lenght;

        let mut cell_length_so_far = 0.0;
        let mut gap_length_so_far = 0.0;
        if let Some(border) = self.border {
            gap_length_so_far += border[0] * length_normalization
        }

        let widget = match orientation {
            GridOrientation::Horizontal => Widget::create(tree, path, layout.size_abs((segment_lenght, segment_size)))?,
            GridOrientation::Vertical => Widget::create(tree, path, layout.size_abs((segment_size, segment_lenght)))?,
        };

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
                    pos_relative: Vec2::new(
                        cell_length_so_far + gap_length_so_far,
                        (segment_size/2.0 - self.cell[x].size.y/2.0) * size_normalization,
                    ),
                    size_relative: (ll, self.cell[x].size.x * size_normalization).into(),
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
                    pos_relative: Vec2::new(
                        (segment_size/2.0 - self.cell[x].size.x/2.0) * size_normalization,
                        cell_length_so_far + gap_length_so_far,
                    ),
                    size_relative: (self.cell[x].size.x * size_normalization, ll).into(),
                    ..default()
                })?);
    
                cell_length_so_far += ll;
    
            }
        }


        Ok((widget, widget_return))
    }


    /// Builds the grid segment in the selected widget, but you can specify in which part
    fn build_in_part_grid<T:Default>(&self, tree: &mut UiTree<T>, widget: impl Borrow<Widget>, orientation: GridOrientation, step: usize, length_pos: f32, size: f32, lenght: f32) -> Result<Vec<Widget>, LunexError> {
        let widget = widget.borrow();

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
                    pos_relative: Vec2::new(
                        cell_length_so_far + gap_length_so_far,
                        length_pos + (segment_size/2.0 - self.cell[x].size.y/2.0) * size_normalization,
                    ),
                    size_relative: (ll, self.cell[x].size.x * size_normalization).into(),
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
                    pos_relative: Vec2::new(
                        length_pos + (segment_size/2.0 - self.cell[x].size.x/2.0) * size_normalization,
                        cell_length_so_far + gap_length_so_far,
                    ),
                    size_relative: (self.cell[x].size.x * size_normalization, ll).into(),
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
    pub fn named(size: Vec2, name: impl Borrow<str>) -> GridCell {
        GridCell {
            size,
            name: Some(name.borrow().into()),
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
