use clap::{Arg, ArgGroup, Command};
use core::ops::Add;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    bytes: bool,
    chars: bool,
    lines: bool,
    words: bool,
}

impl Config {
    fn default() -> Self {
        Config {
            bytes: true,
            chars: false,
            lines: true,
            words: true,
        }
    }
}

type Filenames = Vec<Option<PathBuf>>;

pub fn get_args() -> Result<(Config, Filenames), Box<dyn Error>> {
    let matches = Command::new("wcr")
        .version("0.1.0")
        .author("BeniiAlCo")
        .about("A rust port of wc -- a tool that prints the newline, word, and byte counts for each FILE specified (and a total line  if more that one FILE is specified.\n\tA word is specified as a non-zero-length sequence of characters delimited by whitespace.\n\nWith no FILE, or when FILE is -, read standard input.)\n\nThe options below may be used to select which counts are printed, always in the following order: newline, word, character, byte, maximum line length.\n\tThe default output is equivalent to -lwc.")
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .help("print the byte counts")
                .display_order(0)
                .multiple_occurrences(false))
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .help("print the character counts")
                .display_order(1)
                .multiple_occurrences(false))
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .help("print the newline counts")
                .display_order(2)
                .multiple_occurrences(false))
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .help("print the word counts")
                .display_order(4)
                .multiple_occurrences(false))
        .arg(
            Arg::new("files")
                .takes_value(true)
                .value_name("FILE")
                .multiple_values(true)
                .default_value("-")
                .hide_default_value(true))
        .group(
            ArgGroup::new("selected_output")
                .args(&["bytes", "chars", "lines", "words"])
                .multiple(true))
        .get_matches();

    let config = if matches.is_present("selected_output") {
        Config {
            bytes: matches.is_present("bytes"),
            chars: matches.is_present("chars"),
            lines: matches.is_present("lines"),
            words: matches.is_present("words"),
        }
    } else {
        Config::default()
    };
    let filenames = matches
        .values_of("files")
        .unwrap()
        .map(|filename| {
            if filename == "-" {
                None
            } else {
                Some(PathBuf::from(filename))
            }
        })
        .collect();

    Ok((config, filenames))
}

#[derive(Clone, Copy)]
struct Counter {
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize,
}

impl Counter {
    fn new() -> Self {
        Counter {
            lines: usize::default(),
            words: usize::default(),
            chars: usize::default(),
            bytes: usize::default(),
        }
    }

    fn cumulate_counter_values(&mut self, input: &str) {
        self.lines += 1;
        self.words += input.split_whitespace().count();
        self.chars += input.chars().count();
        self.bytes += input.len();
    }

    fn longest_count(&self) -> usize {
        [
            self.lines.to_string().len(),
            self.words.to_string().len(),
            self.chars.to_string().len(),
            self.bytes.to_string().len(),
        ]
        .into_iter()
        .max()
        .unwrap_or_default()
    }
}

impl Add for Counter {
    type Output = Counter;
    fn add(self, rhs: Counter) -> Self::Output {
        Counter {
            lines: self.lines + rhs.lines,
            words: self.words + rhs.words,
            chars: self.chars + rhs.chars,
            bytes: self.bytes + rhs.bytes,
        }
    }
}

type FileData = Result<Counter, Box<dyn Error>>;
type FileName = String;

struct CounterMetadata {
    total: Option<Counter>,
    file_tally: Vec<(FileName, FileData)>,
    longest_count: usize,
}

impl CounterMetadata {
    fn new_with_total() -> Self {
        CounterMetadata {
            total: Some(Counter::new()),
            file_tally: Vec::new(),
            longest_count: usize::default(),
        }
    }

    fn new_without_total() -> Self {
        CounterMetadata {
            total: None,
            file_tally: Vec::new(),
            longest_count: usize::default(),
        }
    }

    fn add_error_entry(&mut self, filename: String, err: Box<dyn Error>) {
        self.file_tally.push((filename, Err(err)));
    }

    fn add_entry(&mut self, file_counter: Counter, filename: String) {
        if self.total.is_some() {
            self.total = Some(self.total.unwrap() + file_counter)
        }

        self.file_tally.push((filename, Ok(file_counter)));

        let current_longest_count = file_counter.longest_count();
        if current_longest_count > self.longest_count {
            self.longest_count = current_longest_count;
        }
    }

    fn display(&mut self, show_lines: bool, show_words: bool, show_chars: bool, show_bytes: bool) {
        let width = self.longest_count + 1;

        //dbg!(width);
        if let Some(total) = self.total {
            self.file_tally.push(("total".to_string(), Ok(total)))
        }

        for file in &self.file_tally {
            match file {
                (name, Err(err)) => eprintln!("{name}: {err}"),
                (name, Ok(file)) => {
                    let lines = if show_lines {
                        format!("{:>width$}", file.lines, width = width - 1)
                    } else {
                        "".to_string()
                    };
                    let words = if show_words && !show_lines {
                        format!("{:>width$}", file.words, width = width - 1)
                    } else if show_words {
                        format!("{:>width$}", file.words)
                    } else {
                        "".to_string()
                    };
                    let chars = if show_chars && (!show_lines && !show_words) {
                        format!("{:>width$}", file.chars, width = width - 1)
                    } else if show_chars {
                        format!("{:>width$}", file.chars)
                    } else {
                        "".to_string()
                    };
                    let bytes = if show_bytes && (!show_lines && !show_words && !show_chars) {
                        format!("{:>width$}", file.bytes, width = width - 1)
                    } else if show_bytes {
                        format!("{:>width$}", file.bytes)
                    } else {
                        "".to_string()
                    };

                    println!("{}{}{}{} {name}", lines, words, chars, bytes,)
                }
            }
        }
    }
}

pub fn run((config, filenames): (Config, Filenames)) -> Result<(), Box<dyn Error>> {
    let mut meta_counter = if filenames.len() > 1 {
        CounterMetadata::new_with_total()
    } else {
        CounterMetadata::new_without_total()
    };

    for file in filenames {
        let name = file
            .as_ref()
            .map(|file| file.to_string_lossy().to_string())
            .unwrap_or_else(|| "-".to_string());
        let mut line_buffer = String::new();

        match open(&file) {
            Err(err) => meta_counter.add_error_entry(name, err),
            Ok(mut file) => {
                let mut file_counter = Counter::new();

                while let Ok(bytes) = file.read_line(&mut line_buffer) {
                    if bytes == 0 {
                        break;
                    }

                    file_counter.cumulate_counter_values(&line_buffer);

                    line_buffer.clear();
                }

                meta_counter.add_entry(file_counter, name);
            }
        }
    }

    meta_counter.display(config.lines, config.words, config.chars, config.bytes);
    Ok(())
}

fn open(filename: &Option<PathBuf>) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        None => Ok(Box::new(BufReader::new(io::stdin().lock()))),
        Some(filename) => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
