extern crate getopts;

use getopts::Options;
use std::env;
use std::fs::{create_dir, create_dir_all, set_permissions, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {}  [OPTION]... DIRECTORY...", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("m", "", "set mode");
    opts.optflag("p", "", "withouts parents dirs, creates them recursively");
    opts.optflag("h", "", "print help");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let dir = if !matches.free.is_empty() {
        if matches.opt_present("m") {
            matches.free[1].clone()
        } else {
            matches.free[0].clone()
        }
    } else {
        print_usage(&program, opts);
        return;
    };
    let path = Path::new(&dir);
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    } else if matches.opt_present("m") {
        let mode = u32::from_str_radix(&matches.free[0].clone(), 10).unwrap();
        let perm = Permissions::from_mode(mode);
        match create_dir(&path) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };
        match set_permissions(&path, perm) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };
    } else if matches.opt_present("p") {
        match create_dir_all(&path) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };
    } else {
        print_usage(&program, opts);
        return;
    };
}

