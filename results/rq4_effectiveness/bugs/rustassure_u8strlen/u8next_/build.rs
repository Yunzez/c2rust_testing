fn main(){ cc::Build::new().file("u8next_c.c").flag_if_supported("-w").compile("cu8n"); }
