use crate::_private::NonExhaustive;
use crate::core::{TableCore, TableCoreState};
use crate::selection::row::RowSelection;
use crate::util::clear_buf_area;
use rat_cursor::HasScreenCursor;
use rat_event::crossterm::event::Event;
use rat_event::util::MouseFlags;
use rat_event::{HandleEvent, Outcome, Regular, ct_event, event_flow};
use rat_focus::{FocusBuilder, FocusFlag, HasFocus, Navigation};
use rat_reloc::RelocatableState;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::widgets::StatefulWidget;

/// StatefulWidget alike trait.
///
/// This one takes a slice of areas for all the cells in the table,
/// and renders all input widgets as it needs.
pub trait TableEditor<T> {
    /// State associated with the stateful widget.
    type State: TableEditorState<T>;

    /// Standard render call, but with added areas for each cell.
    fn render(
        &self, //
        area: Rect,
        cell_areas: &[Rect],
        buf: &mut Buffer,
        state: &mut Self::State,
    );
}

/// Trait for the editor widget state
pub trait TableEditorState<T>: HasFocus {
    /// Some external context.
    type Context<'a>;
    /// Error type.
    type Err;

    /// Create a fresh value with all the defaults.
    fn create_value<'a>(&self, ctx: &'a Self::Context<'a>) -> Result<T, Self::Err>;

    /// Set the current value for the editor.
    fn set_value<'a>(&mut self, value: T, ctx: &'a Self::Context<'a>) -> Result<(), Self::Err>;

    /// Return the current value from the editor.
    fn value<'a>(&mut self, ctx: &'a Self::Context<'a>) -> Result<Option<T>, Self::Err>;

    /// Returns the currently focused column.
    /// Used to scroll the column to fully visible.
    fn focused_col(&self) -> Option<usize>;

    /// Allows setting the focus for the editor.
    fn set_focused_col(&self, col: usize);
}

/// Editing mode.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    #[default]
    View,
    Edit,
    Insert,
}

pub struct TableEdit<'a, T, E>
where
    E: TableEditor<T> + 'a,
{
    core: TableCore<'a, T, RowSelection>,
    editor: E,
}

