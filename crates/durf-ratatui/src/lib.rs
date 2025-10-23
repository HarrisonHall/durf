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
        // let commands = self.entry.get_commands();
        let commands = ["info"]
            .iter()
            .map(|info| *info)
            .chain(self.entry.get_commands().iter().map(|tab| (*tab).as_str()));
        {
            let mut rect = tab_layouts[0];
            let mut idx = self.entry.result_selection_index;
            rect.height = 1;
            for (i, command) in commands.enumerate() {
                rect.width = command.len() as u16;
                if self.terminal_state.last_frame_inputs.clicked(rect) {
                    idx = i;
                    break;
                }
                rect.x += rect.width + 1;
            }
            self.entry.result_selection_index = idx;
        }

        let commands = ["info"]
            .iter()
            .map(|info| *info)
            .chain(self.entry.get_commands().iter().map(|tab| (*tab).as_str()));
        let tabs = ratatui::widgets::Tabs::new(commands.map(|tab| tab.to_uppercase()))
            .padding("", "")
            .divider(" ")
            // .bg(Color::Green)
            .select(self.entry.result_selection_index)
            .highlight_style((Color::Black, Color::Blue));
        tabs.render(tab_layouts[0], buf);
        match self.entry.get_result() {
            None => {
                EntryInfoWidget(self.entry, self.config).render(tab_layouts[1], buf);
            }
            Some(selected_result) => {
                selected_result.widget().render(tab_layouts[1], buf);
            }
        };
    }
}
