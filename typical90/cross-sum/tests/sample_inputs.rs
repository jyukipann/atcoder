use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let input = r#"3 3
1 1 1
1 1 1
1 1 1"#;
    let  ans = r#"5 5 5
5 5 5
5 5 5"#;
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), ans);
    assert!(output.stderr_str().is_empty());
}

