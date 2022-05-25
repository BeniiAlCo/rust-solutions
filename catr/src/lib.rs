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

        for l in "o\nb\no\n".lines() {
            println!("{l}");
        }

        for l in self
            .input
            .iter()
            .flat_map(|x| Config::open(&x).unwrap().lines())
        {
            if let Ok(line) = l {
                println!("{line}");
            }
        }

        for filename in self.input {
            match Config::open(&filename) {
                Err(err) => eprintln!("Failed to open {filename}: {err}"),
                Ok(output) => {
                    for line in output.lines().enumerate() {
                        if let (i, Ok(line)) = line {
                            let line_number = match self.line_numbers {
                                LineNumbers::Include => format!("     {i}  "),
                                LineNumbers::OnlyNonEmpty => {
                                    if !line.is_empty() {
                                        format!("     {i}  ")
                                    } else {
                                        "".to_string()
                                    }
                                }
                                _ => "".to_string(),
                            };

                            print!("{line_number}{line}");

                            if self.show_ends {
                                print!("$");
                            }
                            print!("\n");
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
        // TODO: other implementations of stdin that I have seen use stdin.lock() -- why is this? (I
        // assume that it has to do with adding in multithreading support later on, which this book
        // just isn't going to cover right now?)
        match filename {
            "-" => Ok(Box::new(BufReader::new(io::stdin()))),
            _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
        }
    }
}
