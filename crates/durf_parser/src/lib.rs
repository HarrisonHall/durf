//! durf parser.

#![allow(unused)]

use std::{ops::DerefMut, rc::Rc, sync::Arc};

use scraper::Element;
use serde::{Deserialize, Serialize};
use tracing::field::DisplayValue;

/// A parsed AST representing a document.
#[allow(unused)]
#[derive(Clone, Debug)]
pub struct Ast {
    pub root: Node,
}

impl Ast {
    /// Minimize the AST.
    pub fn minimize(&mut self) {
        self.root.minimize();
    }
}

/// Flags to adjust parsing behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseFlags {
    /// Nodes that are allowed to be parsed.
    pub allow: Vec<ParseRule>,
    /// Nodes that are skipped with descendents.
    pub skip: Vec<ParseRule>,
    /// Whether or not the parsing condition is set.
    pub parsing: bool,
    /// Remaining depth for the parse.
    pub remaining_depth: usize,
}

impl Default for ParseFlags {
    fn default() -> Self {
        Self {
            allow: Vec::new(),
            skip: Vec::new(),
            parsing: true,
            remaining_depth: 10,
        }
    }
}

impl ParseFlags {
    fn should_parse(&self, elem: &scraper::ElementRef) -> bool {
        self.allow.iter().any(|r| r.matches(elem))
    }

    fn should_skip(&self, elem: &scraper::ElementRef) -> bool {
        self.skip.iter().any(|r| r.matches(elem))
    }
}

/// Simply recusion with guard for parse flags.
struct DepthGuard<'a>(&'a mut ParseFlags);

impl<'a> DepthGuard<'a> {
    fn new(flags: &'a mut ParseFlags) -> Self {
        flags.remaining_depth = flags.remaining_depth.saturating_sub(1);
        Self(flags)
    }
}

impl<'a> Drop for DepthGuard<'a> {
    fn drop(&mut self) {
        self.remaining_depth = self.remaining_depth.saturating_add(1);
    }
}

impl<'a> std::ops::Deref for DepthGuard<'a> {
    type Target = ParseFlags;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> std::ops::DerefMut for DepthGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A rule for matching elements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParseRule {
    #[serde(alias = "element")]
    Element(String),
    #[serde(alias = "class")]
    Class(String),
}

impl ParseRule {
    /// Create a parse rule from an element.
    pub fn from_element(elem: impl Into<String>) -> Self {
        Self::Element(elem.into())
    }

    /// Create a parse rule from a class.
    pub fn from_class(class: impl Into<String>) -> Self {
        Self::Class(class.into())
    }

    /// Check if parse rule matches element.
    fn matches(&self, elem: &scraper::ElementRef) -> bool {
        match &self {
            &ParseRule::Element(e) => {
                let ele_name = elem.value().name.local.to_lowercase();
                ele_name == *e
            }
            &ParseRule::Class(c) => elem
                .value()
                .has_class(c.as_str(), scraper::CaseSensitivity::AsciiCaseInsensitive),
        }
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
    fn from_element_ref(ele: &scraper::ElementRef, flags: &mut ParseFlags) -> Result<Self, Error> {
        Self::from_element_ref_internal(ele, flags)
    }

    fn from_node_ref_text(// node: &scraper::node::Doctype
    ) -> Result<Text, Error> {
        Err(Error::Todo)
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
            let node_text = node.as_text();

            // Parse child elements.
            if let Some(sub_ele_ref) = scraper::ElementRef::wrap(node_ref) {
                if let Ok(sub_text) = Self::from_element_ref_text(&sub_ele_ref, flags.deref_mut()) {
                    text.extend(sub_text);
                }
            }

            // Parse text nodes.
            if let Some(node_text) = node.as_text() {
                let mut minimized_text: String = node_text.to_string();
                minimized_text = minimized_text.replace("\n", "");
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
                write!(f, "Section\n");
                for child in &section.nodes {
                    child.export_string(f, depth + 1)?;
                }
            }
            Self::Text(text) => {
                write!(f, "Text: {}\n", text.to_markdown());
            }
        }

        Ok(())
    }

