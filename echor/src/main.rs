use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("BeniiAlCo")
        .about("A Rust port of the command line tool 'echo'.")
        .arg(
            Arg::new("no_newline")
                .short('n')
                .help("Do not output a newline.")
                .display_order(0),
        )
        .arg(
            Arg::new("no_space_seperated_arguments")
                .short('s')
                .help("Do not seperate arguments with spaces")
                .display_order(1),
        )
        .arg(
            Arg::new("enable_backslash_escapes")
                .short('e')
                .help("Enable interpretation of backslash escapes.")
                .conflicts_with("disable_backslash_escapes")
                .display_order(3),
        )
        .arg(
            Arg::new("disable_backslash_escapes")
                .short('E')
                .help("Disable interpretation of backslash escapes (default).")
                .conflicts_with("enable_backslash_escapes")
                .display_order(2),
        )
        .arg(
            Arg::new("string")
                .takes_value(true)
                .value_name("STRING")
                .multiple_values(true),
        )
        .get_matches();

    let (opt, output) = Opt::parse_arguments(&matches);
    opt.print_string(output);
}

struct Opt {
    newline: bool,
    space_seperated_arguments: bool,
    _backslash_escapes: bool,
}

impl Opt {
    fn parse_arguments(args: &clap::ArgMatches) -> (Self, Vec<&str>) {
        (
            Opt {
                newline: !args.is_present("no_newline"),
                space_seperated_arguments: !args.is_present("no_space_seperated_arguments"),
                _backslash_escapes: false,
            },
            args.values_of("string").unwrap_or_default().collect(),
        )
    }

    fn print_string(&self, output: Vec<&str>) {
        // TODO: Implement backsash escape functionality, as per the behavior of echo in fish.
        let line_end = if self.newline { "\n" } else { "" };

        if self.space_seperated_arguments {
            print!("{}{}", &output.join(" "), line_end);
        } else {
            print!("{}{}", &output.concat(), line_end);
        };
    }
}
