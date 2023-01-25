use crate::options::Options;
use crate::result::Result;
use crate::support::Protocol;
use std::io::Write;

mod blocks;
mod iterm;
mod kitty;
mod sixel;

pub fn preview(stdout: &mut impl Write, options: &Options) -> Result {
    match Protocol::choose_protocol(options) {
        Protocol::Kitty => kitty::preview(stdout, options),
        Protocol::Iterm => iterm::preview(stdout, options),
        Protocol::Sixel => sixel::preview(stdout, options),
        Protocol::Blocks => blocks::preview(stdout, options),
    }
}
