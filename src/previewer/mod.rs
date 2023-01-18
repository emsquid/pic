use crate::options::{Method, Options};
use std::io::Write;

mod blocks;
mod kitty;
mod sixel;

pub fn preview(stdout: &mut impl Write, options: &Options) -> Result<(), std::io::Error> {
    match options.method {
        Method::Kitty => kitty::preview(stdout, options),
        Method::Sixel => sixel::preview(stdout, options),
        Method::Blocks => blocks::preview(stdout, options),
    }
}
