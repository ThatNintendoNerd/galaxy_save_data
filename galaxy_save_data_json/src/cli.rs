use clap::{Parser, ValueEnum};

/// Convert Super Mario Galaxy save files to and from JSON.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The input save or JSON file path.
    pub input: String,

    /// The output save or JSON file path.
    pub output: Option<String>,

    /// The newline-separated hash labels file path.
    #[arg(short, long)]
    pub labels: Option<String>,

    /// Reject hash labels not found in the labels file.
    #[arg(short, long, requires("labels"))]
    pub strict: bool,

    /// The source or target console.
    #[arg(short, long, default_value_t, value_enum)]
    pub platform: Platform,

    /// Parse the save file even if the header is invalid.
    #[arg(short, long)]
    pub force: bool,
}

/// A compatible console.
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum Platform {
    /// Nintendo Wii
    #[default]
    Wii,

    /// NVIDIA Shield TV
    ShieldTv,

    /// Nintendo Switch (Super Mario 3D All-Stars)
    Switch,
}

/// A character encoding for labels prior to hashing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LabelEncoding {
    /// Shift JIS
    ShiftJis,

    /// UTF-8
    Utf8,
}

impl From<Platform> for LabelEncoding {
    fn from(platform: Platform) -> Self {
        match platform {
            Platform::Wii | Platform::ShieldTv => Self::ShiftJis,
            Platform::Switch => Self::Utf8,
        }
    }
}
