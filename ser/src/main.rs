use std::collections::HashMap;
use std::env;

use regex::{Captures, Regex};

const PROG_NAME: &str = "ser";

fn usage() {
    println!("USAGE: {} [-h, --help] <INPUT>", PROG_NAME);
}

fn main() {
    let mut args = env::args();
    args.next();

    let input = if let Some(arg) = args.next() {
        if &arg == "-h" || &arg == "--help" {
            usage();
            return;
        } else {
            arg
        }
    } else {
        eprintln!("no input provided!");
        return;
    };

    let environment = {
        let mut environment = HashMap::new();
        for (label, value) in env::vars() {
            environment.insert(label, value);
        }
        environment
    };

    let r = Regex::new(r"(\{+)([a-zA-Z0-9-_]+)(\}+)").unwrap();

    let repeat = |c: char, n: usize| -> String {
        std::iter::repeat(c).take(n).collect()
    };

    let mut error = None;
    let output = r.replace_all(&input, |cap: &Captures| -> String {
        if cap[1].len() == cap[3].len() {
            if cap[1].len() % 2 == 0 {
                let n = cap[1].len() / 2;
                format!("{}{}{}",
                        repeat('{', n),
                        &cap[2],
                        repeat('}', n))
            } else {
                let n = (cap[1].len() - 1) / 2;
                if let Some(value) = environment.get(&cap[2]) {
                    format!("{}{}{}",
                            repeat('{', n),
                            value,
                            repeat('}', n))
                } else {
                    error = Some(
                        format!("environment variable '{}' does not exist!",
                                &cap[2]));
                    "".to_string()
                }
            }
        } else {
            error = Some("mismatching braces!".to_string());
            "".to_string()
        }
    });

    if let Some(err) = error {
        eprintln!("{}", err);
        return;
    }

    println!("{}", output);
}
