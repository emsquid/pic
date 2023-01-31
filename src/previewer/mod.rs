use crate::options::Options;
use crate::result::Result;
use crate::support::Protocol;
use std::io::Write;

mod blocks;
mod iterm;
mod kitty;
mod sixel;

pub fn preview(stdout: &mut impl Write, options: &Options) -> Result {
    let protocol = Protocol::choose(options);
    for image_path in &options.path {
        match protocol {
            Protocol::Kitty => kitty::preview(stdout, image_path, options)?,
            Protocol::Iterm => iterm::preview(stdout, image_path, options)?,
            Protocol::Sixel => sixel::preview(stdout, image_path, options)?,
            Protocol::Blocks => blocks::preview(stdout, image_path, options)?,
        }
    }
    Ok(())
}
