use std::process::ExitCode;

#[allow(dead_code)]
fn main() -> ExitCode {
    if let Err(err) = uniqr::get_args().and_then(uniqr::run) {
        eprintln!("{err}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
