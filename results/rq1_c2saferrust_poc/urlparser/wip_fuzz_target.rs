#![no_main]
use libfuzzer_sys::fuzz_target;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::ffi::CStr;
use std::os::raw::c_char;
const ORACLE: &str = "/tmp/claude-1000/-home-yunzez-c2rust-testing/1f18b0e9-85a1-4720-97e0-8c9d8d673339/scratchpad/url_base_oracle/target/x86_64-unknown-linux-gnu/release/url_base_oracle";
// Known-confirmed bug function indices to SKIP so we can enumerate the NEXT bug:
//   1 = url_is_ssh (to_str().unwrap() panic on non-UTF-8, bug #2)
const SKIP: &[usize] = &[1];

unsafe fn str_ret(p:*mut c_char)->String{ if p.is_null(){"null".into()}else{format!("s:{}",CStr::from_ptr(p).to_string_lossy())} }
unsafe fn call(idx:usize,url:*mut c_char)->String{ match idx{
 0=>format!("b:{}",url_wip::url_is_protocol(url) as i32),
 1=>format!("b:{}",url_wip::url_is_ssh(url) as i32),
 2=>str_ret(url_wip::url_get_protocol(url)), 3=>str_ret(url_wip::url_get_auth(url)),
 4=>str_ret(url_wip::url_get_hostname(url)), 5=>str_ret(url_wip::url_get_host(url)),
 6=>str_ret(url_wip::url_get_pathname(url)), 7=>str_ret(url_wip::url_get_path(url)),
 8=>str_ret(url_wip::url_get_search(url)), 9=>str_ret(url_wip::url_get_query(url)),
 10=>str_ret(url_wip::url_get_hash(url)), 11=>str_ret(url_wip::url_get_port(url)), _=>"?".into(),}}
fn decode(data:&[u8])->(usize,Vec<u8>){ let idx=(*data.get(0).unwrap_or(&0) as usize)%12;
 let n=(*data.get(1).unwrap_or(&0) as usize)%64;
 let mut s:Vec<u8>=(0..n).map(|i|{let b=*data.get(2+i).unwrap_or(&0); if b==0{1}else{b}}).collect(); s.push(0); (idx,s) }
fn run_oracle(data:&[u8])->Option<String>{
 let mut ch=Command::new(ORACLE).env("ASAN_OPTIONS","symbolize=0:detect_leaks=0:abort_on_error=0:exitcode=1")
   .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn().ok()?;
 ch.stdin.take()?.write_all(data).ok();
 let mut so=ch.stdout.take()?; let (tx,rx)=std::sync::mpsc::channel();
 std::thread::spawn(move||{let mut s=String::new();let _=so.read_to_string(&mut s);let _=tx.send(s);});
 let dl=Instant::now()+Duration::from_millis(2000);
 let st=loop{match ch.try_wait(){Ok(Some(s))=>break s,Ok(None)=>{if Instant::now()>=dl{let _=ch.kill();let _=ch.wait();return None;} std::thread::sleep(Duration::from_millis(1));},Err(_)=>return None}};
 if !st.success(){return None;}
 Some(rx.recv_timeout(Duration::from_millis(500)).ok()?.trim().to_string())
}
fn rust_side(data:&[u8])->String{ let (idx,mut s)=decode(data); unsafe{call(idx,s.as_mut_ptr() as *mut c_char)} }
fuzz_target!(|data:&[u8]|{
 let idx=(*data.get(0).unwrap_or(&0) as usize)%12;
 if SKIP.contains(&idx){return;}
 let c=match run_oracle(data){Some(s)=>s,None=>return};
 let r=rust_side(data);
 if c!=r{ match run_oracle(data){Some(c2) if c2==c=>panic!("divergence fn={} C={:?} Rust={:?}",idx,c,r),_=>return} }
});
