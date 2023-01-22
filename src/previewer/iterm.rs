use crate::options::{Action, Options};
use crate::result::{Error, Result};
use crate::utils::{fit_in_bounds, move_cursor};
use base64::{engine::general_purpose, Engine as _};
use std::fs::File;
use std::io::{Read, Write};

fn display(stdout: &mut impl Write, options: &Options) -> Result {
    let mut image = File::open(&options.path)?;
    let mut buffer = Vec::new();
    image.read_to_end(&mut buffer)?;

    let image_size = imagesize::size(&options.path)?;
    let (width, height) = (image_size.width as u32, image_size.height as u32);
    let (cols, rows) = fit_in_bounds(width, height, options.cols, options.rows, options.upscale)?;
    let data = general_purpose::STANDARD.encode(buffer);

    move_cursor(stdout, options.x, options.y)?;
    stdout.write_all(
        format!(
            "\x1b]1337;File=width={cols};height={rows};inline=1;preserveAspectRatio=1:{data}\x07\n",
        )
        .as_bytes(),
    )?;

    stdout.flush()?;
    Ok(())
}

pub fn preview(stdout: &mut impl Write, options: &Options) -> Result {
    match options.action {
        Action::Display => display(stdout, options),
        _ => Err(Error::ActionSupport(format!(
            "Iterm doesn't support '{}', try '--help'",
            options.action
        ))),
    }
}
