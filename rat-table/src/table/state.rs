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

/// State for the [Table].
pub struct TableState<Selection = RowSelection> {
    pub core: TableCoreState<Selection>,
    pub non_exhaustive: NonExhaustive,
}

impl<Selection: Clone> Clone for TableState<Selection> {
    fn clone(&self) -> Self {
        Self {
            core: self.core.clone(),
            non_exhaustive: NonExhaustive,
        }
    }
}

impl<Selection: Default> Default for TableState<Selection> {
    fn default() -> Self {
        Self {
            core: Default::default(),
            non_exhaustive: NonExhaustive,
        }
    }
}

impl<Selection> HasFocus for TableState<Selection> {
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

impl<Selection> HasScreenCursor for TableState<Selection> {
    fn screen_cursor(&self) -> Option<(u16, u16)> {
        None
    }
}

impl<Selection> RelocatableState for TableState<Selection> {
    fn relocate(&mut self, shift: (i16, i16), clip: Rect) {
        self.core.relocate(shift, clip);
    }
}

// Baseline
impl<Selection> TableState<Selection>
where
    Selection: Default,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn named(name: &str) -> Self {
        Self {
            core: TableCoreState::named(name),
            ..TableState::default()
        }
    }
}

impl<Selection> TableState<Selection> {
    derive_table_widget_state!(BASE);
}

impl<Selection> TableState<Selection> {
    derive_table_widget_state!(AREAS);
}

impl<Selection: TableSelection> TableState<Selection> {
    derive_table_widget_state!(OFFSET);
}

impl<Selection: TableSelection> TableState<Selection> {
    derive_table_widget_state!(SELECTION);
}

impl<Selection: TableSelection> TableState<Selection> {
    derive_table_widget_state!(ITEMS);
}

impl TableState<RowSelection> {
    derive_table_widget_state!(ROWSELECTION);
}

impl TableState<RowSetSelection> {
    derive_table_widget_state!(ROWSETSELECTION);
}

impl TableState<CellSelection> {
    derive_table_widget_state!(CELLSELECTION);
}

impl<Selection> HandleEvent<Event, DoubleClick, DoubleClickOutcome> for TableState<Selection> {
    /// Handles double-click events on the table.
    fn handle(&mut self, event: &Event, _keymap: DoubleClick) -> DoubleClickOutcome {
        self.core.handle(event, DoubleClick)
    }
}

/// Handle all events for recognizing double-clicks.
pub fn handle_doubleclick_events<Selection: TableSelection>(
    state: &mut TableState<Selection>,
    event: &Event,
) -> DoubleClickOutcome {
    state.handle(event, DoubleClick)
}
