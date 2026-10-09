use crate::*;

// Exported prelude
pub mod prelude {
    // All standard exports
    pub use super::UiFlowText;
}


// #===========================#
// #=== TEXT FLOW SIZING CONTROL ===#

/// **Ui Flow Text** - Attach this component to a flow text node to opt into flow-aware text sizing.
/// When `wrap` is enabled, the node's assigned width is fed back into [`TextBounds`] each recompute,
/// causing the text to re-wrap to the width the flow engine gave it. The layout then settles
/// on the next recompute (the wrapped height feeds back into the flow).
///
/// Affected components:
/// - [`TextBounds`] - The maximum text width is overwritten to match the flow-assigned width
///
/// ## 🛠️ Example
/// ```
/// # use bevy_ecs::prelude::*; use bevy_asset::prelude::*; use bevy_lunex::prelude::*; use bevy_text::prelude::*; use bevy_sprite::prelude::*;
/// # fn spawn_text(mut commands: Commands) {
/// commands.spawn((
///     UiLayout::flow().width(UiFlowSize::Fit).pack(),
///     UiFlowText::wrapped(),
///     Text2d::new("Some wrapping paragraph"),
/// ));
/// # }
/// ```
#[derive(Component, Reflect, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct UiFlowText {
    /// Whether the text should wrap to the width assigned by the flow engine.
    pub wrap: bool,
}
impl UiFlowText {
    /// Creates text flow sizing with wrapping enabled.
    pub fn wrapped() -> Self {
        Self { wrap: true }
    }
    /// Creates text flow sizing with wrapping disabled.
    pub fn unwrapped() -> Self {
        Self { wrap: false }
    }
}


// #=========================#
// #=== THE FLOW ALGORITHM ===#

/// Epsilon used for float comparisons.
const FLOW_EPSILON: f32 = 0.01;

/// Maximum re-runs of the layout pipeline when wrapping is involved. A wrapping
/// container's cross size is derived top-down and propagates at most one ancestor
/// level per iteration; three covers the practical nesting depth (wrap container
/// -> Fit parent -> grandparent), and the early exit keeps non-wrap trees at one.
const FIXPOINT_MAX_ITERATIONS: usize = 3;

/// Evaluation context passed through the flow algorithm.
#[derive(Clone, Copy, Debug)]
pub(crate) struct UiFlowContext {
    /// Scale applied to absolute units (from [`UiLayoutRoot::abs_scale`]).
    pub abs_scale: f32,
    /// Size of the layout root (viewport) used by viewport units.
    pub viewport: Vec2,
    /// Font size used by the `Em` unit.
    pub font_size: f32,
}

/// Gets a component of a vector by axis index (`0` = x, `1` = y).
fn axis_get(v: Vec2, axis: usize) -> f32 {
    if axis == 0 { v.x } else { v.y }
}
/// Sets a component of a vector by axis index (`0` = x, `1` = y).
fn axis_set(v: &mut Vec2, axis: usize, value: f32) {
    if axis == 0 { v.x = value } else { v.y = value }
}
/// Converts an [`Align`] in the `-1.0` to `1.0` range into a `0.0` to `1.0` factor.
fn align_factor(align: Align) -> f32 {
    (align.0 + 1.0) / 2.0
}
/// Maps a flow direction to its main axis index (`0` = horizontal, `1` = vertical).
fn axis_of(direction: UiFlowDirection) -> usize {
    if direction.is_horizontal() { 0 } else { 1 }
}

/// Whether the container packs its children onto multiple lines (wrapping or grid).
fn is_wrap_mode(config: &UiLayoutTypeFlow) -> bool {
    config.wrap || !config.grid.is_empty()
}

/// Greedily packs main-axis footprints into lines that fit the given inner extent.
/// Items wider than a whole line get a line of their own.
fn pack_lines_greedy(footprints: &[f32], gap: f32, inner_main: f32) -> Vec<Vec<usize>> {
    let mut lines: Vec<Vec<usize>> = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    let mut current_main = 0.0;
    for (k, &fp) in footprints.iter().enumerate() {
        let next = if current.is_empty() { fp } else { current_main + gap + fp };
        if next > inner_main + FLOW_EPSILON && !current.is_empty() {
            lines.push(std::mem::take(&mut current));
            current_main = fp;
        } else {
            current_main = next;
        }
        current.push(k);
    }
    if !current.is_empty() { lines.push(current) }
    if lines.is_empty() { lines.push(Vec::new()) }
    lines
}

/// Groups child indices into grid lines of `tracks` items per line. With `grid_wrap`
/// disabled, all children stay on a single line.
fn pack_lines_grid(child_count: usize, tracks: usize, grid_wrap: bool) -> Vec<Vec<usize>> {
    let all: Vec<usize> = (0..child_count).collect();
    if grid_wrap { all.chunks(tracks.max(1)).map(|chunk| chunk.to_vec()).collect() } else { vec![all] }
}

/// Extracts the leftover-space claim weight of a node's sizing on an axis:
/// `Grow` claims one share, `Fixed` claims the value of its [`Sp`] component, `Fit` claims nothing.
fn size_sp_weight(config: &UiLayoutTypeFlow, axis: usize) -> f32 {
    match if axis == 0 { config.width.size } else { config.height.size } {
        UiFlowSize::Fit => 0.0,
        UiFlowSize::Grow => 1.0,
        UiFlowSize::Fixed(v) => v.sp_weight(),
    }
}

/// Picks the two margin sides lying on the given axis (`0` = left/right, `1` = top/bottom).
fn margin_sides(margin: &UiFlowPadding, axis: usize) -> (&UiValue<f32>, &UiValue<f32>) {
    if axis == 0 { (&margin.left, &margin.right) } else { (&margin.top, &margin.bottom) }
}

/// A margin side split into its fixed part (all units except [`Sp`]) and its `Sp` weight.
#[derive(Clone, Copy, Default, Debug)]
struct MarginPart {
    fixed: f32,
    weight: f32,
}

/// Computes the effective margins of a child node: each side is the child's own margin when
/// defined, otherwise the default `Sp` margin template derived from the parent's `justify`
/// (main axis) and `align` (cross axis) settings. Children that claim leftover space through
/// their sizing on an axis (`Grow` or `Fixed` with an [`Sp`] component) do not inherit the
/// template on that axis - they fill the space instead, and only their own margins apply.
fn effective_margins(own: &UiFlowPadding, parent: &UiLayoutTypeFlow, child: &UiLayoutTypeFlow, is_first: bool, is_last: bool) -> UiFlowPadding {
    let horizontal = parent.direction.is_horizontal();
    let (main_axis, cross_axis) = if horizontal { (0, 1) } else { (1, 0) };

    // Main-axis template from `justify` (start side, end side).
    let mut main = match parent.justify {
        UiJustify::Start => (0.0, 0.0),
        UiJustify::Center => (if is_first { 1.0 } else { 0.0 }, if is_last { 1.0 } else { 0.0 }),
        UiJustify::End => (if is_first { 1.0 } else { 0.0 }, 0.0),
        UiJustify::SpaceBetween => (if is_first { 0.0 } else { 1.0 }, 0.0),
        UiJustify::SpaceEvenly => (1.0, if is_last { 1.0 } else { 0.0 }),
        UiJustify::SpaceAround => (1.0, 1.0),
    };
    // Cross-axis template from the continuous `Align`: weights (f, 1 - f).
    let f = align_factor(parent.align);
    let mut cross = (f, 1.0 - f);
    // Sizing claims override the templates on their axis.
    if size_sp_weight(child, main_axis) > 0.0 { main = (0.0, 0.0) }
    if size_sp_weight(child, cross_axis) > 0.0 { cross = (0.0, 0.0) }

    let ((l, r), (t, b)) = if horizontal { (main, cross) } else { (cross, main) };
    let inject = |side: &UiValue<f32>, weight: f32| {
        if *side == UiValue::new() && weight > 0.0 { UiValue::from_sp(weight) } else { *side }
    };
    UiFlowPadding {
        left: inject(&own.left, l),
        right: inject(&own.right, r),
        top: inject(&own.top, t),
        bottom: inject(&own.bottom, b),
    }
}

/// Evaluates padding with relative units dropped (used before the node's own size is known).
fn eval_padding_intrinsic(config: &UiLayoutTypeFlow, ctx: &UiFlowContext) -> (f32, f32, f32, f32) {
    let l = config.padding.left.evaluate_intrinsic(ctx.abs_scale, ctx.viewport, ctx.font_size, 0);
    let r = config.padding.right.evaluate_intrinsic(ctx.abs_scale, ctx.viewport, ctx.font_size, 0);
    let t = config.padding.top.evaluate_intrinsic(ctx.abs_scale, ctx.viewport, ctx.font_size, 1);
    let b = config.padding.bottom.evaluate_intrinsic(ctx.abs_scale, ctx.viewport, ctx.font_size, 1);
    (l, r, t, b)
}

/// Evaluates padding fully, resolving relative units against the node's own size.
fn eval_padding_full(config: &UiLayoutTypeFlow, own: Vec2, ctx: &UiFlowContext) -> (f32, f32, f32, f32) {
    let l = config.padding.left.evaluate_axis(ctx.abs_scale, own, ctx.viewport, ctx.font_size, 0);
    let r = config.padding.right.evaluate_axis(ctx.abs_scale, own, ctx.viewport, ctx.font_size, 0);
    let t = config.padding.top.evaluate_axis(ctx.abs_scale, own, ctx.viewport, ctx.font_size, 1);
    let b = config.padding.bottom.evaluate_axis(ctx.abs_scale, own, ctx.viewport, ctx.font_size, 1);
    (l, r, t, b)
}

/// One node inside the flow tree. The tree is a flat arena of these, linked by indices.
#[doc(hidden)]
pub struct FlowItem {
    /// The entity this item was extracted from.
    pub entity: Entity,
    /// Index of the flow parent, if any. `None` marks a maximal flow subtree root.
    pub parent: Option<usize>,
    /// Indices of the flow children.
    pub children: Vec<usize>,
    /// The state-blended flow configuration snapshot.
    pub config: UiLayoutTypeFlow,
    /// Effective margins: the node's own margins with the parent's `align`/`justify`
    /// template filled in on undefined sides.
    pub margin: UiFlowPadding,
    /// Resolved numeric margins `[left, right, top, bottom]` after leftover-space distribution.
    pub resolved_margin: [f32; 4],
    /// Measured content size (text/image). `None` for plain containers.
    pub intrinsic: Option<Vec2>,
    /// Whether the intrinsic size comes from wrap-enabled text.
    pub wrap_text: bool,
    /// Bottom-up content-hugging size.
    pub content: Vec2,
    /// Bottom-up minimum size (shrink floor propagated from children).
    pub min: Vec2,
    /// Final computed size.
    pub size: Vec2,
    /// Final computed position, relative to the parent's top-left corner (y-down).
    pub pos: Vec2,
    /// Wrap lines: groups of child indices per line (a single group when wrapping is disabled).
    pub lines: Vec<Vec<usize>>,
    /// Cross extent of each wrap line.
    pub line_cross: Vec<f32>,
}
impl FlowItem {
    /// Creates a new flow item from a configuration.
    pub(crate) fn new(config: UiLayoutTypeFlow) -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
            parent: None,
            children: Vec::new(),
            margin: config.margin,
            resolved_margin: [0.0; 4],
            config,
            intrinsic: None,
            wrap_text: false,
            content: Vec2::ZERO,
            min: Vec2::ZERO,
            size: Vec2::ZERO,
            pos: Vec2::ZERO,
            lines: Vec::new(),
            line_cross: Vec::new(),
        }
    }
    /// Attaches an intrinsic (measured) size to the item.
    pub(crate) fn with_intrinsic(mut self, intrinsic: Vec2) -> Self {
        self.intrinsic = Some(intrinsic);
        self
    }
    /// Marks the item's intrinsic size as wrap-enabled text.
    pub(crate) fn with_wrap_text(mut self) -> Self {
        self.wrap_text = true;
        self
    }
}

