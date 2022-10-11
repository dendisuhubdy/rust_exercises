use std::process::Command;

#[test]
fn works() {
    assert!(true);
}

#[test]
fn runs() {
    let mut cmd = Command::new("./target/debug/integration_testing");
    let res = cmd.output();
    assert!(res.is_ok());
}
