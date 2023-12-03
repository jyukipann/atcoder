use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let input = r#"12 30
    2023 12 30"#;
    let  ans = "2024 1 1";
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), ans);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let input = r#"36 72
    6789 23 45
    "#;
    let  ans = "6789 23 46";
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), ans);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let input = r#"12 30
    2012 6 20    
    "#;
    let  ans = "2012 6 21";
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), ans);
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let input = r#"12 30
    2012 12 30 
    "#;
    let  ans = "2013 1 1";
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), ans);
    assert!(output.stderr_str().is_empty());
}