pub struct TableEditState<T, S>
where
    S: TableEditorState<T>,
{
    pub core: TableCoreState<RowSelection>,
    pub mode: Mode,
    pub editor: S,
    pub data: Vec<T>,

    pub mouse: MouseFlags,

    pub non_exhaustive: NonExhaustive,
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

impl<T, S> Default for TableEditState<T, S>
where
    S: TableEditorState<T> + Default,
{
    fn default() -> Self {
        Self {
            mode: Default::default(),
            core: Default::default(),
            editor: Default::default(),
            data: Default::default(),
            mouse: Default::default(),
            non_exhaustive: NonExhaustive,
        }
    }
}

impl<T, S> HasFocus for TableEditState<T, S>
where
    S: TableEditorState<T>,
{
    fn build(&self, builder: &mut FocusBuilder) {
        builder.leaf_widget(self);
    }

    fn focus(&self) -> FocusFlag {
        self.core.focus.focus()
    }

    fn area(&self) -> Rect {
        self.core.focus.area()
    }

    fn navigable(&self) -> Navigation {
        match self.mode {
            Mode::View => self.core.focus.navigable(),
            Mode::Edit | Mode::Insert => Navigation::Lock,
        }
    }
}

impl<T, S> HasScreenCursor for TableEditState<T, S>
where
    S: TableEditorState<T> + HasScreenCursor,
{
    fn screen_cursor(&self) -> Option<(u16, u16)> {
        match self.mode {
            Mode::View => None,
            Mode::Edit | Mode::Insert => self.editor.screen_cursor(),
        }
    }
}

impl<T, S> RelocatableState for TableEditState<T, S>
where
    S: TableEditorState<T> + RelocatableState,
{
    fn relocate(&mut self, shift: (i16, i16), clip: Rect) {
        match self.mode {
            Mode::View => {}
            Mode::Edit | Mode::Insert => {
                self.editor.relocate(shift, clip);
            }
        }
    }
}

impl<T, S> TableEditState<T, S>
where
    S: TableEditorState<T>,
{
    pub fn new(editor: S) -> Self {
        Self {
            mode: Mode::View,
            core: Default::default(),
            editor,
            data: Default::default(),
            mouse: Default::default(),
            non_exhaustive: NonExhaustive,
        }
    }

    pub fn named(name: &str, editor: S) -> Self {
        Self {
            mode: Mode::View,
            core: TableCoreState::named(name),
            editor,
            data: Default::default(),
            mouse: Default::default(),
            non_exhaustive: NonExhaustive,
        }
    }
}

impl<T, S> TableEditState<T, S>
where
    S: TableEditorState<T>,
    T: Clone,
{
    /// Clear the widget.
    pub fn clear(&mut self) {
        self.mode = Mode::View;
        self.core.clear_offset();
        self.core.clear_selection();
        self.data.clear();
    }

    /// Clear the selection.
    #[inline]
    pub fn clear_selection(&mut self) {
        self.core.clear_selection()
    }

    /// Data-vec changed.
    pub fn rows_changed(&mut self) {
        self.core.rows_changed(self.data.len());
    }

    /// Set the edit data.
    pub fn set_value(&mut self, data: Vec<T>) {
        self.data = data;
        self.core.rows_changed(self.data.len());
    }

    /// Get the edit data.
    ///
    /// This will not contain the data of the currently
    /// editing row. Instead, it will contain the value
    /// before the edit started.
    pub fn value(&self) -> &Vec<T> {
        &self.data
    }

    /// Editing is active?
    pub fn is_editing(&self) -> bool {
        self.mode == Mode::Edit || self.mode == Mode::Insert
    }

    /// Is the current edit an insert?
    pub fn is_insert(&self) -> bool {
        self.mode == Mode::Insert
    }

    /// Remove the item at the selected row.
    pub fn remove_row(&mut self, row: usize) -> Option<T> {
        if self.mode != Mode::View {
            return None;
        }
        if row < self.data.len() {
            let r = self.data.remove(row);
            self.core.items_removed(row, 1);
            if !self.core.scroll_to_row(row) {
                self.core.scroll_to_row(row.saturating_sub(1));
            }
            Some(r)
        } else {
            None
        }
    }

    /// Edit a new item inserted at the selected row.
    pub fn edit_new<'a>(&mut self, row: usize, ctx: &'a S::Context<'a>) -> Result<(), S::Err> {
        if self.mode != Mode::View {
            return Ok(());
        }
        let value = self.editor.create_value(ctx)?;
        self.editor.set_value(value.clone(), ctx)?;
        self.data.insert(row, value);
        self.core.items_added(row, 1);
        self._start(0, row, Mode::Insert);
        Ok(())
    }

    /// Edit the item at the selected row.
    pub fn edit<'a>(
        &mut self,
        col: usize,
        row: usize,
        ctx: &'a S::Context<'a>,
    ) -> Result<(), S::Err> {
        if self.mode != Mode::View {
            return Ok(());
        }
        {
            let value = &self.data[row];
            self.editor.set_value(value.clone(), ctx)?;
        }
        self._start(col, row, Mode::Edit);
        Ok(())
    }

    fn _start(&mut self, col: usize, row: usize, mode: Mode) {
        if self.core.focus.is_focused() {
            // black magic
            FocusBuilder::build_for(&self.editor).first();
        }

        self.mode = mode;
        self.core.move_to(row);
        self.core.scroll_to_col(col);
        self.editor.set_focused_col(col);
    }

    /// Cancel editing.
    ///
    /// Updates the state to remove the edited row.
    pub fn cancel(&mut self) {
        if self.mode == Mode::View {
            return;
        }
        let Some(row) = self.core.selected() else {
            return;
        };
        if self.mode == Mode::Insert {
            self.data.remove(row);
            self.core.items_removed(row, 1);
        }
        self._stop();
    }

    /// Commit the changes in the editor.
    pub fn commit<'a>(&mut self, ctx: &'a S::Context<'a>) -> Result<(), S::Err> {
        if self.mode == Mode::View {
            return Ok(());
        }
        let Some(row) = self.core.selected() else {
            return Ok(());
        };
        {
            let value = self.editor.value(ctx)?;
            if let Some(value) = value {
                self.data[row] = value;
            } else {
                self.data.remove(row);
                self.core.items_removed(row, 1);
            }
        }
        self._stop();
        Ok(())
    }

    pub fn commit_and_append<'a>(&mut self, ctx: &'a S::Context<'a>) -> Result<(), S::Err> {
        self.commit(ctx)?;
        if let Some(row) = self.core.selected() {
            self.edit_new(row + 1, ctx)?;
        }
        Ok(())
    }

    pub fn commit_and_edit<'a>(&mut self, ctx: &'a S::Context<'a>) -> Result<(), S::Err> {
        let Some(row) = self.core.selected() else {
            return Ok(());
        };

        self.commit(ctx)?;
        if row + 1 < self.data.len() {
            self.core.select(Some(row + 1));
            self.edit(0, row + 1, ctx)?;
        }
        Ok(())
    }

    fn _stop(&mut self) {
        self.mode = Mode::View;
        self.core.scroll_to_col(0);
    }
}