/// The flow layout engine. Holds a flat arena of [`FlowItem`]s extracted from the entity hierarchy
/// and computes their sizes and positions:
///
/// 1. **Margin injection** - each child's undefined margin sides receive the parent's
///    `align`/`justify` default `Sp` margin templates.
/// 2. **Bottom-up pass** - computes content-hugging sizes and minimum sizes from the leaves up,
///    including the children's fixed margins.
/// 3. **Root resolution** - sizes the subtree root inside its (non-flow) parent's box.
/// 4. **Top-down pass** (BFS) - resolves relative sizing, then distributes space:
///    on overflow children shrink water-level (largest first, floored at their minimums);
///    on leftover space all `Sp` claims (margins and sizing) share it proportionally,
///    re-normalized when maximum clamps bind.
/// 5. **Position pass** - assigns each child's position from padding, margins and child sizes.
#[derive(Default)]
#[doc(hidden)]
pub struct FlowLayout {
    items: Vec<FlowItem>,
    /// Whether the last top-down pass changed a wrapping container's cross size,
    /// signalling the need for another fixpoint iteration.
    wrap_dirty: bool,
}
impl FlowLayout {
    /// Clears the arena, retaining its capacity for reuse across frames.
    pub(crate) fn clear(&mut self) {
        self.items.clear();
    }
    /// Pushes a new item into the arena, attaching it to `parent` if given.
    pub(crate) fn push(&mut self, parent: Option<usize>, mut item: FlowItem) -> usize {
        let index = self.items.len();
        item.parent = parent;
        if let Some(parent) = parent {
            self.items[parent].children.push(index);
        }
        self.items.push(item);
        index
    }
    /// Attaches an existing item to a new parent (used for the virtual root container).
    pub(crate) fn reparent(&mut self, child: usize, parent: usize) {
        self.items[child].parent = Some(parent);
        self.items[parent].children.push(child);
    }
    /// Whether the item is the root of a maximal flow subtree (has no flow parent).
    pub(crate) fn is_root(&self, index: usize) -> bool {
        self.items[index].parent.is_none()
    }
    /// Returns the computed `(position, size)` of an item. The position is relative to the
    /// parent's top-left corner, with y pointing down.
    pub(crate) fn result(&self, index: usize) -> (Vec2, Vec2) {
        let item = &self.items[index];
        (item.pos, item.size)
    }
    /// Iterates `(entity, assigned width)` of all wrap-enabled text items in the arena.
    pub(crate) fn wrap_text_widths(&self) -> impl Iterator<Item = (Entity, f32)> + '_ {
        self.items.iter()
            .filter(|item| item.wrap_text)
            .map(|item| (item.entity, item.size.x.max(0.0)))
    }
    /// Iterates all item indices in the subtree rooted at `index` (preorder).
    pub(crate) fn subtree(&self, index: usize) -> Vec<usize> {
        let mut order = Vec::new();
        let mut stack = vec![index];
        while let Some(i) = stack.pop() {
            order.push(i);
            stack.extend(self.items[i].children.iter().rev().copied());
        }
        order
    }

    /// Computes sizes and positions of the whole subtree rooted at `index`.
    /// `parent_size` is the size of the subtree root's (non-flow) parent's box.
    pub(crate) fn compute(&mut self, index: usize, parent_size: Vec2, ctx: &UiFlowContext) {
        // === Phase 0: margin template injection ===
        let order = self.subtree(index);
        self.items[index].margin = self.items[index].config.margin;
        for &i in order.iter().skip(1) {
            if let Some(parent) = self.items[i].parent {
                let is_first = self.items[parent].children.first() == Some(&i);
                let is_last = self.items[parent].children.last() == Some(&i);
                let own = self.items[i].config.margin;
                let parent_config = self.items[parent].config.clone();
                let child_config = self.items[i].config.clone();
                self.items[i].margin = effective_margins(&own, &parent_config, &child_config, is_first, is_last);
            }
        }

        // === Phase 1-4: iterate when wrapping is involved ===
        // Wrapping containers derive their cross size from their lines, which is only known
        // after the top-down pass - ancestors laid out with stale sizes need a re-run.
        // The fixpoint is bounded and exits early once sizes stabilize.
        let has_wrap = order.iter().any(|&i| is_wrap_mode(&self.items[i].config));
        let max_iterations = if has_wrap { FIXPOINT_MAX_ITERATIONS } else { 1 };
        // A previously computed subtree may have exhausted its iterations with the flag
        // still set - always start clean so it does not skew this subtree.
        self.wrap_dirty = false;
        for _ in 0..max_iterations {
            // === Bottom-up content pass (children before parents) ===
            for &i in order.iter().rev() {
                self.compute_content(i, ctx);
            }

            // === Resolve the subtree root inside its parent ===
            self.resolve_root(index, parent_size, ctx);

            // === Top-down sizing pass (parents before children) ===
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(index);
            while let Some(i) = queue.pop_front() {
                self.process(i, ctx);
                for &child in &self.items[i].children.clone() {
                    queue.push_back(child);
                }
            }

            // === Position pass ===
            self.place(index, ctx);

            if !self.wrap_dirty { break }
            self.wrap_dirty = false;
        }
    }

    /// Computes the content-hugging size and minimum size of an item from its children.
    /// `Sp` margin components and relative units are dropped (not resolvable bottom-up).
    fn compute_content(&mut self, index: usize, ctx: &UiFlowContext) {
        let config = self.items[index].config.clone();
        let axis = axis_of(config.direction);
        let cross = 1 - axis;

        // Relative units cannot be resolved yet (own size unknown) - drop them.
        let (pl, pr, pt, pb) = eval_padding_intrinsic(&config, ctx);
        let pad_x = pl + pr;
        let pad_y = pt + pb;
        let gap = config.gap.evaluate_intrinsic(ctx.abs_scale, ctx.viewport, ctx.font_size, axis);
        let child_count = self.items[index].children.len();
        let gap_total = gap * child_count.saturating_sub(1) as f32;

        let (mut content, mut min) = if let Some(intrinsic) = self.items[index].intrinsic {
            // Measured leaf (text/image): content is the measured size, minimum is the same
            // unless the text can re-wrap (then it can shrink to zero width).
            let min = if self.items[index].wrap_text { Vec2::new(0.0, intrinsic.y) } else { intrinsic };
            (intrinsic, min)
        } else {
            // Container: hug the children, including their fixed margins.
            // Wrapping containers estimate their line packing bottom-up when the main extent
            // is knowable (an intrinsically-resolvable `Fixed` value, or the size carried over
            // from the previous fixpoint iteration).
            let mut along_c: f32 = 0.0; let mut along_m: f32 = 0.0;
            let mut cross_c: f32 = 0.0; let mut cross_m: f32 = 0.0;
            // Per-child footprints (fixed margins + content/min), reused for line packing.
            let mut main_fps: Vec<f32> = Vec::with_capacity(child_count);
            let mut main_margin_fps: Vec<(f32, f32)> = Vec::with_capacity(child_count);
            let mut cross_fps: Vec<f32> = Vec::with_capacity(child_count);
            for &child in &self.items[index].children.clone() {
                let margin = &self.items[child].margin;
                let (ms, me) = margin_sides(margin, axis);
                let (cs, ce) = margin_sides(margin, cross);
                let ms = ms.evaluate_intrinsic(ctx.abs_scale, ctx.viewport, ctx.font_size, axis);
                let me = me.evaluate_intrinsic(ctx.abs_scale, ctx.viewport, ctx.font_size, axis);
                let cs = cs.evaluate_intrinsic(ctx.abs_scale, ctx.viewport, ctx.font_size, cross);
                let ce = ce.evaluate_intrinsic(ctx.abs_scale, ctx.viewport, ctx.font_size, cross);
                along_c += ms + axis_get(self.items[child].content, axis) + me;
                along_m += ms + axis_get(self.items[child].min, axis) + me;
                cross_c = cross_c.max(cs + axis_get(self.items[child].content, cross) + ce);
                cross_m = cross_m.max(cs + axis_get(self.items[child].min, cross) + ce);
                main_fps.push(ms + axis_get(self.items[child].content, axis) + me);
                main_margin_fps.push((ms, me));
                cross_fps.push(cs + axis_get(self.items[child].content, cross) + ce);
            }
            let mut content = Vec2::ZERO;
            let mut min = Vec2::ZERO;
            let pad_main = axis_get(Vec2::new(pad_x, pad_y), axis);
            let pad_cross = axis_get(Vec2::new(pad_x, pad_y), cross);
            // Along the flow axis, padding always applies.
            axis_set(&mut content, axis, along_c + gap_total + pad_main);
            axis_set(&mut min, axis, along_m + gap_total + pad_main);
            // Across the flow axis, an empty container hugs to zero (padding does not inflate it).
            if child_count > 0 {
                axis_set(&mut content, cross, cross_c + pad_cross);
                axis_set(&mut min, cross, cross_m + pad_cross);

                // Wrapping containers derive their cross hug from their estimated line packing.
                if is_wrap_mode(&config) {
                    // Grid lines chunk by track count (extent-independent) with knowable track
                    // bases; greedy wrapping needs a knowable main extent (a `Fixed` value,
                    // or the size carried over from the previous fixpoint iteration).
                    let (line_groups, grid_main) = if config.grid.is_empty() {
                        let main_extent = match if axis == 0 { config.width.size } else { config.height.size } {
                            UiFlowSize::Fixed(v) => Some(v.evaluate_intrinsic(ctx.abs_scale, ctx.viewport, ctx.font_size, axis)),
                            _ => {
                                let previous = axis_get(self.items[index].size, axis);
                                (previous > FLOW_EPSILON).then_some(previous)
                            }
                        };
                        let groups = match main_extent {
                            Some(main_extent) => pack_lines_greedy(&main_fps, gap, (main_extent - pad_main).max(0.0)),
                            None => vec![(0..child_count).collect::<Vec<usize>>()],
                        };
                        // Wrapping containers can compress along the main axis down to their
                        // largest child footprint plus their own padding - the single-line
                        // minimum does not apply.
                        let largest = main_fps.iter().copied().fold(0.0, f32::max);
                        axis_set(&mut min, axis, largest + pad_main);
                        (groups, None)
                    } else {
                        let n = config.grid.len();
                        let groups = pack_lines_grid(child_count, n, config.grid_wrap);
                        // Bottom-up grid track bases (`Fit` = footprint, `Fixed` = explicit,
                        // `Grow`/`Sp` = margins only); the longest line bounds the container.
                        let mut longest: f32 = 0.0;
                        for line in &groups {
                            let mut extent = gap * line.len().saturating_sub(1) as f32;
                            for (t, &k) in line.iter().enumerate() {
                                let def = if t < n { config.grid[t] } else { UiFlowSize::Fit };
                                let (ms, me) = main_margin_fps[k];
                                let track = match def {
                                    UiFlowSize::Fit => main_fps[k],
                                    UiFlowSize::Fixed(v) => ms + v.evaluate_intrinsic(ctx.abs_scale, ctx.viewport, ctx.font_size, axis) + me,
                                    UiFlowSize::Grow => ms + me,
                                };
                                extent += track;
                            }
                            longest = longest.max(extent);
                        }
                        (groups, Some(longest))
                    };
                    if let Some(longest) = grid_main {
                        axis_set(&mut content, axis, longest + pad_main);
                        axis_set(&mut min, axis, longest + pad_main);
                    }
                    let total = line_groups.iter()
                        .map(|line| line.iter().map(|&k| cross_fps[k]).fold(0.0, f32::max))
                        .sum::<f32>()
                        + gap * line_groups.len().saturating_sub(1) as f32;
                    axis_set(&mut content, cross, total + pad_cross);
                    axis_set(&mut min, cross, total + pad_cross);
                }
            }
            (content, min)
        };

        // Fixed sizing overrides the content size (relative and `Sp` parts dropped here;
        // the `Sp` component acts as a leftover-space claim in the top-down pass).
        if let UiFlowSize::Fixed(v) = config.width.size {
            content.x = v.evaluate_intrinsic(ctx.abs_scale, ctx.viewport, ctx.font_size, 0);
            min.x = content.x;
        }
        if let UiFlowSize::Fixed(v) = config.height.size {
            content.y = v.evaluate_intrinsic(ctx.abs_scale, ctx.viewport, ctx.font_size, 1);
            min.y = content.y;
        }

        // Clamp to own min/max (only the always-resolvable parts of the units).
        if let Some(v) = config.width.min { let m = v.evaluate_intrinsic(ctx.abs_scale, ctx.viewport, ctx.font_size, 0); content.x = content.x.max(m); min.x = min.x.max(m); }
        if let Some(v) = config.width.max { let m = v.evaluate_intrinsic(ctx.abs_scale, ctx.viewport, ctx.font_size, 0); content.x = content.x.min(m); min.x = min.x.min(m); }
        if let Some(v) = config.height.min { let m = v.evaluate_intrinsic(ctx.abs_scale, ctx.viewport, ctx.font_size, 1); content.y = content.y.max(m); min.y = min.y.max(m); }
        if let Some(v) = config.height.max { let m = v.evaluate_intrinsic(ctx.abs_scale, ctx.viewport, ctx.font_size, 1); content.y = content.y.min(m); min.y = min.y.min(m); }

        self.items[index].content = content.max(Vec2::ZERO);
        self.items[index].min = min.max(Vec2::ZERO);
        // Initial size is the content size; the top-down pass resolves the rest.
        self.items[index].size = self.items[index].content;
    }

    /// Resolves the subtree root's own size and position inside its non-flow parent's box.
    /// Placement goes through the same margin templates: the root's own margins merged with
    /// its `justify`/`align` defaults, `Sp` claims sharing the leftover space of the parent box.
    fn resolve_root(&mut self, index: usize, parent_size: Vec2, ctx: &UiFlowContext) {
        let config = self.items[index].config.clone();
        for axis in 0..2 {
            let sizing = if axis == 0 { config.width } else { config.height };
            let value = match sizing.size {
                UiFlowSize::Fit => axis_get(self.items[index].content, axis),
                UiFlowSize::Grow => axis_get(parent_size, axis),
                UiFlowSize::Fixed(v) => v.evaluate_axis(ctx.abs_scale, parent_size, ctx.viewport, ctx.font_size, axis),
            };
            let mut value = value.max(0.0);
            if let Some(v) = sizing.min { value = value.max(v.evaluate_axis(ctx.abs_scale, parent_size, ctx.viewport, ctx.font_size, axis)) }
            if let Some(v) = sizing.max { value = value.min(v.evaluate_axis(ctx.abs_scale, parent_size, ctx.viewport, ctx.font_size, axis)) }
            axis_set(&mut self.items[index].size, axis, value);
        }

        // Place the root inside the parent through its margin templates (top-left relative, unclamped).
        let margin = effective_margins(&config.margin, &config, &config, true, true);
        let main_axis = axis_of(config.direction);
        for axis in 0..2 {
            let size = axis_get(self.items[index].size, axis);
            let (start, end) = margin_sides(&margin, axis);
            let part_start = MarginPart { fixed: start.evaluate_axis(ctx.abs_scale, parent_size, ctx.viewport, ctx.font_size, axis), weight: start.sp_weight() };
            let part_end = MarginPart { fixed: end.evaluate_axis(ctx.abs_scale, parent_size, ctx.viewport, ctx.font_size, axis), weight: end.sp_weight() };
            let leftover = axis_get(parent_size, axis) - part_start.fixed - part_end.fixed - size;
            let (mut pos, mut u) = (part_start.fixed, 0.0);
            if leftover > 0.0 && part_start.weight + part_end.weight > 0.0 {
                u = leftover / (part_start.weight + part_end.weight);
                pos += u * part_start.weight;
            } else {
                let factor = if axis == main_axis { config.justify.factor() } else { align_factor(config.align) };
                pos += factor * leftover;
            }
            axis_set(&mut self.items[index].pos, axis, pos);
            // Store the resolved margins for consistency.
            let (start, end) = if axis == 0 { (0, 1) } else { (2, 3) };
            self.items[index].resolved_margin[start] = part_start.fixed + u * part_start.weight;
            self.items[index].resolved_margin[end] = part_end.fixed + u * part_end.weight;
        }
    }

    /// Evaluates a child's effective minimum along an axis (bottom-up minimum raised by its min clamp).
    fn effective_min(&self, child: usize, axis: usize, base: Vec2, ctx: &UiFlowContext) -> f32 {
        let item = &self.items[child];
        let mut value = axis_get(item.min, axis);
        let clamp = if axis == 0 { item.config.width.min } else { item.config.height.min };
        if let Some(v) = clamp {
            value = value.max(v.evaluate_axis(ctx.abs_scale, base, ctx.viewport, ctx.font_size, axis));
        }
        value
    }
    /// Evaluates a child's effective maximum along an axis (its max clamp, or infinity).
    fn effective_max(&self, child: usize, axis: usize, base: Vec2, ctx: &UiFlowContext) -> f32 {
        let item = &self.items[child];
        let clamp = if axis == 0 { item.config.width.max } else { item.config.height.max };
        match clamp {
            Some(v) => v.evaluate_axis(ctx.abs_scale, base, ctx.viewport, ctx.font_size, axis),
            None => f32::INFINITY,
        }
    }

    /// The top-down BFS visit of one container. Resolves `Fixed` children, packs wrap/grid
    /// lines, distributes the main-axis space per line (`Sp` claims share the leftover,
    /// water-level shrink on overflow, grid tracks constrain their items) and resolves the
    /// cross axis within each line's extent.
    fn process(&mut self, index: usize, ctx: &UiFlowContext) {
        let config = self.items[index].config.clone();
        let axis = axis_of(config.direction);
        let cross = 1 - axis;
        let own = self.items[index].size;
        let (pl, pr, pt, pb) = eval_padding_full(&config, own, ctx);
        let inner = Vec2::new((own.x - pl - pr).max(0.0), (own.y - pt - pb).max(0.0));
        let gap = config.gap.evaluate_axis(ctx.abs_scale, own, ctx.viewport, ctx.font_size, axis).max(0.0);
        let children = self.items[index].children.clone();
        let child_count = children.len();
        let wrap_mode = is_wrap_mode(&config);
        let track_count = config.grid.len().max(1);

        // Relative units of children resolve against the parent's inner content box,
        // minus the gaps along the flow axis. In wrap mode the line packing is unknown yet,
        // so only the grid track gaps are subtracted.
        let total_gaps = if wrap_mode { gap * track_count.saturating_sub(1) as f32 } else { gap * child_count.saturating_sub(1) as f32 };
        let base_along = (axis_get(inner, axis) - total_gaps).max(0.0);
        let base_cross = axis_get(inner, cross);
        let base = Vec2::new(
            if axis == 0 { base_along } else { base_cross },
            if axis == 1 { base_along } else { base_cross },
        );

        // === Resolve Fixed children (relative units now resolvable; `Sp` claims deferred) ===
        for &child in &children {
            let child_config = self.items[child].config.clone();
            if let UiFlowSize::Fixed(v) = child_config.width.size {
                let mut value = v.evaluate_axis(ctx.abs_scale, base, ctx.viewport, ctx.font_size, 0).max(0.0);
                if let Some(m) = child_config.width.min { value = value.max(m.evaluate_axis(ctx.abs_scale, base, ctx.viewport, ctx.font_size, 0)) }
                if let Some(m) = child_config.width.max { value = value.min(m.evaluate_axis(ctx.abs_scale, base, ctx.viewport, ctx.font_size, 0)) }
                self.items[child].size.x = value;
            }
            if let UiFlowSize::Fixed(v) = child_config.height.size {
                let mut value = v.evaluate_axis(ctx.abs_scale, base, ctx.viewport, ctx.font_size, 1).max(0.0);
                if let Some(m) = child_config.height.min { value = value.max(m.evaluate_axis(ctx.abs_scale, base, ctx.viewport, ctx.font_size, 1)) }
                if let Some(m) = child_config.height.max { value = value.min(m.evaluate_axis(ctx.abs_scale, base, ctx.viewport, ctx.font_size, 1)) }
                self.items[child].size.y = value;
            }
        }

        // === Main-axis margins and footprints ===
        let mut margins: Vec<(MarginPart, MarginPart)> = Vec::with_capacity(child_count);
        let mut main_fps: Vec<f32> = Vec::with_capacity(child_count);
        for &child in &children {
            let (start, end) = margin_sides(&self.items[child].margin, axis);
            let part_start = MarginPart { fixed: start.evaluate_axis(ctx.abs_scale, own, ctx.viewport, ctx.font_size, axis), weight: start.sp_weight() };
            let part_end = MarginPart { fixed: end.evaluate_axis(ctx.abs_scale, own, ctx.viewport, ctx.font_size, axis), weight: end.sp_weight() };
            main_fps.push(part_start.fixed + axis_get(self.items[child].size, axis) + part_end.fixed);
            margins.push((part_start, part_end));
        }

        // === Line packing ===
        let lines: Vec<Vec<usize>> = if !config.grid.is_empty() {
            pack_lines_grid(child_count, config.grid.len(), config.grid_wrap)
        } else if config.wrap {
            pack_lines_greedy(&main_fps, gap, axis_get(inner, axis))
        } else {
            vec![(0..child_count).collect::<Vec<usize>>()]
        };

        // === Main-axis space distribution (per line) ===
        if !config.grid.is_empty() {
            self.distribute_grid_tracks(&children, &lines, &config.grid, &margins, &main_fps, axis, inner, gap, base, ctx);
        } else {
            for line in &lines {
                let line_children: Vec<usize> = line.iter().map(|&k| children[k]).collect();
                let line_margins: Vec<(MarginPart, MarginPart)> = line.iter().map(|&k| margins[k]).collect();
                let mut content = gap * line.len().saturating_sub(1) as f32;
                for &k in line {
                    content += main_fps[k];
                }
                let leftover = axis_get(inner, axis) - content;
                let u = if leftover > FLOW_EPSILON {
                    self.distribute_leftover(&line_children, &line_margins, axis, leftover, base, ctx)
                } else {
                    if leftover < -FLOW_EPSILON {
                        // Overflow within the line: shrink water-level, largest children first.
                        let resizable = self.resizable_children(&line_children, axis);
                        self.redistribute_shrink(&resizable, axis, leftover, base, ctx);
                    }
                    0.0
                };
                // Assign the resolved main-axis margins.
                let (start, end) = if axis == 0 { (0, 1) } else { (2, 3) };
                for (t, &k) in line.iter().enumerate() {
                    let child = children[k];
                    let (part_start, part_end) = line_margins[t];
                    self.items[child].resolved_margin[start] = part_start.fixed + u * part_start.weight;
                    self.items[child].resolved_margin[end] = part_end.fixed + u * part_end.weight;
                }
            }
        }

        // === Cross-axis margins ===
        let cross_margins: Vec<(MarginPart, MarginPart)> = children.iter().map(|&child| {
            let (start, end) = margin_sides(&self.items[child].margin, cross);
            (MarginPart { fixed: start.evaluate_axis(ctx.abs_scale, own, ctx.viewport, ctx.font_size, cross), weight: start.sp_weight() },
             MarginPart { fixed: end.evaluate_axis(ctx.abs_scale, own, ctx.viewport, ctx.font_size, cross), weight: end.sp_weight() })
        }).collect();

        // === Line cross extents ===
        // In wrap mode each line is as tall as its largest child footprint; on a single
        // line the extent is the parent's inner cross axis.
        let line_cross: Vec<f32> = if wrap_mode {
            lines.iter().map(|line| line.iter().map(|&k| {
                let (part_start, part_end) = cross_margins[k];
                part_start.fixed + axis_get(self.items[children[k]].size, cross) + part_end.fixed
            }).fold(0.0, f32::max)).collect()
        } else {
            vec![axis_get(inner, cross)]
        };

        // === Wrapping container cross size ===
        // A `Fit` cross-sized wrapping container hugs the sum of its lines.
        if wrap_mode && child_count > 0 {
            let cross_sizing = if cross == 0 { config.width.size } else { config.height.size };
            if matches!(cross_sizing, UiFlowSize::Fit) {
                let pad_cross = if cross == 0 { pl + pr } else { pt + pb };
                let mut value = line_cross.iter().sum::<f32>() + gap * lines.len().saturating_sub(1) as f32 + pad_cross;
                let (min, max) = if cross == 0 { (&config.width.min, &config.width.max) } else { (&config.height.min, &config.height.max) };
                if let Some(v) = min { value = value.max(v.evaluate_axis(ctx.abs_scale, own, ctx.viewport, ctx.font_size, cross)) }
                if let Some(v) = max { value = value.min(v.evaluate_axis(ctx.abs_scale, own, ctx.viewport, ctx.font_size, cross)) }
                if (value - axis_get(self.items[index].size, cross)).abs() > FLOW_EPSILON { self.wrap_dirty = true }
                axis_set(&mut self.items[index].size, cross, value);
                // Keep the content in sync so ancestors hug the wrapped size on re-runs.
                axis_set(&mut self.items[index].content, cross, value);
            }
        }

        // === Cross-axis resolution within the line extents ===
        for (l, line) in lines.iter().enumerate() {
            let extent = line_cross[l];
            for &k in line {
                let child = children[k];
                let (part_start, part_end) = cross_margins[k];
                let size_claim = size_sp_weight(&self.items[child].config, cross);
                let size_cross = axis_get(self.items[child].size, cross);
                let whitespace = extent - (part_start.fixed + size_cross + part_end.fixed);
                let sum_weights = part_start.weight + part_end.weight + size_claim;

                let mut new_size = size_cross;
                let mut u_cross = 0.0;
                if whitespace > 0.0 && sum_weights > 0.0 {
                    u_cross = whitespace / sum_weights;
                    if size_claim > 0.0 {
                        let value = size_cross + u_cross * size_claim;
                        let max = self.effective_max(child, cross, base, ctx);
                        let min = self.effective_min(child, cross, base, ctx);
                        new_size = min.max(value.min(max));
                    }
                }

                // Classic clamping for resizable children (`Fit` containers, wrap-enabled
                // text, `Grow` without an applied claim), bounded by the line extent.
                // Wrapping containers own their cross size and are never clamped down.
                let item = &self.items[child];
                let sizing = if cross == 0 { item.config.width.size } else { item.config.height.size };
                let resizable = matches!(sizing, UiFlowSize::Fit | UiFlowSize::Grow) && (item.intrinsic.is_none() || item.wrap_text);
                if resizable {
                    let min_floor = self.effective_min(child, cross, base, ctx);
                    let mut value = new_size;
                    if matches!(sizing, UiFlowSize::Grow) && !(whitespace > 0.0 && sum_weights > 0.0) {
                        value = extent.min(self.effective_max(child, cross, base, ctx));
                    }
                    if is_wrap_mode(&item.config) && matches!(sizing, UiFlowSize::Fit) {
                        value = min_floor.max(value);
                    } else {
                        value = min_floor.max(value.min(extent));
                    }
                    new_size = value;
                }
                axis_set(&mut self.items[child].size, cross, new_size);

                // Assign the resolved cross-axis margins.
                let (start, end) = if cross == 0 { (0, 1) } else { (2, 3) };
                self.items[child].resolved_margin[start] = part_start.fixed + u_cross * part_start.weight;
                self.items[child].resolved_margin[end] = part_end.fixed + u_cross * part_end.weight;
            }
        }

        // Store the line structure for the position pass.
        self.items[index].lines = lines;
        self.items[index].line_cross = line_cross;
    }

    /// Sizes the grid tracks of each line and stretches the line's children to their tracks.
    /// `Fit` tracks hug their item's footprint, `Fixed` tracks are explicit and `Sp`/`Grow`
    /// tracks claim shares of the line's leftover space alongside the children's `Sp` margins.
    /// Items are stretched to their track minus their fixed margins, floored at their
    /// minimum and capped at their maximum clamps.
    #[allow(clippy::too_many_arguments)]  // internal single-call helper; bundling would obscure the data flow
    fn distribute_grid_tracks(&mut self, children: &[usize], lines: &[Vec<usize>], grid: &[UiFlowSize], margins: &[(MarginPart, MarginPart)], main_fps: &[f32], axis: usize, inner: Vec2, gap: f32, base: Vec2, ctx: &UiFlowContext) {
        let n = grid.len();
        let (start_slot, end_slot) = if axis == 0 { (0, 1) } else { (2, 3) };
        for line in lines {
            // Track bases and leftover-space claims.
            let mut tracks: Vec<f32> = Vec::with_capacity(line.len());
            let mut claims: Vec<f32> = Vec::with_capacity(line.len());
            for (t, &k) in line.iter().enumerate() {
                let def = if t < n { grid[t] } else { UiFlowSize::Fit };
                let (part_start, part_end) = margins[k];
                match def {
                    UiFlowSize::Fit => {
                        tracks.push(main_fps[k]);
                        claims.push(0.0);
                    }
                    UiFlowSize::Fixed(v) => {
                        tracks.push(part_start.fixed + v.evaluate_axis(ctx.abs_scale, base, ctx.viewport, ctx.font_size, axis) + part_end.fixed);
                        claims.push(v.sp_weight());
                    }
                    UiFlowSize::Grow => {
                        tracks.push(part_start.fixed + part_end.fixed);
                        claims.push(1.0);
                    }
                }
            }
            let claim_sum: f32 = claims.iter().sum();
            let margin_sum: f32 = line.iter().map(|&k| margins[k].0.weight + margins[k].1.weight).sum();
            let content = tracks.iter().sum::<f32>() + gap * line.len().saturating_sub(1) as f32;
            let leftover = axis_get(inner, axis) - content;
            let u = if leftover > 0.0 && claim_sum + margin_sum > 0.0 { leftover / (claim_sum + margin_sum) } else { 0.0 };
            // Apply the track claims and stretch each child to its track.
            for t in 0..tracks.len() {
                tracks[t] += u * claims[t];
            }
            for (t, &k) in line.iter().enumerate() {
                let child = children[k];
                let (part_start, part_end) = margins[k];
                let value = (tracks[t] - part_start.fixed - part_end.fixed).max(0.0);
                let min = self.effective_min(child, axis, base, ctx);
                let max = self.effective_max(child, axis, base, ctx);
                axis_set(&mut self.items[child].size, axis, min.max(value.min(max)));
                self.items[child].resolved_margin[start_slot] = part_start.fixed + u * part_start.weight;
                self.items[child].resolved_margin[end_slot] = part_end.fixed + u * part_end.weight;
            }
        }
    }

    /// Distributes the main-axis leftover space proportionally between all `Sp` claims of the
    /// children (margins and sizing). Sizing claims that would exceed the child's maximum clamp
    /// are pinned at the clamp and the remainder is re-normalized between the surviving claims.
    /// Returns the final value of one `Sp` unit.
    fn distribute_leftover(&mut self, children: &[usize], margins: &[(MarginPart, MarginPart)], axis: usize, leftover: f32, base: Vec2, ctx: &UiFlowContext) -> f32 {
        // Active sizing claims (index into `children`, weight).
        let mut claims: Vec<(usize, f32)> = children.iter().enumerate()
            .filter_map(|(k, &child)| {
                let weight = size_sp_weight(&self.items[child].config, axis);
                (weight > 0.0).then_some((k, weight))
            })
            .collect();
        let margin_sum: f32 = margins.iter().map(|(s, e)| s.weight + e.weight).sum();

        let mut remaining = leftover;
        let mut u;
        loop {
            let size_sum: f32 = claims.iter().map(|(_, weight)| *weight).sum();
            let sum = size_sum + margin_sum;
            if sum <= 0.0 || remaining <= 0.0 { u = 0.0; break }
            u = remaining / sum;

            // Pin children whose claim would exceed their maximum clamp.
            let violators: Vec<usize> = claims.iter()
                .filter(|&&(k, weight)| {
                    let child = children[k];
                    let value = axis_get(self.items[child].size, axis) + u * weight;
                    value > self.effective_max(child, axis, base, ctx) + FLOW_EPSILON
                })
                .map(|&(k, _)| k)
                .collect();
            if violators.is_empty() { break }
            for k in violators {
                let child = children[k];
                let max = self.effective_max(child, axis, base, ctx);
                let previous = axis_get(self.items[child].size, axis);
                remaining -= (max - previous).max(0.0);
                axis_set(&mut self.items[child].size, axis, max);
                claims.retain(|&(ck, _)| ck != k);
            }
        }

        // Apply the size claims of the surviving children (floored at their minimums).
        for &(k, weight) in &claims {
            let child = children[k];
            let value = axis_get(self.items[child].size, axis) + u * weight;
            let min = self.effective_min(child, axis, base, ctx);
            axis_set(&mut self.items[child].size, axis, min.max(value));
        }
        u
    }

    /// Children eligible for redistribution along an axis: `Fit` or `Grow` sizing
    /// (measured nodes only if their text can wrap).
    fn resizable_children(&self, children: &[usize], axis: usize) -> Vec<usize> {
        children.iter().copied().filter(|&child| {
            let item = &self.items[child];
            let sizing = if axis == 0 { item.config.width.size } else { item.config.height.size };
            let sized = matches!(sizing, UiFlowSize::Fit | UiFlowSize::Grow);
            let shrinkable_intrinsic = item.intrinsic.is_none() || item.wrap_text;
            sized && shrinkable_intrinsic
        }).collect()
    }

    /// Shrinks the largest children first towards the size of the next largest ("water level"),
    /// flooring each at its minimum size.
    fn redistribute_shrink(&mut self, resizable: &[usize], axis: usize, mut to_distribute: f32, base: Vec2, ctx: &UiFlowContext) {
        let mut list: Vec<usize> = resizable.to_vec();
        while to_distribute < -FLOW_EPSILON && !list.is_empty() {
            let mut largest = 0.0;
            let mut second_largest = 0.0;
            let mut width_to_add = to_distribute;
            for &child in &list {
                let size = axis_get(self.items[child].size, axis);
                if (size - largest).abs() < FLOW_EPSILON { continue }
                if size > largest { second_largest = largest; largest = size }
                if size < largest { second_largest = second_largest.max(size); width_to_add = second_largest - largest }
            }
            width_to_add = width_to_add.max(to_distribute / list.len() as f32);

            let mut progress = false;
            let mut k = 0;
            while k < list.len() {
                let child = list[k];
                let previous = axis_get(self.items[child].size, axis);
                if (previous - largest).abs() < FLOW_EPSILON {
                    let min_size = self.effective_min(child, axis, base, ctx);
                    let mut new = previous + width_to_add;
                    if new <= min_size {
                        new = min_size;
                        list.swap_remove(k);
                    } else {
                        k += 1;
                    }
                    axis_set(&mut self.items[child].size, axis, new);
                    to_distribute -= new - previous;
                    progress = true;
                } else {
                    k += 1;
                }
            }
            if !progress { break }
        }
    }

    /// Assigns positions to all children in the subtree (relative to their parent's top-left, y-down).
    /// Along the main axis children are placed from padding, resolved margins and sizes; inverted
    /// directions (`RightToLeft`, `BottomToTop`) mirror the placement. Lines stack along the cross
    /// axis, optionally flipped to grow from the opposite side. On the cross axis each child is
    /// placed at its resolved margin within its line - unclaimed whitespace extends past it.
    fn place(&mut self, index: usize, ctx: &UiFlowContext) {
        let mut stack = vec![index];
        while let Some(i) = stack.pop() {
            let config = self.items[i].config.clone();
            let axis = axis_of(config.direction);
            let cross = 1 - axis;
            let forward = matches!(config.direction, UiFlowDirection::LeftToRight | UiFlowDirection::TopToBottom);
            let own = self.items[i].size;
            let (pl, pr, pt, pb) = eval_padding_full(&config, own, ctx);
            let inner = Vec2::new(own.x - pl - pr, own.y - pt - pb);
            let gap = config.gap.evaluate_axis(ctx.abs_scale, own, ctx.viewport, ctx.font_size, axis).max(0.0);
            let children = self.items[i].children.clone();
            let lines = if self.items[i].lines.is_empty() {
                if children.is_empty() { Vec::new() } else { vec![(0..children.len()).collect::<Vec<usize>>()] }
            } else {
                self.items[i].lines.clone()
            };
            let pad_start = Vec2::new(pl, pt);
            let inner_main = axis_get(inner, axis).max(0.0);
            let inner_cross = axis_get(inner, cross).max(0.0);

            let mut cross_offset = 0.0;
            for (l, line) in lines.iter().enumerate() {
                let extent = self.items[i].line_cross.get(l).copied().unwrap_or(0.0);
                // Line start on the cross axis: lines stack from the start edge, or from the
                // opposite edge when flipped (the first line sits at the end, later ones above it).
                let line_cross_start = if !config.flipped {
                    axis_get(pad_start, cross) + cross_offset
                } else {
                    axis_get(pad_start, cross) + inner_cross - cross_offset - extent
                };
                cross_offset += extent + gap;

                let mut offset = 0.0;
                for &k in line {
                    let child = children[k];
                    let margins = self.items[child].resolved_margin;
                    let (m_start, m_end) = if axis == 0 { (margins[0], margins[1]) } else { (margins[2], margins[3]) };
                    let m_cross_start = if cross == 0 { margins[0] } else { margins[2] };
                    let size_main = axis_get(self.items[child].size, axis);

                    let mut pos = Vec2::ZERO;
                    let main = if forward {
                        axis_get(pad_start, axis) + offset + m_start
                    } else {
                        axis_get(pad_start, axis) + inner_main - offset - m_start - size_main
                    };
                    axis_set(&mut pos, axis, main);
                    axis_set(&mut pos, cross, line_cross_start + m_cross_start);
                    self.items[child].pos = pos;
                    offset += m_start + size_main + m_end + gap;
                }
            }
            stack.extend(children.iter().rev().copied());
        }
    }
}


