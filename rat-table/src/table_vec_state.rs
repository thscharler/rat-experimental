use crate::_private::NonExhaustive;
use crate::cell_selection::CellSelection;
use crate::row_selection::RowSelection;
use crate::rowset_selection::RowSetSelection;
use crate::{TableSelection, TableState};
use rat_cursor::HasScreenCursor;
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use rat_reloc::RelocatableState;
use ratatui_core::layout::Rect;
use std::collections::HashSet;

pub struct TableVecState<T, Selection = RowSelection> {
    pub table: TableState<Selection>,
    pub data: Vec<T>,

    pub non_exhaustive: NonExhaustive,
}

impl<T, Selection: Default> Default for TableVecState<T, Selection> {
    fn default() -> Self {
        Self {
            table: TableState::default(),
            data: Default::default(),
            non_exhaustive: NonExhaustive,
        }
    }
}

impl<T: Clone, Selection: Clone> Clone for TableVecState<T, Selection> {
    fn clone(&self) -> Self {
        Self {
            table: self.table.clone(),
            data: self.data.clone(),
            non_exhaustive: NonExhaustive,
        }
    }
}

impl<T, Selection> HasFocus for TableVecState<T, Selection> {
    #[inline]
    fn build(&self, builder: &mut FocusBuilder) {
        self.table.build(builder);
    }

    #[inline]
    fn focus(&self) -> FocusFlag {
        self.table.focus.clone()
    }

    #[inline]
    fn area(&self) -> Rect {
        self.table.area
    }
}

impl<T, Selection> HasScreenCursor for TableVecState<T, Selection> {
    fn screen_cursor(&self) -> Option<(u16, u16)> {
        None
    }
}

impl<T, Selection> RelocatableState for TableVecState<T, Selection> {
    fn relocate(&mut self, shift: (i16, i16), clip: Rect) {
        self.table.relocate(shift, clip);
    }
}

impl<T, Selection> TableVecState<T, Selection>
where
    Selection: Default,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn named(name: &str) -> Self {
        Self {
            table: TableState::named(name),
            ..Self::default()
        }
    }
}

impl<T, Selection> TableVecState<T, Selection> {
    pub fn rows(&self) -> usize {
        self.table.rows()
    }

    pub fn columns(&self) -> usize {
        self.table.columns()
    }
}

impl<T, Selection> TableVecState<T, Selection> {
    /// Returns the whole row-area and the cell-areas for the
    /// given row, if it is visible.
    ///
    /// Attention: These areas might be 0-length if the column is scrolled
    /// beyond the table-area.
    pub fn row_cells(&self, row: usize) -> Option<(Rect, Vec<Rect>)> {
        self.table.row_cells(row)
    }

    /// Cell at given position.
    pub fn cell_at_clicked(&self, pos: (u16, u16)) -> Option<(usize, usize)> {
        self.table.cell_at_clicked(pos)
    }

    /// Column at given position.
    pub fn column_at_clicked(&self, pos: (u16, u16)) -> Option<usize> {
        self.table.column_at_clicked(pos)
    }

    /// Row at given position.
    pub fn row_at_clicked(&self, pos: (u16, u16)) -> Option<usize> {
        self.table.row_at_clicked(pos)
    }

    /// Cell when dragging. Position can be outside the table area.
    /// See [row_at_drag](TableState::row_at_drag), [col_at_drag](TableState::column_at_drag)
    pub fn cell_at_drag(&self, pos: (u16, u16)) -> (usize, usize) {
        self.table.cell_at_drag(pos)
    }

    /// Row when dragging. Position can be outside the table area.
    /// If the position is above the table-area this returns offset - #rows.
    /// If the position is below the table-area this returns offset + page_len + #rows.
    ///
    /// This doesn't account for the row-height of the actual rows outside
    /// the table area, just assumes '1'.
    pub fn row_at_drag(&self, pos: (u16, u16)) -> usize {
        self.table.row_at_drag(pos)
    }

    /// Column when dragging. Position can be outside the table area.
    /// If the position is left of the table area this returns offset - 1.
    /// If the position is right of the table area this returns offset + page_width + 1.
    pub fn column_at_drag(&self, pos: (u16, u16)) -> usize {
        self.table.column_at_drag(pos)
    }
}

// Offset related.
impl<T, Selection: TableSelection> TableVecState<T, Selection> {
    /// Sets both offsets to 0.
    pub fn clear_offset(&mut self) {
        self.table.clear_offset()
    }

    /// Maximum offset that is accessible with scrolling.
    ///
    /// This is shorter than the length by whatever fills the last page.
    /// This is the base for the scrollbar content_length.
    pub fn row_max_offset(&self) -> usize {
        self.table.row_max_offset()
    }

    /// Current vertical offset.
    pub fn row_offset(&self) -> usize {
        self.table.row_offset()
    }

