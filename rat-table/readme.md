![semver](https://img.shields.io/badge/semver-☑-FFD700)
![stable](https://img.shields.io/badge/stability-stable-8A2BE2)
[![crates.io](https://img.shields.io/crates/v/rat-table.svg)](https://crates.io/crates/rat-table)
[![Documentation](https://docs.rs/rat-ftable/badge.svg)](https://docs.rs/rat-table)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/license-APACHE-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
![](https://tokei.rs/b1/github/thscharler/rat-salsa)

This crate is a part of [rat-salsa][refRatSalsa].

For examples see [rat-table GitHub][refGitHubTable].

* [Changes](https://github.com/thscharler/rat-salsa/blob/master/rat-table/changes.md)

# Table widgets for ratatui

Improves over ratatui-Table by rendering only the visible parts of 
the data instead of building Row/Cell structs with the table data. 

This approach uses a standard Iterator for the table-data and uses a
separate [RenderTableRow] trait to do the actual rendering. It is only
called for the visible parts of the table. 

* Row and column scrolling. 
* Table can be wider than the rendered area. 
* Different selection-models (Cell, Row, RowSet)
  * Can provide your own too.
* Event-handling for crossterm.

There are two table widgets 

* Table - Uses an Iterator for its data.
* TableVec - Keeps a Vec of the data in its state. 

And one more widget

* TableEditVec - Table widget + inline editing of a row.
  * Use any widget to edit the cell-value.

[refRatSalsa]: https://docs.rs/rat-salsa/latest/rat_salsa/

[refGitHubTable]: https://github.com/thscharler/rat-salsa/blob/master/rat-table/examples







