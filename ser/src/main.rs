use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process;

use regex::{Captures, Regex};

const PROG_NAME: &str = "ser";

fn usage() {
    println!("USAGE: {} [-h, --help] [FILE]", PROG_NAME);
}

fn exit_err(output: &str) -> ! {
    eprintln!("{}", output);
    process::exit(1)
}

fn main() {
    let mut args = env::args_os();
    args.next();

    let mut buffer = String::new();
    let get_from_stdin = |buffer: &mut String| {
        if let Err(_) = io::stdin().read_to_string(buffer) {
            exit_err("unable to read from stdin!");
        }
    };

    if let Some(arg_os) = args.next() {
        if let Ok(arg) = arg_os.into_string() {
            if &arg == "-h" || &arg == "--help" {
                usage();
                return;
            } else if &arg == "-" {
                get_from_stdin(&mut buffer);
            } else {
                if let Ok(mut handle) = File::open(&arg) {
                    if let Err(_) = handle.read_to_string(&mut buffer) {
                        exit_err("unable to read from file!");
                    }
                } else {
                    exit_err("unable to open file!");
                }
            }
        } else {
            exit_err("unable to parse input!");
        }
    } else {
        get_from_stdin(&mut buffer);
    }
    let buffer = buffer;

    let environment = {
        let mut environment = HashMap::new();
        for (label, value) in env::vars() {
            environment.insert(label, value);
        }
        environment
    };

    let r = Regex::new(r"(~+)#([a-zA-Z-_]+)#(~+)").unwrap();

    let repeat = |c: char, n: usize| -> String {
        std::iter::repeat(c).take(n).collect()
    };

    let mut error = None;
    let output = r.replace_all(&buffer, |cap: &Captures| -> String {
        if cap[1].len() == cap[3].len() {
            if cap[1].len() % 2 == 0 {
                let n = cap[1].len() / 2;
                format!("{}#{}#{}",
                        repeat('~', n),
                        &cap[2],
                        repeat('~', n))
            } else {
                let n = (cap[1].len() - 1) / 2;
                if let Some(value) = environment.get(&cap[2]) {
                    format!("{}{}{}",
                            repeat('~', n),
                            value,
                            repeat('~', n))
                } else {
                    error = Some(
                        format!("environment variable '{}' does not exist!",
                                &cap[2]));
                    "".to_string()
                }
            }
        } else {
            error = Some("mismatched formatting!".to_string());
            "".to_string()
        }
    });

    if let Some(err) = error {
        exit_err(&err);
    }

    print!("{}", output);
}
