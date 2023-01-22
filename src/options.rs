use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Protocol {
    Kitty,
    Sixel,
    Iterm,
    Blocks,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Action {
    Display,
    // (kitty only)
    Load,
    // (kitty only)
    LoadAndDisplay,
    // (kitty only)
    Clear,
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct Options {
    /// Previewing protocol to use
    pub protocol: Protocol,
    /// What to do with the image
    pub action: Action,
    /// Path to the image to preview
    pub path: PathBuf,

    /// id to use (kitty only)
    #[arg(short, long)]
    pub id: Option<u32>,
    /// x position (0 is left)
    #[arg(short, long)]
    pub x: Option<u32>,
    /// y position (0 is top)
    #[arg(short, long)]
    pub y: Option<u32>,
    /// Number of cols to fit the preview in
    #[arg(short, long)]
    pub cols: Option<u32>,
    /// Number of rows to fit the preview in
    #[arg(short, long)]
    pub rows: Option<u32>,
    /// Upscale image if needed
    #[arg(short, long, default_value_t = false)]
    pub upscale: bool,
    /// Do not check for protocol support
    #[arg(short, long, default_value_t = false)]
    pub force: bool,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Protocol::Kitty => write!(f, "Kitty graphics protocol"),
            Protocol::Sixel => write!(f, "Sixel protocol"),
            Protocol::Iterm => write!(f, "iTerm protocol"),
            Protocol::Blocks => write!(f, "ANSI blocks"),
        }
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Action::Display => write!(f, "display"),
            Action::Load => write!(f, "load"),
            Action::LoadAndDisplay => write!(f, "load-and-display"),
            Action::Clear => write!(f, "clear"),
        }
    }
}