    /// Change the vertical offset.
    ///
    /// Due to overscroll it's possible that this is an invalid offset for the widget.
    /// The widget must deal with this situation.
    ///
    /// The widget returns true if the offset changed at all.
    pub fn set_row_offset(&mut self, offset: usize) -> bool {
        self.table.set_row_offset(offset)
    }

    /// Vertical page-size at the current offset.
    pub fn page_len(&self) -> usize {
        self.table.page_len()
    }

    /// Suggested scroll per scroll-event.
    pub fn row_scroll_by(&self) -> usize {
        self.table.row_scroll_by()
    }

    /// Maximum offset that is accessible with scrolling.
    ///
    /// This is shorter than the length of the content by whatever fills the last page.
    /// This is the base for the scrollbar content_length.
    pub fn x_max_offset(&self) -> usize {
        self.table.x_max_offset()
    }

    /// Current horizontal offset.
    pub fn x_offset(&self) -> usize {
        self.table.x_offset()
    }

    /// Change the horizontal offset.
    ///
    /// Due to overscroll it's possible that this is an invalid offset for the widget.
    /// The widget must deal with this situation.
    ///
    /// The widget returns true if the offset changed at all.
    pub fn set_x_offset(&mut self, offset: usize) -> bool {
        self.table.set_x_offset(offset)
    }

    /// Horizontal page-size at the current offset.
    pub fn page_width(&self) -> usize {
        self.table.page_width()
    }

    /// Suggested scroll per scroll-event.
    pub fn x_scroll_by(&self) -> usize {
        self.table.x_scroll_by()
    }

    /// Ensures that the selected item is visible.
    /// Caveat: This doesn't work nicely if you have varying row-heights.
    /// Caveat: Number of rows needs to be correct.
    pub fn scroll_to_selected(&mut self) -> bool {
        self.table.scroll_to_selected()
    }

    /// Ensures that the given row is visible.
    /// Caveat: This doesn't work nicely if you have varying row-heights.
    /// Caveat: Number of rows needs to be correct.
    pub fn scroll_to_row(&mut self, pos: usize) -> bool {
        self.table.scroll_to_row(pos)
    }

    /// Ensures that the given column is completely visible.
    pub fn scroll_to_col(&mut self, pos: usize) -> bool {
        self.table.scroll_to_col(pos)
    }

    /// Ensures that the given position is visible.
    pub fn scroll_to_x(&mut self, pos: usize) -> bool {
        self.table.scroll_to_x(pos)
    }

    /// Reduce the row-offset by n.
    pub fn scroll_up(&mut self, n: usize) -> bool {
        self.table.scroll_up(n)
    }

    /// Increase the row-offset by n.
    pub fn scroll_down(&mut self, n: usize) -> bool {
        self.table.scroll_down(n)
    }

    /// Reduce the col-offset by n.
    pub fn scroll_left(&mut self, n: usize) -> bool {
        self.table.scroll_left(n)
    }

    /// Increase the col-offset by n.
    pub fn scroll_right(&mut self, n: usize) -> bool {
        self.table.scroll_right(n)
    }
}

impl<T, Selection: TableSelection> TableVecState<T, Selection> {
    /// Clear
    pub fn clear(&mut self) {
        self.data.clear();
        self.table.clear_offset();
        self.table.clear_selection();
    }

    /// Clear the selection.
    #[inline]
    pub fn clear_selection(&mut self) {
        self.table.clear_selection()
    }

    /// Data-vec changed.
    pub fn rows_changed(&mut self) {
        self.table.rows_changed(self.data.len());
    }

    /// Set the data-vec.
    pub fn set_value(&mut self, data: Vec<T>) {
        self.data = data;
        self.table.rows_changed(self.data.len());
    }

    /// Add a row to the internal data-vec.
    pub fn append_row(&mut self, value: T) {
        let n = self.data.len();
        self.data.push(value);
        self.table.items_added(n, 1);
    }

    /// Insert a row to the internal data-vec.
    pub fn insert_row(&mut self, row: usize, value: T) {
        self.data.insert(row, value);
        self.table.items_added(row, 1);
    }

    /// Remove a row from the internal data-vec.
    pub fn remove_row(&mut self, row: usize) -> T {
        let r = self.data.remove(row);
        self.table.items_removed(row, 1);
        r
    }
}

impl<T> TableVecState<T, RowSelection> {
    /// Selected row.
    /// The selected row is not constrained by the row-count.
    #[inline]
    pub fn selected(&self) -> Option<usize> {
        self.table.selected()
    }

    /// Select the row.
    /// The selection is not constrained by the row-count.
    #[inline]
    pub fn select(&mut self, row: Option<usize>) -> bool {
        self.table.select(row)
    }

    /// Set the selection to None and set the offset to 0
    #[inline]
    pub fn move_deselect(&mut self) -> bool {
        self.table.move_deselect()
    }

    /// Move the selection to the given row.
    /// Ensures the row is visible afterward.
    #[inline]
    pub fn move_to(&mut self, row: usize) -> bool {
        self.table.move_to(row)
    }

