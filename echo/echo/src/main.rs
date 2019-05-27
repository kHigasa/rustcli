extern crate getopts;

use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [OPTION]... [FILE]...", program);
    print!("{}", opts.usage(&brief));
}

fn echo(input: &str) {
    print!("{}\n", input);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("n", "", "don't output the trailing newline");
    opts.optflag("e", "", "enable interpretation of backslash escapes");
    opts.optflag("h", "", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let input = if !matches.free.is_empty() {
        matches.free[0].clone();
    } else {
        print_usage(&program, opts);
        return;
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    } else if matches.opt_present("n") {
        return;
    } else if matches.opt_present("e") {
        return;
    } else {
        echo(&input);
        return;
    }
}

