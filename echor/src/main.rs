use clap::{Arg, Command};

fn main() {
    let _matches = Command::new("echor")
        .version("0.1.0")
        .author("Benii")
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
        .arg(Arg::new("string").takes_value(true).value_name("STRING"))
        .get_matches();
}
