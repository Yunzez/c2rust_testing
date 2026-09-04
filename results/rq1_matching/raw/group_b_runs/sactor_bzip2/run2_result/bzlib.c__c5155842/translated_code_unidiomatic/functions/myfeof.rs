pub unsafe fn myfeof(f: *mut libc::FILE) -> libc::c_int {
    let c: libc::c_int = libc::fgetc(f);
    if c == -1 {
        return 1;
    }
    libc::ungetc(c, f);
    0
}