// #========================#
// #=== STATE BLENDING CONTROL ===#

/// Resolves the active flow configuration of a [`UiLayout`] by blending the flow parameters of all
/// active states weighted by their transition values. The node participates in the flow only if
/// its [`UiBase`] layout is of the flow type; state layouts of other kinds are ignored.
pub(crate) fn resolve_flow_config(layout: &UiLayout, state: &UiState) -> Option<UiLayoutTypeFlow> {
    let UiLayoutType::Flow(base) = layout.layouts.get(&UiBase::id())? else { return None };

    // Collect active states that define flow layouts.
    let mut entries: Vec<(f32, UiLayoutTypeFlow)> = Vec::new();
    for (state_id, layout_type) in &layout.layouts {
        let UiLayoutType::Flow(config) = layout_type else { continue };
        if let Some(weight) = state.weight(state_id) && weight > 0.0 {
            entries.push((weight, config.clone()));
        }
    }

    // No active states - fall back to the base layout (mirrors the absolute layout blending).
    if entries.is_empty() { return Some(base.clone()) }
    Some(blend_flow_config(&entries))
}

/// Blends multiple weighted flow configurations into one by interpolating numeric parameters.
/// Non-blendable fields (direction, mismatching sizing kinds) are taken from the highest-weight entry.
fn blend_flow_config(entries: &[(f32, UiLayoutTypeFlow)]) -> UiLayoutTypeFlow {
    let (_, dominant) = entries.iter().max_by(|a, b| a.0.total_cmp(&b.0)).expect("entries are non-empty");

    // Incremental weighted mean: acc = lerp(acc, entry, w / (acc_w + w)).
    let mut acc = UiLayoutTypeFlow::new();
    let mut acc_weight = 0.0;
    for (weight, config) in entries {
        let t = if acc_weight + weight > 0.0 { weight / (acc_weight + weight) } else { 0.0 };
        acc = lerp_flow_config(&acc, config, t);
        acc_weight += weight;
    }

    // Non-blendable fields come from the dominant entry.
    acc.direction = dominant.direction;
    acc.justify = dominant.justify;
    acc.wrap = dominant.wrap;
    acc.flipped = dominant.flipped;
    acc.grid = dominant.grid.clone();
    acc.grid_wrap = dominant.grid_wrap;
    let all_same = |predicate: &dyn Fn(&UiLayoutTypeFlow) -> bool| entries.iter().all(|(_, config)| predicate(config));
    if !all_same(&|c| size_kind(&c.width.size) == size_kind(&dominant.width.size)) { acc.width.size = dominant.width.size }
    if !all_same(&|c| size_kind(&c.height.size) == size_kind(&dominant.height.size)) { acc.height.size = dominant.height.size }
    if !all_same(&|c| c.width.min.is_some()) { acc.width.min = dominant.width.min }
    if !all_same(&|c| c.width.max.is_some()) { acc.width.max = dominant.width.max }
    if !all_same(&|c| c.height.min.is_some()) { acc.height.min = dominant.height.min }
    if !all_same(&|c| c.height.max.is_some()) { acc.height.max = dominant.height.max }
    acc
}

