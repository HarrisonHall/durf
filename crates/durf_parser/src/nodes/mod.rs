use super::*;

mod media;
mod section;
mod text;

#[allow(unused)]
pub use media::*;
pub use section::*;
pub use text::*;

/// A boxed RawNode for use in the AST.
#[derive(Clone, Debug)]
pub struct Node(Box<RawNode>);

impl Node {
    /// Create a new nade from a raw node.
    pub fn new(raw_node: RawNode) -> Self {
        raw_node.into()
    }
}

impl std::ops::Deref for Node {
    type Target = RawNode;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl std::ops::DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

impl From<RawNode> for Node {
    fn from(value: RawNode) -> Self {
        Self(Box::new(value))
    }
}

/// A node in the durf AST.
#[derive(Clone, Debug)]
pub enum RawNode {
    Empty,
    Section(Section),
    Text(Text),
}

impl From<Section> for RawNode {
    fn from(value: Section) -> Self {
        RawNode::Section(value)
    }
}

impl From<Text> for RawNode {
    fn from(value: Text) -> Self {
        RawNode::Text(value)
    }
}

impl RawNode {
    pub(crate) fn from_element_ref(
        ele: &scraper::ElementRef,
        flags: &mut ParseFlags,
    ) -> Result<Self, Error> {
        Self::from_element_ref_internal(ele, flags)
    }

    fn from_element_ref_text(
        ele: &scraper::ElementRef,
        flags: &mut ParseFlags,
    ) -> Result<Text, Error> {
        let ele_name = ele.value().name.local.to_ascii_lowercase();

        // Do not exceed depth.
        tracing::trace!("Depth: {} {ele_name}", flags.remaining_depth);
        if flags.remaining_depth == 0 {
            return Err(Error::DepthExceeded);
        }

        let mut flags = DepthGuard::new(flags);

        // Combine children of text element into single text.
        let mut text = Text::new();

        // Check special cases.
        match ele_name.as_ref() {
            "br" | "hr" => {
                return Ok(Text::from_fragment("\n"));
            }
            "rp" => {
                return Ok(Text::new_empty());
            }
            _ => {}
        }

        // Iterate children.
        for node_ref in ele.children() {
            let node = node_ref.value();
            let _node_text = node.as_text();

            // Parse child elements.
            if let Some(sub_ele_ref) = scraper::ElementRef::wrap(node_ref) {
                if let Ok(sub_text) = Self::from_element_ref_text(&sub_ele_ref, flags.deref_mut()) {
                    text.extend(sub_text);
                }
            }

            // Parse text nodes.
            if let Some(node_text) = node.as_text() {
                let mut _minimized_text: String = node_text.to_string();
                _minimized_text = _minimized_text.replace("\n", "");
                // TODO: Better string minimization.
                text.append(TextFragment::from(node_text.as_ref()));
            }
        }

        // For certain elements, we have special handling:
        match ele_name.as_ref() {
            // Ruby will combine and wrap children:
            "ruby" => {
                text.combine_fragments();
                return Ok(text);
            }
            // <rt> is an annotation, only.
            "rt" => {
                text.combine_fragments();
                if let Some(frag) = text.fragments.first_mut() {
                    frag.attributes.annotation = Some(frag.text.clone());
                    frag.text.clear();
                }
                return Ok(text);
            }
            _ => {}
        }

        // Modify child fragments according to element.
        for frag in &mut text.fragments {
            match ele_name.as_ref() {
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                    frag.attributes.heading = match ele_name.as_ref() {
                        "h1" => Some(1),
                        "h2" => Some(2),
                        "h3" => Some(3),
                        "h4" => Some(4),
                        "h5" => Some(5),
                        "h6" => Some(6),
                        _ => {
                            tracing::error!("Invalid header: `{}`", ele_name);
                            None
                        }
                    };
                }
                "p" | "a" | "span" => {
                    if let Some(link) = ele.attr("href") {
                        frag.attributes.link = Some(link.into());
                    }
                }
                "strong" | "b" => {
                    frag.attributes.bold = true;
                }
                "i" | "u" | "em" => {
                    frag.attributes.italic = true;
                }
                "blockquote" | "q" | "pre" | "code" => {
                    frag.attributes.preformatted = true;
                }
                _ => {
                    tracing::debug!("Unsupported text element: {}", ele_name.as_ref());
                }
            }
        }

