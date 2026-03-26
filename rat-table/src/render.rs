use crate::_private::NonExhaustive;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::style::Style;
use std::fmt::{Debug, Formatter};

/// Render-context for rendering a table-cell.
#[derive(Debug)]
pub struct RenderContext {
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

/// Trait for accessing the table-data by the Table.
pub trait RenderTableRow<T> {
    /// Row height for the given item.
    #[allow(unused_variables)]
    fn row_height(&self, row: usize, v: T) -> u16 {
        1
    }

    /// Render the row.
    ///
    /// The given area is the area for the whole row.
    #[allow(unused_variables)]
    fn render_row(
        &self,
        row: usize,
        v: T,
        area: Rect,
        buf: &mut Buffer,
        ctx: &RenderContext,
    ) -> Cells {
        Cells::Render
    }

    /// Render the given cell for the current line.
    ///
    /// The area is the area of the cell.
    fn render_cell(
        &self,
        row: usize,
        column: usize,
        v: T,
        area: Rect,
        buf: &mut Buffer,
        ctx: &RenderContext, //
    );
}

impl<'a, T> Debug for dyn RenderTableRow<T> {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

pub(crate) struct DefaultRender;

impl<T> RenderTableRow<T> for DefaultRender {
    fn render_cell(&self, _: usize, _: usize, _: T, _: Rect, _: &mut Buffer, _: &RenderContext) {}
}
