//! This module contains the compiler implementation.
//! Note that these modules are public for documentation visiblility,
//! But should never be used outside of the module by `common` or `vm`.
//!
//! Each step in the compiler pipeline turns one datatype into another.
//! loosely, starting with `Source` (string + path):
//!
//! 1. Tokens:   `lex.rs`
//! 1. Absrtact ST: `parse.rs`
//! 2. Concrete ST: `desugar.rs`
//! 3. Bytecode: `gen.rs`
//!
//! Note that more steps (e.g. ones applying macro transformations, optimization passes, etc.)
//! may be implemented in the future.

pub mod lex;
pub mod parse;
pub mod desugar;
pub mod gen;

pub mod token;
pub mod ast; // high level pre-macro IR
pub mod rule; // macro transformation
pub mod cst; // post-macro IR

pub mod syntax;

pub use lex::lex;
pub use parse::parse;
pub use desugar::desugar;
pub use gen::gen;
