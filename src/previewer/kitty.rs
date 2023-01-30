use crate::options::Options;
use crate::result::Result;
use crate::utils::{create_temp_file, fit_in_bounds, move_cursor, save_in_tmp_file};
use base64::{engine::general_purpose, Engine as _};
use image::io::Reader;
use std::io::Write;

const KITTY_PREFIX: &str = "pic.tty-graphics-protocol.";
const PROTOCOL_START: &str = "\x1b_G";
const PROTOCOL_END: &str = "\x1b\\";

fn send_graphics_command(stdout: &mut impl Write, command: &str, payload: Option<&str>) -> Result {
    let data = general_purpose::STANDARD.encode(payload.unwrap_or_default());
    let command = format!("{PROTOCOL_START}{command};{data}{PROTOCOL_END}");

    stdout.write_all(command.as_bytes())?;

    stdout.flush()?;
    Ok(())
}

fn clear(stdout: &mut impl Write, id: u32, _options: &Options) -> Result {
    if id == 0 {
        send_graphics_command(stdout, "a=d,d=a", None)
    } else {
        send_graphics_command(stdout, &format!("a=d,d=i,i={id}"), None)
    }
}

fn load(stdout: &mut impl Write, id: u32, options: &Options) -> Result {
    let image = Reader::open(&options.path)?
        .with_guessed_format()?
        .decode()?
        .to_rgba8();
    let (width, height) = image.dimensions();
    let (mut tempfile, pathbuf) = create_temp_file(KITTY_PREFIX)?;
    save_in_tmp_file(image.as_raw(), &mut tempfile)?;

    let command = format!("a=t,t=t,f=32,s={width},v={height},i={id},q=2");
    send_graphics_command(stdout, &command, pathbuf.to_str())
}

fn display(stdout: &mut impl Write, id: Option<u32>, options: &Options) -> Result {
    let (mut tempfile, pathbuf) = create_temp_file(KITTY_PREFIX)?;
    let (command, payload) = if let Some(id) = id {
        let image_size = imagesize::size(&options.path)?;
        let (width, height) = (image_size.width as u32, image_size.height as u32);
        let (cols, rows) =
            fit_in_bounds(width, height, options.cols, options.rows, options.upscale)?;

        let command = format!("a=p,c={cols},r={rows},i={id},q=2");
        (command, None)
    } else {
        let image = Reader::open(&options.path)?
            .with_guessed_format()?
            .decode()?
            .to_rgba8();
        let (width, height) = image.dimensions();
        let (cols, rows) =
            fit_in_bounds(width, height, options.cols, options.rows, options.upscale)?;
        save_in_tmp_file(image.as_raw(), &mut tempfile)?;
        drop(tempfile);

        let command = format!("a=T,t=t,f=32,s={width},v={height},c={cols},r={rows},q=2",);
        (command, pathbuf.to_str())
    };

    move_cursor(stdout, options.x, options.y)?;
    send_graphics_command(stdout, &command, payload)?;

    stdout.write_all(b"\n")?;
    stdout.flush()?;
    Ok(())
}

pub fn preview(stdout: &mut impl Write, options: &Options) -> Result {
    if let Some(id) = options.clear {
        clear(stdout, id, options)?;
    }

    match (options.load, options.display) {
        (Some(id_load), Some(id_display)) => {
            load(stdout, id_load, options)?;
            display(stdout, Some(id_display), options)
        }
        (Some(id), None) => load(stdout, id, options),
        (None, Some(id)) => display(stdout, Some(id), options),
        (None, None) => display(stdout, None, options),
    }
}
