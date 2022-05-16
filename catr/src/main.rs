use clap::{Arg, Command};

fn main() {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("BeniiAlCo")
        .about("rust port of (a subset of) cat")
        //bEns TODO: implement -A, -e, -t, -T, -u, -v
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
                .multiple_occurrences(true),
        )
        .get_matches();
}

enum LineNumbers {
    None,
    All,
    OnlyNonEmpty,
}

struct Config<'a> {
    show_ends: bool,
    line_numbers: LineNumbers,
    squeeze_blank: bool,
    input: Vec<&'a str>,
}

impl<'a> Config<'a> {
    fn new(args: &'a clap::ArgMatches) -> Self {
        Config {
            show_ends: args.is_present("show_ends"),
            line_numbers: {
                match args.is_present("number_nonblank") {
                    true => LineNumbers::OnlyNonEmpty,
                    false if args.is_present("number") => LineNumbers::All,
                    _ => LineNumbers::None,
                }
            },
            squeeze_blank: args.is_present("squeeze_blank"),
            input: args
                .values_of("input")
                .unwrap_or_default()
                .collect::<Vec<_>>(),
        }
    }
}
