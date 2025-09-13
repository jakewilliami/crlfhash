//! File transformations
//!
//! Prior to the hash-calculation step, we may be required to transform the file contents.

use super::file::Path;

#[derive(PartialEq)]
pub enum Transformation {
    Identity,
    Lf,
    Crlf,
}

// TODO: implement composable transformations Transformation::CR.compose(Transformation::LF)
//    impl Transformation {
//        pub fn compose(self, other: Transformation) -> Vec<Transformation> {
//            vec![self, other]
//        }
//    }

impl std::fmt::Display for Transformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Transformation::Identity => write!(f, "Raw"),
            Transformation::Lf => write!(f, "LF"),
            Transformation::Crlf => write!(f, "CRLF"),
        }
    }
}

pub fn apply_transformation(path: &Path, transformation: &Transformation) -> Vec<u8> {
    let contents = std::fs::read(&path.path).expect("file should be readable");

    if transformation == &Transformation::Identity {
        return contents;
    }

    let sep: &[u8] = match transformation {
        Transformation::Identity => unreachable!(),
        Transformation::Lf => b"\n",
        Transformation::Crlf => b"\r\n",
    };

    process_newlines(&contents, sep)
}

// A low-level function like this is preferred over `String.lines()` as the latter
// does more magic-y things.  We need to be precise when revoewomg
fn process_newlines(data: &[u8], sep: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len());
    let mut i = 0;
    while i < data.len() {
        if i + 1 < data.len() && data[i] == b'\r' && data[i + 1] == b'\n' {
            out.extend_from_slice(sep);
            i += 2;
        } else if data[i] == b'\r' || data[i] == b'\n' {
            out.extend_from_slice(sep);
            i += 1;
        } else {
            out.push(data[i]);
            i += 1;
        }
    }
    out
}
