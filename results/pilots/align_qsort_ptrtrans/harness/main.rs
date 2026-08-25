mod ptrans_qsort;
use std::io::{self, Read, Write};
fn main(){
    let op = std::env::args().nth(1).unwrap();
    let mut s=String::new(); io::stdin().read_to_string(&mut s).unwrap();
    let mut it=s.split_whitespace().map(|t| t.parse::<i32>().unwrap());
    let tcount=it.next().unwrap();
    let mut out=String::new();
    for _ in 0..tcount{
        if op=="swap" {
            let mut a=it.next().unwrap(); let mut b=it.next().unwrap();
            ptrans_qsort::swap(Some(&mut a), Some(&mut b));
            out.push_str(&format!("{} {}\n",a,b)); continue;
        }
        let n=it.next().unwrap() as usize; let low=it.next().unwrap(); let high=it.next().unwrap();
        let mut a: Vec<i32> = (0..n).map(|_| it.next().unwrap()).collect();
        if op=="partition" {
            let r=ptrans_qsort::partition(Some(&mut a[..]), low, high);
            out.push_str(&format!("ret={} :",r));
        } else {
            ptrans_qsort::quick_sort(Some(&mut a[..]), low, high);
        }
        for x in &a { out.push(' '); out.push_str(&x.to_string()); }
        out.push('\n');
    }
    io::stdout().write_all(out.as_bytes()).unwrap();
}
