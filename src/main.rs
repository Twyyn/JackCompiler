use jack_compiler::JackCompiler;
use jack_compiler::error::CompilerError;
use std::io::ErrorKind;

fn main() -> Result<(), CompilerError> {
    let source = std::env::args().nth(1).ok_or_else(|| {
        CompilerError::Io(std::io::Error::new(
            ErrorKind::InvalidInput,
            "Usage: jack_compiler <file.jack | directory>",
        ))
    })?;

    let compiler = JackCompiler::new(&source)?;
    compiler.tokenize();

    Ok(())
}
