#![doc = include_str!("../readme.md")]

pub mod core;
mod derive;
mod edit;
pub mod event;
mod render;
pub mod selection;
mod style;
mod table;
mod util;
mod vec;

pub use edit::{Mode, TableEdit, TableEditState, TableEditor, TableEditorState};
pub use render::{Cells, RenderContext};
pub use style::TableStyle;
pub use table::{Table, TableState};
pub use vec::{TableVec, TableVecState};

/// Width for the table-layout.
#[derive(Debug, Default)]
pub enum LayoutWidth {
    /// Fixed width. Columns are layout for this size.
    Length(u16),
    /// Width is the sum of the columns. If the width is
    /// larger than the area, horizontal scrolling is used.
    ///
    /// Note: Constraint::Fill is useless for this.
    /// Such columns will assume a width of 10.
    Columns,
    /// Fit the columns to the table-area.
    #[default]
    Area,
}

mod _private {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct NonExhaustive;
}
