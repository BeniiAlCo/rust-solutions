use clap::{Arg, Command};
use std::error::Error;

#[derive(Debug)]
enum LineNumbers {
    Omit,
    Include,
    OnlyNonEmpty,
}

#[derive(Debug)]
pub struct Config {
    show_ends: bool,
    line_numbers: LineNumbers,
    squeeze_blank: bool,
    input: Vec<String>,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let matches = Command::new("catr")
            .version("0.1.0")
            .author("BeniiAlCo")
            .about("rust port of (a subset of) cat")
            //-bEns TODO: implement -A, -e, -t, -T, -u, -v
            .arg(
                Arg::new("number_nonblank")
                    .short('b')
                    .long("number-nonblank")
                    .help("number non empty output lines, overrides -n")
                    .multiple_occurrences(true)
                    .display_order(1),
            )
            .arg(
                Arg::new("show_ends")
                    .short('E')
                    .long("show-ends")
                    .help("display $ at end of each line")
                    .multiple_occurrences(true)
                    .display_order(2),
            )
            .arg(
                Arg::new("number")
                    .short('n')
                    .long("number")
                    .help("number all output lines")
                    .multiple_occurrences(true)
                    .display_order(3),
            )
            .arg(
                Arg::new("squeeze_blank")
                    .short('s')
                    .long("squeeze-blank")
                    .help("suppress repeated empty output lines")
                    .multiple_occurrences(true)
                    .display_order(4),
            )
            .arg(
                Arg::new("input")
                    .takes_value(true)
                    .value_name("FILE")
                    .multiple_occurrences(true)
                    .default_value("-")
                    .hide_default_value(true),
            )
            .get_matches();

        Ok(Config {
            show_ends: matches.is_present("show_ends"),
            line_numbers: {
                match matches.is_present("number_nonblank") {
                    true => LineNumbers::OnlyNonEmpty,
                    false if matches.is_present("number") => LineNumbers::Include,
                    _ => LineNumbers::Omit,
                }
            },
            squeeze_blank: matches.is_present("squeeze_blank"),
            input: matches
                .values_of("input")
                .unwrap()
                .map(String::from)
                .collect(),
        })
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        dbg!(self);
        Ok(())
    }
}
