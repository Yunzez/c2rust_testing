mod ptrans_qsort;
use std::io::{self, Read};
fn main(){
    let mut s=String::new(); io::stdin().read_to_string(&mut s).unwrap();
    let mut it=s.split_whitespace().map(|t| t.parse::<i32>().unwrap());
    let tcount=it.next().unwrap();
    let mut out=String::new();
    for _ in 0..tcount{
        let n=it.next().unwrap() as usize;
        let mut a: Vec<i32> = (0..n).map(|_| it.next().unwrap()).collect();
        let high = n as i32 - 1;
        ptrans_qsort::quick_sort(Some(&mut a[..]), 0, high);
        for x in &a { out.push_str(&x.to_string()); out.push(' '); }
        out.push('\n');
    }
    print!("{}", out);
}
