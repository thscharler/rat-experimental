use crate::_private::NonExhaustive;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::style::Style;
use std::fmt::{Debug, Formatter};

/// Render-context for rendering a table-cell.
#[derive(Debug)]
pub struct RenderContext {
    /// Row to be rendered.
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

    /// Spacing after the cell. It's guaranteed that this
    /// is writeable in the buffer given to render_cell.
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
