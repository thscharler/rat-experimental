use crate::core::TableCore;
use crate::derive::derive_table_widget;
use crate::selection::TableSelection;
use crate::selection::row::RowSelection;
use crate::vec::TableVecState;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::widgets::StatefulWidget;

///
/// A table that holds a Vec of the data in its state.
///
pub struct TableVec<'a, T, Selection = RowSelection> {
    core: TableCore<'a, T, Selection>,
}

impl<'a, T: 'a, Selection> Default for TableVec<'a, T, Selection> {
    fn default() -> Self {
        Self {
            core: Default::default(),
        }
    }
}

impl<'a, T, Selection> TableVec<'a, T, Selection> {
    pub fn new() -> Self
    where
        Selection: Default,
    {
        Self::default()
    }

    derive_table_widget!();
}

impl<'a, T, Selection> StatefulWidget for TableVec<'a, T, Selection>
where
    Selection: TableSelection + Default,
{
    type State = TableVecState<T, Selection>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut iter = state.data.iter();
        let data = Some((&mut iter) as &mut dyn Iterator<Item = &T>);
        self.core.render(data, area, buf, &mut state.core);
    }
}