/// Kind discriminator for [`UiFlowSize`] used to detect unblendable variant mixes.
fn size_kind(size: &UiFlowSize) -> u8 {
    match size {
        UiFlowSize::Fit => 0,
        UiFlowSize::Grow => 1,
        UiFlowSize::Fixed(_) => 2,
    }
}

/// Interpolates between two flow configurations.
fn lerp_flow_config(a: &UiLayoutTypeFlow, b: &UiLayoutTypeFlow, t: f32) -> UiLayoutTypeFlow {
    let lerp_option = |a: &Option<UiValue<f32>>, b: &Option<UiValue<f32>>| match (a, b) {
        (Some(a), Some(b)) => Some(a.lerp(b, t)),
        (Some(a), None) => Some(*a),
        (None, Some(b)) => Some(*b),
        (None, None) => None,
    };
    UiLayoutTypeFlow {
        direction: if t >= 0.5 { b.direction } else { a.direction },
        width: UiFlowAxis {
            size: match (&a.width.size, &b.width.size) {
                (UiFlowSize::Fixed(va), UiFlowSize::Fixed(vb)) => UiFlowSize::Fixed(va.lerp(vb, t)),
                _ => if t >= 0.5 { b.width.size } else { a.width.size },
            },
            min: lerp_option(&a.width.min, &b.width.min),
            max: lerp_option(&a.width.max, &b.width.max),
        },
        height: UiFlowAxis {
            size: match (&a.height.size, &b.height.size) {
                (UiFlowSize::Fixed(va), UiFlowSize::Fixed(vb)) => UiFlowSize::Fixed(va.lerp(vb, t)),
                _ => if t >= 0.5 { b.height.size } else { a.height.size },
            },
            min: lerp_option(&a.height.min, &b.height.min),
            max: lerp_option(&a.height.max, &b.height.max),
        },
        padding: UiFlowPadding {
            left: a.padding.left.lerp(&b.padding.left, t),
            right: a.padding.right.lerp(&b.padding.right, t),
            top: a.padding.top.lerp(&b.padding.top, t),
            bottom: a.padding.bottom.lerp(&b.padding.bottom, t),
        },
        margin: UiFlowPadding {
            left: a.margin.left.lerp(&b.margin.left, t),
            right: a.margin.right.lerp(&b.margin.right, t),
            top: a.margin.top.lerp(&b.margin.top, t),
            bottom: a.margin.bottom.lerp(&b.margin.bottom, t),
        },
        gap: a.gap.lerp(&b.gap, t),
        align: Align(a.align.0 + (b.align.0 - a.align.0) * t),
        justify: if t >= 0.5 { b.justify } else { a.justify },
        wrap: if t >= 0.5 { b.wrap } else { a.wrap },
        flipped: if t >= 0.5 { b.flipped } else { a.flipped },
        grid: if t >= 0.5 { b.grid.clone() } else { a.grid.clone() },
        grid_wrap: if t >= 0.5 { b.grid_wrap } else { a.grid_wrap },
    }
}


