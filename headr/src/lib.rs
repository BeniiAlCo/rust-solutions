use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::io::{Seek, SeekFrom};
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
    kind: HeadKind,
    print_headers: bool,
    files: Vec<Option<String>>,
}

#[derive(Clone, Copy)]
enum HeadKind {
    Bytes(i64),
    Lines(i64),
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
                .validator(|s| s.parse::<i64>())
                .conflicts_with("lines")
                .help("Print the first NUM bytes of each file;\n\tWith the leading '-', print all but the last NUM bytes of each file.")
                .display_order(0))
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .takes_value(true)
                .value_name("[-]NUM")
                .default_value("10")
                .validator(|s| s.parse::<i64>())
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

    Ok(Config {
        kind: if matches.is_present("bytes") {
            HeadKind::Bytes(matches.value_of_t("bytes")?)
        } else {
            HeadKind::Lines(matches.value_of_t("lines")?)
        },
        print_headers: !matches.is_present("quiet"),
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
    if config.print_headers {}
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {e}", filename.unwrap_or_default()),
            Ok(mut file) => {
                //print!("==> {} <==", filename.unwrap_or_default());
                match config.kind {
                    HeadKind::Bytes(num) => {
                        let mut buffer = Vec::new();
                        match num.cmp(&0) {
                            std::cmp::Ordering::Less => {
                                file.read_to_end(&mut buffer)?;
                                buffer = buffer
                                    .into_iter()
                                    .rev()
                                    .skip(num.abs().try_into().unwrap())
                                    .rev()
                                    .collect();
                            }
                            std::cmp::Ordering::Equal => {
                                println!();
                                continue;
                            }
                            std::cmp::Ordering::Greater => {
                                file.take(num.try_into().unwrap())
                                    .read_to_end(&mut buffer)?;
                            }
                        }
                        print!("{}", from_utf8(&buffer).unwrap_or_default());
                    }
                    HeadKind::Lines(num) => {
                        for line in file.lines().take(num.try_into().unwrap()) {
                            println!("{}", line?);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn open(filename: &Option<String>) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        None => Ok(Box::new(BufReader::new(io::stdin().lock()))),
        Some(filename) => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
