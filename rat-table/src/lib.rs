pub mod core;
pub mod event;
pub mod render;
pub mod selection;
pub mod style;
pub mod table;
mod table_edit;
mod table_state;
mod util;
pub mod vec;

pub use table_edit::*;

/// Width for the table-layout.
#[derive(Debug, Default)]
pub enum TableWidth {
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
