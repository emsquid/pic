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
    Ok(())
}

fn display(stdout: &mut impl Write, options: &Options) -> Result {
    let image = image::open(&options.path)?;
    let (width, height) = (image.width(), image.height());
    let term_size = TermSize::from_ioctl()?;
    let (cols, rows) = fit_in_bounds(width, height, options.cols, options.rows, options.upscale)?;
    let rgba = resize(&image, cols, rows * 2).to_rgba8();

    let mut backgrounds = vec![[0; 4]; cols as usize];

    for (r, row) in rgba.enumerate_rows() {
        let is_bg = r % 2 == 0;

        for (c, pixel) in row.enumerate() {
            let overflow = (c as u32) + options.x.unwrap_or(0) >= term_size.cols;

            if !overflow {
                let rgb = pixel.2 .0;

                match (is_bg, pixel_is_transparent(rgb)) {
                    (true, _) => {
                        backgrounds[c] = rgb;
                    }
                    (false, true) => {
                        if pixel_is_transparent(backgrounds[c]) {
                            write_color_block(stdout, " ", "", "")?;
                        } else {
                            let ansi_fg = ansi_color(backgrounds[c], false);
                            write_color_block(stdout, TOP_BLOCK, "", &ansi_fg)?;
                        };
                    }
                    (false, false) => {
                        if pixel_is_transparent(backgrounds[c]) {
                            let ansi_fg = ansi_color(rgb, false);
                            write_color_block(stdout, BOTTOM_BLOCK, "", &ansi_fg)?;
                        } else {
                            let ansi_bg = ansi_color(backgrounds[c], true);
                            let ansi_fg = ansi_color(rgb, false);
                            write_color_block(stdout, BOTTOM_BLOCK, &ansi_bg, &ansi_fg)?;
                        };
                    }
                }
            }
        }

        if !is_bg {
            stdout.write_all(b"\n")?;
        } else {
            // if bg, get ready for writing next line
            move_cursor(stdout, options.x, options.y)?;
        };
    }

    stdout.flush()?;
    Ok(())
}

pub fn preview(stdout: &mut impl Write, options: &Options) -> Result {
    match options.action {
        Action::Display => display(stdout, options),
        _ => Err(Error::ActionSupport("Blocks doesn't support load/clear")),
    }
}
