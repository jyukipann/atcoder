use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let input = r#"5 4 7
    3 1 4 9 7"#;
    let  ans = "";
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), ans);
    assert!(output.stderr_str().is_empty());
}

