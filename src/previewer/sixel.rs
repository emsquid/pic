use crate::options::{Action, Options};
use crate::utils::{fit_in_bounds, get_cell_size, move_cursor};
use sixel_rs::encoder::Encoder;
use sixel_rs::optflags::{EncodePolicy, ResampleMethod, SizeSpecification::Pixel};
use std::io::{Error, Write};

pub fn display(stdout: &mut impl Write, options: &Options) -> Result<(), Error> {
    let size = imagesize::size(&options.path).unwrap();
    let (width, height) = (size.width as u32, size.height as u32);
    let (cols, rows) = fit_in_bounds(width, height, options.cols, options.rows, options.upscale);
    let (col_size, row_size) = match get_cell_size() {
        (0, 0) => (15, 30),
        (c, r) => (c, r),
    };

    let encoder = Encoder::new().unwrap();
    encoder.set_width(Pixel((cols * col_size) as u64)).unwrap();
    encoder.set_height(Pixel((rows * row_size) as u64)).unwrap();
    encoder.set_resampling(ResampleMethod::Nearest).unwrap();
    encoder.set_encode_policy(EncodePolicy::Fast).unwrap();
    encoder.use_static().unwrap();

    move_cursor(stdout, options.x, options.y)?;
    encoder.encode_file(&options.path).unwrap();

    stdout.write(b"\n")?;
    stdout.flush()
}

pub fn preview(stdout: &mut impl Write, options: &Options) -> Result<(), Error> {
    match options.action {
        Action::Display => display(stdout, options),
        _ => Ok(eprintln!("Error: these actions aren't implemented for sixel method: load/load-and-display/clear
                          \n\nUsage: pic blocks display <PATH>\n\nFor more information, try '--help'"))
    }
}
