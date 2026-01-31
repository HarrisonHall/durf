use super::*;

impl WidgetSize for durf_parser::Section {
    fn height(&self, area: ratatui::prelude::Rect, style: &DurfWidgetStyle) -> usize {
        let mut total = match style.general.section.border {
            BorderStyle::None => 0,
            BorderStyle::Spacing => 1,
            BorderStyle::Indent => 0,
            BorderStyle::FullBorder => 2,
        };
        let area = Rect {
            x: 0,
            y: 0,
            width: match style.general.section.border {
                BorderStyle::None => area.width,
                BorderStyle::Spacing => area.width,
                BorderStyle::Indent => area.width.saturating_sub(1),
                BorderStyle::FullBorder => area.width.saturating_sub(2),
            },
            height: 0,
        };
        for node in &self.nodes {
            total += match &**node {
                durf_parser::RawNode::Empty => 0,
                durf_parser::RawNode::Section(s) => s.height(area, style),
                durf_parser::RawNode::Text(t) => t.height(area, style),
            };
        }
        total
    }
}
