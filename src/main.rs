use std::borrow::Cow;
use std::ffi::OsStr;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let path;

    let args: Vec<_> = std::env::args_os().collect();
    if args.len() <= 1 {
        path = OsStr::new("target.txt");
    } else if args.len() == 2 {
        path = &args[1];
    } else {
        let program_name = args.get(0).map(|x| x.to_string_lossy()).unwrap_or(Cow::Borrowed("command-launcher"));

        eprintln!("Usage: {} <file>", program_name);
        std::process::exit(1);
    }

    let cmdline = std::fs::read(path)?;
    let cmdline = String::from_utf8(cmdline).context("The file was not valid UTF-8")?;

    let opts = shellish_parse::ParseOptions::new().comment_char(Some('#')).no_strict_escapes();
    let cmdline = shellish_parse::parse(&cmdline, opts)?;

    if cmdline.is_empty() {
        eprintln!("Target command line is empty");
        std::process::exit(1);
    }

    let code = std::process::Command::new(&cmdline[0]).args(&cmdline[1..]).status()?;
    std::process::exit(code.code().unwrap_or(1));
}
