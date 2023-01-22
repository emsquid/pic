use crate::options::{Method, Options};
use crate::result::{Error, Result};
use crate::support::has_support;
use std::io::Write;

mod blocks;
mod iterm;
mod kitty;
mod sixel;

pub fn preview(stdout: &mut impl Write, options: &Options) -> Result {
    if options.force || has_support(options.method) {
        match options.method {
            Method::Kitty => kitty::preview(stdout, options),
            Method::Sixel => sixel::preview(stdout, options),
            Method::Iterm => iterm::preview(stdout, options),
            Method::Blocks => blocks::preview(stdout, options),
        }
    } else {
        let err = format!("Your terminal doesn't support {}", options.method);
        Err(Error::MethodSupport(err))
    }
}
