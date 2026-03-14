use crate::lexer::token::kind::Identifier;
use std::{collections::HashMap, fmt::{self, Display}};

// --- Type ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Char,
    Boolean,
    Class(Identifier),
}

// --- Kind ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Static,
    Field,
    Argument,
    Local,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Static => "static",
            Self::Field => "this",
            Self::Local =>"local",
            Self::Argument => "argument"
        };
        write!(f, "{s}")
    }
}

// --- Symbol Table Entry ---

#[derive(Debug)]
pub struct Entry {
    type_: Type,
    kind: Kind,
    index: usize,
}

// --- Scope ---
#[derive(Debug, Default)]
pub struct Scope<'s> {
    entries: HashMap<&'s Identifier, Entry>,
    counts: [usize; 4],
}

impl<'s>  Scope<'s> {
    fn define(&mut self, name: &'s Identifier, type_: Type, kind: Kind) {
        let index = self.counts[kind as usize];
        self.counts[kind as usize] += 1;
        self.entries.insert(name, Entry { type_, kind, index });
    }

    fn get(&self, name: &'s Identifier) -> Option<&Entry> {
        self.entries.get(name)
    }

    fn count(&self, kind: Kind) -> usize {
        self.counts[kind as usize]
    }

    fn reset(&mut self) {
        self.entries.clear();
        self.counts = [0; 4];
    }
}

// --- Symbol Table ---
#[derive(Debug, Default)]
pub struct SymbolTable<'s> {
    class: Scope<'s>,
    subroutine: Scope<'s>,
}


impl<'s> SymbolTable<'s> {
    pub fn new() -> Self {
        Self {
            class: Scope::default(),
            subroutine: Scope::default()
        }
    }

    pub fn define(&mut self, name: &'s Identifier, type_: Type, kind: Kind) {
        match kind {
            Kind::Field | Kind::Static => self.class.define(name, type_, kind),
            Kind::Argument | Kind::Local => self.subroutine.define(name, type_, kind),
        }
    }

    pub fn lookup(&self, name: &'s Identifier) -> Option<&Entry> {
        self.subroutine.get(name).or_else(||self.class.get(name))
    }

    pub fn count(&self, kind: Kind) -> usize {
        match kind {
            Kind::Field | Kind::Static => self.class.count(kind),
            Kind::Argument | Kind::Local => self.subroutine.count(kind),
        }
    }

    pub fn reset_class(&mut self) {
        self.class.reset();
    }

    pub fn reset_subroutie(&mut self) {
        self.subroutine.reset();
    }
}

