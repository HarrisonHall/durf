//! durf parser.

#![allow(unused)]

use std::{rc::Rc, sync::Arc};

use tracing::field::DisplayValue;

/// A parsed AST representing a document.
#[allow(unused)]
struct Ast {
    root: RawNode,
}

/// A node in an AST.
enum RawNode {
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
    fn from_element_ref(ele: &scraper::ElementRef) -> Result<Self, Error> {
        Self::from_element_ref_internal(ele, 10)
    }

    fn from_node_ref_text(// node: &scraper::node::Doctype
    ) -> Result<Text, Error> {
        Err(Error::Todo)
    }

    fn from_element_ref_text(
        ele: &scraper::ElementRef,
        remaining_depth: usize,
    ) -> Result<Text, Error> {
        let ele_name = ele.value().name.local.to_ascii_lowercase();
        tracing::info!("Depth: {remaining_depth} {ele_name}");
        // Do not exceed depth.
        if remaining_depth == 0 {
            return Err(Error::DepthExceeded);
        }

        // Combine children of text element into single text.
        let mut text = Text::new();

        // Check special cases.
        let ele_name = ele.value().name.local.to_ascii_lowercase();
        match ele_name.as_ref() {
            "br" | "hr" => {
                return Ok(Text::from_fragment("\n"));
            }
            _ => {}
        }

        // Iterate children.
        for node_ref in ele.children() {
            let node = node_ref.value();
            let node_text = node.as_text();

            // Parse child elements.
            if let Some(sub_ele_ref) = scraper::ElementRef::wrap(node_ref) {
                if let Ok(sub_text) = Self::from_element_ref_text(&sub_ele_ref, remaining_depth - 1)
                {
                    text.extend(sub_text);
                }
            }

            // Parse text nodes.
            if let Some(node_text) = node.as_text() {
                text.append(TextFragment::from(node_text.as_ref()));
            }
        }

        // Modify fragments according to element.
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
                "strong" | "emphasis" | "b" => {
                    frag.attributes.bold = true;
                }
                "i" | "u" => {
                    frag.attributes.italic = true;
                }
                "blockquote" | "q" | "pre" | "code" => {
                    frag.attributes.preformatted = true;
                }
                _ => {}
            }
        }

        Ok(text)
    }

    fn from_element_ref_internal(
        ele: &scraper::ElementRef,
        remaining_depth: usize,
    ) -> Result<Self, Error> {
        // Do not exceed depth.
        if remaining_depth == 0 {
            return Err(Error::DepthExceeded);
        }

        // Match on element name.
        let ele_name = ele.value().name.local.to_ascii_lowercase();
        tracing::info!("Tag {ele_name}");
        match ele_name.as_ref() {
            // TODO: Parse head as meta!
            "html" | "header" | "footer" | "body" | "div" | "section" | "article" | "main"
            | "nav" => {
                let mut section = Section::new_set();
                for child in ele.child_elements() {
                    match RawNode::from_element_ref_internal(&child, remaining_depth - 1) {
                        Ok(parsed_child) => section.nodes.push(parsed_child.into()),
                        Err(e) => {
                            tracing::warn!("Failed to parse child: {e:?}");
                        }
                    }
                }
                return Ok(section.into());
            }
            "menu" | "ul" => {
                let mut section = Section::new_list();
                for child in ele.child_elements() {
                    match RawNode::from_element_ref_internal(&child, remaining_depth - 1) {
                        Ok(parsed_child) => section.nodes.push(parsed_child.into()),
                        Err(e) => {
                            tracing::warn!("Failed to parse child: {e:?}");
                        }
                    }
                }
                return Ok(section.into());
            }
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "p" | "a" | "span" | "strong" | "em"
            | "i" | "s" | "u" | "blockquote" | "q" | "hr" | "br" | "pre" | "code" => {
                return Ok(Self::from_element_ref_text(ele, remaining_depth - 1)?.into());
            }
            _ => {
                tracing::warn!("Unsupported element: {}", ele_name,);

                Err(Error::Todo)
            }
        }
    }

    fn export_string(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        for _ in 0..depth {
            write!(f, " ")?;
        }
        match self {
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
            Self::Section(section) => {
                if section.nodes.len() == 0 {
                    return;
                }

                // Minimize nodes.
                for node in &mut section.nodes {
                    node.minimize();
                }

                // Remove empty sections.
                section.nodes.retain(|n| match &**n {
                    RawNode::Section(s) => !s.is_empty(),
                    _ => true,
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
                // text.text = text.text.trim().into();
            }
        }
    }
}

struct Node(Box<RawNode>);

impl Node {}

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

struct Section {
    nodes: Vec<Node>,
    ordering: SectionOrdering,
}

impl Section {
    fn new_set() -> Self {
        Self {
            nodes: Vec::new(),
            ordering: SectionOrdering::Set,
        }
    }

    fn new_list() -> Self {
        Self {
            nodes: Vec::new(),
            ordering: SectionOrdering::List,
        }
    }

    fn new_enumeration() -> Self {
        Self {
            nodes: Vec::new(),
            ordering: SectionOrdering::Enumeration,
        }
    }

    fn is_empty(&self) -> bool {
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
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum SectionOrdering {
    /// Just items, in order.
    Set,
    /// Bulleted items.
    List,
    /// Ordered, enumerated items
    Enumeration,
}

struct Text {
    fragments: Vec<TextFragment>,
}

impl Text {
    fn new() -> Self {
        Self {
            fragments: Vec::with_capacity(4),
        }
    }

    fn from_fragment(frag: &str) -> Self {
        Self {
            fragments: vec![frag.into()],
        }
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

            if total_formatted.len() > 0 {
                // total_formatted = format!("{total_formatted} {formatted}");
                total_formatted = format!("{total_formatted}{formatted}");
            } else {
                total_formatted = formatted;
            }
        }

        total_formatted
    }
}

struct TextFragment {
    text: String,
    attributes: TextAttributes,
}

impl TextFragment {
    fn new() -> Self {
        Self {
            text: String::with_capacity(32),
            attributes: TextAttributes::default(),
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

struct TextAttributes {
    preformatted: bool,
    italic: bool,
    bold: bool,
    heading: Option<u8>,
    link: Option<String>,
}

impl TextAttributes {
    fn new() -> Self {
        Self::default()
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
    pub fn from_html(document: &str) -> Result<Ast, Error> {
        let parsed_doc = scraper::Html::parse_document(document);
        let parsed_root = parsed_doc.root_element();
        let new_root = RawNode::from_element_ref(&parsed_root)?;

        Ok(Ast { root: new_root })
    }
}

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AST:\n")?;
        self.root.export_string(f, 0)
    }
}

#[derive(Copy, Clone, Debug)]
enum Error {
    Todo,
    DepthExceeded,
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
        let ast = Ast::from_html(page);
        assert!(ast.is_ok());
        let mut ast = ast.unwrap();
        tracing::info!("{ast}");
        ast.root.minimize();
        tracing::info!("{ast}");
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
        let ast = Ast::from_html(page);
        assert!(ast.is_ok());
        let mut ast = ast.unwrap();
        tracing::info!("{ast}");
        ast.root.minimize();
        tracing::info!("{ast}");
    }
}
