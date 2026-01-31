use super::*;

/// A parsed AST representing a document.
#[allow(unused)]
#[derive(Clone, Debug)]
pub struct Ast {
    pub root: Node,
}

impl Ast {
    /// Pares the AST from HTML.
    pub fn from_html(document: &str, flags: ParseFlags) -> Result<Ast, Error> {
        let mut flags = flags;
        let parsed_doc = scraper::Html::parse_document(document);
        let parsed_root = parsed_doc.root_element();
        let new_root = RawNode::from_element_ref(&parsed_root, &mut flags)?;

        Ok(Ast {
            root: Node::new(new_root),
        })
    }

    /// Experimental: parse the AST from text.
    pub fn from_text(document: &str, _flags: ParseFlags) -> Result<Ast, Error> {
        let mut section = Section::new_set();

        for p in document.split("\n") {
            section
                .nodes
                .push(Node::new(RawNode::Text(Text::from_fragment(p))));
        }

        Ok(Ast {
            root: Node::new(RawNode::Section(section)),
        })
    }

    /// Minimize the AST.
    pub fn minimize(&mut self) {
        self.root.minimize();
    }
}

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AST:\n")?;
        self.root.export_string(f, 0)
    }
}
