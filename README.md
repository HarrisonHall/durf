# durf

Durf is a dumb, document parser/surfer.

> [!WARNING]
>
> This crate is not production-ready!

- `durf_parser` - Parse documents into durf AST.
- `durf_ratatui` - durf AST [ratatui](https://ratatui.rs/) widget.
- `durf_engine` - System for opening, fetching, caching, and linking durf ASTs.
- `durf_browser` - GUI browser for durf documents.

## Features

- `durf_parser`
  - [x] Parse from HTML
  - [ ] Parse from markdown
  - [x] Support parse flags
  - [ ] Parse lists
  - [ ] Parse classes
- `durf_ratatui`
  - [x] Ratatui widget
  - [x] Basic style flags
  - [ ] Advanced style flags
  - [ ] Navigate by focus
- `durf_engine`
  - [ ] Fetch engine
  - [ ] Cache mechanism
  - [ ] Document linking
- `durf_browser`
  - [ ] GUI
  - [ ] Advanced Ratatui widget
