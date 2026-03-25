use crate::_private::NonExhaustive;
use crate::selection::RowSelection;
use crate::table_render::DefaultRender;
use crate::table_state::TableState;
use crate::table_style::TableStyle;
use crate::util::{fallback_select_style, revert_style, transfer_buffer};
use crate::{TableContext, TableRender, TableSelection, TableWidth};
use rat_focus::HasFocus;
use rat_scrolled::{Scroll, ScrollArea, ScrollAreaState};
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Constraint, Flex, Layout, Rect};
use ratatui_core::style::Style;
use ratatui_core::text::{Span, Text};
use ratatui_core::widgets::Widget;
use ratatui_widgets::block::Block;
use std::borrow::Cow;
use std::cmp::max;
use std::marker::PhantomData;
use std::rc::Rc;

/// Table widget.
pub struct TableCore<'a, T, Selection = RowSelection> {
    header: Vec<Text<'a>>,
    footer: Vec<Text<'a>>,
    render: Box<dyn for<'b> TableRender<&'b T> + 'a>,
    no_row_count: bool,

    widths: Vec<Constraint>,
    flex: Flex,
    column_spacing: u16,
    layout_width: TableWidth,

    style: Style,
    block: Option<Block<'a>>,
    hscroll: Option<Scroll<'a>>,
    vscroll: Option<Scroll<'a>>,
    header_style: Option<Style>,
    footer_style: Option<Style>,
    focus_style: Option<Style>,

    auto_styles: bool,
    select_row_style: Option<Style>,
    show_row_focus: bool,
    select_column_style: Option<Style>,
    show_column_focus: bool,
    select_cell_style: Option<Style>,
    show_cell_focus: bool,
    select_header_style: Option<Style>,
    show_header_focus: bool,
    select_footer_style: Option<Style>,
    show_footer_focus: bool,

    show_empty: bool,
    empty_str: Cow<'a, str>,

    _phantom: PhantomData<Selection>,
}

impl<'a, T, Selection> Default for TableCore<'a, T, Selection> {
    fn default() -> Self {
        Self {
            header: Default::default(),
            footer: Default::default(),
            render: Box::new(DefaultRender),
            no_row_count: Default::default(),
            widths: Default::default(),
            flex: Default::default(),
            column_spacing: Default::default(),
            layout_width: Default::default(),
            style: Default::default(),
            block: Default::default(),
            hscroll: Default::default(),
            vscroll: Default::default(),
            header_style: Default::default(),
            footer_style: Default::default(),
            focus_style: Default::default(),
            auto_styles: true,
            select_row_style: Default::default(),
            show_row_focus: true,
            select_column_style: Default::default(),
            show_column_focus: Default::default(),
            select_cell_style: Default::default(),
            show_cell_focus: Default::default(),
            select_header_style: Default::default(),
            show_header_focus: Default::default(),
            select_footer_style: Default::default(),
            show_footer_focus: Default::default(),
            show_empty: Default::default(),
            empty_str: Cow::Borrowed(" \u{2205} "),
            _phantom: Default::default(),
        }
    }
}

impl<'a, T, Selection> TableCore<'a, T, Selection> {
    /// New, empty Table.
    pub fn new() -> Self
    where
        Selection: Default,
    {
        Self::default()
    }

    /// Don't try to find the number of rows by iterating all values.
    #[inline]
    pub fn no_row_count(mut self, no_row_count: bool) -> Self {
        self.no_row_count = no_row_count;
        self
    }

    #[inline]
    pub fn render_with(mut self, render: impl for<'b> TableRender<&'b T> + 'a) -> Self {
        self.render = Box::new(render);
        self
    }

