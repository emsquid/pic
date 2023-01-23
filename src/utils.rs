use crate::{result::Result, support};
use ansi_colours::ansi256_from_rgb;
use image::{codecs::png::PngEncoder, DynamicImage, ImageEncoder};
use std::{
    fs::File,
    io::{Error, Write},
    path::PathBuf,
};

#[derive(Clone, Default, Debug)]
pub struct TermSize {
    /// The amount of visible rows in the pty
    pub(crate) rows: u32,
    /// The amount of visible columns in the pty
    pub(crate) cols: u32,
    /// The width of the view in pixels
    pub(crate) width: u32,
    /// The height of the view in pixels
    pub(crate) height: u32,
}

impl TermSize {
    pub fn new(rows: u16, cols: u16, width: u16, height: u16) -> Self {
        Self {
            rows: rows as u32,
            cols: cols as u32,
            width: width as u32,
            height: height as u32,
        }
    }

    pub fn get_cell_size(&self) -> Option<(u32, u32)> {
        if self.cols == 0 || self.rows == 0 {
            return None;
        }
        Some((self.width / self.cols, self.height / self.rows))
    }

    pub fn from_ioctl() -> Result<Self> {
        // TODO: find a way to make that safe
        unsafe {
            let mut ws = libc::winsize {
                ws_row: 0,
                ws_col: 0,
                ws_xpixel: 0,
                ws_ypixel: 0,
            };
            let ret = libc::ioctl(0, libc::TIOCGWINSZ, &mut ws);
            if ret == 0 {
                Ok(TermSize::new(
                    ws.ws_row,
                    ws.ws_col,
                    ws.ws_xpixel,
                    ws.ws_ypixel,
                ))
            } else {
                Err(Error::last_os_error().into())
            }
        }
    }
}

pub fn get_temp_file(prefix: &str) -> Result<(File, PathBuf)> {
    let (tempfile, pathbuf) = tempfile::Builder::new()
        .prefix(prefix)
        .tempfile_in("/tmp/")?
        .keep()?;

    Ok((tempfile, pathbuf))
}

pub fn save_in_tmp_file(buffer: &[u8], file: &mut File) -> Result {
    file.write_all(buffer)?;
    file.flush()?;
    Ok(())
}

#[allow(dead_code)]
pub fn save_cursor(stdout: &mut impl Write) -> Result {
    stdout.write_all(b"\x1b[s")?;
    stdout.flush()?;
    Ok(())
}

#[allow(dead_code)]
pub fn restore_cursor(stdout: &mut impl Write) -> Result {
    stdout.write_all(b"\x1b[u")?;
    stdout.flush()?;
    Ok(())
}

#[allow(dead_code)]
pub fn move_cursor_up(stdout: &mut impl Write, x: u32) -> Result {
    let binding = format!("\x1b[{}A", x + 1);
    stdout.write_all(binding.as_bytes())?;
    stdout.flush()?;
    Ok(())
}

#[allow(dead_code)]
pub fn move_cursor_down(stdout: &mut impl Write, x: u32) -> Result {
    let binding = format!("\x1b[{}B", x + 1);
    stdout.write_all(binding.as_bytes())?;
    stdout.flush()?;
    Ok(())
}

pub fn move_cursor_column(stdout: &mut impl Write, col: u32) -> Result {
    let binding = format!("\x1b[{}G", col + 1);
    stdout.write_all(binding.as_bytes())?;
    stdout.flush()?;
    Ok(())
}

pub fn move_cursor_row(stdout: &mut impl Write, row: u32) -> Result {
    let binding = format!("\x1b[{}d", row + 1);
    stdout.write_all(binding.as_bytes())?;
    stdout.flush()?;
    Ok(())
}

pub fn move_cursor_pos(stdout: &mut impl Write, col: u32, row: u32) -> Result {
    let binding = format!("\x1b[{};{}H", row + 1, col + 1);
    stdout.write_all(binding.as_bytes())?;
    stdout.flush()?;
    Ok(())
}

pub fn move_cursor(stdout: &mut impl Write, col: Option<u32>, row: Option<u32>) -> Result {
    match (col, row) {
        (Some(x), None) => move_cursor_column(stdout, x),
        (None, Some(y)) => move_cursor_row(stdout, y),
        (Some(x), Some(y)) => move_cursor_pos(stdout, x, y),
        (None, None) => Ok(()),
    }
}

#[allow(dead_code)]
pub fn hide_cursor(stdout: &mut impl Write) -> Result {
    stdout.write_all(b"\x1b[?25l")?;
    stdout.flush()?;
    Ok(())
}

#[allow(dead_code)]
pub fn show_cursor(stdout: &mut impl Write) -> Result {
    stdout.write_all(b"\x1b[?25h")?;
    stdout.flush()?;
    Ok(())
}

pub fn fit_in_bounds(
    width: u32,
    height: u32,
    cols: Option<u32>,
    rows: Option<u32>,
    upscale: bool,
) -> Result<(u32, u32)> {
    let term_size = TermSize::from_ioctl()?;
    let (col_size, row_size) = match term_size.get_cell_size() {
        Some((0, 0)) | None => (15, 30),
        Some((c, r)) => (c, r),
    };
    let cols = cols.unwrap_or(term_size.cols);
    let rows = rows.unwrap_or(term_size.rows);

    let (bound_width, bound_height) = (cols * col_size, rows * row_size);

    if !upscale && width < bound_width && height < bound_height {
        return Ok((width / col_size, height / row_size));
    }

    let w_ratio = width * bound_height;
    let h_ratio = bound_width * height;

    if w_ratio >= h_ratio {
        Ok((
            cols,
            std::cmp::max((height * bound_width) / (width * row_size), 1),
        ))
    } else {
        Ok((
            std::cmp::max((width * bound_height) / (height * col_size), 1),
            rows,
        ))
    }
}

pub fn resize(image: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    image.resize_exact(width, height, image::imageops::Triangle)
}

pub fn pixel_is_transparent(rgb: [u8; 4]) -> bool {
    rgb[3] < 25
}

pub fn ansi_rgb(rgb: [u8; 4], bg: bool) -> String {
    match bg {
        false => format!("\x1b[38;2;{};{};{}m", rgb[0], rgb[1], rgb[2]),
        true => format!("\x1b[48;2;{};{};{}m", rgb[0], rgb[1], rgb[2]),
    }
}

pub fn ansi_indexed(rgb: [u8; 4], bg: bool) -> String {
    let index = ansi256_from_rgb((rgb[0], rgb[1], rgb[2]));
    match bg {
        false => format!("\x1b[38;5;{}m", index),
        true => format!("\x1b[48;5;{}m", index),
    }
}

pub fn ansi_color(rgb: [u8; 4], bg: bool) -> String {
    match support::truecolor() {
        true => ansi_rgb(rgb, bg),
        false => ansi_indexed(rgb, bg),
    }
}

// image is mainly supposed to be a GIF
pub fn convert_to_image_buffer(image: &DynamicImage, width: u32, height: u32) -> Result<Vec<u8>> {
    let mut image_buffer = Vec::new();
    PngEncoder::new(&mut image_buffer).write_image(
        image.as_bytes(),
        width,
        height,
        image.color(),
    )?;
    Ok(image_buffer)
}
