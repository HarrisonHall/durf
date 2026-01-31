use super::*;

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
pub(crate) fn frag_to_span(value: &durf_parser::TextFragment) -> ratatui::text::Span<'static> {
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
