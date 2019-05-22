extern crate getopts;
use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [OPTION]... [FILE]...", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if !matches.free.is_empty() {
        matches.free[0].clone();
    } else {
        print_usage(&program, opts);
        return
    }
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
}

