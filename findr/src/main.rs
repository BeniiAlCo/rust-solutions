use std::process::ExitCode;

fn main() -> ExitCode {
    if let Err(err) = findr::get_args().and_then(findr::run) {
        eprintln!("{err}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
