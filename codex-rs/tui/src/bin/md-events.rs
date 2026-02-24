use std::io::Read;
use std::io::Write;
use std::io::{self};

fn main() {
    let mut input = String::new();
    if let Err(err) = io::stdin().read_to_string(&mut input) {
        let _ = writeln!(std::io::stderr(), "failed to read stdin: {err}");
        std::process::exit(1);
    }

    let parser = pulldown_cmark::Parser::new(&input);
    for event in parser {
        let _ = writeln!(std::io::stdout(), "{event:?}");
    }
}
