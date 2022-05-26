use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
        if self.squeeze_blank {}
        match self.line_numbers {
            _ => {}
        }

        let mut line_number = 0;

        for filename in self.input {
            match Config::open_file(&filename) {
                Err(e) => eprintln!("Failed to open {filename}: {e}"),
                Ok(o) => {
                    for line in o.lines() {
                        let middle = line?;

                        // TODO: This uses an additional string allocation to use the format macro
                        // -- I need to check how the println macro works to see if this truly
                        // matters, or if it merely pre-emts an allocation anyway.
                        // If it need not use an allocation, I should see if I can generate the
                        // line number in another way; If it will allocate to print, I should
                        // define the expected length of the string here to save on having to
                        // expand it when concatinating the line parts and printing.
                        let start = match self.line_numbers {
                            LineNumbers::Omit => format!(""),
                            LineNumbers::Include => {
                                line_number = line_number + 1;
                                format!("{:>6}\t", line_number)
                            }
                            LineNumbers::OnlyNonEmpty => {
                                if !middle.is_empty() {
                                    line_number = line_number + 1;
                                    format!("{:>6}\t", line_number)
                                } else {
                                    format!("")
                                }
                            }
                        };

                        let end = if self.show_ends { "$" } else { "" };

                        println!("{start}{middle}{end}");
                    }
                }
            }
        }

        Ok(())
    }

    fn open_file(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
        // TODO: other implementations of stdin that I have seen use stdin.lock() -- why is this? (I
        // assume that it has to do with adding in multithreading support later on, which this book
        // just isn't going to cover right now?)
        match filename {
            "-" => Ok(Box::new(BufReader::new(io::stdin()))),
            _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
        }
    }
}
