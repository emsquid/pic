use crate::options::{Action, Options};
use crate::result::{Error, Result};
use crate::utils::{fit_in_bounds, move_cursor, TermSize};
use console::{Key, Term};
use sixel_rs::encoder::Encoder;
use sixel_rs::optflags::{EncodePolicy, ResampleMethod, SizeSpecification::Pixel};
use std::env;
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

fn check_term() -> bool {
    let term = env::var("TERM").unwrap_or_default();
    match term.as_str() {
        "xterm" | "xterm-256color" | "yaft-256color" | "st-256color" | "foot" | "foot-extra"
        | "mlterm" => true,
        _ => false,
    }
}

fn check_attrs() -> Result<bool> {
    let mut stdout = Term::stdout();
    stdout.write(b"\x1b[c")?;
    stdout.flush()?;

    let mut response = String::new();
    while let Ok(key) = stdout.read_key() {
        if let Key::Char(chr) = key {
            response.push(chr);
            if chr == 'c' {
                break;
            }
        }
    }

    Ok(response.contains(";4;") || response.contains(";4c"))
}

fn check_support() -> bool {
    check_term() && check_attrs().unwrap_or(false)
}

pub fn preview(stdout: &mut impl Write, options: &Options) -> Result {
    match options.force || check_support() {
        true => match options.action {
            Action::Display => display(stdout, options),
            _ => Err(Error::ActionSupport("Sixel doesn't support load/clear")),
        },
        false => Err(Error::MethodSupport(
            "Your terminal doesn't support Sixel protocol",
        )),
    }
}
