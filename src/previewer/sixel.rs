use crate::options::{Action, Options};
use std::io::{Error, Write};

pub fn preview(_stdout: &mut impl Write, options: &Options) -> Result<(), Error> {
    match options.action {
        Action::Load => Ok(()),
        Action::Display => Ok(()),
        Action::LoadAndDisplay => Ok(()),
        Action::Clear => Ok(()),
    }
}
