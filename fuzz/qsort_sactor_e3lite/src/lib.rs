pub fn swap(a: &mut i32, b: &mut i32) {
    fn inner(a: &mut i32, b: &mut i32) {
        std::mem::swap(&mut (*a), &mut (*b));
    }
    inner(a, b)
}
pub fn partition(arr: &mut [i32], low: i32, high: i32) -> i32 {
    fn do_swap(a: &mut i32, b: &mut i32) {
        swap(a, b);
    }
    let pivot_index = high as usize;
    let pivot = arr[pivot_index];
    let mut i = low - 1;
    let mut j = low;
    while j < high {
        let j_usize = j as usize;
        if arr[j_usize] <= pivot {
            i += 1;
            let i_usize = i as usize;
            if i_usize != j_usize {
                if i_usize < j_usize {
                    let (left, right) = arr.split_at_mut(j_usize);
                    let a = &mut left[i_usize];
                    let b = &mut right[0];
                    do_swap(a, b);
                } else {
                    let (left, right) = arr.split_at_mut(i_usize);
                    let a = &mut right[0];
                    let b = &mut left[j_usize];
                    do_swap(a, b);
                }
            }
        }
        j += 1;
    }
    let i_plus_1 = i + 1;
    let i1_usize = i_plus_1 as usize;
    let high_usize = high as usize;
    if i1_usize != high_usize {
        if i1_usize < high_usize {
            let (left, right) = arr.split_at_mut(high_usize);
            let a = &mut left[i1_usize];
            let b = &mut right[0];
            do_swap(a, b);
        } else {
            let (left, right) = arr.split_at_mut(i1_usize);
            let a = &mut right[0];
            let b = &mut left[high_usize];
            do_swap(a, b);
        }
    }
    i_plus_1
}
pub fn quick_sort(arr: &mut [i32], low: i32, high: i32) {
    fn inner_quick_sort(arr: &mut [i32], low: i32, high: i32) {
        if low < high {
            let i = partition(arr, low, high);
            inner_quick_sort(arr, low, i - 1);
            inner_quick_sort(arr, i + 1, high);
        }
    }
    inner_quick_sort(arr, low, high);
}
pub fn prog_main() {
    use std::env;
    fn atoi_like(s: &str) -> i32 {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let mut i = 0;
        while i < len && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        let mut sign = 1i32;
        if i < len {
            match bytes[i] {
                b'-' => {
                    sign = -1;
                    i += 1;
                }
                b'+' => {
                    i += 1;
                }
                _ => {}
            }
        }
        let mut value: i32 = 0;
        let mut found_digit = false;
        while i < len {
            let b = bytes[i];
            if b.is_ascii_digit() {
                found_digit = true;
                let digit = (b - b'0') as i32;
                value = value.wrapping_mul(10).wrapping_add(digit);
                i += 1;
            } else {
                break;
            }
        }
        if !found_digit {
            0
        } else {
            value.wrapping_mul(sign)
        }
    }
    let args: Vec<String> = env::args().collect();
    let mut n: i32 = (args.len() as i32) - 1;
    if n <= 0 {
        println!();
        return;
    }
    let mut arr = [0i32; 256];
    if n > 256 {
        n = 256;
    }
    let n_usize = n as usize;
    for (i, dst) in arr.iter_mut().take(n_usize).enumerate() {
        *dst = atoi_like(&args[i + 1]);
    }
    quick_sort(&mut arr[..n_usize], 0, n - 1);
    for (i, value) in arr[..n_usize].iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{value}");
    }
    println!();
}
