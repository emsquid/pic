use crate::support::Protocol;
use clap::{arg, command, Parser};
use std::path::PathBuf;

/// Options for previewing an image in terminal
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

impl Options {
    /// New options for images
    pub fn new(path: Vec<PathBuf>) -> Self {
        Self {
            path,
            protocol: None,
            x: None,
            y: None,
            cols: None,
            rows: None,
            spacing: None,
            upscale: false,
            gif_static: false,
            gif_loop: false,
            load: None,
            display: None,
            clear: None,
        }
    }

    /// Set position of images in the terminal
    pub fn set_position(&mut self, x: Option<u32>, y: Option<u32>) {
        self.x = x;
        self.y = y;
    }

    /// Set size of images in the terminal
    pub fn set_size(&mut self, cols: Option<u32>, rows: Option<u32>) {
        self.cols = cols;
        self.rows = rows;
    }

    /// Set spacing of images in the terminal
    pub fn set_spacing(&mut self, spacing: Option<u32>) {
        self.spacing = spacing;
    }

    /// Upscale images
    pub fn upscale(&mut self) {
        self.upscale = true;
    }

    /// Set GIFs to be static
    pub fn set_static(&mut self) {
        self.gif_static = true;
        self.gif_loop = false;
    }

    /// Set GIFs to loop
    pub fn set_loop(&mut self) {
        self.gif_static = false;
        self.gif_loop = true;
    }

    /// Set options for kitty
    pub fn set_kitty(&mut self, load: Option<u32>, display: Option<u32>, clear: Option<u32>) {
        if self.protocol == Some(Protocol::Kitty) {
            self.load = load;
            self.display = display;
            self.clear = clear;
        }
    }
}
