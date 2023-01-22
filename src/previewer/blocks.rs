use crate::options::{Action, Options};
use crate::result::{Error, Result};
use crate::utils::{
    ansi_color, fit_in_bounds, move_cursor, pixel_is_transparent, resize, TermSize,
};
use std::io::Write;

const ANSI_CLEAR: &str = "\x1b[m";
const TOP_BLOCK: &str = "\u{2580}";
const BOTTOM_BLOCK: &str = "\u{2584}";

fn write_color_block(stdout: &mut impl Write, block: &str, ansi_bg: &str, ansi_fg: &str) -> Result {
    stdout.write_all(format!("{ansi_bg}{ansi_fg}{block}{ANSI_CLEAR}").as_bytes())?;
    stdout.flush()?;
    Ok(())
}

fn display(stdout: &mut impl Write, options: &Options) -> Result {
    let image = image::open(&options.path)?;
    let (width, height) = (image.width(), image.height());
    let term_size = TermSize::from_ioctl()?;
    let (cols, rows) = fit_in_bounds(width, height, options.cols, options.rows, options.upscale)?;
    let rgba = resize(&image, cols, rows * 2).to_rgba8();

    move_cursor(stdout, options.x, options.y)?;
    let mut backgrounds = vec![[0; 4]; cols as usize];
    for (r, row) in rgba.enumerate_rows() {
        let is_bg = r % 2 == 0;

        for (c, pixel) in row.enumerate() {
            let overflow = (c as u32) + options.x.unwrap_or(0) >= term_size.cols;

            if !overflow {
                if is_bg {
                    backgrounds[c] = pixel.2 .0;
                } else {
                    let rgb_fg = pixel.2 .0;
                    let rgb_bg = backgrounds[c];

                    match (pixel_is_transparent(rgb_fg), pixel_is_transparent(rgb_bg)) {
                        (true, true) => write_color_block(stdout, " ", "", "")?,
                        (true, false) => {
                            let ansi_fg = ansi_color(rgb_bg, false);
                            write_color_block(stdout, TOP_BLOCK, "", &ansi_fg)?
                        }
                        (false, true) => {
                            let ansi_fg = ansi_color(rgb_fg, false);
                            write_color_block(stdout, BOTTOM_BLOCK, "", &ansi_fg)?;
                        }
                        (false, false) => {
                            let ansi_bg = ansi_color(rgb_bg, true);
                            let ansi_fg = ansi_color(rgb_fg, false);
                            write_color_block(stdout, BOTTOM_BLOCK, &ansi_bg, &ansi_fg)?;
                        }
                    }
                }
            }
        }

        if !is_bg {
            stdout.write_all(b"\n")?;
        } else {
            // if bg, get ready for writing next line (only need to move col)
            move_cursor(stdout, options.x, None)?;
        };
    }

    Ok(())
}

pub fn preview(stdout: &mut impl Write, options: &Options) -> Result {
    match options.action {
        Action::Display => display(stdout, options),
        _ => Err(Error::ActionSupport(format!(
            "Blocks doesn't support '{}', try '--help'",
            options.action
        ))),
    }
}