    /// Set the table-header.
    #[inline]
    pub fn header<I>(mut self, header: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Text<'a>>,
    {
        self.header = header.into_iter().map(|v| v.into()).collect();
        self
    }

    /// Set the table-footer.
    #[inline]
    pub fn footer<I>(mut self, footer: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Text<'a>>,
    {
        self.footer = footer.into_iter().map(|v| v.into()).collect();
        self
    }

    /// Column widths as Constraints.
    #[inline]
    pub fn widths<I>(mut self, widths: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Constraint>,
    {
        self.widths = widths.into_iter().map(|v| v.into()).collect();
        self
    }

    /// Flex for layout.
    #[inline]
    pub fn flex(mut self, flex: Flex) -> Self {
        self.flex = flex;
        self
    }

    /// Spacing between columns.
    #[inline]
    pub fn column_spacing(mut self, spacing: u16) -> Self {
        self.column_spacing = spacing;
        self
    }

    /// Set the display width of the table.
    /// If this is not set, the width of the rendered area is used.
    #[inline]
    pub fn layout_width(mut self, width: TableWidth) -> Self {
        self.layout_width = width;
        self
    }

    /// Draws a block around the table widget.
    #[inline]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self.block = self.block.map(|v| v.style(self.style));
        self
    }

    /// Sets the border-style for the Block, if any.
    pub fn border_style(mut self, style: Style) -> Self {
        self.block = self.block.map(|v| v.border_style(style));
        self
    }

    /// Sets the title-style for the Block, if any.
    pub fn title_style(mut self, style: Style) -> Self {
        self.block = self.block.map(|v| v.title_style(style));
        self
    }

    /// Scrollbars
    pub fn scroll(mut self, scroll: Scroll<'a>) -> Self {
        self.hscroll = Some(scroll.clone().override_horizontal());
        self.vscroll = Some(scroll.override_vertical());
        self
    }

    /// Scrollbars
    pub fn hscroll(mut self, scroll: Scroll<'a>) -> Self {
        self.hscroll = Some(scroll.override_horizontal());
        self
    }

    /// Scrollbars
    pub fn vscroll(mut self, scroll: Scroll<'a>) -> Self {
        self.vscroll = Some(scroll.override_vertical());
        self
    }

    /// Set all styles as a bundle.
    #[inline]
    pub fn styles(mut self, styles: TableStyle) -> Self {
        self.style = styles.style;
        if styles.block.is_some() {
            self.block = styles.block;
        }
        if let Some(border_style) = styles.border_style {
            self.block = self.block.map(|v| v.border_style(border_style));
        }
        if let Some(title_style) = styles.title_style {
            self.block = self.block.map(|v| v.title_style(title_style));
        }
        self.block = self.block.map(|v| v.style(self.style));

        if let Some(styles) = styles.scroll {
            self.hscroll = self.hscroll.map(|v| v.styles(styles.clone()));
            self.vscroll = self.vscroll.map(|v| v.styles(styles));
        }
        if styles.header.is_some() {
            self.header_style = styles.header;
        }
        if styles.footer.is_some() {
            self.footer_style = styles.footer;
        }
        if styles.select_row.is_some() {
            self.select_row_style = styles.select_row;
        }
        self.show_row_focus = styles.show_row_focus;
        if styles.select_column.is_some() {
            self.select_column_style = styles.select_column;
        }
        self.show_column_focus = styles.show_column_focus;
        if styles.select_cell.is_some() {
            self.select_cell_style = styles.select_cell;
        }
        self.show_cell_focus = styles.show_cell_focus;
        if styles.select_header.is_some() {
            self.select_header_style = styles.select_header;
        }
        self.show_header_focus = styles.show_header_focus;
        if styles.select_footer.is_some() {
            self.select_footer_style = styles.select_footer;
        }
        self.show_footer_focus = styles.show_footer_focus;
        if styles.focus_style.is_some() {
            self.focus_style = styles.focus_style;
        }
        self.show_empty = styles.show_empty;
        if let Some(empty_str) = styles.empty_str {
            self.empty_str = empty_str;
        }
        self
    }

    /// Base style for the table.
    #[inline]
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self.block = self.block.map(|v| v.style(self.style));
        self
    }

    /// Base style for the table.
    #[inline]
    pub fn header_style(mut self, style: Option<Style>) -> Self {
        self.header_style = style;
        self
    }

    /// Base style for the table.
    #[inline]
    pub fn footer_style(mut self, style: Option<Style>) -> Self {
        self.footer_style = style;
        self
    }

    /// Style for a selected row. The chosen selection must support
    /// row-selection for this to take effect.
    #[inline]
    pub fn select_row_style(mut self, select_style: Option<Style>) -> Self {
        self.select_row_style = select_style;
        self
    }

    /// Add the focus-style to the row-style if the table is focused.
    #[inline]
    pub fn show_row_focus(mut self, show: bool) -> Self {
        self.show_row_focus = show;
        self
    }

    /// Style for a selected column. The chosen selection must support
    /// column-selection for this to take effect.
    #[inline]
    pub fn select_column_style(mut self, select_style: Option<Style>) -> Self {
        self.select_column_style = select_style;
        self
    }

    /// Add the focus-style to the column-style if the table is focused.
    #[inline]
    pub fn show_column_focus(mut self, show: bool) -> Self {
        self.show_column_focus = show;
        self
    }

    /// Style for a selected cell. The chosen selection must support
    /// cell-selection for this to take effect.
    #[inline]
    pub fn select_cell_style(mut self, select_style: Option<Style>) -> Self {
        self.select_cell_style = select_style;
        self
    }

    /// Add the focus-style to the cell-style if the table is focused.
    #[inline]
    pub fn show_cell_focus(mut self, show: bool) -> Self {
        self.show_cell_focus = show;
        self
    }

    /// Style for a selected header cell. The chosen selection must
    /// support column-selection for this to take effect.
    #[inline]
    pub fn select_header_style(mut self, select_style: Option<Style>) -> Self {
        self.select_header_style = select_style;
        self
    }

    /// Add the focus-style to the header-style if the table is focused.
    #[inline]
    pub fn show_header_focus(mut self, show: bool) -> Self {
        self.show_header_focus = show;
        self
    }

    /// Style for a selected footer cell. The chosen selection must
    /// support column-selection for this to take effect.
    #[inline]
    pub fn select_footer_style(mut self, select_style: Option<Style>) -> Self {
        self.select_footer_style = select_style;
        self
    }

    /// Add the footer-style to the table-style if the table is focused.
    #[inline]
    pub fn show_footer_focus(mut self, show: bool) -> Self {
        self.show_footer_focus = show;
        self
    }

    /// This style will be patched onto the selection to indicate that
    /// the widget has the input focus.
    ///
    /// The selection must support some kind of selection for this to
    /// be effective.
    #[inline]
    pub fn focus_style(mut self, focus_style: Option<Style>) -> Self {
        self.focus_style = focus_style;
        self
    }

    /// Show an indicator if the table is empty.
    #[inline]
    pub fn show_empty(mut self, show: bool) -> Self {
        self.show_empty = show;
        self
    }

    #[inline]
    pub fn show_empty_str(mut self, str: impl Into<Cow<'a, str>>) -> Self {
        self.empty_str = str.into();
        self
    }

    /// Inherent width.
    pub fn width(&self) -> u16 {
        let sa = ScrollArea::new()
            .style(self.style)
            .block(self.block.as_ref())
            .h_scroll(self.hscroll.as_ref())
            .v_scroll(self.vscroll.as_ref());
        let padding = sa.padding();
        let width = total_width(self, 0);

        width + padding.left + padding.right
    }

    /// Inherent height.
    pub fn heigth(&self) -> u16 {
        let sa = ScrollArea::new()
            .style(self.style)
            .block(self.block.as_ref())
            .h_scroll(self.hscroll.as_ref())
            .v_scroll(self.vscroll.as_ref());
        let padding = sa.padding();
        let header = self
            .header
            .iter()
            .map(|v| v.height() as u16)
            .max()
            .unwrap_or(0);
        let footer = self
            .footer
            .iter()
            .map(|v| v.height() as u16)
            .max()
            .unwrap_or(0);

        1 + header + footer + padding.top + padding.bottom
    }
}

fn total_width<T, Selection>(widget: &TableCore<'_, T, Selection>, area_width: u16) -> u16 {
    match widget.layout_width {
        TableWidth::Length(len) => len,
        TableWidth::Columns => {
            let mut width = 0;
            for w in widget.widths.iter().copied() {
                match w {
                    Constraint::Min(v) => width += v + widget.column_spacing,
                    Constraint::Max(v) => width += v + widget.column_spacing,
                    Constraint::Length(v) => width += v + widget.column_spacing,
                    Constraint::Percentage(p) => {
                        width += (((area_width as u32) * (p as u32)) / 100) as u16;
                    }
                    Constraint::Ratio(n, d) => {
                        width += (((area_width as u32) * n) / d) as u16;
                    }
                    Constraint::Fill(_) => {
                        /* unsupported, use dummy */
                        width += 10;
                    }
                }
            }
            max(width, area_width)
        }
        TableWidth::Area => area_width,
    }
}

// Do the column-layout. Fill in missing columns, if necessary.
fn layout_columns<T, Selection>(
    widget: &TableCore<'_, T, Selection>,
    area_width: u16,
) -> (u16, Rc<[Rect]>, Rc<[Rect]>) {
    let width = total_width(widget, area_width);
    let area = Rect::new(0, 0, width, 0);

    let (layout, spacers) = Layout::horizontal(&widget.widths)
        .flex(widget.flex)
        .spacing(widget.column_spacing)
        .split_with_spacers(area);

    (width, layout, spacers)
}

// Layout header/table/footer
#[inline]
fn layout_areas<T, Selection>(widget: &TableCore<'_, T, Selection>, area: Rect) -> Rc<[Rect]> {
    let heights = vec![
        Constraint::Length(
            widget
                .header
                .iter()
                .map(|v| v.height() as u16)
                .max()
                .unwrap_or(0),
        ),
        Constraint::Fill(1),
        Constraint::Length(
            widget
                .footer
                .iter()
                .map(|v| v.height() as u16)
                .max()
                .unwrap_or(0),
        ),
    ];

    Layout::vertical(heights).split(area)
}

impl<T, Selection> TableCore<'_, T, Selection>
where
    Selection: TableSelection,
{
    pub fn render(
        self,
        data: Option<&mut dyn Iterator<Item = &T>>,
        area: Rect,
        buf: &mut Buffer,
        state: &mut TableState<Selection>,
    ) {
        if let Some(mut data) = data {
            render_table(&self, Some(&mut data), area, buf, state);
        } else {
            render_table(&self, None, area, buf, state);
        }
    }
}

pub fn render_table<'a, T, Selection: TableSelection>(
    widget: &TableCore<'a, T, Selection>,
    mut data: Option<&mut dyn Iterator<Item = &T>>,
    area: Rect,
    buf: &mut Buffer,
    state: &mut TableState<Selection>,
) {
    state.rows = data.as_mut().map(|v| v.size_hint().0).unwrap_or(0);
    state.columns = widget.widths.len();
    state.area = area;

    let sa = ScrollArea::new()
        .style(widget.style)
        .block(widget.block.as_ref())
        .h_scroll(widget.hscroll.as_ref())
        .v_scroll(widget.vscroll.as_ref());
    state.inner = sa.inner(area, Some(&state.hscroll), Some(&state.vscroll));

    let l_rows = layout_areas(widget, state.inner);
    state.header_area = l_rows[0];
    state.table_area = l_rows[1];
    state.footer_area = l_rows[2];

    // horizontal layout
    let (width, l_columns, l_spacers) = layout_columns(widget, state.table_area.width);
    calculate_column_areas(state.columns, l_columns.as_ref(), l_spacers.as_ref(), state);

    // render block only
    sa.render_block(area, buf);

    // render header & footer
    render_header(
        widget,
        state.columns,
        width,
        l_columns.as_ref(),
        l_spacers.as_ref(),
        state.header_area,
        buf,
        state,
    );
    render_footer(
        widget,
        state.columns,
        width,
        l_columns.as_ref(),
        l_spacers.as_ref(),
        state.footer_area,
        buf,
        state,
    );

    // render table
    state.row_areas.clear();
    state.vscroll.set_page_len(0);
    state.hscroll.set_page_len(area.width as usize);

    let mut row_buf = Buffer::empty(Rect::new(0, 0, width, 1));
    let mut row;
    let mut row_y = state.table_area.y;
    let mut row_heights = Vec::new();

    let mut ctx = TableContext {
        focus: state.focus.get(),
        selected_cell: false,
        selected_row: false,
        selected_column: false,
        style: widget.style,
        row_style: None,
        select_style: None,
        space_area: Default::default(),
        row_area: Default::default(),
        non_exhaustive: NonExhaustive,
    };

    'll: {
        let Some(data) = data else {
            break 'll;
        };
        let Some(mut value) = data.nth(state.vscroll.offset()) else {
            break 'll;
        };

        row = Some(state.vscroll.offset());
        loop {
            ctx.row_style = widget.render.row_style(row.expect("row"), &value);
            let row_height = widget.render.row_height(row.expect("row"), &value);

            // We render each row to a temporary buffer.
            // For ease of use we start each row at 0,0.
            // We still only render at least partially visible cells.
            let render_row_area = Rect::new(0, 0, width, row_height);
            ctx.row_area = render_row_area;
            row_buf.resize(render_row_area);
            row_heights.push(render_row_area.height);

            if widget.auto_styles {
                if let Some(row_style) = ctx.row_style {
                    row_buf.set_style(render_row_area, row_style);
                } else {
                    row_buf.set_style(render_row_area, widget.style);
                }
            }

            // Target area for the finished row.
            let visible_row_area = Rect::new(
                state.table_area.x,
                row_y,
                state.table_area.width,
                render_row_area.height,
            )
            .intersection(state.table_area);
            state.row_areas.push(visible_row_area);
            // only count fully visible rows.
            if render_row_area.height == visible_row_area.height {
                state.vscroll.set_page_len(state.vscroll.page_len() + 1);
            }

            // can skip this entirely
            if render_row_area.height > 0 {
                let mut col = 0;
                loop {
                    if col >= state.columns {
                        break;
                    }

                    let render_cell_area = Rect::new(
                        l_columns[col].x,
                        0,
                        l_columns[col].width,
                        render_row_area.height,
                    );
                    ctx.space_area = Rect::new(
                        l_spacers[col + 1].x,
                        0,
                        l_spacers[col + 1].width,
                        render_row_area.height,
                    );

                    if state.selection.is_selected_cell(col, row.expect("row")) {
                        ctx.selected_cell = true;
                        ctx.selected_row = false;
                        ctx.selected_column = false;
                        ctx.select_style = patch_select(
                            widget.select_cell_style,
                            widget.focus_style,
                            state.focus.get(),
                            widget.show_cell_focus,
                        );
                    } else if state.selection.is_selected_row(row.expect("row")) {
                        ctx.selected_cell = false;
                        ctx.selected_row = true;
                        ctx.selected_column = false;
                        // use a fallback if no row-selected style is set.
                        ctx.select_style = if widget.select_row_style.is_some() {
                            patch_select(
                                widget.select_row_style,
                                widget.focus_style,
                                state.focus.get(),
                                widget.show_row_focus,
                            )
                        } else {
                            patch_select(
                                Some(widget.style),
                                widget.focus_style,
                                state.focus.get(),
                                widget.show_row_focus,
                            )
                        };
                    } else if state.selection.is_selected_column(col) {
                        ctx.selected_cell = false;
                        ctx.selected_row = false;
                        ctx.selected_column = true;
                        ctx.select_style = patch_select(
                            widget.select_column_style,
                            widget.focus_style,
                            state.focus.get(),
                            widget.show_column_focus,
                        );
                    } else {
                        ctx.selected_cell = false;
                        ctx.selected_row = false;
                        ctx.selected_column = false;
                        ctx.select_style = None;
                    }

                    // partially visible?
                    if render_cell_area.right() > state.hscroll.offset as u16
                        || render_cell_area.left() < state.hscroll.offset as u16 + area.width
                    {
                        if widget.auto_styles {
                            if let Some(select_style) = ctx.select_style {
                                row_buf.set_style(render_cell_area, select_style);
                                row_buf.set_style(ctx.space_area, select_style);
                            }
                        }
                        widget.render.render_cell(
                            row.expect("row"),
                            col,
                            &value,
                            render_cell_area,
                            &mut row_buf,
                            &ctx,
                        );
                    }

                    col += 1;
                }

                // render shifted and clipped row.
                transfer_buffer(
                    &mut row_buf,
                    state.hscroll.offset() as u16,
                    visible_row_area,
                    buf,
                );
            }

            if visible_row_area.bottom() >= state.table_area.bottom() {
                break;
            }

            value = if let Some(value) = data.nth(0) {
                value
            } else {
                break;
            };
            row = Some(row.expect("row").saturating_add(1));
            row_y += render_row_area.height;
        }

        if !widget.no_row_count {
            // skip to a guess for the last page.
            // the guess uses row-height is 1, which may read a few more lines than
            // absolutely necessary.
            let skip_rows = state
                .rows
                .saturating_sub(row.map_or(0, |v| v + 1))
                .saturating_sub(state.table_area.height as usize);
            // if we can still skip some rows, then the data so far is useless.
            if skip_rows > 0 {
                row_heights.clear();
            }
            let nth_row = skip_rows;
            // collect the remaining row-heights.
            if let Some(mut value) = data.nth(nth_row) {
                let mut sum_height = row_heights.iter().sum::<u16>();
                row = Some(row.map_or(nth_row, |row| row + nth_row + 1));
                loop {
                    let row_height = widget.render.row_height(row.expect("row"), &value);
                    row_heights.push(row_height);

                    // Keep a rolling sum of the heights and drop unnecessary info.
                    // We don't need more info, and there will be a lot more otherwise.
                    sum_height += row_height;
                    if sum_height.saturating_sub(row_heights.first().copied().unwrap_or_default())
                        > state.table_area.height
                    {
                        let lost_height = row_heights.remove(0);
                        sum_height -= lost_height;
                    }

                    value = if let Some(value) = data.nth(0) {
                        value
                    } else {
                        break;
                    };

                    row = Some(row.expect("row") + 1);
                }
            } else {
                // skip failed, maybe again?
                // leave everything as is and report later.
            }

            state.rows = row.map_or(0, |v| v + 1);
            // have we got a page worth of data?
            if let Some(last_page) = calc_last_page(state.table_area.height, row_heights) {
                state.vscroll.set_max_offset(state.rows - last_page);
            } else {
                // we don't have enough data to establish the last page.
                // either there are not enough rows or the given row-count
                // was off. make a guess.
                state
                    .vscroll
                    .set_max_offset(state.rows.saturating_sub(state.table_area.height as usize));
            }
            state
                .hscroll
                .set_max_offset(width.saturating_sub(state.table_area.width) as usize);
            state.selection.validate_rows(state.rows);
            state.selection.validate_cols(state.columns);
        } else {
            state.rows = usize::MAX;
            state.vscroll.set_max_offset(usize::MAX);
            state
                .hscroll
                .set_max_offset(width.saturating_sub(state.table_area.width) as usize);
        }
    }

    if state.rows == 0 && widget.show_empty && !state.inner.is_empty() {
        let area = Rect::new(state.table_area.x, state.table_area.y, 3, 1);
        let style = if state.is_focused() {
            widget.focus_style.unwrap_or_default()
        } else {
            widget.style
        };
        Span::from(widget.empty_str.as_ref())
            .style(style)
            .render(area, buf);
    }

    // render only the scrollbars.
    ScrollArea::new()
        .style(widget.style)
        .block(widget.block.as_ref())
        .h_scroll(widget.hscroll.as_ref())
        .v_scroll(widget.vscroll.as_ref())
        .render_scrollbars(
            area,
            buf,
            &mut ScrollAreaState::new()
                .h_scroll(&mut state.hscroll)
                .v_scroll(&mut state.vscroll),
        );
}