impl<'a, T, S> HandleEvent<Event, &'a S::Context<'a>, Result<Outcome, S::Err>>
    for TableEditState<T, S>
where
    S: HandleEvent<Event, &'a S::Context<'a>, Result<Outcome, S::Err>>,
    S: TableEditorState<T>,
    T: Clone,
{
    #[allow(clippy::collapsible_if)]
    fn handle(&mut self, event: &Event, ctx: &'a S::Context<'a>) -> Result<Outcome, S::Err> {
        if self.mode == Mode::Edit || self.mode == Mode::Insert {
            if self.is_focused() {
                event_flow!({
                    let r = self.editor.handle(event, ctx)?;
                    if let Some(col) = self.editor.focused_col() {
                        if self.core.scroll_to_col(col) {
                            Outcome::Changed
                        } else {
                            r
                        }
                    } else {
                        r
                    }
                });

                event_flow!(match event {
                    ct_event!(keycode press Esc) => {
                        self.cancel();
                        Outcome::Changed
                    }
                    ct_event!(keycode press Enter) => {
                        if self.core.selected() < Some(self.core.rows().saturating_sub(1)) {
                            self.commit_and_edit(ctx)?;
                            Outcome::Changed
                        } else {
                            self.commit_and_append(ctx)?;
                            Outcome::Changed
                        }
                    }
                    ct_event!(keycode press Up) => {
                        self.commit(ctx)?;
                        if self.data.is_empty() {
                            self.edit_new(0, ctx)?;
                        } else if let Some(row) = self.core.selected()
                            && row > 0
                        {
                            self.core.select(Some(row));
                        }
                        Outcome::Changed
                    }
                    ct_event!(keycode press Down) => {
                        self.commit(ctx)?;
                        if self.data.is_empty() {
                            self.edit_new(0, ctx)?;
                        } else if let Some(row) = self.core.selected()
                            && row + 1 < self.data.len()
                        {
                            self.core.select(Some(row + 1));
                        }
                        Outcome::Changed
                    }
                    _ => Outcome::Continue,
                });
            }

            Ok(Outcome::Continue)
        } else {
            if self.core.focus.gained_focus() {
                if self.data.is_empty() {
                    self.edit_new(0, ctx)?;
                }
            }

            event_flow!(match event {
                ct_event!(mouse any for m) if self.mouse.doubleclick(self.core.table_area, m) => {
                    if let Some((col, row)) = self.core.cell_at_clicked((m.column, m.row)) {
                        self.edit(col, row, ctx)?;
                        Outcome::Changed
                    } else {
                        Outcome::Continue
                    }
                }
                _ => Outcome::Continue,
            });

            if self.is_focused() {
                event_flow!(match event {
                    ct_event!(keycode press Insert) => {
                        if let Some(row) = self.core.selected() {
                            self.edit_new(row, ctx)?;
                        }
                        Outcome::Changed
                    }
                    ct_event!(keycode press Delete) => {
                        if let Some(row) = self.core.selected() {
                            self.remove_row(row);
                            if self.data.is_empty() {
                                self.edit_new(0, ctx)?;
                            }
                        }
                        Outcome::Changed
                    }
                    ct_event!(keycode press Enter) | ct_event!(keycode press F(2)) => {
                        if let Some(row) = self.core.selected() {
                            self.edit(0, row, ctx)?;
                            Outcome::Changed
                        } else if self.core.rows() == 0 {
                            self.edit_new(0, ctx)?;
                            Outcome::Changed
                        } else {
                            Outcome::Continue
                        }
                    }
                    ct_event!(keycode press Down) => {
                        if let Some(row) = self.core.selected() {
                            if row == self.core.rows().saturating_sub(1) {
                                self.edit_new(row + 1, ctx)?;
                                Outcome::Changed
                            } else {
                                Outcome::Continue
                            }
                        } else if self.core.rows() == 0 {
                            self.edit_new(0, ctx)?;
                            Outcome::Changed
                        } else {
                            Outcome::Continue
                        }
                    }
                    _ => {
                        Outcome::Continue
                    }
                });
            }

            event_flow!(self.core.handle(event, Regular));

            Ok(Outcome::Continue)
        }
    }
}
