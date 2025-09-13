//! Handle file type inference
//!
//! Infer type of file, if valid, and associated error reporting

use super::trans::Transformation;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{self, PathBuf},
};

pub struct Path {
    pub path: PathBuf,
    pub validity: PathValid,
    pub trans: Vec<Transformation>,
}

impl From<&String> for Path {
    fn from(path: &String) -> Self {
        let path = path::Path::new(&path).to_path_buf();
        let validity = PathValid::from(&path);

        // TODO: should do something more clever here to figure out what the identity is (it might be mixed) and only report the remainder
        let mut trans = vec![];

        if validity.exists {
            trans.push(Transformation::Identity);
        }

        if validity.is_valid {
            trans.push(Transformation::Lf);
            trans.push(Transformation::Crlf);
        }

        Self {
            path,
            validity,
            trans,
        }
    }
}

pub struct PathValid {
    pub exists: bool,
    is_valid: bool,
}

impl PathValid {
    fn nonexistent() -> Self {
        Self {
            exists: false,
            is_valid: false,
        }
    }

    fn valid() -> Self {
        Self {
            exists: true,
            is_valid: true,
        }
    }

    fn invalid() -> Self {
        Self {
            exists: true,
            is_valid: false,
        }
    }
}

impl From<&PathBuf> for PathValid {
    fn from(path: &PathBuf) -> Self {
        // Check that path exists
        if !path.exists() {
            return Self::nonexistent();
        }

        // Check that the file can be inferred
        let kind = infer::get_from_path(path).expect("file should be readable");

        if let Some(kind) = kind {
            // Valid file to extract if the matcher type is a text file
            if kind.matcher_type() == infer::MatcherType::Text {
                // Can double check that every character is plain text
                if path.is_plain_text() {
                    Self::valid()
                } else {
                    Self::invalid()
                }
            } else {
                Self::invalid()
            }
        } else if path.is_plain_text() {
            Self::valid()
        } else {
            Self::invalid()
        }
    }
}

trait FileTypeInference {
    fn is_plain_text(&self) -> bool;
}

impl FileTypeInference for PathBuf {
    // Plain text, but not necessarily ASCII
    fn is_plain_text(&self) -> bool {
        let file = File::open(self).unwrap();
        let reader = BufReader::new(file);

        // Try reading the file as UTF-8 directly
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    // Check each character for being printable or whitespace
                    if !line.chars().all(|c| c.is_whitespace() || c.is_printable()) {
                        return false;
                    }
                }
                Err(_) => {
                    // Non-UTF-8 sequence encountered
                    return false;
                }
            }
        }

        true
    }
}

trait CharExt {
    fn is_printable(&self) -> bool;
}

impl CharExt for char {
    fn is_printable(&self) -> bool {
        // Allow whitespace but disallow other control chars
        !self.is_control() || self.is_whitespace()
    }
}
