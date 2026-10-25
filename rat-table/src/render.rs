use crate::_private::NonExhaustive;
use ratatui_core::layout::Rect;
use ratatui_core::style::Style;
use std::fmt::Debug;

/// Render-context for rendering a table-cell.
#[derive(Debug)]
pub struct RenderContext<'a> {
    /// Current row.
    pub row: usize,

    /// Focus flag is set.
    pub focus: bool,

    /// Cell is selected.
    pub selected_cell: bool,
    /// Row of the cell is selected.
    pub selected_row: bool,
    /// Column of the cell is selected.
    pub selected_column: bool,

    /// Base style
    pub style: Style,
    /// Selection style if any.
    pub select_style: Option<Style>,

    /// Column layout for all columns.
    /// Only the x-position and width are set,
    /// y-position and height are 0.
    pub column_layout: &'a [Rect],
    /// Spacing after the current cell.
    /// Will be empty when calling row-rendering.
    pub space_area: Rect,
    /// Total area for the current row.
    pub row_area: Rect,

    /// Construct with `..Default::default()`
    pub non_exhaustive: NonExhaustive,
}

/// Return value for [RenderTableRow::render_row].
pub enum Cells {
    /// Skip rendering individual cells.
    Skip,
    /// Render the individual cells.
    Render,
    /// Render the individual cells but don't draw the background.
    NoBackground,
}
