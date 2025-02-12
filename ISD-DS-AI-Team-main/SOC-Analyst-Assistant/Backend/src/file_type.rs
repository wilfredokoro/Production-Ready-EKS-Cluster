#![allow(clippy::upper_case_acronyms)]
#![allow(dead_code)]

/// Enum to represent file types and their magic numbers
pub enum FileType {
    PDF,
    PNG,
    JPEG,
    GIF87a,
    GIF89a,
    ZIP,
    GZIP,
    MP3,
    EXE,
    Unknown,
}

impl FileType {
    /// Match the magic number to the file type
    pub fn from_magic_number(bytes: &[u8]) -> FileType {
        match bytes {
            // PDF
            [0x25, 0x50, 0x44, 0x46, ..] => FileType::PDF,
            // PNG
            [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] => FileType::PNG,
            // JPEG
            [0xFF, 0xD8, 0xFF, ..] => FileType::JPEG,
            // GIF87a
            [0x47, 0x49, 0x46, 0x38, 0x37, 0x61, ..] => FileType::GIF87a,
            // GIF89a
            [0x47, 0x49, 0x46, 0x38, 0x39, 0x61, ..] => FileType::GIF89a,
            // ZIP
            [0x50, 0x4B, 0x03, 0x04, ..] => FileType::ZIP,
            // GZIP
            [0x1F, 0x8B, ..] => FileType::GZIP,
            // MP3
            [0x49, 0x44, 0x33, ..] => FileType::MP3,
            // EXE
            [0x4D, 0x5A, ..] => FileType::EXE,
            // Unknown
            _ => FileType::Unknown,
        }
    }

    /// Get a human-readable description of the file type
    pub fn description(&self) -> &'static str {
        match self {
            FileType::PDF => "PDF (Portable Document Format)",
            FileType::PNG => "PNG (Portable Network Graphics)",
            FileType::JPEG => "JPEG (Joint Photographic Experts Group)",
            FileType::GIF87a => "GIF87a (Graphics Interchange Format)",
            FileType::GIF89a => "GIF89a (Graphics Interchange Format)",
            FileType::ZIP => "ZIP (Archive File)",
            FileType::GZIP => "GZIP (GNU Zip)",
            FileType::MP3 => "MP3 (MPEG Audio Layer III)",
            FileType::EXE => "EXE (Executable File)",
            FileType::Unknown => "Unknown file type",
        }
    }
}
