fn main() {
    if let Err(e) = catr::Config::new().and_then(catr::Config::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
