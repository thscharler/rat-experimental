//!
//! Render an iterator that counts to u128::MAX, and
//! then still doesn't stop.
//!
//! Table stops displaying things at usize::MAX + some.
//!

#![allow(dead_code)]

use crate::mini_salsa::mock_init;
use crate::mini_salsa::{MiniSalsaState, run_ui, setup_logging};
use rat_scrolled::{Scroll, ScrollStyle};
use rat_table::event::Outcome;
use rat_table::selection::{RowSelection, rowselection};
use rat_table::{Table, TableContext, TableRender, TableState};
use rat_theme4::StyleName;
use rat_theme4::theme::SalsaTheme;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::{Constraint, Flex, Layout, Rect};
use ratatui_core::style::Style;
use ratatui_core::text::Span;
use ratatui_core::widgets::{StatefulWidget, Widget};
use ratatui_crossterm::crossterm::event::Event;
use ratatui_widgets::block::Block;
use ratatui_widgets::borders::BorderType;

mod data;
mod mini_salsa;

fn main() -> Result<(), anyhow::Error> {
    setup_logging()?;

    let mut state = State {
        table: Default::default(),
    };

    run_ui("iter_endless", mock_init, event, render, &mut state)
}

struct State {
    pub(crate) table: TableState<RowSelection>,
}

fn render(
    buf: &mut Buffer,
    area: Rect,
    ctx: &mut MiniSalsaState,
    state: &mut State,
) -> Result<(), anyhow::Error> {
    let l0 = Layout::horizontal([Constraint::Percentage(61)])
        .flex(Flex::Center)
        .split(area);

    struct Count(u128);

    impl Iterator for Count {
        type Item = u128;

        fn next(&mut self) -> Option<Self::Item> {
            self.nth(0)
        }

        // implementing nth is essential here otherwise rendering once
        // every 60 years is a bit too long.
        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            self.0 = self.0.saturating_add(n as u128).saturating_add(1);
            Some(self.0)
        }
    }

    struct R;

    impl TableRender<u128> for R {
        fn render_cell(
            &self,
            _row: usize,
            column: usize,
            v: u128,
            area: Rect,
            buf: &mut Buffer,
            _ctx: &TableContext,
        ) {
            match column {
                0 => {
                    Span::from(v.to_string()).render(area, buf);
                }
                _ => {}
            };
        }
    }

    Table::default()
        .data(Count(0))
        .no_row_count(true)
        .widths([Constraint::Length(21)])
        .column_spacing(1)
        .header(vec!["Nr"])
        .footer(vec!["..."])
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("huge-iterator"),
        )
        .vscroll(Scroll::new())
        .flex(Flex::Center)
        .styles(table(&ctx.theme))
        .select_row_style(Some(ctx.theme.p.gray(3)))
        .render_with(R)
        .render(l0[0], buf, &mut state.table);
    Ok(())
}

fn table(th: &SalsaTheme) -> rat_table::TableStyle {
    rat_table::TableStyle {
        style: th.style(Style::CONTAINER_BASE),
        select_row: Some(th.style(Style::SELECT)),
        show_row_focus: true,
        focus_style: Some(th.style(Style::FOCUS)),
        border_style: Some(th.style(Style::CONTAINER_BORDER_FG)),
        scroll: Some(scroll(th)),
        header: Some(th.style(Style::HEADER)),
        footer: Some(th.style(Style::FOOTER)),
        ..Default::default()
    }
}

fn scroll(th: &SalsaTheme) -> ScrollStyle {
    ScrollStyle {
        thumb_style: Some(th.style(Style::CONTAINER_BORDER_FG)),
        track_style: Some(th.style(Style::CONTAINER_BORDER_FG)),
        min_style: Some(th.style(Style::CONTAINER_BORDER_FG)),
        begin_style: Some(th.style(Style::CONTAINER_ARROW_FG)),
        end_style: Some(th.style(Style::CONTAINER_ARROW_FG)),
        ..Default::default()
    }
}

fn event(
    event: &Event,
    _ctx: &mut MiniSalsaState,
    state: &mut State,
) -> Result<Outcome, anyhow::Error> {
    let r = rowselection::handle_events(&mut state.table, true, event);
    Ok(r.into())
}
