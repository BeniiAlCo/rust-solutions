use std::process::ExitCode;

fn main() -> ExitCode {
    // Word Count Port.
    // Displays number of lines; number of words; number of bytes from stdin/files

    if let Err(e) = wcr::get_args().and_then(wcr::run) {
        eprintln!("{e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
