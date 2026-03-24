use crate::TableContext;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::style::Style;
use std::fmt::{Debug, Formatter};

/// Trait for accessing the table-data by the Table.
pub trait TableRender<T> {
    /// Row height for the given item.
    #[allow(unused_variables)]
    fn row_height(&self, row: usize, v: &T) -> u16 {
        1
    }

    /// Row style for the current line.
    #[allow(unused_variables)]
    fn row_style(&self, row: usize, v: &T) -> Option<Style> {
        None
    }

    /// Render the given cell for the current line.
    fn render_cell(
        &self,
        row: usize,
        column: usize,
        v: &T,
        area: Rect,
        buf: &mut Buffer,
        ctx: &TableContext, //
    );
}

impl<'a, T> Debug for dyn TableRender<T> {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

pub(crate) struct DefaultRender;

impl<T> TableRender<T> for DefaultRender {
    fn render_cell(&self, _: usize, _: usize, _: &T, _: Rect, _: &mut Buffer, _: &TableContext) {}
}
