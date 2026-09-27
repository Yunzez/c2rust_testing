pub unsafe fn url_data_inspect(data: *mut url_data) {
    use libc::printf;
    if data.is_null() {
        return;
    }
    let d = &*data;
    printf("#url =>\n\0".as_ptr() as *const i8);
    printf("    .href: \"%s\"\n\0".as_ptr() as *const i8, d.href);
    printf(
        "    .protocol: \"%s\"\n\0".as_ptr() as *const i8,
        d.protocol,
    );
    printf("    .host: \"%s\"\n\0".as_ptr() as *const i8, d.host);
    printf("    .auth: \"%s\"\n\0".as_ptr() as *const i8, d.auth);
    printf(
        "    .hostname: \"%s\"\n\0".as_ptr() as *const i8,
        d.hostname,
    );
    printf(
        "    .pathname: \"%s\"\n\0".as_ptr() as *const i8,
        d.pathname,
    );
    printf("    .search: \"%s\"\n\0".as_ptr() as *const i8, d.search);
    printf("    .path: \"%s\"\n\0".as_ptr() as *const i8, d.path);
    printf("    .hash: \"%s\"\n\0".as_ptr() as *const i8, d.hash);
    printf("    .query: \"%s\"\n\0".as_ptr() as *const i8, d.query);
    printf("    .port: \"%s\"\n\0".as_ptr() as *const i8, d.port);
}
