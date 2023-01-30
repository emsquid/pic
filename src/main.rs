use clap::Parser;

mod options;
mod previewer;
mod result;
mod support;
mod utils;

fn main() {
    let mut stdout = std::io::stdout();
    let options = options::Options::parse();

    if let Err(err) = previewer::preview(&mut stdout, &options) {
        eprintln!("{err}");
    };
}
