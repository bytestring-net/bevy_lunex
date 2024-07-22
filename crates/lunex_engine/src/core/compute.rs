use bevy::ecs::component::Component;

use crate::import::*;
use crate::NodeGeneralTrait;
use crate::NodeTopDataTrait;
use crate::UiNode;
use crate::UiTree;
use crate::Rectangle3D;
use crate::Layout;

/// Trait with [`UiTree`] layout computation methods.
pub trait UiNodeTreeComputeTrait {
    /// Compute the layout of the [`UiTree`].
    fn compute(&mut self, parent: Rectangle3D);
}
impl <T, N: Default + Component> UiNodeTreeComputeTrait for UiTree<T, N> {
    fn compute(&mut self, parent: Rectangle3D) {

        let mut abs_scale = 1.0;
        let mut font_size = 16.0;

        if let Some(master_data) = self.obtain_topdata() {
            abs_scale = master_data.abs_scale;
            font_size = master_data.font_size;
        }

        self.node.compute_all(parent, abs_scale, parent.size, font_size);
    }
}


/// Trait with [`UiNode`] layout computation methods. Includes private methods.
trait UiNodeComputeTrait {
    fn compute_all(&mut self, parent: Rectangle3D, absolute_scale: f32, viewport_size: Vec2, font_size: f32);
    //fn compute_content(&mut self, ancestor_size: Vec2, ancestor_padding: Vec4, abs_scale: f32, font_size: f32) -> Vec2;
    //fn compute_stack(&mut self, ancestor_size: Vec2, ancestor_padding: Vec4, abs_scale: f32, font_size: f32, horizontal: bool) -> Vec2;
    //fn align_stack(&mut self, ancestor_position: Vec2);
}
impl <N:Default + Component> UiNodeComputeTrait for UiNode<N> { 
    /// Triggers the recursion in the right manner.
    fn compute_all(&mut self, parent: Rectangle3D, absolute_scale: f32, viewport_size: Vec2, mut font_size: f32) {

        // Get depth before mutating self
        let depth = self.get_depth();
        
        let skip = true;
        let is_parametric = false;

        // Check here if computation is required for partial recalculation

        // Compute my layout and return computed rectangle for recursion
        let my_rectangle = if let Some(node_data) = &mut self.data {

            // Overwrite passed style with font size
            if let Some(fnt) = node_data.font_size { font_size = fnt }

            // Compute node layout

            let layout_0 = node_data.layout.get(&node_data.layout_index[0]).unwrap_or(node_data.layout.get(&0).unwrap());
            let layout_0: Option<Rectangle3D> = match layout_0 {
                Layout::Div(_) => {
                    None
                },
                Layout::Boundary(l) => {
                    Some(l.compute(parent.into(), absolute_scale, viewport_size, font_size).into())
                },
                Layout::Window(l) => {
                    Some(l.compute(parent.into(), absolute_scale, viewport_size, font_size).into())
                },
                Layout::Solid(l)  => {
                    Some(l.compute(parent.into(), absolute_scale, viewport_size, font_size).into())
                },
            };

            let layout_1 = node_data.layout.get(&node_data.layout_index[1]).unwrap_or(node_data.layout.get(&0).unwrap());
            let layout_1: Option<Rectangle3D> = match layout_1 {
                Layout::Div(_) => {
                    None
                },
                Layout::Boundary(l) => {
                    Some(l.compute(parent.into(), absolute_scale, viewport_size, font_size).into())
                },
                Layout::Window(l) => {
                    Some(l.compute(parent.into(), absolute_scale, viewport_size, font_size).into())
                },
                Layout::Solid(l)  => {
                    Some(l.compute(parent.into(), absolute_scale, viewport_size, font_size).into())
                },
            };

            /* match &node_data.layout {
                Layout::Div(_) => {
                    is_parametric = true;
                },
                Layout::Boundary(l) => {
                    skip = false;
                },
                Layout::Window(l) => {
                    skip = false;
                },
                Layout::Solid(l)  => {
                    skip = false;
                },
            } */

            if let Some(l0) = layout_0 {
                if let Some(l1) = layout_1 {
                    node_data.rectangle = l0.lerp(l1, node_data.layout_tween);
                };
            };

            // Adding depth
            node_data.rectangle.pos.z = (depth + node_data.depth_bias)*absolute_scale;
            node_data.rectangle

        } else { return; };

        if skip == false {
            if is_parametric {
                //compute divs with inherited scale
                //self.compute_content(parent.size, Vec4::ZERO, absolute_scale, font_size);
            } else {
                //compute divs with my rectangle scale
                //self.compute_content(my_rectangle.size, Vec4::ZERO, absolute_scale, font_size);
            }
        }

        // Enter recursion
        for (_, subnode) in &mut self.nodes {
            subnode.compute_all(my_rectangle, absolute_scale, viewport_size, font_size);
        }
    }
    /* /// Computes the content only.
    fn compute_content(&mut self, ancestor_size: Vec2, ancestor_padding: Vec4, abs_scale: f32, font_size: f32) -> Vec2 {

        let stack_options = self.data.as_ref().unwrap().stack;

        match stack_options.direction {
            StackDirection::Horizontal => self.compute_stack(ancestor_size, ancestor_padding, abs_scale, font_size, true),
            StackDirection::Vertical => self.compute_stack(ancestor_size, ancestor_padding, abs_scale, font_size, false),
        }
    }
    /// This will compute the stack and position nodes ONLY locally as if every matrix starts at 0,0.
    /// Secondary pass after alignment of parent nodes is required.
    fn compute_stack(&mut self, ancestor_size: Vec2, ancestor_padding: Vec4, abs_scale: f32, font_size: f32, horizontal: bool) -> Vec2 {

        let mut matrix: Vec<Vec<&mut Node<NodeData<N>>>> = Vec::new();
        let mut content_size = Vec2::ZERO;

        // Sort mutable pointers into matrix
        let mut i = 0;
        matrix.push(Vec::new());
        for (_, subnode) in &mut self.nodes {
            if let Some(subnode_data) = &subnode.data {
                if let Layout::Div(layout) = &subnode_data.layout {
                    let br = layout.force_break;
                    matrix[i].push(subnode);
                    if br {
                        i += 1;
                        matrix.push(Vec::new());
                    }
                }
            }
        }


        // INSIDE MATRIX =================================================================

        let gap = self.data.as_ref().unwrap().stack.gap.evaluate(abs_scale, ancestor_size, font_size);
        let align = self.data.as_ref().unwrap().stack.node_alignment.0;


        let mut line_cursor = if horizontal { ancestor_padding.y } else { ancestor_padding.x };

        //--------------------------//
        let mut _i = 0;             //
        let _i_max = matrix.len();  //
        for line in &mut matrix {   //
            // INSIDE LINE =================================================================

            // Register that is shared between the two passes
            let mut comline = ComputedLine {
                divs: Vec::new(),
                line_length: 0.0,
            };

            // First pass to compute sizes--//
            let mut _ii = 0;                //
            let _ii_max = line.len();       //
            for subnode in &mut *line {     //
                // INSIDE SUBNODE =================================================================

                // Fetch data
                let subnode_data = subnode.data.as_ref().unwrap();
                let layout = if let Layout::Div(layout) = subnode_data.layout { layout } else { unreachable!() };

                // Get padding & margin => compute range of motion
                let padding = layout.compute_padding(ancestor_size, abs_scale, font_size);
                let margin = layout.compute_margin(ancestor_size, abs_scale, font_size);
                let border = layout.compute_border(ancestor_size, abs_scale, font_size);

                // Enter recursion to get the right content size
                let potential_content = subnode.compute_content(ancestor_size, padding, abs_scale, font_size);

                // Fetch data again, because they were modified
                let subnode_data = subnode.data.as_mut().unwrap();
                let mut subnode_content = subnode_data.content_size;

                // Overwrite subnode content if div contains no subdivs
                if potential_content != Vec2::ZERO { subnode_content = potential_content }

                // Compute size and (line_length)
                let size = layout.compute_size(subnode_content, padding, border);
                let line_length = if horizontal { margin.y + size.y + margin.w } else { margin.x + size.x + margin.z };

                // Push into register
                comline.line_length = f32::max(comline.line_length, line_length);
                comline.divs.push(ComputedDiv { size, margin });


                // END OF INSIDE SUBNODE =================================================================
                _ii += 1;
            }

            if _i != 0 { line_cursor += if horizontal { gap.x } else { gap.y } }
            let mut cursor = if horizontal { ancestor_padding.x } else { ancestor_padding.y };

            // Second pass to align them----//
            let mut _ii = 0;                //
            let _ii_max = line.len();       //
            for subnode in &mut *line {     //
                // INSIDE SUBNODE =================================================================

                // Fetch data
                let subnode_data = subnode.data.as_ref().unwrap();
                let layout = if let Layout::Div(layout) = subnode_data.layout { layout } else { unreachable!() };
                
                let margin = comline.divs[_ii].margin;
                let size = comline.divs[_ii].size;

                let possible_size = if horizontal {comline.line_length - margin.y - margin.w } else {comline.line_length - margin.x - margin.z };

                let mut my_align = align;
                let my_offset;
                
                if horizontal {
                    if let Some(align) = layout.align_y { my_align = align.0 }

                    if _ii != 0 { cursor += gap.x }
                    cursor += margin.x;
                    let off = margin.y + possible_size/2.0 - size.y/2.0;
                    my_offset = Vec2::new(cursor, off + (off - margin.x) * my_align);
                    cursor += size.x;
                    cursor += margin.z;

                } else {
                    if let Some(align) = layout.align_x { my_align = align.0 }

                    if _ii != 0 { cursor += gap.y }
                    cursor += margin.y;
                    let off = margin.x + possible_size/2.0 - size.x/2.0;
                    my_offset = Vec2::new(off + (off - margin.x) * my_align, cursor);
                    cursor += size.y;
                    cursor += margin.w;
                };
                

                // Fetch data again, because they were modified
                let subnode_data = subnode.data.as_mut().unwrap();

                let _xxx = my_offset + if horizontal { Vec2::new(0.0, line_cursor) } else { Vec2::new(line_cursor, 0.0) };
                subnode_data.rectangle.pos.x = _xxx.x;
                subnode_data.rectangle.pos.y = _xxx.y;
                subnode_data.rectangle.size = size;


                let subnode_data = subnode.data.as_ref().unwrap();
                subnode.align_stack(subnode_data.rectangle.pos.xy());


                // END OF INSIDE SUBNODE =================================================================
                _ii += 1;
            }

            // Set content size
            if horizontal {
                content_size.x = f32::max(content_size.x, cursor - ancestor_padding.x)
            } else {
                content_size.y = f32::max(content_size.y, cursor - ancestor_padding.y)
            }

            line_cursor += comline.line_length;

            // END OF INSIDE LINE =================================================================
            _i += 1;
        }

        // Set content size
        if horizontal {
            content_size.y = line_cursor - ancestor_padding.y
        } else {
            content_size.x = line_cursor - ancestor_padding.x
        }
        
        // END OF INSIDE MATRIX =========================================================
        content_size
    }
    /// This is the secondary pass to align the nodes.
    fn align_stack(&mut self, ancestor_position: Vec2) {

        for (_, subnode) in &mut self.nodes {
            if let Some(subnode_data) = &mut subnode.data {
                if let Layout::Div(_) = &subnode_data.layout {

                    subnode_data.rectangle.pos.x += ancestor_position.x;
                    subnode_data.rectangle.pos.y += ancestor_position.y;

                }
                subnode.align_stack(ancestor_position);
            }
        }

    } */
}
/* 
struct ComputedDiv {
    size: Vec2,
    margin: Vec4,
}
struct ComputedLine {
    divs: Vec<ComputedDiv>,
    line_length: f32,
}
 */