// #================================================================#
// #=== UNIT TESTS - verifying the flow algorithm in isolation  ===#

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> UiFlowContext {
        UiFlowContext { abs_scale: 1.0, viewport: Vec2::new(1000.0, 600.0), font_size: 16.0 }
    }
    /// A leaf with an explicit fixed size.
    fn fixed(size: Vec2) -> FlowItem {
        FlowItem::new(UiLayoutTypeFlow::new().width(size.x).height(size.y))
    }
    /// A `Fit`-sized container whose content hugs the given size (via a fixed child).
    fn fit(flow: &mut FlowLayout, parent: usize, size: Vec2) -> usize {
        let item = flow.push(Some(parent), FlowItem::new(UiLayoutTypeFlow::new()));
        flow.push(Some(item), fixed(size));
        item
    }
    /// A `Fit`-sized wrap-enabled text leaf with a measured size (shrinkable to zero width).
    fn wrap_text(size: Vec2) -> FlowItem {
        FlowItem::new(UiLayoutTypeFlow::new()).with_intrinsic(size).with_wrap_text()
    }
    /// A `Grow`-sized container with a content size of zero.
    fn grow() -> FlowItem {
        FlowItem::new(UiLayoutTypeFlow::new().width(UiFlowSize::Grow).height(UiFlowSize::Grow))
    }
    fn approx(a: f32, b: f32, epsilon: f32) -> bool {
        (a - b).abs() <= epsilon
    }
    fn assert_vec2(actual: Vec2, expected: Vec2) {
        assert!(approx(actual.x, expected.x, 0.1) && approx(actual.y, expected.y, 0.1),
            "expected {expected:?}, got {actual:?}");
    }

    #[test]
    fn row_hugs_content() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new()));
        let a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        let b = flow.push(Some(root), fixed(Vec2::new(200.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        let (pos, size) = flow.result(root);
        assert_vec2(pos, Vec2::ZERO);
        assert_vec2(size, Vec2::new(300.0, 50.0));
        let (pos, size) = flow.result(a);
        assert_vec2(pos, Vec2::ZERO);
        assert_vec2(size, Vec2::new(100.0, 50.0));
        let (pos, _) = flow.result(b);
        assert_vec2(pos, Vec2::new(100.0, 0.0));
    }

    #[test]
    fn row_gap_and_padding() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new()
            .gap(Ab(10.0)).padding_all(Ab(20.0))));
        let a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        let b = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // width = 20 + 100 + 10 + 100 + 20, height = 20 + 50 + 20
        let (_, size) = flow.result(root);
        assert_vec2(size, Vec2::new(250.0, 90.0));
        let (pos, _) = flow.result(a);
        assert_vec2(pos, Vec2::new(20.0, 20.0));
        let (pos, _) = flow.result(b);
        assert_vec2(pos, Vec2::new(130.0, 20.0));
    }

    #[test]
    fn column_layout() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new()
            .direction(UiFlowDirection::TopToBottom).gap(Ab(10.0))));
        let a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        let b = flow.push(Some(root), fixed(Vec2::new(60.0, 30.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        let (_, size) = flow.result(root);
        assert_vec2(size, Vec2::new(100.0, 90.0));
        let (pos, _) = flow.result(a);
        assert_vec2(pos, Vec2::ZERO);
        let (pos, _) = flow.result(b);
        assert_vec2(pos, Vec2::new(0.0, 60.0));
    }

    #[test]
    fn grow_children_share_leftover() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).height(100.0)));
        let a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        let b = flow.push(Some(root), grow());
        let c = flow.push(Some(root), grow());
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        assert_vec2(flow.result(a).1, Vec2::new(100.0, 50.0));
        // 300 leftover split between two grow children
        assert_vec2(flow.result(b).1, Vec2::new(150.0, 100.0));
        assert_vec2(flow.result(c).1, Vec2::new(150.0, 100.0));
        assert_vec2(flow.result(a).0, Vec2::ZERO);
        assert_vec2(flow.result(b).0, Vec2::new(100.0, 0.0));
        assert_vec2(flow.result(c).0, Vec2::new(250.0, 0.0));
    }

    #[test]
    fn grow_respects_max_clamp() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).height(100.0)));
        let _a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        let b = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().width(UiFlowSize::Grow).max_width(Ab(50.0))));
        let c = flow.push(Some(root), grow());
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        assert_vec2(flow.result(b).1, Vec2::new(50.0, 0.0));
        // The grow child without a max clamp takes the rest (and fills the cross axis).
        assert_vec2(flow.result(c).1, Vec2::new(250.0, 100.0));
    }

    #[test]
    fn shrink_reduces_largest_first() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(250.0).height(100.0)));
        let a = flow.push(Some(root), wrap_text(Vec2::new(200.0, 50.0)));
        let b = flow.push(Some(root), wrap_text(Vec2::new(100.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // Deficit of 50: the larger child yields until both are ~150 wide.
        assert_vec2(flow.result(a).1, Vec2::new(150.0, 50.0));
        assert_vec2(flow.result(b).1, Vec2::new(100.0, 50.0));
    }

    #[test]
    fn shrink_floors_at_minimum() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(100.0).height(100.0)));
        // Wrapping text with a minimum width clamp above the available space
        let a = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().min_width(Ab(120.0))).with_intrinsic(Vec2::new(200.0, 50.0)).with_wrap_text());
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // The child shrinks only down to its minimum of 120.
        assert_vec2(flow.result(a).1, Vec2::new(120.0, 50.0));
    }

    #[test]
    fn fixed_content_does_not_shrink() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(250.0).height(100.0)));
        // A fit container of fixed content is rigid: its minimum equals its content.
        let a = fit(&mut flow, root, Vec2::new(200.0, 50.0));
        let b = fit(&mut flow, root, Vec2::new(100.0, 50.0));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // Neither child can shrink below its fixed content, so they overflow instead.
        assert_vec2(flow.result(a).1, Vec2::new(200.0, 50.0));
        assert_vec2(flow.result(b).1, Vec2::new(100.0, 50.0));
    }

    #[test]
    fn fixed_percent_resolves_against_inner_size() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(100.0).height(100.0).gap(Ab(10.0))));
        let a = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().width(Rl(50.0)).height(50.0)));
        let b = fit(&mut flow, root, Vec2::new(30.0, 40.0));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // Percent base = (100 - 10 gap) = 90, so Rl(50) = 45.
        assert_vec2(flow.result(a).1, Vec2::new(45.0, 50.0));
        assert_vec2(flow.result(a).0, Vec2::ZERO);
        assert_vec2(flow.result(b).0, Vec2::new(55.0, 0.0));
    }

    #[test]
    fn relative_sizing_excluded_from_fit_hug() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new()));
        let a = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().width(Rl(50.0)).height(10.0)));
        let b = fit(&mut flow, root, Vec2::new(40.0, 10.0));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // The Fit root hugs only the non-relative child (40 wide), then the
        // relative child resolves to 50% of that (20) and overflows alongside it.
        assert_vec2(flow.result(root).1, Vec2::new(40.0, 10.0));
        assert_vec2(flow.result(a).1, Vec2::new(20.0, 10.0));
        assert_vec2(flow.result(b).1, Vec2::new(40.0, 10.0));
    }

    #[test]
    fn min_max_clamps_on_fit_container() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().min_width(Ab(80.0)).max_width(Ab(120.0))));
        let _child = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());
        assert_vec2(flow.result(root).1, Vec2::new(100.0, 50.0));

        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().max_width(Ab(60.0))));
        let _child = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());
        assert_vec2(flow.result(root).1, Vec2::new(60.0, 50.0));
    }

    #[test]
    fn align_center_distributes_leftover() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(300.0).height(100.0).align(Align::CENTER).justify(UiJustify::Center)));
        let a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        let (pos, _) = flow.result(a);
        assert_vec2(pos, Vec2::new(100.0, 25.0));
    }

    #[test]
    fn align_end_shifts_content() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(300.0).height(100.0).justify(UiJustify::End)));
        let a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        let (pos, _) = flow.result(a);
        assert_vec2(pos, Vec2::new(200.0, 0.0));
    }

    #[test]
    fn cross_grow_fills_parent() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(300.0).height(100.0)));
        let a = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().width(Ab(100.0)).height(UiFlowSize::Grow)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());
        assert_vec2(flow.result(a).1, Vec2::new(100.0, 100.0));
    }

    #[test]
    fn cross_fit_clamped_to_parent() {
        let mut flow = FlowLayout::default();
        // A column parent with a narrow width: the wrapping text child is clamped to it.
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new()
            .direction(UiFlowDirection::TopToBottom).width(Ab(100.0)).height(Ab(100.0))));
        let a = flow.push(Some(root), wrap_text(Vec2::new(200.0, 20.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // The fit child is clamped down to the parent's inner width.
        assert_vec2(flow.result(a).1, Vec2::new(100.0, 20.0));
    }

    #[test]
    fn cross_min_floor_overflows_instead_of_clamping() {
        let mut flow = FlowLayout::default();
        // A row parent clamped to a small height by its own fixed size.
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(300.0).height(Ab(40.0))));
        let a = fit(&mut flow, root, Vec2::new(100.0, 50.0));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // The fit child is floored at its minimum and overflows instead of being clamped below it.
        assert_vec2(flow.result(a).1, Vec2::new(100.0, 50.0));
    }

    #[test]
    fn nested_containers() {
        let mut flow = FlowLayout::default();
        // A classic sidebar layout: fixed sidebar (column with gap) + grow main area
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(1000.0).height(600.0)));
        let sidebar = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new()
            .direction(UiFlowDirection::TopToBottom).gap(Ab(10.0)).width(Ab(300.0)).height(UiFlowSize::Grow)));
        let main = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().width(UiFlowSize::Grow).height(UiFlowSize::Grow)));
        let s1 = flow.push(Some(sidebar), fixed(Vec2::new(50.0, 100.0)));
        let s2 = flow.push(Some(sidebar), fixed(Vec2::new(50.0, 100.0)));

        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        assert_vec2(flow.result(sidebar).1, Vec2::new(300.0, 600.0));
        assert_vec2(flow.result(main).1, Vec2::new(700.0, 600.0));
        assert_vec2(flow.result(main).0, Vec2::new(300.0, 0.0));
        assert_vec2(flow.result(s1).0, Vec2::ZERO);
        assert_vec2(flow.result(s2).0, Vec2::new(0.0, 110.0));
    }

    #[test]
    fn intrinsic_leaf_sizing() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().gap(Ab(10.0))));
        let a = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new()).with_intrinsic(Vec2::new(120.0, 30.0)));
        let b = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new()).with_intrinsic(Vec2::new(80.0, 30.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        assert_vec2(flow.result(root).1, Vec2::new(210.0, 30.0));
        assert_vec2(flow.result(a).1, Vec2::new(120.0, 30.0));
        assert_vec2(flow.result(b).0, Vec2::new(130.0, 0.0));
    }

    #[test]
    fn wrap_text_shrinks_and_keeps_height() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(100.0).height(100.0)));
        let a = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new()).with_intrinsic(Vec2::new(200.0, 20.0)).with_wrap_text());
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // Wrapping text has a zero-width minimum, so it shrinks to the container width.
        assert_vec2(flow.result(a).1, Vec2::new(100.0, 20.0));
    }

    #[test]
    fn unwrapped_text_does_not_shrink() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(100.0).height(100.0)));
        let a = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new()).with_intrinsic(Vec2::new(200.0, 20.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // Non-wrapping text overflows instead of shrinking.
        assert_vec2(flow.result(a).1, Vec2::new(200.0, 20.0));
    }

    #[test]
    fn fit_root_placement_in_absolute_parent() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().align(Align::CENTER).justify(UiJustify::Center)));
        let _child = flow.push(Some(root), fixed(Vec2::new(300.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        let (pos, size) = flow.result(root);
        assert_vec2(size, Vec2::new(300.0, 50.0));
        assert_vec2(pos, Vec2::new(350.0, 275.0));
    }

    #[test]
    fn grow_root_fills_absolute_parent() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(UiFlowSize::Grow).height(UiFlowSize::Grow)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());
        assert_vec2(flow.result(root).1, Vec2::new(1000.0, 600.0));
        assert_vec2(flow.result(root).0, Vec2::ZERO);
    }

    #[test]
    fn units_resolve_in_flow() {
        let mut flow = FlowLayout::default();
        // gap of 1em (16px), padding of 10Vw (100px)
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().gap(Em(1.0)).padding_x(Vw(10.0))));
        let a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        let b = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // width = 100 + 100 + 100 + 16 + 100 = 416
        let (_, size) = flow.result(root);
        assert_vec2(size, Vec2::new(416.0, 50.0));
        let (pos, _) = flow.result(a);
        assert_vec2(pos, Vec2::new(100.0, 0.0));
        let (pos, _) = flow.result(b);
        assert_vec2(pos, Vec2::new(216.0, 0.0));
    }

    #[test]
    fn empty_container_has_zero_cross_size() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().padding_all(Ab(16.0))));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());
        assert_vec2(flow.result(root).1, Vec2::new(32.0, 0.0));
    }

    #[test]
    fn grow_distributes_proportionally() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).height(100.0)));
        // Two grow children with different content sizes (100 and 200): the leftover is
        // split proportionally to the weights, preserving the content-size difference.
        let a = fit(&mut flow, root, Vec2::new(100.0, 50.0));
        let b = fit(&mut flow, root, Vec2::new(200.0, 50.0));
        let a_config = flow.items[a].config.clone();
        flow.items[a].config = UiLayoutTypeFlow { width: UiFlowAxis::new(UiFlowSize::Grow), ..a_config };
        let b_config = flow.items[b].config.clone();
        flow.items[b].config = UiLayoutTypeFlow { width: UiFlowAxis::new(UiFlowSize::Grow), ..b_config };
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // 100 leftover split evenly: both gain 50 on top of their content.
        assert_vec2(flow.result(a).1, Vec2::new(150.0, 50.0));
        assert_vec2(flow.result(b).1, Vec2::new(250.0, 50.0));
    }

    #[test]
    fn state_blending_interpolates_parameters() {
        let base = UiLayoutTypeFlow::new().gap(Ab(10.0));
        let hover = UiLayoutTypeFlow::new().gap(Ab(20.0));
        let blended = blend_flow_config(&[(0.5, base), (0.5, hover)]);
        assert!(approx(blended.gap.evaluate_intrinsic(1.0, Vec2::new(1000.0, 600.0), 16.0, 0), 15.0, 0.001));
    }

    #[test]
    fn state_blending_picks_dominant_direction() {
        let row = UiLayoutTypeFlow::new();
        let column = UiLayoutTypeFlow::new().direction(UiFlowDirection::TopToBottom);
        let blended = blend_flow_config(&[(0.8, row.clone()), (0.2, column.clone())]);
        assert_eq!(blended.direction, UiFlowDirection::LeftToRight);
        let blended = blend_flow_config(&[(0.2, row), (0.8, column)]);
        assert_eq!(blended.direction, UiFlowDirection::TopToBottom);
    }

    #[test]
    fn state_blending_picks_dominant_justify() {
        let start = UiLayoutTypeFlow::new();
        let between = UiLayoutTypeFlow::new().justify(UiJustify::SpaceBetween);
        let blended = blend_flow_config(&[(0.7, start.clone()), (0.3, between.clone())]);
        assert_eq!(blended.justify, UiJustify::Start);
        let blended = blend_flow_config(&[(0.3, start), (0.7, between)]);
        assert_eq!(blended.justify, UiJustify::SpaceBetween);
    }

    #[test]
    fn state_blending_interpolates_margins() {
        let a = UiLayoutTypeFlow::new().margin_x(Ab(10.0));
        let b = UiLayoutTypeFlow::new().margin_x(Ab(20.0));
        let blended = blend_flow_config(&[(0.5, a.clone()), (0.5, b.clone())]);
        let value = blended.margin.left.evaluate_intrinsic(1.0, Vec2::new(1000.0, 600.0), 16.0, 0);
        assert!(approx(value, 15.0, 0.001));
    }

    // #=== JUSTIFY MODES (margin templates) ===#

    /// A 400px row with three 50px children (250px leftover) for each justify mode.
    fn justify_row(flow: &mut FlowLayout, justify: UiJustify) -> (usize, usize, usize) {
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).height(100.0).justify(justify)));
        let a = flow.push(Some(root), fixed(Vec2::new(50.0, 50.0)));
        let b = flow.push(Some(root), fixed(Vec2::new(50.0, 50.0)));
        let c = flow.push(Some(root), fixed(Vec2::new(50.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());
        (a, b, c)
    }

    #[test]
    fn justify_start_packs_children() {
        let mut flow = FlowLayout::default();
        let (a, b, c) = justify_row(&mut flow, UiJustify::Start);
        assert_vec2(flow.result(a).0, Vec2::new(0.0, 0.0));
        assert_vec2(flow.result(b).0, Vec2::new(50.0, 0.0));
        assert_vec2(flow.result(c).0, Vec2::new(100.0, 0.0));
    }

    #[test]
    fn justify_center_centers_block() {
        let mut flow = FlowLayout::default();
        let (a, b, c) = justify_row(&mut flow, UiJustify::Center);
        assert_vec2(flow.result(a).0, Vec2::new(125.0, 0.0));
        assert_vec2(flow.result(b).0, Vec2::new(175.0, 0.0));
        assert_vec2(flow.result(c).0, Vec2::new(225.0, 0.0));
    }

    #[test]
    fn justify_end_pins_to_edge() {
        let mut flow = FlowLayout::default();
        let (a, b, c) = justify_row(&mut flow, UiJustify::End);
        assert_vec2(flow.result(a).0, Vec2::new(250.0, 0.0));
        assert_vec2(flow.result(b).0, Vec2::new(300.0, 0.0));
        assert_vec2(flow.result(c).0, Vec2::new(350.0, 0.0));
    }

    #[test]
    fn justify_space_between() {
        let mut flow = FlowLayout::default();
        let (a, b, c) = justify_row(&mut flow, UiJustify::SpaceBetween);
        assert_vec2(flow.result(a).0, Vec2::new(0.0, 0.0));
        assert_vec2(flow.result(b).0, Vec2::new(175.0, 0.0));
        assert_vec2(flow.result(c).0, Vec2::new(350.0, 0.0));
    }

    #[test]
    fn justify_space_evenly() {
        let mut flow = FlowLayout::default();
        let (a, b, c) = justify_row(&mut flow, UiJustify::SpaceEvenly);
        assert_vec2(flow.result(a).0, Vec2::new(62.5, 0.0));
        assert_vec2(flow.result(b).0, Vec2::new(175.0, 0.0));
        assert_vec2(flow.result(c).0, Vec2::new(287.5, 0.0));
    }

    #[test]
    fn justify_space_around() {
        let mut flow = FlowLayout::default();
        let (a, b, c) = justify_row(&mut flow, UiJustify::SpaceAround);
        // 1sp = 250/6 ~= 41.67: edges 1sp, interior gaps 2sp.
        assert_vec2(flow.result(a).0, Vec2::new(41.67, 0.0));
        assert_vec2(flow.result(b).0, Vec2::new(175.0, 0.0));
        assert_vec2(flow.result(c).0, Vec2::new(308.33, 0.0));
    }

    #[test]
    fn justify_space_between_single_child_pins_to_start() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).height(100.0).justify(UiJustify::SpaceBetween)));
        let a = flow.push(Some(root), fixed(Vec2::new(50.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());
        assert_vec2(flow.result(a).0, Vec2::ZERO);
    }

    // #=== SP SIZING CLAIMS ===#

    #[test]
    fn sp_sizing_distributes_proportionally() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).height(100.0)));
        let a = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().width(Sp(3.0))));
        let b = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().width(Sp(1.0))));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // 3 Sp vs 1 Sp split the 400px leftover 3:1.
        assert_vec2(flow.result(a).1, Vec2::new(300.0, 0.0));
        assert_vec2(flow.result(b).1, Vec2::new(100.0, 0.0));
        assert_vec2(flow.result(a).0, Vec2::ZERO);
        assert_vec2(flow.result(b).0, Vec2::new(300.0, 0.0));
    }

    #[test]
    fn sp_sizing_adds_on_top_of_base() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).height(100.0)));
        let a = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().width(Ab(50.0) + Sp(1.0))));
        let b = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().width(Sp(1.0))));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // 350 leftover split between two equal claims: 50 + 175 and 175.
        assert_vec2(flow.result(a).1, Vec2::new(225.0, 0.0));
        assert_vec2(flow.result(b).1, Vec2::new(175.0, 0.0));
        assert_vec2(flow.result(b).0, Vec2::new(225.0, 0.0));
    }

    #[test]
    fn sp_sizing_respects_max_clamp() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).height(100.0)));
        let a = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().width(Sp(3.0)).max_width(Ab(200.0))));
        let b = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().width(Sp(1.0))));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // `a` pins at 200, `b` re-normalizes and takes the remaining 200.
        assert_vec2(flow.result(a).1, Vec2::new(200.0, 0.0));
        assert_vec2(flow.result(b).1, Vec2::new(200.0, 0.0));
    }

    #[test]
    fn grow_shares_leftover_with_justify_margins() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).height(100.0).justify(UiJustify::SpaceAround)));
        let a = fit(&mut flow, root, Vec2::new(100.0, 50.0));
        let b = flow.push(Some(root), grow());
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // 300 leftover shared between `a`'s two template margins and `b`'s grow claim: 1sp = 100.
        assert_vec2(flow.result(a).0, Vec2::new(100.0, 0.0));
        assert_vec2(flow.result(b).0, Vec2::new(300.0, 0.0));
        assert_vec2(flow.result(b).1, Vec2::new(100.0, 100.0));
    }

    #[test]
    fn sp_resolves_to_zero_on_overflow() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(120.0).height(100.0).justify(UiJustify::SpaceAround)));
        let a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        let b = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // No leftover: all `Sp` margins resolve to 0, rigid children overflow.
        assert_vec2(flow.result(a).0, Vec2::ZERO);
        assert_vec2(flow.result(b).0, Vec2::new(100.0, 0.0));
    }

    // #=== MARGINS ===#

    #[test]
    fn fixed_margins_offset_children_and_participate_in_hug() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new()));
        let a = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().margin_x(Ab(10.0)).width(100.0).height(50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // The Fit root hugs the child plus its margins: 10 + 100 + 10 wide.
        assert_vec2(flow.result(root).1, Vec2::new(120.0, 50.0));
        assert_vec2(flow.result(a).0, Vec2::new(10.0, 0.0));
    }

    #[test]
    fn margin_overrides_justify_template_per_side() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).height(100.0).justify(UiJustify::SpaceBetween)));
        let a = flow.push(Some(root), fixed(Vec2::new(50.0, 50.0)));
        let b = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().margin_left(Sp(2.0)).width(50.0).height(50.0)));
        let c = flow.push(Some(root), fixed(Vec2::new(50.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // Pool: b's overridden 2sp + c's template 1sp = 3 shares of 250 => 83.33 each.
        assert_vec2(flow.result(a).0, Vec2::ZERO);
        assert_vec2(flow.result(b).0, Vec2::new(216.67, 0.0));
        assert_vec2(flow.result(c).0, Vec2::new(350.0, 0.0));
    }

    #[test]
    fn own_sp_margin_aligns_child_end() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).height(100.0)));
        // A child that claims the whole leftover as its left margin aligns itself to the end.
        let a = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().margin_left(Sp(1.0)).width(50.0).height(50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        assert_vec2(flow.result(a).0, Vec2::new(350.0, 0.0));
    }

    #[test]
    fn align_end_pushes_child_to_bottom() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).height(100.0).align(Align::END)));
        let a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // `align: END` injects `margin_top: 1sp` -> the child's whole whitespace is claimed above.
        assert_vec2(flow.result(a).0, Vec2::new(0.0, 50.0));
    }

    #[test]
    fn align_template_suppressed_for_cross_grow() {
        let mut flow = FlowLayout::default();
        // `align: END` with a grow-height child: the child fills instead of being pushed down.
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).height(100.0).align(Align::END)));
        let a = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().width(100.0).height(UiFlowSize::Grow)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        assert_vec2(flow.result(a).1, Vec2::new(100.0, 100.0));
        assert_vec2(flow.result(a).0, Vec2::ZERO);
    }

    #[test]
    fn cross_sp_sizing_with_margin() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).height(100.0)));
        let a = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().margin_top(Ab(20.0)).width(100.0).height(Sp(1.0))));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // The height claim takes the whitespace left by the fixed top margin.
        assert_vec2(flow.result(a).1, Vec2::new(100.0, 80.0));
        assert_vec2(flow.result(a).0, Vec2::new(0.0, 20.0));
    }

    // #=== INVERTED DIRECTIONS ===#

    #[test]
    fn right_to_left_row_mirrors_placement() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).height(100.0).direction(UiFlowDirection::RightToLeft)));
        let a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        let b = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // First child sits at the right edge, the second to its left.
        assert_vec2(flow.result(a).0, Vec2::new(300.0, 0.0));
        assert_vec2(flow.result(b).0, Vec2::new(200.0, 0.0));
    }

    #[test]
    fn bottom_to_top_column_mirrors_placement() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(100.0).height(300.0).direction(UiFlowDirection::BottomToTop)));
        let a = flow.push(Some(root), fixed(Vec2::new(50.0, 100.0)));
        let b = flow.push(Some(root), fixed(Vec2::new(50.0, 100.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // First child sits at the bottom, the second above it.
        assert_vec2(flow.result(a).0, Vec2::new(0.0, 200.0));
        assert_vec2(flow.result(b).0, Vec2::new(0.0, 100.0));
    }

    #[test]
    fn right_to_left_justify_end_pins_to_left() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).height(100.0).direction(UiFlowDirection::RightToLeft).justify(UiJustify::End)));
        let a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // In a RTL row, "end" is the left edge.
        assert_vec2(flow.result(a).0, Vec2::ZERO);
    }

    // #=== LINE WRAPPING ===#

    #[test]
    fn wrap_packs_lines_and_hugs_cross() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(250.0).wrapping().gap(Ab(10.0))));
        let a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        let b = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        let c = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // 100 + 10 + 100 fits, the third item wraps: the cross hug sums both lines.
        assert_vec2(flow.result(root).1, Vec2::new(250.0, 110.0));
        assert_vec2(flow.result(a).0, Vec2::ZERO);
        assert_vec2(flow.result(b).0, Vec2::new(110.0, 0.0));
        assert_vec2(flow.result(c).0, Vec2::new(0.0, 60.0));
    }

    #[test]
    fn wrap_grow_claims_resolve_per_line() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(250.0).wrapping().gap(Ab(10.0))));
        let a = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().width(Ab(100.0) + Sp(1.0)).height(50.0)));
        let b = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().width(Ab(100.0) + Sp(1.0)).height(50.0)));
        let c = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().width(Ab(100.0) + Sp(1.0)).height(50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // Line 1 has 40 leftover split between two claims: both grow to 120.
        assert_vec2(flow.result(a).1, Vec2::new(120.0, 50.0));
        assert_vec2(flow.result(b).1, Vec2::new(120.0, 50.0));
        assert_vec2(flow.result(b).0, Vec2::new(130.0, 0.0));
        // Line 2 has 150 leftover claimed by the single item.
        assert_vec2(flow.result(c).1, Vec2::new(250.0, 50.0));
        assert_vec2(flow.result(c).0, Vec2::new(0.0, 60.0));
    }

    #[test]
    fn wrap_aligns_within_line_extent() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(250.0).height(Ab(100.0)).wrapping().align(Align::END).gap(Ab(10.0))));
        let _a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        let b = flow.push(Some(root), fixed(Vec2::new(100.0, 20.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // `b` sits in the line of the 50-tall `a`: `align: END` pushes it to the line bottom.
        assert_vec2(flow.result(b).0, Vec2::new(110.0, 30.0));
    }

    #[test]
    fn wrap_grow_cross_fills_line_extent() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(250.0).wrapping().gap(Ab(10.0))));
        let _a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        let b = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().width(100.0).height(UiFlowSize::Grow)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // Both children share the first line: the grow-height child fills its 50-tall extent.
        assert_vec2(flow.result(b).1, Vec2::new(100.0, 50.0));
        assert_vec2(flow.result(b).0, Vec2::new(110.0, 0.0));
    }

    #[test]
    fn wrap_oversized_item_gets_own_line() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(100.0).wrapping()));
        let a = flow.push(Some(root), fixed(Vec2::new(150.0, 50.0)));
        let b = flow.push(Some(root), fixed(Vec2::new(50.0, 30.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        assert_vec2(flow.result(a).0, Vec2::ZERO);
        assert_vec2(flow.result(b).0, Vec2::new(0.0, 50.0));
    }

    #[test]
    fn wrap_flipped_stacks_lines_from_end() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(250.0).height(Ab(120.0)).wrapping().flipped().gap(Ab(10.0))));
        let a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        let b = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        let c = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // The first line sits at the bottom, the second line above it.
        assert_vec2(flow.result(a).0, Vec2::new(0.0, 70.0));
        assert_vec2(flow.result(b).0, Vec2::new(110.0, 70.0));
        assert_vec2(flow.result(c).0, Vec2::new(0.0, 10.0));
    }

    #[test]
    fn wrap_grow_main_container_settles_through_fixpoint() {
        let mut flow = FlowLayout::default();
        // A Fit parent whose Grow-width wrap child derives its height from its lines:
        // the wrapped cross size only becomes known during the top-down pass.
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0)));
        let wrap = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().width(UiFlowSize::Grow).wrapping().gap(Ab(10.0))));
        let other = flow.push(Some(root), fixed(Vec2::new(50.0, 30.0)));
        let a = flow.push(Some(wrap), fixed(Vec2::new(120.0, 50.0)));
        let _b = flow.push(Some(wrap), fixed(Vec2::new(120.0, 50.0)));
        let c = flow.push(Some(wrap), fixed(Vec2::new(120.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // The wrap container settles at 350 wide (next to the fixed 50px child), packs two
        // lines (120 + 10 + 120 = 240 fits, 370 overflows), and the Fit parent hugs the
        // resulting 110px cross size.
        assert_vec2(flow.result(root).1, Vec2::new(400.0, 110.0));
        assert_vec2(flow.result(wrap).1, Vec2::new(350.0, 110.0));
        assert_vec2(flow.result(other).0, Vec2::new(350.0, 0.0));
        assert_vec2(flow.result(a).0, Vec2::ZERO);
        assert_vec2(flow.result(c).0, Vec2::new(0.0, 60.0));
    }

    #[test]
    fn wrap_minimum_includes_padding() {
        let mut flow = FlowLayout::default();
        // A Grow-width wrap container with padding, overflowing its parent: it can only
        // compress down to its padding plus the largest child footprint.
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(70.0)));
        let wrap = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new()
            .width(UiFlowSize::Grow).padding_x(Ab(10.0)).wrapping()));
        let _a = flow.push(Some(wrap), fixed(Vec2::new(60.0, 20.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // The container floors at 10 + 60 + 10 = 80 and overflows the 70-wide parent
        // instead of compressing below its own padding.
        assert_vec2(flow.result(wrap).1, Vec2::new(80.0, 20.0));
    }

    #[test]
    fn empty_grid_keeps_wrapping_enabled() {
        let mut flow = FlowLayout::default();
        // `.grid(vec![])` must not disable wrapping set through `.wrapping()`.
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new()
            .width(250.0).wrapping().grid(Vec::<UiFlowSize>::new()).gap(Ab(10.0))));
        let a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        let b = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        let c = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // Two items fit, the third wraps: identical to plain wrapping.
        assert_vec2(flow.result(root).1, Vec2::new(250.0, 110.0));
        assert_vec2(flow.result(a).0, Vec2::ZERO);
        assert_vec2(flow.result(b).0, Vec2::new(110.0, 0.0));
        assert_vec2(flow.result(c).0, Vec2::new(0.0, 60.0));
    }

    // #=== GRID ===#

    #[test]
    fn grid_fit_tracks_hug_items() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).gap(Ab(10.0)).grid([UiFlowSize::Fit, UiFlowSize::Fit])));
        let a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        let b = flow.push(Some(root), fixed(Vec2::new(200.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // Tracks hug their items, the leftover stays after the last track.
        assert_vec2(flow.result(a).1, Vec2::new(100.0, 50.0));
        assert_vec2(flow.result(b).1, Vec2::new(200.0, 50.0));
        assert_vec2(flow.result(a).0, Vec2::ZERO);
        assert_vec2(flow.result(b).0, Vec2::new(110.0, 0.0));
    }

    #[test]
    fn grid_fixed_tracks_stretch_items() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).gap(Ab(10.0)).grid([UiFlowSize::Fixed(Ab(150.0).into()), UiFlowSize::Fixed(Ab(150.0).into())])));
        let a = flow.push(Some(root), fixed(Vec2::new(100.0, 50.0)));
        let b = flow.push(Some(root), fixed(Vec2::new(50.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // Items adhere strictly to the fixed tracks.
        assert_vec2(flow.result(a).1, Vec2::new(150.0, 50.0));
        assert_vec2(flow.result(b).1, Vec2::new(150.0, 50.0));
        assert_vec2(flow.result(b).0, Vec2::new(160.0, 0.0));
    }

    #[test]
    fn grid_sp_tracks_share_leftover() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).gap(Ab(10.0)).grid([Sp(1.0).into(), Sp(3.0).into()])));
        let a = flow.push(Some(root), fixed(Vec2::new(0.0, 50.0)));
        let b = flow.push(Some(root), fixed(Vec2::new(0.0, 50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // 390 leftover split 1:3 between the tracks.
        assert_vec2(flow.result(a).1, Vec2::new(97.5, 50.0));
        assert_vec2(flow.result(b).1, Vec2::new(292.5, 50.0));
        assert_vec2(flow.result(b).0, Vec2::new(107.5, 0.0));
    }

    #[test]
    fn grid_grow_tracks_wrap_lines() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(400.0).gap(Ab(10.0)).grid([UiFlowSize::Grow, UiFlowSize::Grow])));
        let mut children = Vec::new();
        for _ in 0..5 {
            children.push(flow.push(Some(root), fixed(Vec2::new(50.0, 50.0))));
        }
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // Two 195-wide tracks per line; the fifth item's lone track takes the whole row.
        assert_vec2(flow.result(children[0]).1, Vec2::new(195.0, 50.0));
        assert_vec2(flow.result(children[1]).0, Vec2::new(205.0, 0.0));
        assert_vec2(flow.result(children[2]).0, Vec2::new(0.0, 60.0));
        assert_vec2(flow.result(children[4]).1, Vec2::new(400.0, 50.0));
        assert_vec2(flow.result(children[4]).0, Vec2::new(0.0, 120.0));
        // The Fit cross hugs three lines: 50 + 10 + 50 + 10 + 50.
        assert_vec2(flow.result(root).1, Vec2::new(400.0, 170.0));
    }

    #[test]
    fn grid_no_wrap_stays_single_line() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new().width(500.0).gap(Ab(10.0)).grid([UiFlowSize::Fixed(Ab(100.0).into())]).grid_wrap(false)));
        let mut children = Vec::new();
        for _ in 0..4 {
            children.push(flow.push(Some(root), fixed(Vec2::new(100.0, 40.0))));
        }
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // One defined track + three implicit auto tracks, all on a single line.
        for (k, &child) in children.iter().enumerate() {
            assert_vec2(flow.result(child).1, Vec2::new(100.0, 40.0));
            assert_vec2(flow.result(child).0, Vec2::new((k as f32) * 110.0, 0.0));
        }
        assert_vec2(flow.result(root).1, Vec2::new(500.0, 40.0));
    }

    #[test]
    fn vertical_grid_tracks_run_along_y() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new()
            .direction(UiFlowDirection::TopToBottom)
            .width(120.0)
            .grid([UiFlowSize::Fixed(Ab(60.0).into()), UiFlowSize::Fixed(Ab(60.0).into())])));
        let a = flow.push(Some(root), fixed(Vec2::new(30.0, 20.0)));
        let b = flow.push(Some(root), fixed(Vec2::new(30.0, 30.0)));
        let c = flow.push(Some(root), fixed(Vec2::new(30.0, 40.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // Tracks run along the y axis: two 60-tall rows hug the fixed tracks and the
        // container's Fit height. The wrapped third item starts the next line along x.
        assert_vec2(flow.result(root).1, Vec2::new(120.0, 120.0));
        assert_vec2(flow.result(a).1, Vec2::new(30.0, 60.0));
        assert_vec2(flow.result(b).0, Vec2::new(0.0, 60.0));
        assert_vec2(flow.result(c).0, Vec2::new(30.0, 0.0));
        assert_vec2(flow.result(c).1, Vec2::new(30.0, 60.0));
    }

    #[test]
    fn grid_items_respect_max_clamp() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new()
            .width(400.0).grid([UiFlowSize::Fixed(Ab(150.0).into())])));
        let a = flow.push(Some(root), FlowItem::new(UiLayoutTypeFlow::new().max_width(Ab(50.0)).height(50.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // The item is stretched to its track but capped at its own maximum clamp.
        assert_vec2(flow.result(a).1, Vec2::new(50.0, 50.0));
    }

    #[test]
    fn grid_flipped_stacks_lines_from_end() {
        let mut flow = FlowLayout::default();
        let root = flow.push(None, FlowItem::new(UiLayoutTypeFlow::new()
            .width(Ab(50.0)).height(Ab(120.0)).flipped().grid([UiFlowSize::Fixed(Ab(50.0).into())])));
        let a = flow.push(Some(root), fixed(Vec2::new(50.0, 30.0)));
        let b = flow.push(Some(root), fixed(Vec2::new(50.0, 30.0)));
        flow.compute(root, Vec2::new(1000.0, 600.0), &ctx());

        // One item per line; flipped stacks the first line at the bottom edge.
        assert_vec2(flow.result(a).0, Vec2::new(0.0, 90.0));
        assert_vec2(flow.result(b).0, Vec2::new(0.0, 60.0));
    }

    /// Mirrors the doc examples so they stay compile-checked without running the heavy doctests.
    #[test]
    fn public_api_doc_examples_compile() {
        let _size: UiFlowSize = UiFlowSize::Fit;
        let _size: UiFlowSize = UiFlowSize::Grow;
        let _size: UiFlowSize = UiFlowSize::Fixed(Ab(50.0).into());
        let _size: UiFlowSize = UiFlowSize::Fixed(Rl(50.0).into());
        let _size: UiFlowSize = UiFlowSize::Fixed(Sp(1.0).into());
        let _axis: UiFlowAxis = UiFlowAxis::new(UiFlowSize::Grow).max(Rl(80.0).into());
        let _padding: UiFlowPadding = UiFlowPadding::x(Ab(16.0));
        let _margin: UiFlowPadding = UiFlowPadding::bottom(Sp(1.0));
        let _value: UiValue<f32> = Ab(50.0) + Sp(1.0);
        let _layout: UiLayout = UiLayout::flow()
            .direction(UiFlowDirection::TopToBottom)
            .gap(Ab(8.0))
            .padding_all(Ab(16.0))
            .width(UiFlowSize::Grow)
            .height(Rl(50.0))
            .align(Align::CENTER)
            .justify(UiJustify::SpaceBetween)
            .margin_x(Ab(4.0))
            .pack();
        let _layout: UiLayout = UiLayout::flow()
            .width(Ab(400.0))
            .wrapping()
            .flipped()
            .pack();
        let _layout: UiLayout = UiLayout::flow()
            .width(Ab(400.0))
            .grid([UiFlowSize::Grow, UiFlowSize::Fit, Sp(1.0).into()])
            .grid_wrap(false)
            .pack();
    }
}
