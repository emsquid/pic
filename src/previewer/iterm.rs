use crate::options::{Action, Options};
use crate::result::{Error, Result};
use crate::utils::{fit_in_bounds, move_cursor};
use base64::{engine::general_purpose, Engine as _};
use std::env;
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

fn check_term() -> bool {
    let program = env::var("TERM_PROGRAM").unwrap_or_default();
    let lc = env::var("LC_TERMINAL").unwrap_or_default();
    program.contains("iTerm")
        || program.contains("WezTerm")
        || lc.contains("iTerm")
        || lc.contains("WezTerm")
}

fn check_support() -> bool {
    check_term()
}

pub fn preview(stdout: &mut impl Write, options: &Options) -> Result {
    match options.force || check_support() {
        true => match options.action {
            Action::Display => display(stdout, options),
            _ => Err(Error::ActionSupport("Iterm doesn't support load/clear")),
        },
        false => Err(Error::MethodSupport(
            "Your terminal doesn't support iTerm protocol",
        )),
    }
}
