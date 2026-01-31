use super::*;

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
    pub(crate) fn should_parse(&self, elem: &scraper::ElementRef) -> bool {
        self.allow.iter().any(|r| r.matches(elem))
    }

    pub(crate) fn should_skip(&self, elem: &scraper::ElementRef) -> bool {
        self.skip.iter().any(|r| r.matches(elem))
    }
}

/// Simply recusion with guard for parse flags.
pub(crate) struct DepthGuard<'a>(&'a mut ParseFlags);

impl<'a> DepthGuard<'a> {
    pub(crate) fn new(flags: &'a mut ParseFlags) -> Self {
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
