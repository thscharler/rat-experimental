mod cell_selection;
pub mod event;
mod no_selection;
mod row_selection;
mod rowset_selection;
mod table;
mod table_context;
mod table_render;
mod table_selection;
mod table_state;
mod table_style;
mod util;

pub use table::*;
pub use table_context::*;
pub use table_render::*;
pub use table_selection::*;
pub use table_state::*;
pub use table_style::*;

/// Different selection models for Table.
pub mod selection {
    pub use crate::cell_selection::CellSelection;
    pub mod cellselection {
        pub use crate::cell_selection::{handle_events, handle_mouse_events};
    }
    pub use crate::no_selection::NoSelection;
    pub mod noselection {
        pub use crate::no_selection::{handle_events, handle_mouse_events};
    }
    pub use crate::row_selection::RowSelection;
    pub mod rowselection {
        pub use crate::row_selection::{handle_events, handle_mouse_events};
    }
    pub use crate::rowset_selection::RowSetSelection;
    pub mod rowsetselection {
        pub use crate::rowset_selection::{handle_events, handle_mouse_events};
    }
}

mod _private {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct NonExhaustive;
}
