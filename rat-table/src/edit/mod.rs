use rat_focus::HasFocus;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;

mod widget;
mod state;

pub use widget::*;
pub use state::*;

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
