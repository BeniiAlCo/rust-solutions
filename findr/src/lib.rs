use clap::{Arg, Command};
use regex::Regex;
use std::error::Error;
use walkdir::WalkDir;

// Findr
// Rust Port of Find.
// Finds entries in one or more Paths; these entries can be filtered by files, links, directories,
// or names that match an optional pattern.
//
// -Findr must have one or more positional arguments that indicate tho paths to search.
// -For each path, Findr will recursively search for all files and directories found therein.
// E.G. if I am in the tests/inputs directory, and indicate '.', Findr will list all the contents.
// -Use the '-type' option to specify the type of output to be displayed:
// f = files
// l = links
// d = directories
//-Use the '-name' option to locate items matching a file glob pattern.
//E.G. -name *.csv will find all entries ending in .csv
//must be escaped or put in quotes: '-name \*.csv' or '-name "*.csv"'
//-Use the '-o' option to or together multiple options.
//-Error if path does not exist
//-Print filename if path exists

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

pub struct Config {
    names: Vec<Regex>,
    entry_types: Option<Vec<EntryType>>,
    paths: Vec<String>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("findr")
        .version("0.1.0")
        .author("BeniiAlCo")
        .about("A Rust port of find")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .takes_value(true)
                .value_name("NAME")
                .multiple_values(true)
                .help("Name")
                .default_value("")
                .validator(|n| n.parse::<Regex>())
                .hide_default_value(true)
                .display_order(0),
        )
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .takes_value(true)
                .value_name("TYPE")
                .multiple_values(true)
                .help("Entry type")
                .possible_values(["f", "d", "l"])
                .display_order(1),
        )
        .arg(
            Arg::new("path")
                .takes_value(true)
                .value_name("PATH")
                .multiple_values(true)
                .help("search paths")
                .default_value("."),
        )
        .get_matches();

    Ok(Config {
        names: matches.values_of_t("name")?,
        entry_types: if matches.is_present("type") {
            Some(
                matches
                    .values_of("type")
                    .unwrap()
                    .into_iter()
                    .map(|entry_type| match entry_type {
                        "f" => EntryType::File,
                        "d" => EntryType::Dir,
                        "l" => EntryType::Link,
                        _ => unreachable!("Impossible Type"),
                    })
                    .collect(),
            )
        } else {
            None
        },
        paths: matches.values_of_t("path")?,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(err) => eprintln!("{err}"),
                Ok(entry) => {
                    let entry_type_file = |entry: walkdir::DirEntry| entry.file_type().is_file();
                    let entry_type_link = |entry: walkdir::DirEntry| entry.file_type().is_symlink();
                    let entry_type_dir = |entry: walkdir::DirEntry| entry.file_type().is_dir();

                    if let Some(ref entry_type) = config.entry_types {
                        if entry_type
                            .iter()
                            .map(|entry_type| match entry_type {
                                EntryType::File => entry_type_file,
                                EntryType::Link => entry_type_link,
                                EntryType::Dir => entry_type_dir,
                            })
                            .all(|test| !test(entry.clone()))
                        {
                            continue;
                        }
                    }

                    if config
                        .names
                        .clone()
                        .into_iter()
                        .any(|re| re.is_match(entry.file_name().to_str().unwrap_or_default()))
                    {
                        println!("{}", entry.path().display())
                    }
                }
            }
        }
    }

    Ok(())
}
