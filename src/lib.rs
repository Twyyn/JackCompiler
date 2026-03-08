pub mod error;
pub mod lexer;
pub mod parser;

use std::fs;
use std::path::{Path, PathBuf};

use crate::error::CompilerError;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::parser::ast::Class;

pub const JACK_INT_MAX: u32 = 32767;

// --- Source File ---
#[derive(Debug)]
pub struct SourceFile {
    pub name: String,
    pub contents: String,
    pub output_path: PathBuf,
}

impl SourceFile {
    #[must_use]
    pub fn new(name: String, contents: String, output_path: PathBuf) -> Self {
        Self {
            name,
            contents,
            output_path,
        }
    }
}

// --- Jack Compiler ---

#[derive(Debug)]
pub struct JackCompiler {
    pub source_files: Vec<SourceFile>,
}

impl JackCompiler {
    #[must_use]
    pub fn new(source_files: Vec<SourceFile>) -> Self {
        Self { source_files }
    }
    /// Creates a new `JackCompiler` from a source file or directory.
    ///
    /// Reads all `.jack` source files from disk eagerly on construction.
    ///
    /// # Errors
    ///
    /// Returns a `CompilerError` if the source path is invalid, no Jack files are found,
    /// or if there is an I/O error reading the source files.
    pub fn from_path(path: &str) -> Result<Self, CompilerError> {
        let source = Path::new(path);

        let (jack_files, output_dir) = if source.is_dir() {
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

            (files, source)
        } else if Self::is_jack_file(source) {
            let files: Vec<PathBuf> = vec![source.into()];
            let output_dir = source.parent().unwrap_or_else(|| Path::new("."));

            (files, output_dir)
        } else {
            return Err(CompilerError::InvalidPath);
        };

        let source_files = jack_files
            .into_iter()
            .map(|path| {
                let name = path
                    .file_stem()
                    .ok_or(CompilerError::InvalidPath)?
                    .to_str()
                    .ok_or(CompilerError::InvalidPath)?
                    .to_owned();

                let contents = fs::read_to_string(&path).map_err(CompilerError::Io)?;

                let output_path = output_dir
                    .join(path.file_name().ok_or(CompilerError::InvalidPath)?)
                    .with_extension("vm");

                Ok(SourceFile::new(name, contents, output_path))
            })
            .collect::<Result<Vec<SourceFile>, CompilerError>>()?;

        Ok(Self::new(source_files))
    }

    /// Compiles all loaded source files into a list of parsed [`Class`] nodes.
    ///
    /// Tokenizes and parses each source file in order, collecting all top-level
    /// class definitions across the entire input set.
    ///
    /// # Errors
    ///
    /// Returns a `CompilerError` if tokenization fails for any source file (e.g.,
    /// unrecognized character, malformed token), or if the parser encounters invalid
    /// or unexpected syntax while building the AST.
    pub fn compile(&self) -> Result<Vec<Class>, CompilerError> {
        let mut classes = Vec::new();
        for file in &self.source_files {
            let tokens = Lexer::new(&file.contents).tokenize()?;
            let mut parsed = Parser::new(tokens).parse()?;
            classes.append(&mut parsed);
        }

        Ok(classes)
    }

    // --- Filesystem Helpers ---

    fn is_jack_file(source: &Path) -> bool {
        source.is_file() && source.extension().is_some_and(|ext| ext == "jack")
    }
}
