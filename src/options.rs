use crate::support::Protocol;
use clap::{arg, command, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Options {
    /// Image(s) to preview
    #[arg(num_args(1..))]
    pub path: Vec<PathBuf>,

    /// Previewing protocol to use
    #[arg(short, long)]
    pub protocol: Option<Protocol>,
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
    /// Spacing between images if more than one file is provided
    #[arg(long)]
    pub spacing: Option<u32>,
    /// Upscale image if needed
    #[arg(short, long)]
    pub upscale: bool,
    /// Only show first frame of GIFs
    #[arg(short = 's', long = "static", conflicts_with("gif_loop"))]
    pub gif_static: bool,
    /// Loop GIFs infinitely
    #[arg(short = 'l', long = "loop")]
    pub gif_loop: bool,

    /// Load image with the given id (kitty only)
    #[arg(long, value_name = "ID")]
    pub load: Option<u32>,
    /// Display image with the given id (kitty only)
    #[arg(long, value_name = "ID")]
    pub display: Option<u32>,
    /// Clear image with the given id (0 for all) (kitty only)
    #[arg(long, value_name = "ID")]
    pub clear: Option<u32>,
}
