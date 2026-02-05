#[derive(Clone, Debug)]
pub struct Text {
    pub fragments: Vec<TextFragment>,
}

impl Text {
    pub fn new() -> Self {
        Self {
            fragments: Vec::with_capacity(4),
        }
    }

    pub fn new_empty() -> Self {
        Self {
            fragments: Vec::with_capacity(0),
        }
    }

    pub fn from_fragment(frag: &str) -> Self {
        Self {
            fragments: vec![frag.into()],
        }
    }

    /// Combine fragments, removing attributes.
    pub fn combine_fragments(&mut self) {
        // TODO: Handle combining additional attributes.
        let mut attributes = TextAttributes::new();
        let mut text = String::new();
        let mut annotation = String::new();
        for frag in self.fragments.iter() {
            text += frag.as_ref();
            annotation += match &frag.attributes.annotation {
                Some(a) => a.as_str(),
                None => "",
            };
        }
        if !annotation.is_empty() {
            attributes.annotation = Some(annotation);
        }
        let frag = TextFragment::new(text, Some(attributes));
        self.fragments = vec![frag];
    }

    pub fn append(&mut self, frag: TextFragment) {
        self.fragments.push(frag);
    }

    pub fn extend(&mut self, mut text: Text) {
        self.fragments.append(&mut text.fragments);
    }

    pub fn to_markdown(&self) -> String {
        let mut total_formatted = String::new();

        for frag in &self.fragments {
            let mut formatted = frag.text.clone();

            if frag.attributes.preformatted {
                formatted = format!("`{formatted}`");
            }
            if frag.attributes.italic {
                formatted = format!("*{formatted}*");
            }
            if frag.attributes.bold {
                formatted = format!("**{formatted}**");
            }
            if let Some(link) = &frag.attributes.link {
                formatted = format!("[{formatted}]({link})");
            }
            if let Some(heading) = &frag.attributes.heading {
                formatted = format!(
                    "{} {}",
                    (0..*heading).map(|_| "#").collect::<String>(),
                    formatted
                );
            }
            if let Some(annotation) = &frag.attributes.annotation {
                formatted = format!("{}({})", formatted, annotation);
            }

            if total_formatted.len() > 0 {
                // total_formatted = format!("{total_formatted} {formatted}");
                total_formatted = format!("{total_formatted}{formatted}");
            } else {
                total_formatted = formatted;
            }
        }

        total_formatted
    }

    pub fn clean(&mut self) {
        // TODO: Clean via parse flags.
        for frag in self.fragments.iter_mut() {
            frag.text = frag.text.replace("　", "");
        }
    }

    pub fn collect(&self) -> String {
        let mut total = String::new();
        for fragment in &self.fragments {
            total += fragment.text.as_str();
        }
        total
    }
}

#[derive(Clone, Debug)]
pub struct TextFragment {
    pub text: String,
    pub attributes: TextAttributes,
}

impl TextFragment {
    pub fn new(text: impl Into<String>, attributes: Option<TextAttributes>) -> Self {
        Self {
            text: text.into(),
            attributes: match attributes {
                Some(attr) => attr,
                None => TextAttributes::default(),
            },
        }
    }

    #[allow(unused)]
    fn append(&mut self, text: impl AsRef<str>) {
        let trimmed = text.as_ref().trim();
        if trimmed.len() > 0 {
            // self.text.push(' ');
            self.text.push_str(trimmed);
        }
    }
}

impl Default for TextFragment {
    fn default() -> Self {
        Self {
            text: String::with_capacity(32),
            attributes: TextAttributes::default(),
        }
    }
}

impl From<&str> for TextFragment {
    fn from(value: &str) -> Self {
        Self {
            text: value.into(),
            attributes: TextAttributes::default(),
        }
    }
}

impl AsRef<str> for TextFragment {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

/// Attributes for a text fragment.
#[derive(Clone, Debug)]
pub struct TextAttributes {
    /// Preformatted, code, or mono font.
    pub preformatted: bool,
    /// Italic font.
    pub italic: bool,
    /// Bold font.
    pub bold: bool,
    /// Heading value: None, 1-6.
    pub heading: Option<u8>,
    /// A link/reference.
    pub link: Option<String>,
    /// An annotated tooltip.
    pub tooltip: Option<String>,
    /// A text annotation.
    pub annotation: Option<String>,
}

impl TextAttributes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_plain(&self) -> bool {
        if self.preformatted {
            return false;
        }
        if self.bold {
            return false;
        }
        if self.italic {
            return false;
        }
        if self.heading.is_some() {
            return false;
        }
        if self.link.is_some() {
            return false;
        }
        if self.tooltip.is_some() {
            return false;
        }
        if self.annotation.is_some() {
            return false;
        }

        true
    }
}

impl Default for TextAttributes {
    fn default() -> Self {
        Self {
            preformatted: false,
            italic: false,
            bold: false,
            heading: None,
            link: None,
            tooltip: None,
            annotation: None,
        }
    }
}
