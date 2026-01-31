use super::*;

/// A section of nodes.
#[derive(Clone, Debug)]
pub struct Section {
    /// Nodes in the section.
    pub nodes: Vec<Node>,
    /// Ordering for the nodes.
    pub ordering: SectionOrdering,
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

/// Ordering for nodes in the section.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SectionOrdering {
    /// Just items, in order.
    Set,
    /// Bulleted items.
    List,
    /// Ordered, enumerated items
    Enumeration,
}
