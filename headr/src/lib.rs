use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::str::from_utf8;

// head
// display the first lines of a file.
// defaults to the first 10.
// takes multiple options, multiple files.
// no file or '-' is stdin.
// if multiple files, precede with a header giving the file name.
//
// -c, --bytes[-]NUM :  print the first NUM bytes of each file; if leading '-', print all but the
// last NUM bytes of each file
//
// -n, --lines=[-]NUM :  print the first NUM lines; if leading, all but last NUM lines
//
// -q, --quiet, --silent :  don't print headers with file names
//
// -v, --verbose :  always print headers with file names
//
// -z, --zero-terminated :  lines are deliniated by 'NUL', instead of '\n'
//
// header looks like "===> FILENAME <==="

pub struct Config {
    output_kind: HeadKind,
    output_size: usize,
    output_sign: Sign,
    print_headers: bool,
    files: Vec<Option<String>>,
}

enum HeadKind {
    Bytes,
    Lines,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Debug)]
enum Sign {
    Negative,
    Zero,
    Positive,
}

pub fn get_args() -> Result<Config, Box<dyn Error>> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("BeniiAlCo")
        .about("A rust port of head -- a tool that prints a given number of lines/bytes from the front of a file. The default is 10 lines. With no FILE, or when FILE is -, read standard input.")
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .takes_value(true)
                .allow_hyphen_values(true)
                .value_name("[-]NUM")
                .validator(|s| {if s.as_bytes()[0] == b'-' {s[1..].parse::<usize>()} else {s.parse::<usize>()}})
                .conflicts_with("lines")
                .help("Print the first NUM bytes of each file;\n\tWith the leading '-', print all but the last NUM bytes of each file.")
                .display_order(0))
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .takes_value(true)
                .allow_hyphen_values(true)
                .value_name("[-]NUM")
                .default_value("10")
                .validator(|s| {if s.as_bytes()[0] == b'-' {s[1..].parse::<usize>()} else {s.parse::<usize>()}})
                .conflicts_with("bytes")
                .display_order(1))
        .arg(
            Arg::new("quiet")
                .short('q')
                .visible_alias("silent")
                .long("quiet")
                .conflicts_with("verbose")
                .display_order(2))
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .conflicts_with("quiet")
                .display_order(3))
        .arg(
            Arg::new("zero-terminated")
                .short('z')
                .long("zero-terminated")
                .display_order(4))
        .arg(
            Arg::new("file")
                .takes_value(true)
                .multiple_values(true)
                .value_name("FILE")
                .default_value("-")
                .hide_default_value(true))
        .get_matches();

    let (output_kind, output_size, output_sign) = {
        let (output_kind, output) = if matches.is_present("bytes") {
            (HeadKind::Bytes, matches.value_of("bytes"))
        } else {
            (HeadKind::Lines, matches.value_of("lines"))
        };

        match output.unwrap() {
            zero if (zero.as_bytes()[0] == b'-' && zero[1..].parse::<usize>()?.eq(&0))
                || (zero.as_bytes()[0] != b'-' && zero.parse::<usize>()?.eq(&0)) =>
            {
                (output_kind, 0, Sign::Zero)
            }

            negative if negative.as_bytes()[0] == b'-' => {
                (output_kind, negative[1..].parse::<usize>()?, Sign::Negative)
            }

            positive => (output_kind, positive.parse::<usize>()?, Sign::Positive),
        }
    };

    Ok(Config {
        output_kind,
        output_size,
        output_sign,
        print_headers: {
            matches.is_present("verbose")
                || (matches.occurrences_of("file") > 1 && !matches.is_present("quiet"))
        },
        files: {
            matches
                .values_of("file")
                .unwrap()
                .map(|file| {
                    if file == "-" {
                        None
                    } else {
                        Some(file.to_string())
                    }
                })
                .collect()
        },
    })
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut filenames = config.files.into_iter().peekable();
    while let Some(filename) = filenames.next() {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {e}", filename.unwrap_or_default()),
            Ok(mut file) => {
                if config.print_headers {
                    println!("==> {} <==", filename.unwrap_or_default())
                }
                match config.output_sign {
                    Sign::Zero => continue,
                    Sign::Positive => match config.output_kind {
                        HeadKind::Bytes => {
                            let mut buffer = Vec::new();
                            file.take(config.output_size.try_into().unwrap())
                                .read_to_end(&mut buffer)?;
                            print!("{}", from_utf8(&buffer).unwrap_or_default());
                        }
                        HeadKind::Lines => {
                            for line in file.lines().take(config.output_size) {
                                println!("{}", line?);
                            }
                        }
                    },
                    Sign::Negative => {
                        match config.output_kind {
                            HeadKind::Bytes => {
                                let mut buffer = Vec::new();

                                // TODO: there's gotta be a way to do this without assigning to a
                                // vector? using the seek trait perhaps?

                                file.read_to_end(&mut buffer)?;
                                buffer.truncate(buffer.len().saturating_sub(config.output_size));

                                print!("{}", from_utf8(&buffer).unwrap_or_default());
                            }
                            HeadKind::Lines => {
                                let lines = file
                                    .lines()
                                    .map(|line| line.unwrap())
                                    .collect::<Vec<String>>();
                                let number_of_lines = lines.len();
                                for line in lines
                                    .into_iter()
                                    .take(number_of_lines.saturating_sub(config.output_size))
                                {
                                    println!("{}", line);
                                }
                            }
                        }
                    }
                };
                if config.print_headers && filenames.peek().is_some() {
                    println!()
                }
            }
        }
    }

    //    for filename in config.files {
    //        match open(&filename) {
    //            Err(e) => eprintln!("Failed to open {}: {e}", filename.unwrap_or_default()),
    //            Ok(mut file) => {
    //print!("==> {} <==", filename.unwrap_or_default());
    // NOTE: I should check whether or not headings will be used at the config argument
    // parsing stage, not here! perhaps also checking whether the supplied c/n number
    // is negative too, to avoid all of the abs().try_into().unwrap()'s ?
    //                match config.kind {
    //                    HeadKind::Bytes(num) => {
    //                        let mut buffer = Vec::new();
    //                        match num.cmp(&0) {
    //                            std::cmp::Ordering::Less => {
    //                                file.read_to_end(&mut buffer)?;
    //                                buffer = buffer
    //                                    .into_iter()
    //                                    .rev()
    //                                    .skip(num.abs().try_into().unwrap())
    //                                    .rev()
    //                                    .collect();
    //                            }
    //                            std::cmp::Ordering::Equal => {
    //                                println!();
    //                                continue;
    //                            }
    //                            std::cmp::Ordering::Greater => {
    //                                file.take(num.try_into().unwrap())
    //                                    .read_to_end(&mut buffer)?;
    //                            }
    //                        }
    //                        print!("{}", from_utf8(&buffer).unwrap_or_default());
    //                    }
    //                    HeadKind::Lines(num) => {
    //                        for line in file.lines().take(num.try_into().unwrap()) {
    //                            println!("{}", line?);
    //                        }
    //                    }
    //                }
    //            }
    //        }
    //    }
    Ok(())
}

pub fn open(filename: &Option<String>) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        None => Ok(Box::new(BufReader::new(io::stdin().lock()))),
        Some(filename) => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
