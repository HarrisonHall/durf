//! Durf engine.

use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Engine {
    cache: Vec<CacheEntry>,
}

impl Engine {
    /// Create a new engine.
    pub async fn new() -> Result<Self, Error> {
        Ok(Self { cache: Vec::new() })
    }

    /// Add entry to engine.
    fn add_entry(&mut self, source: &str, ast: durf_parser::Ast) -> () {
        self.cache.push(CacheEntry {
            source: source.into(),
            ast,
        });
    }

    /// Get entry from engine.
    fn get_entry<'a>(&'a self, source: &str) -> Option<&'a CacheEntry> {
        for item in &self.cache {
            if item.source == source {
                return Some(item);
            }
        }
        None
    }

    /// Load document from uri into engine.
    pub async fn load<'a>(&'a mut self, uri: impl AsRef<str>) -> Result<&'a CacheEntry, Error> {
        let uri = uri.as_ref();

        // Check if present in cache.
        {
            // https://github.com/rust-lang/rust/issues/54663
            // if let Some(entry) = self.get_entry(uri) {
            //     return Ok(entry);
            // }
            let this = self as *const Engine;
            if let Some(entry) = unsafe { (*this).get_entry(uri) } {
                return Ok(entry);
            }
        }

        // Otherwise, load:
        let flags = durf_parser::ParseFlags::default();

        // If website, fetch content.
        if uri.starts_with("http://") || uri.starts_with("https://") {
            // TODO: Share client!.
            let client = reqwest::ClientBuilder::new()
                // .user_agent(&config.parse.html.user_agent)
                .build()
                .map_err(|e| Error::Fetcher(format!("Failed to build client: {e}")))?;
            let res = client
                .get(uri)
                .send()
                .await
                .map_err(|e| Error::Fetcher(format!("Failed to GET: {e}")))?;
            let body = res
                .text()
                .await
                .map_err(|e| Error::Fetcher(format!("No text: {e}")))?;

            let mut ast = durf_parser::Ast::from_html(&body, flags)?;
            ast.minimize();
            self.add_entry(uri, ast);
            return self.get_entry(uri).ok_or(Error::Unknown);
        }

        // If still not parsed, try to treat as file.
        {
            // Remove file uri.
            let mut uri = uri.to_string();
            if uri.starts_with("file://") {
                uri = uri.replace("file://", "");
            }

            let normalized_path = PathBuf::from_str(uri.as_str())
                .map_err(|e| Error::Fetcher(format!("Unable to find file: {e}")))?;

            // TODO: If file doesn't exist, try to get it relative to the config file.

            let body = std::fs::read_to_string(&normalized_path).map_err(|e| {
                Error::Fetcher(format!(
                    "Unable to read file: '{}': {e}",
                    normalized_path.to_string_lossy(),
                ))
            })?;

            // We parsed the actual document.
            let mut ast = durf_parser::Ast::from_html(&body, flags)?;
            ast.minimize();
            self.add_entry(&uri, ast);
            return self.get_entry(&uri).ok_or(Error::Unknown);
        }
    }
}

/// Document within the engine's cache.
#[derive(Clone, Debug)]
pub struct CacheEntry {
    source: String,
    ast: durf_parser::Ast,
}

impl CacheEntry {
    pub fn source(&self) -> &str {
        self.source.as_str()
    }

    pub fn ast(&self) -> &durf_parser::Ast {
        &self.ast
    }
}

/// Engine error.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Parser error.")]
    Parser(#[from] durf_parser::Error),
    #[error("Fetch error.")]
    Fetcher(String),
    #[error("Unknown error.")]
    Unknown,
}
