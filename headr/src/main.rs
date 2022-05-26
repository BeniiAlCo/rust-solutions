use std::process::ExitCode;

fn main() -> ExitCode {
    if let Err(e) = headr::get_args().and_then(headr::run) {
        eprintln!("{e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
