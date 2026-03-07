use jack_compiler::JackCompiler;

fn main() {
    let source = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: jack_compiler <file.jack | directory>");
        std::process::exit(1);
    });

    let compiler = JackCompiler::new(&source).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    compiler.parse();
}
