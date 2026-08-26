#![allow(non_upper_case_globals,non_camel_case_types,non_snake_case,dead_code,unused_mut,unused_assignments,unused_unsafe,unused_parens,unused_imports)]
pub mod wip;
/// Shared decoding: seed crc = first 4 bytes LE; then chunk stream (1 byte L, min(L,rest) bytes).
pub fn run(data:&[u8])->(u64,u32){
    let mut crc:u64=0; let mut p=0usize; let mut n=0u32;
    if data.len()>=4 { crc=u32::from_le_bytes([data[0],data[1],data[2],data[3]]) as u64; p=4; }
    while p<data.len(){ let mut l=data[p] as usize; p+=1; if l>data.len()-p { l=data.len()-p; }
        crc=wip::crc32_z(crc,&data[p..p+l],l); p+=l; n+=1; }
    (crc,n)
}
