use crate::core::TableCore;
use crate::edit::{Mode, TableEditVecState, TableEditor};
use crate::selection::row::RowSelection;
use crate::util::clear_buf_area;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::widgets::StatefulWidget;

/// Widget that supports row-wise editing of a table.
///
/// This widget keeps a `Vec<RowData>` and modifies it.
///
/// It's parameterized with a `Editor` widget, that renders
/// the input line and handles events.
pub struct TableEditVec<'a, T, E>
where
    E: TableEditor<T> + 'a,
{
    core: TableCore<'a, T, RowSelection>,
    editor: E,
}

impl<'a, T, E> TableEditVec<'a, T, E>
where
    E: TableEditor<T> + 'a,
{
    pub fn new(editor: E) -> Self {
        Self {
            core: Default::default(),
            editor,
        }
    }
}

// attr

impl<'a, T, E> StatefulWidget for TableEditVec<'a, T, E>
where
    E: TableEditor<T> + 'a,
{
    type State = TableEditVecState<T, E::State>;

    #[allow(clippy::collapsible_else_if)]
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut iter = state.data.iter();
        let data = Some((&mut iter) as &mut dyn Iterator<Item = &T>);

        self.core.render(data, area, buf, &mut state.table);

        if state.mode == Mode::Insert || state.mode == Mode::Edit {
            if let Some(row) = state.table.selected() {
                // but it might be out of view
                if let Some((row_area, cell_areas)) = state.table.row_cells(row) {
                    clear_buf_area(row_area, buf);
                    self.editor
                        .render(row_area, &cell_areas, buf, &mut state.editor);
                }
            }
        }
    }
}
