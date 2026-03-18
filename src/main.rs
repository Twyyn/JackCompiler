use jack_compiler::JackCompiler;

use std::process;

fn main() {
    let source = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: jack_compiler <file.jack | directory>");
        process::exit(1);
    });

    let compiler = match JackCompiler::from_path(&source) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };

    if let Err(e) = compiler.compile() {
        eprintln!("{e}");
        process::exit(1);
    }
}