#[allow(clippy::too_many_arguments)]
fn render_footer<T, Selection: TableSelection>(
    widget: &TableCore<'_, T, Selection>,
    columns: usize,
    width: u16,
    l_columns: &[Rect],
    l_spacers: &[Rect],
    area: Rect,
    buf: &mut Buffer,
    state: &mut TableState<Selection>,
) {
    let height = widget
        .footer
        .iter()
        .map(|v| v.height() as u16)
        .max()
        .unwrap_or(0);

    if height > 0 {
        let render_row_area = Rect::new(0, 0, width, height);
        let mut row_buf = Buffer::empty(render_row_area);

        row_buf.set_style(render_row_area, widget.style);
        if let Some(footer_style) = widget.footer_style {
            row_buf.set_style(render_row_area, footer_style);
        }

        for (col, text) in widget.footer.iter().enumerate() {
            if col >= columns {
                break;
            }

            let render_cell_area =
                Rect::new(l_columns[col].x, 0, l_columns[col].width, area.height);
            let render_space_area = Rect::new(
                l_spacers[col + 1].x,
                0,
                l_spacers[col + 1].width,
                area.height,
            );

            if state.selection.is_selected_column(col) {
                if let Some(selected_style) = patch_select(
                    widget.select_footer_style,
                    widget.focus_style,
                    state.focus.get(),
                    widget.show_footer_focus,
                ) {
                    row_buf.set_style(render_cell_area, selected_style);
                    row_buf.set_style(render_space_area, selected_style);
                }
            };

            // partially visible?
            if render_cell_area.right() > state.hscroll.offset as u16
                || render_cell_area.left() < state.hscroll.offset as u16 + area.width
            {
                text.render(render_cell_area, &mut row_buf);
            }
        }

        // render shifted and clipped row.
        transfer_buffer(&mut row_buf, state.hscroll.offset() as u16, area, buf);
    }
}