    fn minimize(&mut self) {
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

#[derive(Clone, Debug)]
pub struct Node(Box<RawNode>);

impl Node {
    fn new(raw_node: RawNode) -> Self {
        Self(Box::new(raw_node))
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

#[derive(Clone, Debug)]
pub struct Section {
    pub nodes: Vec<Node>,
    ordering: SectionOrdering,
}

impl Section {
    pub fn new_set() -> Self {
        Self {
            nodes: Vec::new(),
            ordering: SectionOrdering::Set,
        }
    }

    pub fn new_list() -> Self {
        Self {
            nodes: Vec::new(),
            ordering: SectionOrdering::List,
        }
    }

    pub fn new_enumeration() -> Self {
        Self {
            nodes: Vec::new(),
            ordering: SectionOrdering::Enumeration,
        }
    }

    pub fn is_empty(&self) -> bool {
        for child in &self.nodes {
            match &**child {
                RawNode::Section(s) => {
                    if !s.is_empty() {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        true
    }

    pub fn nodes(&self) -> &[Node] {
        self.nodes.as_slice()
    }

    pub fn ordering(&self) -> &SectionOrdering {
        &self.ordering
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SectionOrdering {
    /// Just items, in order.
    Set,
    /// Bulleted items.
    List,
    /// Ordered, enumerated items
    Enumeration,
}

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

    fn from_fragment(frag: &str) -> Self {
        Self {
            fragments: vec![frag.into()],
        }
    }

    /// Combine fragments, removing attributes.
    fn combine_fragments(&mut self) {
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
        let mut frag = TextFragment::new(text, Some(attributes));
        self.fragments = vec![frag];
    }

    fn append(&mut self, frag: TextFragment) {
        self.fragments.push(frag);
    }

    fn extend(&mut self, mut text: Text) {
        self.fragments.append(&mut text.fragments);
    }

    fn to_markdown(&self) -> String {
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

    fn clean(&mut self) {
        // TODO: Remove?
        for frag in self.fragments.iter_mut() {
            frag.text.replace("　", "");
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
        if let Some(_) = self.heading {
            return false;
        }
        if let Some(_) = self.link {
            return false;
        }
        if let Some(_) = self.tooltip {
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

struct Media {
    source: Box<Vec<u8>>,
    media_type: MediaType,
}

enum MediaType {
    Image,
    Video,
}

impl Ast {
    pub fn from_html(document: &str, flags: ParseFlags) -> Result<Ast, Error> {
        let mut flags = flags;
        let parsed_doc = scraper::Html::parse_document(document);
        let parsed_root = parsed_doc.root_element();
        let new_root = RawNode::from_element_ref(&parsed_root, &mut flags)?;

        Ok(Ast {
            root: Node::new(new_root),
        })
    }

    pub fn from_text(document: &str, flags: ParseFlags) -> Result<Ast, Error> {
        let mut flags = flags;

        let mut section = Section::new_set();

        for p in document.split("\n") {
            section
                .nodes
                .push(Node::new(RawNode::Text(Text::from_fragment(p))));
        }

        // let parsed_doc = scraper::Html::parse_document(document);
        // let parsed_root = parsed_doc.root_element();
        // let new_root = RawNode::from_element_ref(&parsed_root, &mut flags)?;

        Ok(Ast {
            root: Node::new(RawNode::Section(section)),
        })
    }
}

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AST:\n")?;
        self.root.export_string(f, 0)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Error {
    Todo,
    DepthExceeded,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DurfError {:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn parse_page_1() {
        let page = r#"
            <!doctype html>
            <html>
              <head>
                <!-- Meta -->
                <meta charset="utf-8" />
                <meta name="description" content="Homepage of Harrison Hall" />
                <meta name="author" content="Harrison Hall" />
                <meta
                  name="keywords"
                  content="Harrison Hall, Harrison, hachha, hocko, blog, tech, projects, resume"
                />

                <!-- Style -->
                <link rel="stylesheet" href="/styles/spectre/spectre.min.css" />
                <link rel="stylesheet" href="/styles/phosphor/style.css" />

                <!-- Custom styles -->
                <link rel="stylesheet" href="/styles/site_colors.css" />
                <link rel="stylesheet" href="/styles/content.css" />

                <!-- Mobile support -->
                <meta name="viewport" content="width=device-width, initial-scale=1" />

                <!-- Feeds -->
                <link
                  rel="alternate"
                  type="application/atom+xml"
                  title="hachha.dev blog"
                  href="/blog.feed"
                />
                <link
                  rel="alternate"
                  type="application/atom+xml"
                  title="hachha.dev links"
                  href="/links.feed"
                />
                <link
                  rel="alternate"
                  type="application/atom+xml"
                  title="hachha.dev slipfeed"
                  href="https://feeds.hachha.dev/all/feed"
                />

                <!-- Firefox FOUC fix -->
                <script>
                  let FF_FOUC_FIX;
                </script>
              </head>

              <body class="index-page">
                <header class="navbar" style="z-index: 1">
                  <section class="navbar-section">
                    <a href="/" class="btn btn-link text-secondary"><b>hachha.dev</b></a>
                  </section>
                  <section class="navbar-center">
                    <a href="/blog" class="btn btn-link text-gray">Blog</a>
                    <a href="/links" class="btn btn-link text-gray">Links</a>
                    <a
                      href="https://www.linkedin.com/in/harrison-hall-525b81123/"
                      target="”_blank”"
                      class="btn btn-link text-gray"
                      >Resume</a
                    >
                    <a href="/projects" class="btn btn-link text-gray">Projects</a>
                    <a
                      href="https://github.com/trackl-games"
                      target="”_blank”"
                      class="btn btn-link text-gray"
                      >Games</a
                    >
                  </section>
                  <section class="navbar-section"></section>
                </header>

                <div class="hero">
                  <div class="container grid-lg text-center">
                    <h1 style="font-size: 4em"><b>Harrison Hall</b></h1>
                    <p>Check out <a href="https://github.com/harrisonhall/slipstream">slipstream</a>!</p>
                  </div>
                </div>

                <img
                    src="/media/profile-b.png" alt=""
                    class="image-circle img-responsive"
                style="max-width: 256px; max-height: 256px;"
                >
                <!-- -->
                <div class="section" >
                    <div class="container grid-md">
                        <div class="empty">
                      <div class="container grid-xs">
                        <div
                          class="columns"
                          style="padding-left: 2em; padding-right: 2em; text-align: center"
                        >
                          <div class="column col-4 col-mx-auto">
                            <a
                              href="https://github.com/harrisonhall"
                              class="icon-link"
                              target="_blank"
                            >
                              <i class="ph-fill ph-github-logo"></i>
                            </a>
                          </div>
                          <div class="column col-4 col-mx-auto">
                            <a
                              href="https://www.linkedin.com/in/harrison-hall-525b81123/"
                              class="icon-link"
                              target="_blank"
                            >
                              <i class="ph-fill ph-linkedin-logo"></i>
                            </a>
                          </div>
                          <div class="column col-4 col-mx-auto">
                            <a
                              href="https://mastodon.social/@harryhallyall"
                              class="icon-link"
                              target="_blank"
                            >
                              <i class="ph-fill ph-mastodon-logo"></i>
                            </a>
                          </div>
                        </div>
                      </div>
                    </div>
                </div>
                </div>
                <!-- -->
                <div class="container grid-md" style="padding-left: 2em; padding-right: 2em;">
                    <div class="divider text-center"  ></div>
                </div>

                <div style="text-align: center">
                  <div class="section" >
                      <div class="container grid-md">
                          <div class="container">
                      <div class="columns">
                        <div class="column col-12">
                          <p></p>
                        </div>
                      </div>
                    </div>
                </div>
                  </div>
                </div>

                <div class="footer-spacer"></div>
                <footer class="text-center">
                  <div class="container grid-lg" id="copyright">
                    <p>
                      <span>© Harrison Hall 2025</span>
                      <br />
                      <a href="https://github.com/HarrisonHall/hachha.dev">v0.10.6</a>
                    </p>
                  </div>
                </footer>
              </body>
            </html>
        "#;
        let ast = Ast::from_html(page, ParseFlags::default());
        assert!(ast.is_ok());
        let mut ast = ast.unwrap();
        tracing::trace!("{ast}");
        ast.root.minimize();
        tracing::trace!("{ast}");
    }

    #[test_log::test]
    fn parse_page_2() {
        let page = r#"
        <html><head>
            <!-- Meta -->
            <meta charset="utf-8">
            <meta name="description" content="Homepage of Harrison Hall">
            <meta name="author" content="Harrison Hall">
            <meta name="keywords" content="Harrison Hall, Harrison, hachha, hocko, blog, tech, projects, resume">
    
            <!-- Style -->
            <link rel="stylesheet" href="/styles/spectre/spectre.min.css">
            <link rel="stylesheet" href="/styles/phosphor/style.css">
    
            <!-- Custom styles -->
            <link rel="stylesheet" href="/styles/site_colors.css">
            <link rel="stylesheet" href="/styles/content.css">
    
            <!-- Mobile support -->
            <meta name="viewport" content="width=device-width, initial-scale=1">
    
            <!-- Feeds -->
            <link rel="alternate" type="application/atom+xml" title="hachha.dev blog" href="/blog.feed">
            <link rel="alternate" type="application/atom+xml" title="hachha.dev links" href="/links.feed">
            <link rel="alternate" type="application/atom+xml" title="hachha.dev slipfeed" href="https://feeds.hachha.dev/all/feed">
    
            <!-- Firefox FOUC fix -->
            <script>
              let FF_FOUC_FIX;
            </script>
         <link rel="stylesheet" href="/styles/highlight.js/catppuccin-mocha.min.css">
         <script src="/styles/highlight.js/highlight.min.js"></script>
         <script>
           hljs.highlightAll();
         </script>

          <style>:is([id*='google_ads_iframe'],[id*='taboola-'],.taboolaHeight,.taboola-placeholder,#top-ad,#credential_picker_container,#credentials-picker-container,#credential_picker_iframe,[id*='google-one-tap-iframe'],#google-one-tap-popup-container,.google-one-tap__module,.google-one-tap-modal-div,#amp_floatingAdDiv,#ez-content-blocker-container) {display:none!important;min-height:0!important;height:0!important;}</style></head>

          <body class="blog-page">
            <header class="navbar" style="z-index: 1">
              <section class="navbar-section">
                <a href="/" class="btn btn-link text-secondary"><b>hachha.dev</b></a>
              </section>
              <section class="navbar-center">
                <a href="/blog" class="btn btn-link text-gray">Blog</a>
                <a href="/links" class="btn btn-link text-gray">Links</a>
                <a href="https://www.linkedin.com/in/harrison-hall-525b81123/" target="”_blank”" class="btn btn-link text-gray">Resume</a>
                <a href="/projects" class="btn btn-link text-gray">Projects</a>
                <a href="https://github.com/trackl-games" target="”_blank”" class="btn btn-link text-gray">Games</a>
              </section>
              <section class="navbar-section"></section>
            </header>

            <article class="blog-article">
              <div class="hero">
                <div class="container grid-lg text-center">
                  <h1 style="font-size: 4em"><b>Slipstream!</b></h1>
                  <h2 style="font-size: 2em"><b>Slipstream is out!</b></h2>
                  <span class="chip">2025-02-22</span>
                </div>
              </div>

              <div class="blog-markdown">
                <div class="section">
                    <div class="container grid-md">
                        <p>You heard it here first, <code>slipstream</code> is
        <a href="https://github.com/HarrisonHall/slipstream/releases/tag/slipstream-1.0.0"><em>out</em></a>,
        just in time for
        <a href="https://en.wikipedia.org/wiki/National_Cat_Day#Japan">cat day</a>!</p>
        <p><img src="/blog/slipstream_2/web_ui.png" alt="slipstream"></p>
        <p>I couldn't be happier with the result, but it's worth noting this isn't what I
        originally promised. Where are custom lua filters? Tracking read articles (read:
        headlines)? Super fancy tui?</p>
        <p>After using <code>slipknot</code> for a while, I realized I didn't actually care about many
        of those features. If I need a new filter, I can just push a new version of
        slipstream out. My readers can track what I've read, and I no longer care about
        sharing that between devices.</p>
        <p>So what happened to <code>slipknot</code>? <code>slipstream</code> now contains all of what was
        <code>slipknot</code>. I didn't see a reason to keep them separate or reimplement features.
        <code>slipstream</code> is basically <code>slipknot</code> with the default addresses going to the web
        view (atom feeds are now accessible with an extra <code>/feed</code> in the path).
        Honestly, I felt the name <code>slipknot</code> was a little aggressive, I wanted something
        with "slip" in it, but didn't think too much about it.</p>
        <p>But seriously, <a href="https://feeds.hachha.dev/">check it out</a>! The source remains on
        <a href="https://github.com/HarrisonHall/slipstream">github</a>.</p>
        <h2>Future Plans</h2>
        <p>I may still revisit my own tui in the future, but for now <code>newsraft</code> (tui) and
        <code>feeder</code> (mobile) are completely sufficient for my own needs.</p>
        <p>There are some outstanding tasks I need to eventually finish up.</p>
        <ul>
        <li><code>slipfeed</code>
        <ul>
        <li><input type="checkbox" disabled=""> Add other built-in feed implementations (e.g. activitypub)</li>
        </ul>
        </li>
        <li><code>slipstream</code>
        <ul>
        <li><input type="checkbox" disabled=""> Add more filters (regex/pomsky, allowlists, etc.)</li>
        <li><input type="checkbox" disabled=""> OPML conversion support</li>
        <li><input type="checkbox" disabled=""> Use sqlite for storing entries and feed definitions</li>
        <li><input type="checkbox" disabled=""> Support atom exports</li>
        </ul>
        </li>
        </ul>
        <p>...but I don't need any of these now, so who knows when they'll be completed.
        ¯\_(ツ)_/¯</p>

                    </div>
                </div>
              </div>
            </article>

            <div class="footer-spacer"></div>
            <footer class="text-center">
              <div class="container grid-lg" id="copyright">
                <p>
                  <span>© Harrison Hall 2025</span>
                  <br>
                  <a href="https://github.com/HarrisonHall/hachha.dev">v0.10.6</a>
                </p>
              </div>
            </footer>
  

        </body></html>
        "#;
        let ast = Ast::from_html(page, ParseFlags::default());
        assert!(ast.is_ok());
        let mut ast = ast.unwrap();
        tracing::trace!("{ast}");
        ast.root.minimize();
        tracing::trace!("{ast}");
    }

    #[test_log::test]
    fn parse_page_3_jp() {
        let page = r#"
        <article class="easy-article">
          <div class="article-figure">
                        <figure id="js-article-figure" class="is-show"><img src="https://news.web.nhk/news/easy/ogp/ne2026012811533/8fCtO2v6MrdvTGU7BWUUAXvbchlpXViEvlLPInyG.jpg" alt="" onerror="this.src='/news/easy/images/noimg_default_easy_m.jpg';"></figure>
                      </div>

          <h1 class="article-title">
              29<ruby>日<rt>にち</rt></ruby>から<ruby>日本海<rt>にほんかい</rt></ruby><ruby>側<rt>がわ</rt></ruby>などでまた<ruby>雪<rt>ゆき</rt></ruby>がたくさん<ruby>降<rt>ふ</rt></ruby>りそう
          </h1>
          <p class="article-date" id="js-article-date">2026年1月28日 19時20分</p>
          <div class="article-top-tool">
            <div class="article-buttons">
              <a href="" class="article-buttons__audio js-open-audio">
                <span> ニュースを<ruby>聞<rt>き</rt></ruby>く </span>
              </a>
              <a href="" class="article-buttons__ruby js-toggle-ruby is-ruby --pc">
                <ruby>漢字<rt>かんじ</rt></ruby>の<ruby>読<rt>よ</rt></ruby>み<ruby>方<rt>かた</rt></ruby>を<ruby>消<rt>け</rt></ruby>す
              </a>
            </div>

            <a href="" class="article-buttons__ruby js-toggle-ruby is-ruby --sp">
              <ruby>漢字<rt>かんじ</rt></ruby>の<ruby>読<rt>よ</rt></ruby>み<ruby>方<rt>かた</rt></ruby>を<ruby>消<rt>け</rt></ruby>す
            </a>

            <div class="audio-player" id="js-audio-wrapper">
              <div id="js-audio-inner"></div>
            </div>
          </div>
          <div class="article-body" id="js-article-body">
              <p><span class="colorL"><ruby>東北地方<rt>とうほくちほう</rt></ruby></span><span class="colorB">から</span><span class="colorL"><ruby>中国地方<rt>ちゅうごくちほう</rt></ruby></span><span class="colorB">まで</span><span class="colorB">の</span><span class="colorL"><ruby>日本海<rt>にほんかい</rt></ruby></span><span class="color4"><ruby>側<rt>がわ</rt></ruby></span><span class="color4">など</span><span class="colorB">で</span><span class="colorB">、</span><span class="colorB">29</span><span class="color4"><ruby>日<rt>にち</rt></ruby></span><span class="colorB">から</span><span class="color4">また</span><span class="color4"><ruby>雪<rt>ゆき</rt></ruby></span><span class="colorB">が</span><span class="color4">たくさん</span><span class="color4"><ruby>降<rt>ふ</rt></ruby>り</span><span class="colorB">そう</span><span class="colorB">です</span><span class="colorB">。</span></p>
<p><span class="colorC"><ruby>気象庁<rt>きしょうちょう</rt></ruby></span><span class="colorB">によると</span><span class="colorB">、</span><span class="colorB">29</span><span class="color4"><ruby>日<rt>にち</rt></ruby></span><span class="colorB">の</span><span class="color4"><ruby>夕方<rt>ゆうがた</rt></ruby></span><span class="colorB">まで</span><span class="colorB">の</span><span class="colorB">24</span><span class="color4"><ruby>時間<rt>じかん</rt></ruby></span><span class="colorB">に</span><span class="colorB">、</span><span class="colorL"><ruby>新潟県<rt>にいがたけん</rt></ruby></span><span class="colorB">と</span><span class="colorL"><ruby>北陸地方<rt>ほくりくちほう</rt></ruby></span><span class="colorB">の</span><span class="color4"><ruby>多<rt>おお</rt></ruby>い</span><span class="colorB">ところ</span><span class="colorB">で</span><span class="colorB">60</span><span class="color2">cm</span><span class="colorB">、</span><span class="colorL"><ruby>青森県<rt>あおもりけん</rt></ruby></span><span class="colorB">で</span><span class="colorB">50</span><span class="color2">cm</span><span class="colorB">、</span><span class="colorL"><ruby>近畿地方<rt>きんきちほう</rt></ruby></span><span class="colorB">で</span><span class="colorB">40</span><span class="color2">cm</span><span class="color4">ぐらい</span><span class="colorB">の</span><span class="color4"><ruby>雪<rt>ゆき</rt></ruby></span><span class="colorB">が</span><span class="color4"><ruby>降<rt>ふ</rt></ruby>る</span><span class="color3"><ruby>心配<rt>しんぱい</rt></ruby></span><span class="colorB">が</span><span class="color4">あり</span><span class="colorB">ます</span><span class="colorB">。</span><span class="color4">いつも</span><span class="colorB">は</span><span class="color4"><ruby>雪<rt>ゆき</rt></ruby></span><span class="colorB">が</span><span class="color4"><ruby>少<rt>すく</rt></ruby>ない</span><span class="colorL"><ruby>太平洋<rt>たいへいよう</rt></ruby></span><span class="color4"><ruby>側<rt>がわ</rt></ruby></span><span class="color4">でも</span><span class="color4"><ruby>雪<rt>ゆき</rt></ruby></span><span class="colorB">が</span><span class="color4"><ruby>降<rt>ふ</rt></ruby>る</span><span class="colorB">ところ</span><span class="colorB">が</span><span class="color4">あり</span><span class="colorB">そう</span><span class="colorB">です</span><span class="colorB">。</span></p>
<p><span class="color3"><ruby>交通<rt>こうつう</rt></ruby></span><span class="colorB">が</span><span class="color4"><ruby>止<rt>と</rt></ruby>まる</span><span class="colorB">かも</span><span class="colorB">しれ</span><span class="colorB">ませ</span><span class="colorB">ん</span><span class="colorB">。</span><span class="color4"><ruby>天気<rt>てんき</rt></ruby></span><span class="colorB">や</span><span class="color3"><ruby>交通<rt>こうつう</rt></ruby></span><span class="colorB">の</span><span class="color2"><ruby>情報<rt>じょうほう</rt></ruby></span><span class="colorB">を</span><span class="color4"><ruby>見<rt>み</rt></ruby></span><span class="colorB">て</span><span class="color3">ください</span><span class="colorB">。</span><span class="color4"><ruby>雪<rt>ゆき</rt></ruby></span><span class="colorB">を</span><span class="color3"><ruby>片<rt>かた</rt></ruby>づける</span><span class="color4">とき</span><span class="colorB">の</span><span class="color3"><ruby>事故<rt>じこ</rt></ruby></span><span class="colorB">にも</span><span class="color0"><ruby>気<rt>き</rt></ruby></span><span class="color0">をつけ</span><span class="colorB">て</span><span class="color3">ください</span><span class="colorB">。</span></p>
          </div>

          <div class="article-info">
            <div class="article-info__color">
              <ul class="color__list">
                <li class="--person">
                  … <ruby>人<rt>ひと</rt></ruby>の<ruby>名前<rt>なまえ</rt></ruby>
                </li>
                <li class="--place">
                  … <ruby>国<rt>くに</rt></ruby>や<ruby>県<rt>けん</rt></ruby>、<ruby>町<rt>まち</rt></ruby>、<ruby>場所<rt>ばしょ</rt></ruby>などの<ruby>名前<rt>なまえ</rt></ruby>
                </li>
                <li class="--group">
                  … <ruby>会社<rt>かいしゃ</rt></ruby>やグループなどの<ruby>名前<rt>なまえ</rt></ruby>
                </li>
              </ul>
              <a href="" class="color__toggle" id="js-toggle-color">ことばの<ruby>色<rt>いろ</rt></ruby>を<ruby>消<rt>け</rt></ruby>す</a>
            </div>
          </div>
          <div class="article-share">
            <div class="nhk-snsbtn" data-nhksns-disable="google" data-nhksns-description=" "></div>
          </div>
                    <div class="article-link" id="js-regular-news-wrapper">
            <a href="https://news.web.nhk/newsweb/na/na-k10015037371000" class="btn btn__no-ruby" target="_blank" id="js-regular-news">NEWS WEBでよむ</a>
          </div>
          </article>
        "#;
        let ast = Ast::from_html(page, ParseFlags::default());
        assert!(ast.is_ok());
        let mut ast = ast.unwrap();
        tracing::trace!("{ast}");
        ast.root.minimize();
        tracing::trace!("{ast}");
    }

    #[test_log::test]
    fn parse_page_4_jp() {
        let page = r#"
        <div>
            <p>Test1</p>
            <p class="calibre3"><span xmlns="http://www.w3.org/1999/xhtml" class="kobospan" id="kobo.16.1">　そう、無敵だと信じていた〝</span><ruby><span xmlns="http://www.w3.org/1999/xhtml" class="kobospan" id="kobo.17.1">王宮城塞</span><rt>キャッスルガード</rt></ruby><span xmlns="http://www.w3.org/1999/xhtml" class="kobospan" id="kobo.18.1">〟が破られたのは、フェルドウェイからしても想定外過ぎた。慎重な性格でなかったとしても、撤退を選択するに十分な理由であろう。</span></p>
        </div>
        <p>Test2</p>
        "#;
        let ast = Ast::from_html(page, ParseFlags::default());
        assert!(ast.is_ok());
        let mut ast = ast.unwrap();
        tracing::trace!("{ast}");
        ast.root.minimize();
        tracing::trace!("{ast}");
    }
}
