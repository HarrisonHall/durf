//! Durf ratatui widget.

use ratatui::{
    prelude::{Buffer, Color, Constraint, Direction, Layout, Rect, Style, Widget},
    style::Stylize,
    widgets::{Block, Clear, Wrap},
};

use durf_parser::Ast;

pub struct AstWidgetState {}

pub struct AstWidget<'a> {
    ast: &'a Ast,
    state: &'a mut AstWidgetState,
}

impl<'a> Widget for AstWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // Render outline.
        let block = Block::bordered().title("test").fg(Color::White);
        let inner_block = block.inner(area);
        block.render(area, buf);

        // Render loaded entry.
        let tab_layouts = Layout::default()
            .direction(Direction::Vertical)
            .constraints(&[Constraint::Min(1), Constraint::Percentage(100)])
            .split(inner_block);

        let tab_names = &["Tab 1", "Tab 2"];
        let tabs = ratatui::widgets::Tabs::new(tab_names.iter().map(|tab| tab.to_uppercase()))
            .padding("", "")
            .divider(" ")
            // .bg(Color::Green)
            .select(0)
            .highlight_style((Color::Black, Color::Blue));
        tabs.render(tab_layouts[0], buf);
    }
}
