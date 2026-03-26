use crate::core::TableCore;
use crate::render::RenderTableRow;
use crate::selection::TableSelection;
use crate::selection::row::RowSelection;
use crate::style::TableStyle;
use crate::{TableState, TableWidth};
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
pub struct Table<'a, T, Selection = RowSelection> {
    core: TableCore<'a, T, Selection>,
    data: Option<Box<dyn Iterator<Item = &'a T> + 'a>>,
}

impl<'a, T, Selection> Default for Table<'a, T, Selection> {
    fn default() -> Self {
        Self {
            core: Default::default(),
            data: Default::default(),
        }
    }
}

impl<'a, T: 'a, Selection> Table<'a, T, Selection> {
    pub fn new() -> Self
    where
        Selection: Default,
    {
        Self::default()
    }

    #[inline]
    pub fn data(mut self, data: impl IntoIterator<Item = &'a T> + 'a) -> Self {
        self.data = Some(Box::new(data.into_iter()));
        self
    }

    /// Don't try to find the number of rows by iterating all values.
    #[inline]
    pub fn no_row_count(mut self, no_row_count: bool) -> Self {
        self.core = self.core.no_row_count(no_row_count);
        self
    }

    #[inline]
    pub fn render_with(mut self, render: impl for<'b> RenderTableRow<&'b T> + 'a) -> Self {
        self.core = self.core.render_with(render);
        self
    }

    /// Set the table-header.
    #[inline]
    pub fn header<I>(mut self, header: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Text<'a>>,
    {
        self.core = self.core.header(header);
        self
    }

    /// Set the table-footer.
    #[inline]
    pub fn footer<I>(mut self, footer: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Text<'a>>,
    {
        self.core = self.core.footer(footer);
        self
    }

    /// Column widths as Constraints.
    #[inline]
    pub fn widths<I>(mut self, widths: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Constraint>,
    {
        self.core = self.core.widths(widths);
        self
    }

    /// Flex for layout.
    #[inline]
    pub fn flex(mut self, flex: Flex) -> Self {
        self.core = self.core.flex(flex);
        self
    }

    /// Spacing between columns.
    #[inline]
    pub fn column_spacing(mut self, spacing: u16) -> Self {
        self.core = self.core.column_spacing(spacing);
        self
    }

    /// Set the display width of the table.
    /// If this is not set, the width of the rendered area is used.
    #[inline]
    pub fn layout_width(mut self, width: TableWidth) -> Self {
        self.core = self.core.layout_width(width);
        self
    }

    /// Draws a block around the table widget.
    #[inline]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.core = self.core.block(block);
        self
    }

    /// Sets the border-style for the Block, if any.
    pub fn border_style(mut self, style: Style) -> Self {
        self.core = self.core.border_style(style);
        self
    }

    /// Sets the title-style for the Block, if any.
    pub fn title_style(mut self, style: Style) -> Self {
        self.core = self.core.title_style(style);
        self
    }

    /// Scrollbars
    pub fn scroll(mut self, scroll: Scroll<'a>) -> Self {
        self.core = self.core.scroll(scroll);
        self
    }

    /// Scrollbars
    pub fn hscroll(mut self, scroll: Scroll<'a>) -> Self {
        self.core = self.core.hscroll(scroll);
        self
    }

    /// Scrollbars
    pub fn vscroll(mut self, scroll: Scroll<'a>) -> Self {
        self.core = self.core.vscroll(scroll);
        self
    }

    /// Set all styles as a bundle.
    #[inline]
    pub fn styles(mut self, styles: TableStyle) -> Self {
        self.core = self.core.styles(styles);
        self
    }

    /// Base style for the table.
    #[inline]
    pub fn style(mut self, style: Style) -> Self {
        self.core = self.core.style(style);
        self
    }

    /// Base style for the table.
    #[inline]
    pub fn header_style(mut self, style: Option<Style>) -> Self {
        self.core = self.core.header_style(style);
        self
    }

    /// Base style for the table.
    #[inline]
    pub fn footer_style(mut self, style: Option<Style>) -> Self {
        self.core = self.core.footer_style(style);
        self
    }

    /// Style for a selected row. The chosen selection must support
    /// row-selection for this to take effect.
    #[inline]
    pub fn select_row_style(mut self, select_style: Option<Style>) -> Self {
        self.core = self.core.select_row_style(select_style);
        self
    }

    /// Add the focus-style to the row-style if the table is focused.
    #[inline]
    pub fn show_row_focus(mut self, show: bool) -> Self {
        self.core = self.core.show_row_focus(show);
        self
    }

    /// Style for a selected column. The chosen selection must support
    /// column-selection for this to take effect.
    #[inline]
    pub fn select_column_style(mut self, select_style: Option<Style>) -> Self {
        self.core = self.core.select_column_style(select_style);
        self
    }

    /// Add the focus-style to the column-style if the table is focused.
    #[inline]
    pub fn show_column_focus(mut self, show: bool) -> Self {
        self.core = self.core.show_column_focus(show);
        self
    }

    /// Style for a selected cell. The chosen selection must support
    /// cell-selection for this to take effect.
    #[inline]
    pub fn select_cell_style(mut self, select_style: Option<Style>) -> Self {
        self.core = self.core.select_cell_style(select_style);
        self
    }

    /// Add the focus-style to the cell-style if the table is focused.
    #[inline]
    pub fn show_cell_focus(mut self, show: bool) -> Self {
        self.core = self.core.show_cell_focus(show);
        self
    }

    /// Style for a selected header cell. The chosen selection must
    /// support column-selection for this to take effect.
    #[inline]
    pub fn select_header_style(mut self, select_style: Option<Style>) -> Self {
        self.core = self.core.select_header_style(select_style);
        self
    }

    /// Add the focus-style to the header-style if the table is focused.
    #[inline]
    pub fn show_header_focus(mut self, show: bool) -> Self {
        self.core = self.core.show_header_focus(show);
        self
    }

    /// Style for a selected footer cell. The chosen selection must
    /// support column-selection for this to take effect.
    #[inline]
    pub fn select_footer_style(mut self, select_style: Option<Style>) -> Self {
        self.core = self.core.select_footer_style(select_style);
        self
    }

    /// Add the footer-style to the table-style if the table is focused.
    #[inline]
    pub fn show_footer_focus(mut self, show: bool) -> Self {
        self.core = self.core.show_footer_focus(show);
        self
    }

    /// This style will be patched onto the selection to indicate that
    /// the widget has the input focus.
    ///
    /// The selection must support some kind of selection for this to
    /// be effective.
    #[inline]
    pub fn focus_style(mut self, focus_style: Option<Style>) -> Self {
        self.core = self.core.focus_style(focus_style);
        self
    }

    /// Show an indicator if the table is empty.
    #[inline]
    pub fn show_empty(mut self, show: bool) -> Self {
        self.core = self.core.show_empty(show);
        self
    }

    #[inline]
    pub fn show_empty_str(mut self, str: impl Into<Cow<'a, str>>) -> Self {
        self.core = self.core.show_empty_str(str);
        self
    }

    /// Inherent width.
    pub fn width(&self) -> u16 {
        self.core.width()
    }

    /// Inherent height.
    pub fn heigth(&self) -> u16 {
        self.core.heigth()
    }
}

impl<'a, T, Selection> StatefulWidget for Table<'a, T, Selection>
where
    T: 'a,
    Selection: TableSelection + Default,
{
    type State = TableState<Selection>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if let Some(mut data) = self.data {
            self.core
                .render(Some(&mut data), area, buf, &mut state.core);
        } else {
            self.core.render(None, area, buf, &mut state.core);
        }
    }
}
