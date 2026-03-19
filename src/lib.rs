pub mod codegen;
pub mod compiler;
pub mod error;
pub mod lexer;
pub mod parser;

pub const JACK_INT_MAX: u32 = 32767;

// Re-export the public API
pub use compiler::JackCompiler;
