//! Durf ratatui widget.

use ratatui::{
    prelude::{Buffer, Color, Rect, Widget},
    style::Stylize,
    widgets::{Block, Padding, Paragraph, StatefulWidget, Wrap},
};

/// Durf ratatui widget.
/// Requires a state and style to be managed separately.
pub struct DurfWidget<'a> {
    pub ast: &'a durf_parser::Ast,
    pub state: &'a mut DurfWidgetState,
    pub style: &'a DurfWidgetStyle,
}

impl<'a> DurfWidget<'a> {
    /// Generate the widget.
    /// This is designed for immediate-mode use and does no internal allocation.
    pub fn new(
        ast: &'a durf_parser::Ast,
        state: &'a mut DurfWidgetState,
        style: &'a DurfWidgetStyle,
    ) -> Self {
        Self { ast, state, style }
    }
}

impl<'a> Widget for DurfWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // Re-render if width changes.
        if area.width != self.state.rendered_area.width {
            // Render the full widget/buffer.
            let total_height = self.ast.root.height(area, self.style);
            let full_content_area = Rect::new(0, 0, area.width, total_height as u16);
            let mut full_buf = Buffer::empty(full_content_area.clone());

            let mut ctx = DurfNodeWidgetContext::new(&self.state);

            let widget = DurfNodeWidget {
                node: &self.ast.root,
                state: self.state,
                style: self.style,
                ctx: &mut ctx,
            };
            widget.render(full_content_area, &mut full_buf);
            self.state.rendered_area = area.clone();
            self.state.buf = full_buf;
        }

        // Render to the rect.
        let offset = self.state.scroll;
        let visible_content = self
            .state
            .buf
            .content
            .iter()
            .skip(area.width as usize * offset)
            .take(area.area() as usize);
        for (i, cell) in visible_content.enumerate() {
            let x = i as u16 % area.width;
            let y = i as u16 / area.width;
            buf[(area.x + x, area.y + y)] = cell.clone();
        }

        // Render floating scrollbar.
        let scrollbar_needed = self.state.buf.area().height as u16 > area.height;
        if scrollbar_needed {
            let mut scrollbar_state =
                ratatui::widgets::ScrollbarState::new(self.state.scrollbar_height())
                    .position(offset);
            let scrollbar = ratatui::widgets::Scrollbar::new(
                ratatui::widgets::ScrollbarOrientation::VerticalRight,
            )
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));
            scrollbar.render(
                area.inner(ratatui::layout::Margin {
                    // vertical: 1,
                    vertical: 0,
                    horizontal: 0,
                }),
                buf,
                &mut scrollbar_state,
            );
        }
    }
}

/// Style for changing how the widget is rendered.
#[derive(Debug, Clone, Default)]
pub struct DurfWidgetStyle {
    pub general: GeneralStyle,
}

/// General style for elements.
#[derive(Debug, Clone, Default)]
pub struct GeneralStyle {
    pub section: SectionStyle,
    pub text: TextStyle,
}

/// Section style.
#[derive(Debug, Clone, Default)]
pub struct SectionStyle {
    pub border: BorderStyle,
}

/// Section border style.
#[derive(Debug, Clone, Default)]
pub enum BorderStyle {
    /// Do not show a border.
    None,
    /// Indent the section.
    Indent,
    /// Provide spacing after the section.
    Spacing,
    /// Show a full border around the section.
    #[default]
    FullBorder,
}

/// Text style.
#[derive(Debug, Clone, Default)]
pub struct TextStyle {}

/// State of the durf widget.
/// This accounts for scroll, focus, and other interactions.
/// Elements are counted recursively.
#[derive(Debug, Clone)]
pub struct DurfWidgetState {
    /// The vertical scroll offest.
    scroll: usize,
    /// The focus offset.
    #[allow(unused)]
    focus: usize,
    /// The entire rendered buffer.
    buf: ratatui::buffer::Buffer,
    /// The last rendered total area.
    rendered_area: ratatui::prelude::Rect,
    /// Focusable elements.
    #[allow(unused)]
    focusable: Vec<FocusableNode>,
}

impl Default for DurfWidgetState {
    fn default() -> Self {
        Self {
            scroll: 0,
            focus: 0,
            buf: ratatui::buffer::Buffer::empty(Rect::default()),
            rendered_area: Rect::new(0, 0, 0, 0),
            focusable: Vec::new(),
        }
    }
}

impl DurfWidgetState {
    pub fn scroll(&mut self, lines: i32) {
        if lines > 0 {
            self.scroll = self
                .scroll
                .saturating_add(lines as usize)
                .min(self.scrollbar_height());
        } else {
            self.scroll = self.scroll.saturating_sub(lines.abs() as usize);
        }
    }

    fn scrollbar_height(&self) -> usize {
        (self.buf.area().height as usize).saturating_sub(self.rendered_area.height as usize)
    }
}

/// Dynamic private context for rendering the internal durf widgets.
struct DurfNodeWidgetContext {
    /// Widget height offset.
    offset: usize,
    /// Total height rendered so far.
    #[allow(unused)]
    height: usize,
}

impl DurfNodeWidgetContext {
    fn new(state: &DurfWidgetState) -> Self {
        Self {
            offset: state.scroll,
            height: 0,
        }
    }
}

/// Internal widget for rendering a durf AST node.
struct DurfNodeWidget<'a> {
    node: &'a durf_parser::Node,
    state: &'a mut DurfWidgetState,
    style: &'a DurfWidgetStyle,
    ctx: &'a mut DurfNodeWidgetContext,
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
trait WidgetSize {
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

impl WidgetSize for durf_parser::Text {
    fn height(&self, area: ratatui::prelude::Rect, _style: &DurfWidgetStyle) -> usize {
        if area.width == 0 {
            return 0;
        }

        let total_chars = self.fragments.iter().fold(0usize, |acc, el| {
            acc + {
                if let Some(heading) = &el.attributes.heading {
                    el.text.chars().count() + *heading as usize + 1
                } else {
                    el.text.chars().count()
                }
            }
        });
        let total_lines = total_chars.div_ceil(area.width as usize);
        total_lines
    }
}

/// Convert a fragment into a span.
fn frag_to_span(value: &durf_parser::TextFragment) -> ratatui::text::Span<'static> {
    let mut span = ratatui::text::Span::raw(value.text.to_string());

    if let Some(heading) = &value.attributes.heading {
        span = span.content(format!("{} {}", "#".repeat(*heading as usize), &value.text));
        span = match *heading {
            1 => span.fg(Color::Red),
            2 => span.fg(Color::Blue),
            3 => span.fg(Color::Green),
            4 => span.fg(Color::Red),
            5 => span.fg(Color::Blue),
            6 => span.fg(Color::Green),
            _ => span,
        }
    }

    if value.attributes.bold {
        span = span.bold();
    }
    if value.attributes.italic {
        span = span.italic();
    }
    if value.attributes.preformatted {
        span = span.bg(Color::White);
        span = span.fg(Color::Black);
    }
    if let Some(_) = &value.attributes.link {
        span = span.underlined();
    }

    span
}

/// A focusable node.
#[derive(Debug, Clone, Default)]
#[allow(unused)]
struct FocusableNode {
    /// Node index of element, counted recursively.
    offset: usize,
    /// Scroll height of element.
    height: usize,
}
