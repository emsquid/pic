use crate::options::{Action, Options};
use crate::result::Result;
use crate::utils::{fit_in_bounds, move_cursor, TermSize};
use sixel_rs::encoder::Encoder;
use sixel_rs::optflags::{EncodePolicy, ResampleMethod, SizeSpecification::Pixel};
use std::io::Write;

pub fn display(stdout: &mut impl Write, options: &Options) -> Result {
    let image_size = imagesize::size(&options.path)?;
    let (width, height) = (image_size.width as u32, image_size.height as u32);
    let (cols, rows) = fit_in_bounds(width, height, options.cols, options.rows, options.upscale)?;

    let term_size = TermSize::from_ioctl()?;
    let (col_size, row_size) = match term_size.get_cell_size() {
        Some((0, 0)) | None => (15, 30),
        Some((c, r)) => (c, r),
    };

    let encoder = Encoder::new()?;
    encoder.set_width(Pixel((cols * col_size) as u64))?;
    encoder.set_height(Pixel((rows * row_size) as u64))?;
    encoder.set_resampling(ResampleMethod::Nearest)?;
    encoder.set_encode_policy(EncodePolicy::Fast)?;
    encoder.use_static()?;

    move_cursor(stdout, options.x, options.y)?;
    encoder.encode_file(&options.path)?;

    stdout.flush()?;
    Ok(())
}

pub fn preview(stdout: &mut impl Write, options: &Options) -> Result {
    match options.action {
        Action::Display => display(stdout, options),
        _ => Ok(eprintln!("Error: these actions aren't implemented for sixel method: load/load-and-display/clear
                          \n\nUsage: pic blocks display <PATH>\n\nFor more information, try '--help'"))
    }
}
