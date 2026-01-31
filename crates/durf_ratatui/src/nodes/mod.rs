use super::*;

mod section;
mod text;

#[allow(unused)]
pub(crate) use section::*;
#[allow(unused)]
pub(crate) use text::*;

/// Internal widget for rendering a durf AST node.
pub(crate) struct DurfNodeWidget<'a> {
    pub(crate) node: &'a durf_parser::Node,
    pub(crate) state: &'a mut DurfWidgetState,
    pub(crate) style: &'a DurfWidgetStyle,
    pub(crate) ctx: &'a mut DurfNodeWidgetContext,
}

impl<'a> Widget for DurfNodeWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // Stop rendering with no space left.
        if area.height == 0 {
            return;
        }

        // Render bsaed on element.
        match &**self.node {
            durf_parser::RawNode::Empty => {
                // Empty does not render.
            }
            durf_parser::RawNode::Text(text) => {
                let line =
                    ratatui::text::Line::from_iter(text.fragments.iter().map(|f| frag_to_span(f)));
                // let line = ratatui::widgets::Paragraph::from(
                //     text.fragments.iter().map(|f| frag_to_span(f)),
                // );
                // line.width();
                // let paragraph = ratatui::widgets::Paragraph::from(line);
                // line.render(area, buf);
                // let text = ratatui::text::Text::from_iter(&[line]);
                let text = ratatui::text::Text::from(vec![line]);
                // let p = Paragraph::from(text.fragments.iter().map(|f| frag_to_span(f)));
                let p = Paragraph::new(text).wrap(Wrap { trim: false });
                p.render(area, buf);
            }
            durf_parser::RawNode::Section(section) => {
                // Render outline.
                let block = match self.style.general.section.border {
                    BorderStyle::None => Block::new(),
                    BorderStyle::Indent => Block::new().padding(Padding {
                        left: 1,
                        right: 0,
                        top: 0,
                        bottom: 0,
                    }),
                    BorderStyle::Spacing => Block::new().padding(Padding {
                        left: 0,
                        right: 0,
                        top: 0,
                        bottom: 1,
                    }),
                    BorderStyle::FullBorder => Block::bordered()
                        .fg(Color::White)
                        .border_type(ratatui::widgets::BorderType::Rounded),
                };
                let mut inner_block = block.inner(area);
                block.render(area, buf);

                // Render inner area.
                for inner_node in &section.nodes {
                    self.ctx.offset += 1;
                    let node_widget = DurfNodeWidget {
                        node: inner_node,
                        state: self.state,
                        style: self.style,
                        ctx: self.ctx,
                    };
                    node_widget.render(inner_block, buf);
                    let max_rendered_height = inner_node.height(inner_block, self.style);
                    inner_block.height = inner_block
                        .height
                        .saturating_sub(max_rendered_height as u16);
                    inner_block.y = inner_block.y.saturating_add(max_rendered_height as u16);
                }
            }
        }
    }
}

/// Internal trait for determining the size of widgets.
pub(crate) trait WidgetSize {
    /// Get the height of a widget.
    fn height(&self, area: ratatui::prelude::Rect, style: &DurfWidgetStyle) -> usize;
}

impl WidgetSize for durf_parser::Node {
    fn height(&self, area: ratatui::prelude::Rect, style: &DurfWidgetStyle) -> usize {
        match &**self {
            durf_parser::RawNode::Empty => 0,
            durf_parser::RawNode::Section(s) => s.height(area, style),
            durf_parser::RawNode::Text(t) => t.height(area, style),
        }
    }
}
