use clap::Parser;

mod options;
mod previewer;
mod result;
mod support;
mod utils;

fn main() {
    let mut stdout = std::io::stdout();
    let mut options = options::Options::parse_from(wild::args());

    if let Err(err) = previewer::preview(&mut stdout, &mut options) {
        eprintln!("{err}");
    };
}
