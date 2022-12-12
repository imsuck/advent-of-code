pub fn get_input() -> String {
    format!(
        "src/bin/{}/input.txt",
        std::env::current_exe()
            .unwrap()
            .display()
            .to_string()
            .split('/')
            .last()
            .unwrap()
    )
}
