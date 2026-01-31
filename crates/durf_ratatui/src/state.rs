use super::*;

/// State of the durf widget.
/// This accounts for scroll, focus, and other interactions.
/// Elements are counted recursively.
#[derive(Debug, Clone)]
pub struct DurfWidgetState {
    /// The vertical scroll offest.
    pub(crate) scroll: usize,
    /// The focus offset.
    #[allow(unused)]
    pub(crate) focus: usize,
    /// The entire rendered buffer.
    pub(crate) buf: ratatui::buffer::Buffer,
    /// The last rendered total area.
    pub(crate) rendered_area: ratatui::prelude::Rect,
    /// Focusable elements.
    #[allow(unused)]
    pub(crate) focusable: Vec<FocusableNode>,
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

    pub(crate) fn scrollbar_height(&self) -> usize {
        (self.buf.area().height as usize).saturating_sub(self.rendered_area.height as usize)
    }
}

/// A focusable node.
#[derive(Debug, Clone, Default)]
#[allow(unused)]
pub(crate) struct FocusableNode {
    /// Node index of element, counted recursively.
    offset: usize,
    /// Scroll height of element.
    height: usize,
}
