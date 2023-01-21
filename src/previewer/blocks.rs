use crate::options::{Action, Options};
use crate::utils::{
    convert_to_ansi, fit_in_bounds, move_cursor, pixel_is_transparent, resize, TermSize,
};
use std::io::{Error, Write};

const TOP_BLOCK: &str = "\u{2580}";
const BOTTOM_BLOCK: &str = "\u{2584}";

fn write_color_block(
    stdout: &mut impl Write,
    block: &str,
    ansi_bg: &str,
    ansi_fg: &str,
) -> Result<(), Error> {
    stdout.write_all(format!("{ansi_bg}{ansi_fg}{block}\x1b[m").as_bytes())
}

fn display(stdout: &mut impl Write, options: &Options) -> Result<(), Error> {
    let image = image::open(&options.path).unwrap();
    let (width, height) = (image.width(), image.height());
    let (cols, rows) = fit_in_bounds(width, height, options.cols, options.rows, options.upscale)
        .unwrap_or_default();
    let term_size = TermSize::from_ioctl()?;
    let rgba = resize(&image, cols, rows * 2).to_rgba8();

    let mut backgrounds: Vec<[u8; 4]> = vec![[0; 4]; cols as usize];

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
                            let ansi_fg = convert_to_ansi(backgrounds[c], false);
                            write_color_block(stdout, TOP_BLOCK, "", &ansi_fg)?;
                        };
                    }
                    (false, false) => {
                        if pixel_is_transparent(backgrounds[c]) {
                            let ansi_fg = convert_to_ansi(rgb, false);
                            write_color_block(stdout, BOTTOM_BLOCK, "", &ansi_fg)?;
                        } else {
                            let ansi_bg = convert_to_ansi(backgrounds[c], true);
                            let ansi_fg = convert_to_ansi(rgb, false);
                            write_color_block(stdout, BOTTOM_BLOCK, &ansi_bg, &ansi_fg)?;
                        };
                    }
                }
            }
        }

        if !is_bg {
            stdout.write_all(b"\n")?;
        } else {
            // if not bg, get ready for writing next line
            move_cursor(stdout, options.x, options.y)?;
        };
    }

    stdout.flush()
}

pub fn preview(stdout: &mut impl Write, options: &Options) -> Result<(), Error> {
    match options.action {
        Action::Display => display(stdout, options),
        _ => Ok(eprintln!("Error: these actions aren't implemented for blocks method: load/load-and-display/clear
                          \n\nUsage: pic blocks display <PATH>\n\nFor more information, try '--help'"))
    }
}
