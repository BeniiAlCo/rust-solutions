use assert_cmd::{prelude::*, Command};

#[test]
fn works() {
    assert!(true.eq(&true));
}

#[test]
#[should_panic]
fn fails() {
    assert!(true.eq(&false));
}

#[test]
fn runs() {
    let cmd = Command::new("exa").unwrap();
    cmd.assert().success();
}

// This only tests whether the program terminated correctly.
// That is, it tests that the syntax of the program was correct, allowing rustc to compile it.
#[test]
fn runs_hello() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}

// We want to test the semantics of the program too though.
// We want to make sure it behaves as we intend it to, rather than just that it behaves in one possible way that a rust program is allowed to behave.
// This program only outputs to STDOUT, so that is all of the functionality we can test.
#[test]
fn prints_hello() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

// This tests for failure with any reported error code
#[test]
fn false_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

// We can also check for specific error codes, to ensure the correct code is emitted under the correct circumstances (note, we could also do this to check the program runs correctly, but as the program running & terminating correctly, and the program emitting the error code 0 only will always coincide, there is no difference between the two).
#[test]
fn false_emits_1() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().code(1);
}

#[test]
fn false_2_ok() {
    let mut cmd = Command::cargo_bin("false_2").unwrap();
    cmd.assert().failure();
}

// false_2 aborts rather than emitting an error code -- to the terminal, this looks like an error telling it that the program has been interrupted before it had the chance to emit a code.
#[test]
fn false_2_emits_sigabrt() {
    let mut cmd = Command::cargo_bin("false_2").unwrap();
    cmd.assert().interrupted();
}
