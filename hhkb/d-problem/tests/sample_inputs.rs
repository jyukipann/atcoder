use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let input = r#"3 2
1 2
2 3
"#;
    let  ans = "Yes\n";
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
    let input = r#"3 3
1 2 3
2 3 1
"#;
    let  ans = "No\n";
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
    let input = r#"7 8
1 6 2 7 5 4 2 2
3 2 7 2 1 2 3 3
"#;
    let  ans = "Yes\n";
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), ans);
    assert!(output.stderr_str().is_empty());
}