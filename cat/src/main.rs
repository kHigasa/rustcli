extern crate getopts;
use getopts::Options;
use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Stdin;
use std::fs::File;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [OPTION]... [FILE]...", program);
    print!("{}", opts.usage(&brief));
}

fn do_cat(stream: &mut BufRead) -> () {
    let mut buffer = String::new();
    loop {
        match stream.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                print!("{}", buffer);
                buffer.clear();
            }
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }
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
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    if matches.free.is_empty() {
        let stdin: Stdin = std::io::stdin();
        let mut buf = BufReader::new(stdin);
        do_cat(&mut buf);
    } else {
        args[1..].iter().for_each(|file| {
            match File::open(file) {
                Ok(file) => {
                    let mut buf = BufReader::new(file);
                    do_cat(&mut buf);
                }
                Err(e) => println!("{}: {}", file, e),
            }
        })
    }
}

