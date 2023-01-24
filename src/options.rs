use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Protocol {
    Kitty,
    Sixel,
    Iterm,
    Blocks,
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct Options {
    /// Previewing protocol to use
    pub protocol: Protocol,
    /// Image to preview
    pub path: PathBuf,

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
    #[arg(short, long)]
    pub upscale: bool,
    /// Only show first frame of GIFs
    #[arg(short = 's', long = "static")]
    pub gif_static: bool,
    /// Load image with the given id (kitty only)
    #[arg(short, long, value_name = "ID")]
    pub load: Option<u32>,
    /// Display image with the given id (kitty only)
    #[arg(short, long, value_name = "ID")]
    pub display: Option<u32>,
    /// Clear image with the given id (0 for all) (kitty only)
    #[arg(short, long, value_name = "ID")]
    pub clear: Option<u32>,
    /// Do not check for protocol support
    #[arg(short, long)]
    pub force: bool,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Protocol::Kitty => write!(f, "Kitty graphics protocol"),
            Protocol::Sixel => write!(f, "Sixel protocol"),
            Protocol::Iterm => write!(f, "iTerm protocol"),
            Protocol::Blocks => write!(f, "Unicode blocks"),
        }
    }
}
