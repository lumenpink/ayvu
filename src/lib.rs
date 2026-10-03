//! Ayvu - Declarative Comic Strip Generator
//! Inspired by Microsoft Comic Chat (1996)
//!
//! Transforms conversational plain text into expressive multi-panel comic strips.

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn version() -> &'static str {
    VERSION
}
