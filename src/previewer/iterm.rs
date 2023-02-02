use crate::options::Options;
use crate::result::Result;
use crate::utils::{convert_to_image_buffer, fit_in_bounds, handle_spacing, move_cursor};
use base64::{engine::general_purpose, Engine as _};
use image::ImageFormat;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

fn display(stdout: &mut impl Write, image_path: &PathBuf, options: &mut Options) -> Result {
    let mut image = File::open(image_path)?;
    let mut buffer = Vec::new();
    image.read_to_end(&mut buffer)?;

    let image_size = imagesize::size(image_path)?;
    let (width, height) = (image_size.width as u32, image_size.height as u32);
    let (cols, rows) = fit_in_bounds(width, height, options.cols, options.rows, options.upscale)?;

    let data = match (image::guess_format(&buffer)?, options.gif_static) {
        (ImageFormat::Gif, true) => {
            let gif = image::load_from_memory(&buffer)?;
            general_purpose::STANDARD.encode(convert_to_image_buffer(&gif, width, height)?)
        }
        _ => general_purpose::STANDARD.encode(buffer),
    };

    let command = format!("\x1b]1337;File=width={cols};height={rows};inline=1;:{data}\x07\r");

    move_cursor(stdout, options.x, options.y)?;
    stdout.write_all(command.as_bytes())?;

    stdout.flush()?;
    Ok(())
}

pub fn preview(stdout: &mut impl Write, image_path: &PathBuf, options: &mut Options) -> Result {
    display(stdout, image_path, options)?;
    handle_spacing(stdout, options.spacing)?;
    Ok(())
}