    /// Move the selection up n rows.
    /// Ensures the row is visible afterward.
    #[inline]
    pub fn move_up(&mut self, n: usize) -> bool {
        self.table.move_up(n)
    }

    /// Move the selection down n rows.
    /// Ensures the row is visible afterward.
    #[inline]
    pub fn move_down(&mut self, n: usize) -> bool {
        self.table.move_down(n)
    }
}

impl<T> TableVecState<T, RowSetSelection> {
    /// Anything selected?
    #[inline]
    pub fn has_selection(&mut self) -> bool {
        self.table.has_selection()
    }

    /// Selected rows.
    #[inline]
    pub fn selected(&self) -> HashSet<usize> {
        self.table.selected()
    }

    /// Change the lead-selection. Limits the value to the number of rows.
    /// If extend is false the current selection is cleared and both lead and
    /// anchor are set to the given value.
    /// If extend is true, the anchor is kept where it is and lead is changed.
    /// Everything in the range `anchor..lead` is selected. It doesn't matter
    /// if anchor < lead.
    #[inline]
    pub fn set_lead(&mut self, row: Option<usize>, extend: bool) -> bool {
        self.table.set_lead(row, extend)
    }

    /// Current lead.
    #[inline]
    pub fn lead(&self) -> Option<usize> {
        self.table.lead()
    }

    /// Current anchor.
    #[inline]
    pub fn anchor(&self) -> Option<usize> {
        self.table.anchor()
    }

    /// Retire the current anchor/lead selection to the set of selected rows.
    /// Resets lead and anchor and starts a new selection round.
    #[inline]
    pub fn retire_selection(&mut self) {
        self.table.retire_selection();
    }

    /// Add to selection. Only works for retired selections, not for the
    /// active anchor-lead range.
    ///
    /// To be sure call [retire_selection] first.
    #[inline]
    pub fn add_selected(&mut self, idx: usize) {
        self.table.add_selected(idx);
    }

    /// Remove from selection. Only works for retired selections, not for the
    /// active anchor-lead range.
    ///
    /// To be sure call [retire_selection] first.
    #[inline]
    pub fn remove_selected(&mut self, idx: usize) {
        self.table.remove_selected(idx);
    }

    /// Move the selection to the given row.
    /// Ensures the row is visible afterward.
    #[inline]
    pub fn move_to(&mut self, row: usize, extend: bool) -> bool {
        self.table.move_to(row, extend)
    }

    /// Move the selection up n rows.
    /// Ensures the row is visible afterward.
    #[inline]
    pub fn move_up(&mut self, n: usize, extend: bool) -> bool {
        self.table.move_up(n, extend)
    }

    /// Move the selection down n rows.
    /// Ensures the row is visible afterward.
    #[inline]
    pub fn move_down(&mut self, n: usize, extend: bool) -> bool {
        self.table.move_down(n, extend)
    }
}

impl<T> TableVecState<T, CellSelection> {
    #[inline]
    pub fn has_selection(&mut self) -> bool {
        self.table.has_selection()
    }

    /// Selected cell.
    #[inline]
    pub fn selected(&self) -> Option<(usize, usize)> {
        self.table.selected()
    }

    /// Select a cell.
    #[inline]
    pub fn select_cell(&mut self, select: Option<(usize, usize)>) -> bool {
        self.table.select_cell(select)
    }

    /// Select a row. Column stays the same.
    #[inline]
    pub fn select_row(&mut self, row: Option<usize>) -> bool {
        self.table.select_row(row)
    }

    /// Select a column, row stays the same.
    #[inline]
    pub fn select_column(&mut self, column: Option<usize>) -> bool {
        self.table.select_column(column)
    }

    /// Select a cell, limit to maximum.
    #[inline]
    pub fn move_to(&mut self, select: (usize, usize)) -> bool {
        self.table.move_to(select)
    }

    /// Select a row, limit to maximum.
    #[inline]
    pub fn move_to_row(&mut self, row: usize) -> bool {
        self.table.move_to_row(row)
    }

    /// Select a cell, clamp between 0 and maximum.
    #[inline]
    pub fn move_to_col(&mut self, col: usize) -> bool {
        self.table.move_to_col(col)
    }

    /// Move the selection up n rows.
    /// Ensures the row is visible afterward.
    #[inline]
    pub fn move_up(&mut self, n: usize) -> bool {
        self.table.move_up(n)
    }

    /// Move the selection down n rows.
    /// Ensures the row is visible afterward.
    #[inline]
    pub fn move_down(&mut self, n: usize) -> bool {
        self.table.move_down(n)
    }

    /// Move the selection left n columns.
    /// Ensures the row is visible afterward.
    #[inline]
    pub fn move_left(&mut self, n: usize) -> bool {
        self.table.move_left(n)
    }

    /// Move the selection right n columns.
    /// Ensures the row is visible afterward.
    #[inline]
    pub fn move_right(&mut self, n: usize) -> bool {
        self.table.move_right(n)
    }
}
