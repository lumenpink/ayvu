//! Ayvu - Declarative Comic Strip Generator
//! Inspired by Microsoft Comic Chat (1996)
//!
//! Transforms conversational plain text into expressive multi-panel comic strips.

pub mod assets;
pub mod backdrop;
pub mod balloon;
pub mod character;
pub mod model;
pub mod parser;
pub mod props;
pub mod renderer;

#[cfg(not(target_arch = "wasm32"))]
pub mod server;

#[cfg(feature = "wasm")]
pub mod wasm;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn version() -> &'static str {
    VERSION
}
