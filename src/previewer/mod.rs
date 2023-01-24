use crate::options::{Options, Protocol};
use crate::result::{Error, Result};
use crate::support::has_support;
use std::io::Write;

mod blocks;
mod iterm;
mod kitty;
mod sixel;

pub fn preview(stdout: &mut impl Write, options: &Options) -> Result {
    if options.force || has_support(options.protocol) {
        match options.protocol {
            Protocol::Kitty => kitty::preview(stdout, options),
            Protocol::Sixel => sixel::preview(stdout, options),
            Protocol::Iterm => iterm::preview(stdout, options),
            Protocol::Blocks => blocks::preview(stdout, options),
        }
    } else {
        let err = format!("Your terminal doesn't support {}", options.protocol);
        Err(Error::ProtocolSupport(err))
    }
}
