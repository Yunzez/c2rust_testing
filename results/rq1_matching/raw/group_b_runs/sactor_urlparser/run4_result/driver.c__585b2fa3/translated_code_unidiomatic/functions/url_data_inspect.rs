pub unsafe fn url_data_inspect(data: *mut url_data) {
    use libc::printf;
    printf(b"#url =>\n\0".as_ptr() as *const i8);
    printf(b"    .href: \"%s\"\n\0".as_ptr() as *const i8, (*data).href);
    printf(
        b"    .protocol: \"%s\"\n\0".as_ptr() as *const i8,
        (*data).protocol,
    );
    printf(b"    .host: \"%s\"\n\0".as_ptr() as *const i8, (*data).host);
    printf(b"    .auth: \"%s\"\n\0".as_ptr() as *const i8, (*data).auth);
    printf(
        b"    .hostname: \"%s\"\n\0".as_ptr() as *const i8,
        (*data).hostname,
    );
    printf(
        b"    .pathname: \"%s\"\n\0".as_ptr() as *const i8,
        (*data).pathname,
    );
    printf(
        b"    .search: \"%s\"\n\0".as_ptr() as *const i8,
        (*data).search,
    );
    printf(b"    .path: \"%s\"\n\0".as_ptr() as *const i8, (*data).path);
    printf(b"    .hash: \"%s\"\n\0".as_ptr() as *const i8, (*data).hash);
    printf(
        b"    .query: \"%s\"\n\0".as_ptr() as *const i8,
        (*data).query,
    );
    printf(b"    .port: \"%s\"\n\0".as_ptr() as *const i8, (*data).port);
}
