use crate::core::TableCore;
use crate::derive_table_widget;
use crate::selection::TableSelection;
use crate::selection::row::RowSelection;
use crate::table::TableState;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::widgets::StatefulWidget;

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

    derive_table_widget!();
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
