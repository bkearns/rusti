use std::process::Command;

fn repl_run(args: &[&str]) -> String {
    let rusti = if cfg!(windows) { "target/debug/rusti.exe" } else { "target/debug/rusti" };

    match Command::new(rusti).args(args).env("HOME", "data").output() {
        Ok(out) => String::from_utf8(out.stdout).unwrap(),
        Err(e) => panic!("failed to spawn process: {}", e)
    }
}

fn repl_cmd(cmd: &str) -> String {
    repl_run(&["--no-rc", "-c", cmd])
}

fn repl_eval(code: &str) -> String {
    repl_run(&["--no-rc", "-e", code])
}

fn repl_file(path: &str) -> String {
    repl_run(&["--no-rc", path])
}

#[test]
fn test_eval() {
    assert_eq!(repl_eval(r#"println!("Hello, world!");"#), "Hello, world!\n");
    assert_eq!(repl_eval(r#"vec![1, 2, 3]"#), "[1, 2, 3]\n");
    assert_eq!(repl_eval("let a = 1; a"), "1\n");
    assert_eq!(repl_eval("fn foo() -> u32 { 2 } foo()"), "2\n");
    assert_eq!(repl_eval("fn foo() -> u32 { 3 }; foo()"), "3\n");
}

#[test]
fn test_file() {
    assert_eq!(repl_file("data/test_file.rs"), "foo\n123 = i32\nbar\n");
}

#[test]
fn test_print() {
    assert_eq!(repl_cmd(".print 1"), "1\n");
    assert_eq!(repl_cmd(r#".p "Hello!""#), "Hello!\n");
}

#[test]
fn test_rc() {
    assert_eq!(repl_run(&["-e", r#"println!("hi, rc!");"#]), "rc says hi\nhi, rc!\n");
}

#[test]
fn test_type() {
    assert_eq!(repl_cmd(".type 1"), "1 = i32\n");
    assert_eq!(repl_cmd(r#".t "hai2u""#), "\"hai2u\" = &'static str\n");
    assert_eq!(repl_cmd(":t &1"), "&1 = &i32\n");
    assert_eq!(repl_cmd(".t vec![1u32]"), "vec![1u32] = collections::vec::Vec<u32>\n");
}
