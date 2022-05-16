use assert_cmd::{cargo, Command};

type TestResult = Result<(), cargo::CargoError>;

fn run(args: &[&str], expected: &'static str) -> TestResult {
    Command::cargo_bin("echor")
        .map(|mut actual| actual.args(args).assert().success().stdout(expected))?;
    Ok(())
}

#[test]
fn hello0() -> TestResult {
    let expected = include_str!("expected/hello0.txt");
    run(&[""], expected)
}

#[test]
fn hello0_no_newline() -> TestResult {
    let expected = include_str!("expected/hello0.n.txt");
    run(&["-n", ""], expected)
}

#[test]
fn hello0_no_space_separators() -> TestResult {
    let expected = include_str!("expected/hello0.s.txt");
    run(&["-s", ""], expected)
}

#[test]
fn hello0_no_newline_no_space_separators() -> TestResult {
    let expected = include_str!("expected/hello0.n.s.txt");
    run(&["-n", "-s", ""], expected)
}

#[test]
fn hello1() -> TestResult {
    let expected = include_str!("expected/hello1.txt");
    run(&["Hello"], expected)
}

#[test]
fn hello1_no_newline() -> TestResult {
    let expected = include_str!("expected/hello1.n.txt");
    run(&["-n", "Hello"], expected)
}

#[test]
fn hello1_no_space_separators() -> TestResult {
    let expected = include_str!("expected/hello1.s.txt");
    run(&["-s", "Hello"], expected)
}

#[test]
fn hello1_no_newline_no_space_separators() -> TestResult {
    let expected = include_str!("expected/hello1.n.s.txt");
    run(&["-n", "-s", "Hello"], expected)
}

#[test]
fn hello2() -> TestResult {
    let expected = include_str!("expected/hello2.txt");
    run(&["Hello there"], expected)
}

#[test]
fn hello2_no_newline() -> TestResult {
    let expected = include_str!("expected/hello2.n.txt");
    run(&["-n", "Hello there"], expected)
}

#[test]
fn hello2_no_space_separators() -> TestResult {
    let expected = include_str!("expected/hello2.s.txt");
    run(&["-s", "Hello there"], expected)
}

#[test]
fn hello2_no_newline_no_space_separators() -> TestResult {
    let expected = include_str!("expected/hello2.n.s.txt");
    run(&["-n", "-s", "Hello there"], expected)
}

#[test]
fn hello3() -> TestResult {
    let expected = include_str!("expected/hello3.txt");
    run(&["Hello  there"], expected)
}

#[test]
fn hello3_no_newline() -> TestResult {
    let expected = include_str!("expected/hello3.n.txt");
    run(&["-n", "Hello  there"], expected)
}

#[test]
fn hello3_no_space_separators() -> TestResult {
    let expected = include_str!("expected/hello3.s.txt");
    run(&["-s", "Hello  there"], expected)
}

#[test]
fn hello3_no_newline_no_space_separators() -> TestResult {
    let expected = include_str!("expected/hello3.n.s.txt");
    run(&["-n", "-s", "Hello  there"], expected)
}

#[test]
fn hello4() -> TestResult {
    let expected = include_str!("expected/hello4.txt");
    run(&["Hello", "there"], expected)
}

#[test]
fn hello4_no_newline() -> TestResult {
    let expected = include_str!("expected/hello4.n.txt");
    run(&["-n", "Hello", "there"], expected)
}

#[test]
fn hello4_no_space_separators() -> TestResult {
    let expected = include_str!("expected/hello4.s.txt");
    run(&["-s", "Hello", "there"], expected)
}

#[test]
fn hello4_no_newline_no_space_separators() -> TestResult {
    let expected = include_str!("expected/hello4.n.s.txt");
    run(&["-n", "-s", "Hello", "there"], expected)
}
