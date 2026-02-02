//! Prelude.

pub use crate::state::*;
pub use crate::style::*;
pub use crate::util;
pub use crate::widget::*;
pub use ratatui::layout::{Position, Rect};

pub(crate) mod internal {
    pub(crate) use ratatui::{
        prelude::{Buffer, Color, Rect, Widget},
        style::Stylize,
        widgets::{Block, Padding, Paragraph, StatefulWidget, Wrap},
    };

    pub(crate) use crate::nodes::{DurfNodeWidget, WidgetSize};
}
