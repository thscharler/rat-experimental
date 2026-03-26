use crate::_private::NonExhaustive;
use crate::core::TableCoreState;
use crate::derive_table_widget_state;
use crate::event::DoubleClickOutcome;
use crate::selection::TableSelection;
use crate::selection::cell::CellSelection;
use crate::selection::row::RowSelection;
use crate::selection::rowset::RowSetSelection;
use rat_cursor::HasScreenCursor;
use rat_event::crossterm::event::Event;
use rat_event::{DoubleClick, HandleEvent};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus};
use rat_reloc::RelocatableState;
use ratatui_core::layout::Rect;
use std::collections::HashSet;

/// State for [TableVec].
pub struct TableVecState<T, Selection = RowSelection> {
    pub core: TableCoreState<Selection>,
    pub data: Vec<T>,

    pub non_exhaustive: NonExhaustive,
}

impl<T, Selection: Default> Default for TableVecState<T, Selection> {
    fn default() -> Self {
        Self {
            core: TableCoreState::default(),
            data: Default::default(),
            non_exhaustive: NonExhaustive,
        }
    }
}

impl<T: Clone, Selection: Clone> Clone for TableVecState<T, Selection> {
    fn clone(&self) -> Self {
        Self {
            core: self.core.clone(),
            data: self.data.clone(),
            non_exhaustive: NonExhaustive,
        }
    }
}

impl<T, Selection> HasFocus for TableVecState<T, Selection> {
    fn build(&self, builder: &mut FocusBuilder) {
        builder.leaf_widget(self);
    }

    #[inline]
    fn focus(&self) -> FocusFlag {
        self.core.focus.clone()
    }

    #[inline]
    fn area(&self) -> Rect {
        self.core.area
    }
}

impl<T, Selection> HasScreenCursor for TableVecState<T, Selection> {
    fn screen_cursor(&self) -> Option<(u16, u16)> {
        None
    }
}

impl<T, Selection> RelocatableState for TableVecState<T, Selection> {
    fn relocate(&mut self, shift: (i16, i16), clip: Rect) {
        self.core.relocate(shift, clip);
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
            core: TableCoreState::named(name),
            ..Self::default()
        }
    }
}

impl<T, Selection> TableVecState<T, Selection> {
    derive_table_widget_state!(BASE);
}

impl<T, Selection> TableVecState<T, Selection> {
    derive_table_widget_state!(AREAS);
}

// Offset related.
impl<T, Selection: TableSelection> TableVecState<T, Selection> {
    derive_table_widget_state!(OFFSET);
}

impl<T, Selection: TableSelection> TableVecState<T, Selection> {
    derive_table_widget_state!(SELECTION);
}

impl<T, Selection: TableSelection> TableVecState<T, Selection> {
    /// Clear
    #[inline]
    pub fn clear(&mut self) {
        self.core.clear_offset();
        self.core.clear_selection();
        self.data.clear();
    }

    /// Data-vec changed.
    #[inline]
    pub fn rows_changed(&mut self) {
        self.core.rows_changed(self.data.len());
    }

    /// Set the data-vec.
    #[inline]
    pub fn set_value(&mut self, data: Vec<T>) {
        self.data = data;
        self.core.rows_changed(self.data.len());
    }

    /// Get the edit data.
    ///
    /// This will not contain the data of the currently
    /// editing row. Instead, it will contain the value
    /// before the edit started.
    #[inline]
    pub fn value(&self) -> &Vec<T> {
        &self.data
    }

    /// Add a row to the internal data-vec.
    #[inline]
    pub fn append_row(&mut self, value: T) {
        let n = self.data.len();
        self.data.push(value);
        self.core.items_added(n, 1);
    }

    /// Insert a row to the internal data-vec.
    #[inline]
    pub fn insert_row(&mut self, row: usize, value: T) {
        self.data.insert(row, value);
        self.core.items_added(row, 1);
    }

    /// Remove a row from the internal data-vec.
    #[inline]
    pub fn remove_row(&mut self, row: usize) -> T {
        let r = self.data.remove(row);
        self.core.items_removed(row, 1);
        r
    }
}

impl<T> TableVecState<T, RowSelection> {
    derive_table_widget_state!(ROWSELECTION);
}

impl<T> TableVecState<T, RowSetSelection> {
    derive_table_widget_state!(ROWSETSELECTION);
}

impl<T> TableVecState<T, CellSelection> {
    derive_table_widget_state!(CELLSELECTION);
}

impl<Selection> HandleEvent<Event, DoubleClick, DoubleClickOutcome> for TableVecState<Selection> {
    /// Handles double-click events on the table.
    fn handle(&mut self, event: &Event, _keymap: DoubleClick) -> DoubleClickOutcome {
        self.core.handle(event, DoubleClick)
    }
}

/// Handle all events for recognizing double-clicks.
pub fn handle_doubleclick_events<Selection: TableSelection>(
    state: &mut TableVecState<Selection>,
    event: &Event,
) -> DoubleClickOutcome {
    state.handle(event, DoubleClick)
}
