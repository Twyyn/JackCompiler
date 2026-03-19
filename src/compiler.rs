use std::fs;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use crate::error::CompilerError;
use crate::lexer::Lexer;
use crate::parser::Parser;

// --- SourceFile ---

#[derive(Debug)]
pub struct SourceFile {
    pub name: String,
    pub contents: String,
    pub output_path: PathBuf,
}

impl SourceFile {
    fn from_path(path: &Path) -> Result<Self, CompilerError> {
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        let contents = fs::read_to_string(path)?;
        let output_path = path.with_extension("vm");

        Ok(Self {
            name,
            contents,
            output_path,
        })
    }
}

// --- JackCompiler ---

#[derive(Debug)]
pub struct JackCompiler {
    source_files: Vec<SourceFile>,
}

impl JackCompiler {
    /// Creates a new `JackCompiler` from a file or directory path.
    ///
    /// # Errors
    ///
    /// Returns a `CompilerError` if the path is invalid, no `.jack` files
    /// are found, or any file cannot be read.
    pub fn from_path(path: &str) -> Result<Self, CompilerError> {
        let source = Path::new(path);
        let jack_files = Self::collect_jack_files(source)?;

        let source_files = jack_files
            .into_iter()
            .map(|p| SourceFile::from_path(&p))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { source_files })
    }

    /// Compiles all loaded source files, writing `.vm` output alongside each.
    ///
    /// # Errors
    ///
    /// Returns a `CompilerError` on lexer, parser, or I/O failure.
    pub fn compile(&self) -> Result<(), CompilerError> {
        for source in &self.source_files {
            self.compile_file(source)?;
        }
        Ok(())
    }

    #[allow(clippy::unused_self)]
    fn compile_file(&self, source: &SourceFile) -> Result<(), CompilerError> {
        let lexer = Lexer::new(&source.contents);
        let mut parser = Parser::new(lexer);
        let classes = parser.parse()?;

        let file = fs::File::create(&source.output_path)?;
        let mut writer = BufWriter::new(file);

        // TODO: codegen — write VM instructions for each class
        for class in &classes {
            writeln!(writer, "// class {}", class.name)?;
        }

        writer.flush()?;
        Ok(())
    }

    // ── Filesystem ───────────────────────────────────────────────────

    fn collect_jack_files(source: &Path) -> Result<Vec<PathBuf>, CompilerError> {
        if source.is_dir() {
            let mut files: Vec<PathBuf> = fs::read_dir(source)?
                .filter_map(|entry| {
                    let path = entry.ok()?.path();
                    Self::is_jack_file(&path).then_some(path)
                })
                .collect();

            if files.is_empty() {
                return Err(CompilerError::NoJackFiles);
            }

            files.sort();
            Ok(files)
        } else if Self::is_jack_file(source) {
            Ok(vec![source.into()])
        } else {
            Err(CompilerError::InvalidPath)
        }
    }

    fn is_jack_file(path: &Path) -> bool {
        path.is_file() && path.extension().is_some_and(|ext| ext == "jack")
    }
}
