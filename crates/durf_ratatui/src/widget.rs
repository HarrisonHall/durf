use super::*;

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

    pub fn handle_click(&mut self, pos: Position) -> Option<DurfEvent> {
        for focusable in &self.state.focusable {
            for rect in &focusable.rect {
                if rect.contains(pos) {
                    self.state.focused_element = Some(focusable.index);
                    self.state.should_rerender = true;
                    // return Some(DurfEvent::FollowLink()))
                    return None;
                }
            }
        }
        if self.state.focused_element.is_some() {
            self.state.focused_element = None;
            self.state.should_rerender = true;
        }
        None
    }
}

impl<'a> Widget for DurfWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // Re-render if width changes.
        self.state.should_rerender |= area.width != self.state.rendered_area.width;
        if self.state.should_rerender {
            // Render the full widget/buffer.
            let total_height = self.ast.root.height(area, self.style);
            let full_content_area = Rect::new(0, 0, area.width, total_height as u16);
            let mut full_buf = Buffer::empty(full_content_area.clone());

            let mut ctx = DurfNodeWidgetContext::new(&self.state);
            self.state.focusable.clear();

            let widget = DurfNodeWidget {
                node: &self.ast.root,
                state: self.state,
                style: self.style,
                ctx: &mut ctx,
            };
            widget.render(full_content_area, &mut full_buf);
            self.state.rendered_area = area.clone();
            self.state.buf = full_buf;
            self.state.should_rerender = false;
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

/// Dynamic private context for rendering the internal durf widgets.
pub(crate) struct DurfNodeWidgetContext {
    /// Widget height offset.
    pub(crate) offset: usize,
    /// Total height rendered so far.
    #[allow(unused)]
    pub(crate) height: usize,
    /// Widget index.
    pub(crate) index: usize,
}

impl DurfNodeWidgetContext {
    pub(crate) fn new(state: &DurfWidgetState) -> Self {
        Self {
            offset: state.scroll,
            height: 0,
            index: 0,
        }
    }
}

pub enum DurfEvent {
    FollowLink(String),
}
