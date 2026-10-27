use crate::core::TableCore;
use crate::derive::derive_table_widget;
use crate::edit::{Mode, TableEditState, TableEditor};
use crate::selection::row::RowSelection;
use crate::util::clear_buf_area;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::widgets::StatefulWidget;

/// Table-widget and inline editor for one table-row.
///
/// This widget keeps a `Vec<RowData>` and modifies it.
///
/// It's parameterized with an `Editor` widget, that renders
/// the input-widgets.
pub struct TableEdit<'a, T, E>
where
    E: TableEditor<T> + 'a,
{
    core: TableCore<'a, T, RowSelection>,
    editor: E,
}

impl<'a, T, E> TableEdit<'a, T, E>
where
    E: TableEditor<T> + 'a,
{
    pub fn new(editor: E) -> Self {
        Self {
            core: Default::default(),
            editor,
        }
    }

    derive_table_widget!();
}

impl<'a, T, E> StatefulWidget for TableEdit<'a, T, E>
where
    E: TableEditor<T> + 'a,
{
    type State = TableEditState<T, E::State>;

    #[allow(clippy::collapsible_else_if)]
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut iter = state.data.iter();
        let data = Some((&mut iter) as &mut dyn Iterator<Item = &T>);

        self.core.render(data, area, buf, &mut state.core);

        if state.mode == Mode::Insert || state.mode == Mode::Edit {
            if let Some(row) = state.core.selected() {
                // but it might be out of view
                if let Some((row_area, cell_areas)) = state.core.row_cells(row) {
                    clear_buf_area(row_area, buf);
                    self.editor
                        .render(row_area, &cell_areas, buf, &mut state.editor);
                }
            }
        }
    }
}
