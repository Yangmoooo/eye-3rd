pub(crate) mod zip;

use std::path::Path;

#[derive(PartialEq, Eq)]
pub enum Format {
    Mp4,
    Mkv,
    Zip,
    Sevenz,
    Unk,
}

impl From<&Path> for Format {
    fn from(p: &Path) -> Self {
        match p.extension().and_then(|ext| ext.to_str()) {
            Some("mp4") => Self::Mp4,
            Some("mkv") => Self::Mkv,
            Some("zip") => Self::Zip,
            _ => Self::Unk,
        }
    }
}
