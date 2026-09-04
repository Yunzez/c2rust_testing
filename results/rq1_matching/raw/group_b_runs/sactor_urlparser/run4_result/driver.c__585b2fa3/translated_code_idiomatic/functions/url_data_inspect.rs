pub fn url_data_inspect(data: &UrlData) {
    fn print_field(label: &str, value: &str) {
        println!("    .{}: \"{}\"", label, value);
    }
    println!("#url =>");
    print_field("href", &data.href);
    print_field("protocol", &data.protocol);
    print_field("host", &data.host);
    print_field("auth", &data.auth);
    print_field("hostname", &data.hostname);
    print_field("pathname", &data.pathname);
    print_field("search", &data.search);
    print_field("path", &data.path);
    print_field("hash", &data.hash);
    print_field("query", &data.query);
    print_field("port", &data.port);
}
