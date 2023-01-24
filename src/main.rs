use clap::Parser;

mod options;
mod previewer;
mod result;
mod support;
mod utils;

fn main() {
    let mut stdout = std::io::stdout();
    let options = options::Options::parse();

    // prevents cursor flickering
    utils::hide_cursor(&mut stdout).unwrap();
    match previewer::preview(&mut stdout, &options) {
        Ok(()) => {}
        Err(err) => eprintln!("{err}"),
    };
    utils::show_cursor(&mut stdout).unwrap();
}
