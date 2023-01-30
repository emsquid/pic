use crate::{options::Options, result::Result};
use clap::ValueEnum;
use console::{Key, Term};
use std::{env, io::Write};

// add supported Terminals based on their eventual environment variables
const KITTY_SUPPORTED: [&str; 2] = ["xterm-kitty", "WezTerm"];

const SIXEL_SUPPORTED: [&str; 7] = [
    "xterm-256color",
    "xterm",
    "yaft-256color",
    "st-256color",
    "foot-extra",
    "foot",
    "mlterm",
];

const ITERM_SUPPORTED: [&str; 3] = ["iTerm", "WezTerm", "mintty"];

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Protocol {
    Kitty,
    Sixel,
    Iterm,
    Blocks,
}

impl Protocol {
    pub fn choose(options: &Options) -> Self {
        if let Some(protocol) = options.protocol {
            protocol
        } else if Protocol::support_iterm() {
            Protocol::Iterm
        } else if Protocol::support_kitty() {
            Protocol::Kitty
        } else if Protocol::support_sixel() {
            Protocol::Sixel
        } else {
            Protocol::Blocks
        }
    }

    fn support_kitty() -> bool {
        // Term check
        let term = env::var("TERM").unwrap_or_default();
        let program = env::var("TERM_PROGRAM").unwrap_or_default();
        // Attrs check (send a kitty request)
        let attrs = vec![vec!["OK"]];
        let kitty_command = b"\x1b_Gi=31,s=1,v=1,a=q,t=d,f=24;AAAA\x1b\\";

        (find_match(&KITTY_SUPPORTED, &term) || find_match(&KITTY_SUPPORTED, &program))
            && check_primary_attributes(&attrs, Some(kitty_command)).unwrap_or(false)
    }

    fn support_sixel() -> bool {
        // Term check
        let term = env::var("TERM").unwrap_or_default();
        // Attrs check (4 is for sixel support)
        let attrs = vec![vec![";4;", ";4c"]];

        find_match(&SIXEL_SUPPORTED, &term)
            && check_primary_attributes(&attrs, None).unwrap_or(false)
    }

    fn support_iterm() -> bool {
        // Term check
        let program = env::var("TERM_PROGRAM").unwrap_or_default();
        let lc = env::var("LC_TERMINAL").unwrap_or_default();

        find_match(&ITERM_SUPPORTED, &program) || find_match(&ITERM_SUPPORTED, &lc)
    }
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Protocol::Kitty => write!(f, "Kitty graphics protocol"),
            Protocol::Sixel => write!(f, "Sixel protocol"),
            Protocol::Iterm => write!(f, "iTerm protocol"),
            Protocol::Blocks => write!(f, "Unicode blocks"),
        }
    }
}

pub fn find_match(list: &[&str], var: &str) -> bool {
    list.iter().any(|s| var.contains(s))
}

pub fn check_primary_attributes(attrs: &[Vec<&str>], subcommand: Option<&[u8]>) -> Result<bool> {
    let mut stdout = Term::stdout();
    let command = [subcommand.unwrap_or_default(), b"\x1b[c"].concat();
    stdout.write_all(&command)?;
    stdout.flush()?;

    let mut response = String::new();
    // what if the terminal doesn't answer ?
    while !response.contains('c') {
        match stdout.read_key() {
            Ok(Key::Char(chr)) => response.push(chr),
            Ok(Key::UnknownEscSeq(esc)) => response.extend(esc),
            Err(_) => break,
            _ => (),
        }
    }

    // check if each groups of attrs has at least a match
    Ok(attrs.iter().all(|group| find_match(group, &response)))
}

pub fn truecolor() -> bool {
    let colorterm = env::var("COLORTERM").unwrap_or_default();
    matches!(colorterm.as_str(), "truecolor" | "24bit")
}
