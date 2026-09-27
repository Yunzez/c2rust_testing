// verbatim from benchmark/pairs/rq4/optipng_c2saferrust/translated/optipng_c2saferrust.rs:47840-47869
fn optimize_cmf(data: &mut [u8], data_size: usize) {
    if data_size <= 16384 {
        let mut z_cmf: u32 = data[0] as u32;
        if z_cmf & 0xf == 8 && z_cmf & 0xf0 <= 0x70 {
            let mut z_cinfo: u32;
            let mut half_z_window_size: u32;
            z_cinfo = z_cmf >> 4;
            half_z_window_size = 1 << (z_cinfo + 7);
            if data_size <= half_z_window_size as usize {
                loop {
                    half_z_window_size >>= 1;
                    z_cinfo -= 1;
                    if !(z_cinfo > 0 && data_size <= half_z_window_size as usize) {
                        break;
                    }
                }
                z_cmf = (z_cmf & 0xf) | (z_cinfo << 4);
                data[0] = z_cmf as u8;
                let mut tmp: u32 = (data[1] & 0xe0) as u32;
                tmp += (0x1f - ((z_cmf << 8) + tmp) % 0x1f);
                data[1] = tmp as u8;
            }
        }
    }
}
fn main() {
    for &n in &[1usize, 64, 100, 128] {
        let mut d = [0x08u8, 0x1d];
        let r = std::panic::catch_unwind(move || { optimize_cmf(&mut d, n); d });
        match r { Ok(d) => println!("Rust data_size={:3} -> {:02x} {:02x}", n, d[0], d[1]), Err(_) => println!("Rust data_size={:3} -> PANIC", n) }
    }
}
