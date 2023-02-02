use crate::options::Options;
use crate::result::Result;
use crate::support::Protocol;
use std::io::Write;

mod blocks;
mod iterm;
mod kitty;
mod sixel;

pub fn preview(stdout: &mut impl Write, options: &mut Options) -> Result {
    let protocol = Protocol::choose(options);
    let image_paths = options.path.clone();
    // If there is more than one path, render `-y` flag useless
    // TODO: Does not work if the only path is a directory
    if options.y.is_some() && image_paths.len() > 1 {
        options.y = None;
        // Notify about spacing flag
    }

    for image_path in &image_paths {
        if image_path.is_dir() {
            continue;
        }

        match protocol {
            Protocol::Kitty => kitty::preview(stdout, image_path, options)?,
            Protocol::Iterm => iterm::preview(stdout, image_path, options)?,
            Protocol::Sixel => sixel::preview(stdout, image_path, options)?,
            Protocol::Blocks => blocks::preview(stdout, image_path, options)?,
        }
    }
    Ok(())
}
