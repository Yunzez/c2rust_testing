 fn mmed3(a: u8, b: u8, c: u8) -> u8 {
    let mut min = a;

    if b < min {
        min = b;
    }
    if c < min {
        min = c;
    }

    min
}
