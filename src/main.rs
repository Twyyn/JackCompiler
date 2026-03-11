use jack_compiler::JackCompiler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = std::env::args()
        .nth(1)
        .ok_or("Usage: jack_compiler <file.jack | directory>")?;

    let compiler = JackCompiler::from_path(&source)?;

    match compiler.compile() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{e}");
        }
    }

    Ok(())
}