#[allow(clippy::too_many_arguments)]
fn render_header<T, Selection: TableSelection>(
    widget: &TableCore<'_, T, Selection>,
    columns: usize,
    width: u16,
    l_columns: &[Rect],
    l_spacers: &[Rect],
    area: Rect,
    buf: &mut Buffer,
    state: &mut TableState<Selection>,
) {
    let height = widget
        .header
        .iter()
        .map(|v| v.height() as u16)
        .max()
        .unwrap_or(0);

    if height > 0 {
        let render_row_area = Rect::new(0, 0, width, height);
        let mut row_buf = Buffer::empty(render_row_area);

        row_buf.set_style(render_row_area, widget.style);
        if let Some(header_style) = widget.header_style {
            row_buf.set_style(render_row_area, header_style);
        }

        for (col, text) in widget.header.iter().enumerate() {
            if col >= columns {
                break;
            }

            let render_cell_area =
                Rect::new(l_columns[col].x, 0, l_columns[col].width, area.height);
            let render_space_area = Rect::new(
                l_spacers[col + 1].x,
                0,
                l_spacers[col + 1].width,
                area.height,
            );

            if state.selection.is_selected_column(col) {
                if let Some(selected_style) = patch_select(
                    widget.select_header_style,
                    widget.focus_style,
                    state.focus.get(),
                    widget.show_header_focus,
                ) {
                    row_buf.set_style(render_cell_area, selected_style);
                    row_buf.set_style(render_space_area, selected_style);
                }
            };

            // partially visible?
            if render_cell_area.right() > state.hscroll.offset as u16
                || render_cell_area.left() < state.hscroll.offset as u16 + area.width
            {
                text.render(render_cell_area, &mut row_buf);
            }
        }

        // render shifted and clipped row.
        transfer_buffer(&mut row_buf, state.hscroll.offset() as u16, area, buf);
    }
}

