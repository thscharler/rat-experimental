use crate::_private::NonExhaustive;
use ratatui_core::layout::Rect;
use ratatui_core::style::Style;

/// Render-context for rendering a table-cell.
#[derive(Debug)]
pub struct TableContext {
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
    /// Row style if any.
    pub row_style: Option<Style>,
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