        Ok(text)
    }

    fn from_element_ref_internal(
        elem: &scraper::ElementRef,
        flags: &mut ParseFlags,
    ) -> Result<Self, Error> {
        // Do not exceed depth.
        if flags.remaining_depth == 0 {
            return Err(Error::DepthExceeded);
        }

        let mut flags = DepthGuard::new(flags);

        let mut toggled_parsing = false;
        if !flags.parsing {
            if flags.should_parse(elem) {
                toggled_parsing = true;
                flags.parsing = true;
            }
        }
        if flags.should_skip(elem) {
            return Ok(Self::Empty);
        }

        // Match on element name.
        let ele_name = elem.value().name.local.to_ascii_lowercase();
        tracing::trace!("Tag {ele_name}");
        let parsed: Result<Self, Error> = match ele_name.as_ref() {
            // TODO: Parse head as meta!
            "html" | "header" | "footer" | "body" | "div" | "section" | "article" | "main"
            | "nav" => {
                let mut section = Section::new_set();
                for child in elem.child_elements() {
                    match RawNode::from_element_ref_internal(&child, flags.deref_mut()) {
                        Ok(parsed_child) => section.nodes.push(parsed_child.into()),
                        Err(e) => {
                            tracing::debug!("Failed to parse child: {e:?}");
                        }
                    }
                }
                Ok(section.into())
            }
            "menu" | "ul" => {
                let mut section = Section::new_list();
                for child in elem.child_elements() {
                    match RawNode::from_element_ref_internal(&child, flags.deref_mut()) {
                        Ok(parsed_child) => section.nodes.push(parsed_child.into()),
                        Err(e) => {
                            tracing::debug!("Failed to parse child: {e:?}");
                        }
                    }
                }
                Ok(section.into())
            }
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "p" | "a" | "span" | "strong" | "em"
            | "i" | "s" | "u" | "blockquote" | "q" | "hr" | "br" | "pre" | "code" => {
                if flags.parsing {
                    let res = match Self::from_element_ref_text(elem, flags.deref_mut()) {
                        Ok(t) => Ok(t.into()),
                        Err(e) => Err(e),
                    };
                    res
                } else {
                    Ok(RawNode::Empty)
                }
            }
            _ => {
                tracing::debug!("Unsupported element: {}", ele_name,);

                Err(Error::Todo)
            }
        };

        if toggled_parsing {
            flags.parsing = false;
        }
        parsed
    }

    pub fn export_string(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        for _ in 0..depth {
            write!(f, " ")?;
        }
        match self {
            Self::Empty => {}
            Self::Section(section) => {
                write!(f, "Section\n").ok();
                for child in &section.nodes {
                    child.export_string(f, depth + 1)?;
                }
            }
            Self::Text(text) => {
                write!(f, "Text: {}\n", text.to_markdown()).ok();
            }
        }

        Ok(())
    }

    pub(crate) fn minimize(&mut self) {
        match self {
            Self::Empty => {}
            Self::Section(section) => {
                if section.nodes.len() == 0 {
                    return;
                }

                // Remove empty sections.
                section.nodes.retain(|n| match &**n {
                    RawNode::Empty => false,
                    RawNode::Section(s) => !s.is_empty(),
                    RawNode::Text(_) => true,
                });

                // Minimize nodes.
                for node in &mut section.nodes {
                    node.minimize();
                }

                // Remove empty sections.
                section.nodes.retain(|n| match &**n {
                    RawNode::Empty => false,
                    RawNode::Section(s) => !s.is_empty(),
                    RawNode::Text(_) => true,
                });

                // Collapse subsection.
                if section.nodes.len() == 1 && section.ordering == SectionOrdering::Set {
                    if let Some(mut node) = section.nodes.pop() {
                        // self = &mut *node;
                        // self = &mut *node;
                        std::mem::swap(self, &mut *node);
                    }
                }
            }
            Self::Text(text) => {
                text.clean();
                // text.text = text.text.trim().into();
            }
        }
    }
}
