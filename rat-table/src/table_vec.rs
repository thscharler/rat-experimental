use crate::row_selection::RowSelection;
use crate::table_vec_state::TableVecState;
use crate::{Table, TableRender, TableSelection, TableStyle, TableWidth};
use rat_scrolled::Scroll;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Constraint, Flex, Rect};
use ratatui_core::style::Style;
use ratatui_core::text::Text;
use ratatui_core::widgets::StatefulWidget;
use ratatui_widgets::block::Block;
use std::borrow::Cow;

/// Table using internal storage to hold a Vec of
/// items that are the table-items.
pub struct TableVec<'a, T, Selection = RowSelection> {
    table: Table<'a, &'a T, Selection>,
}

impl<'a, T, Selection> Default for TableVec<'a, T, Selection> {
    fn default() -> Self {
        Self {
            table: Default::default(),
        }
    }
}

impl<'a, T: 'a, Selection> TableVec<'a, T, Selection> {
    pub fn new() -> Self
    where
        Selection: Default,
    {
        Self::default()
    }

    pub fn render_with(mut self, render: impl TableRender<&'a T> + 'a) -> Self {
        self.table = self.table.render_with(render);
        self
    }

    /// Set the table-header.
    #[inline]
    pub fn header<I>(mut self, header: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Text<'a>>,
    {
        self.table = self.table.header(header);
        self
    }

    /// Set the table-footer.
    #[inline]
    pub fn footer<I>(mut self, footer: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Text<'a>>,
    {
        self.table = self.table.footer(footer);
        self
    }

    /// Column widths as Constraints.
    #[inline]
    pub fn widths<I>(mut self, widths: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Constraint>,
    {
        self.table = self.table.widths(widths);
        self
    }

    /// Flex for layout.
    #[inline]
    pub fn flex(mut self, flex: Flex) -> Self {
        self.table = self.table.flex(flex);
        self
    }

    /// Spacing between columns.
    #[inline]
    pub fn column_spacing(mut self, spacing: u16) -> Self {
        self.table = self.table.column_spacing(spacing);
        self
    }

    /// Set the display width of the table.
    /// If this is not set, the width of the rendered area is used.
    #[inline]
    pub fn layout_width(mut self, width: TableWidth) -> Self {
        self.table = self.table.layout_width(width);
        self
    }

    /// Draws a block around the table widget.
    #[inline]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.table = self.table.block(block);
        self
    }

    /// Sets the border-style for the Block, if any.
    pub fn border_style(mut self, style: Style) -> Self {
        self.table = self.table.border_style(style);
        self
    }

    /// Sets the title-style for the Block, if any.
    pub fn title_style(mut self, style: Style) -> Self {
        self.table = self.table.title_style(style);
        self
    }

    /// Scrollbars
    pub fn scroll(mut self, scroll: Scroll<'a>) -> Self {
        self.table = self.table.scroll(scroll);
        self
    }

    /// Scrollbars
    pub fn hscroll(mut self, scroll: Scroll<'a>) -> Self {
        self.table = self.table.hscroll(scroll);
        self
    }

    /// Scrollbars
    pub fn vscroll(mut self, scroll: Scroll<'a>) -> Self {
        self.table = self.table.vscroll(scroll);
        self
    }

    /// Set all styles as a bundle.
    #[inline]
    pub fn styles(mut self, styles: TableStyle) -> Self {
        self.table = self.table.styles(styles);
        self
    }

    /// Base style for the table.
    #[inline]
    pub fn style(mut self, style: Style) -> Self {
        self.table = self.table.style(style);
        self
    }

    /// Base style for the table.
    #[inline]
    pub fn header_style(mut self, style: Option<Style>) -> Self {
        self.table = self.table.header_style(style);
        self
    }

    /// Base style for the table.
    #[inline]
    pub fn footer_style(mut self, style: Option<Style>) -> Self {
        self.table = self.table.footer_style(style);
        self
    }

    /// Style for a selected row. The chosen selection must support
    /// row-selection for this to take effect.
    #[inline]
    pub fn select_row_style(mut self, select_style: Option<Style>) -> Self {
        self.table = self.table.select_row_style(select_style);
        self
    }

    /// Add the focus-style to the row-style if the table is focused.
    #[inline]
    pub fn show_row_focus(mut self, show: bool) -> Self {
        self.table = self.table.show_row_focus(show);
        self
    }

    /// Style for a selected column. The chosen selection must support
    /// column-selection for this to take effect.
    #[inline]
    pub fn select_column_style(mut self, select_style: Option<Style>) -> Self {
        self.table = self.table.select_column_style(select_style);
        self
    }

    /// Add the focus-style to the column-style if the table is focused.
    #[inline]
    pub fn show_column_focus(mut self, show: bool) -> Self {
        self.table = self.table.show_column_focus(show);
        self
    }

    /// Style for a selected cell. The chosen selection must support
    /// cell-selection for this to take effect.
    #[inline]
    pub fn select_cell_style(mut self, select_style: Option<Style>) -> Self {
        self.table = self.table.select_cell_style(select_style);
        self
    }

    /// Add the focus-style to the cell-style if the table is focused.
    #[inline]
    pub fn show_cell_focus(mut self, show: bool) -> Self {
        self.table = self.table.show_cell_focus(show);
        self
    }

    /// Style for a selected header cell. The chosen selection must
    /// support column-selection for this to take effect.
    #[inline]
    pub fn select_header_style(mut self, select_style: Option<Style>) -> Self {
        self.table = self.table.select_header_style(select_style);
        self
    }

    /// Add the focus-style to the header-style if the table is focused.
    #[inline]
    pub fn show_header_focus(mut self, show: bool) -> Self {
        self.table = self.table.show_header_focus(show);
        self
    }

    /// Style for a selected footer cell. The chosen selection must
    /// support column-selection for this to take effect.
    #[inline]
    pub fn select_footer_style(mut self, select_style: Option<Style>) -> Self {
        self.table = self.table.select_footer_style(select_style);
        self
    }

    /// Add the footer-style to the table-style if the table is focused.
    #[inline]
    pub fn show_footer_focus(mut self, show: bool) -> Self {
        self.table = self.table.show_footer_focus(show);
        self
    }

    /// This style will be patched onto the selection to indicate that
    /// the widget has the input focus.
    ///
    /// The selection must support some kind of selection for this to
    /// be effective.
    #[inline]
    pub fn focus_style(mut self, focus_style: Option<Style>) -> Self {
        self.table = self.table.focus_style(focus_style);
        self
    }

    /// Show an indicator if the table is empty.
    #[inline]
    pub fn show_empty(mut self, show: bool) -> Self {
        self.table = self.table.show_empty(show);
        self
    }

    #[inline]
    pub fn show_empty_str(mut self, str: impl Into<Cow<'a, str>>) -> Self {
        self.table = self.table.show_empty_str(str);
        self
    }

    /// Inherent width.
    pub fn width(&self) -> u16 {
        self.table.width()
    }

    /// Inherent height.
    pub fn heigth(&self) -> u16 {
        self.table.heigth()
    }
}

impl<'a, T, Selection> StatefulWidget for TableVec<'a, T, Selection>
where
    T: Copy + 'a,
    Selection: TableSelection + Default,
{
    type State = TableVecState<T, Selection>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let table = self.table.data(&state.data);
        table.render(area, buf, &mut state.table);
    }
}
