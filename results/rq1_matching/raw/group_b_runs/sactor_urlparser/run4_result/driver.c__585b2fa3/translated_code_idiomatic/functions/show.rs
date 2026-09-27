pub fn show(label: &str, v: Option<String>) {
    fn print_line(label: &str, value: Option<&str>) {
        match value {
            Some(text) => println!("{label}: {text}"),
            None => println!("{label}: (null)"),
        }
    }
    print_line(label, v.as_deref());
}
