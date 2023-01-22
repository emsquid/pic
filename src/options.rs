use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Method {
    Kitty,
    Sixel,
    Iterm,
    Blocks,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Action {
    Load,
    Display,
    LoadAndDisplay,
    Clear,
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct Options {
    /// Previewing method to use
    pub method: Method,
    /// What to do with the image
    pub action: Action,
    /// Path to the image to preview
    pub path: PathBuf,

    /// id to use
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
    /// Do not check for method support
    #[arg(short, long, default_value_t = false)]
    pub force: bool,
}
