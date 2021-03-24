use unwrap::unwrap;

pub fn replace_exit_type_and_exited_cleanly(path: &str) {
    println!("File to replace exit_type and exited_cleanly: {}", path);
    let text = unwrap!(std::fs::read_to_string(path));
    let text = text.replace(r#""exit_type":"Crashed""#, r#""exit_type":"Normal""#);
    let text = text.replace(r#""exited_cleanly":false"#, r#""exited_cleanly":true"#);
    unwrap!(std::fs::write(path, &text));
    println!("Done! You can now start Chrome without the annoying Restore dialog.");
}