fn calculate_column_areas<Selection>(
    columns: usize,
    l_columns: &[Rect],
    l_spacers: &[Rect],
    state: &mut TableState<Selection>,
) {
    state.column_areas.clear();
    state.column_layout.clear();

    let mut col = 0;
    let shift = state.hscroll.offset() as isize;
    loop {
        if col >= columns {
            break;
        }

        state.column_layout.push(Rect::new(
            l_columns[col].x,
            0,
            l_columns[col].width + l_spacers[col + 1].width,
            0,
        ));

        let cell_x1 = l_columns[col].x as isize;
        let cell_x2 = (l_columns[col].x + l_columns[col].width + l_spacers[col + 1].width) as isize;

        let squish_x1 = cell_x1.saturating_sub(shift);
        let squish_x2 = cell_x2.saturating_sub(shift);

        let abs_x1 = max(0, squish_x1) as u16;
        let abs_x2 = max(0, squish_x2) as u16;

        let v_area = Rect::new(
            state.table_area.x + abs_x1,
            state.table_area.y,
            abs_x2 - abs_x1,
            state.table_area.height,
        );
        state
            .column_areas
            .push(v_area.intersection(state.table_area));

        col += 1;
    }
}

#[expect(clippy::collapsible_else_if)]
fn patch_select(
    style: Option<Style>,
    focus_style: Option<Style>,
    focus: bool,
    show: bool,
) -> Option<Style> {
    if let Some(style) = style {
        if let Some(focus_style) = focus_style {
            if focus && show {
                Some(style.patch(focus_style))
            } else {
                Some(fallback_select_style(style))
            }
        } else {
            if focus && show {
                Some(revert_style(style))
            } else {
                Some(fallback_select_style(style))
            }
        }
    } else {
        None
    }
}

fn calc_last_page(height: u16, mut row_heights: Vec<u16>) -> Option<usize> {
    let mut sum_heights = 0;
    let mut n_rows = 0;
    while let Some(h) = row_heights.pop() {
        sum_heights += h;
        n_rows += 1;
        if sum_heights >= height {
            break;
        }
    }

    if sum_heights < height {
        None
    } else {
        Some(n_rows)
    }
}
