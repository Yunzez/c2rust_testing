

pub fn swap(a: Option<&mut i32>, b: Option<&mut i32>) {
    match (a, b) {
        (Some(a_ref), Some(b_ref)) => {
            // Handles possible aliasing safely and idiomatically
            std::mem::swap(a_ref, b_ref);
        }
        _ => {
            // If either is None (null in C), do nothing (matches C behavior of dereferencing only when non-null)
        }
    }
}
/// Translated from:
/// int partition (int arr[], int low, int high)
pub fn partition(arr: Option<&mut [i32]>, low: i32, high: i32) -> i32 {
    // Handle nullable `arr` (arr == NULL in C)
    let arr = match arr {
        Some(a) => a,
        None => {
            // In C, dereferencing a NULL pointer is UB. Here we choose to return 0 safely.
            return 0;
        }
    };
    // The C code assumes indices are valid; here we do minimal checks to avoid panics.
    if low < 0 || high < 0 {
        return 0;
    }
    let low_usize = low as usize;
    let high_usize = high as usize;
    if high_usize >= arr.len() || low_usize > high_usize {
        return 0;
    }
    let pivot = arr[high_usize];
    let mut i: i32 = low - 1;
    let mut j = low;
    while j <= high - 1 {
        let j_usize = j as usize;
        if j_usize >= arr.len() {
            break;
        }
        if arr[j_usize] <= pivot {
            i += 1;
            let i_usize = i as usize;
            if i_usize < arr.len() {
                // swap(&arr[i], &arr[j]);
                let (left, right) = arr.split_at_mut(j_usize.max(i_usize));
                // Obtain mutable references to the two elements without aliasing.
                let a_ref: Option<&mut i32>;
                let b_ref: Option<&mut i32>;
                if i_usize <= j_usize {
                    a_ref = left.get_mut(i_usize);
                    b_ref = right.get_mut(j_usize - i_usize);
                } else {
                    a_ref = right.get_mut(i_usize - j_usize);
                    b_ref = left.get_mut(j_usize);
                }
                swap(a_ref, b_ref);
            }
        }
        j += 1;
    }
    let ip1 = i + 1;
    let ip1_usize = ip1 as usize;
    if ip1_usize < arr.len() && high_usize < arr.len() {
        // swap(&arr[i + 1], &arr[high]);
        let (left, right) = arr.split_at_mut(high_usize.max(ip1_usize));
        let a_ref: Option<&mut i32>;
        let b_ref: Option<&mut i32>;
        if ip1_usize <= high_usize {
            a_ref = left.get_mut(ip1_usize);
            b_ref = right.get_mut(high_usize - ip1_usize);
        } else {
            a_ref = right.get_mut(ip1_usize - high_usize);
            b_ref = left.get_mut(high_usize);
        }
        swap(a_ref, b_ref);
    }
    ip1
}
pub fn quick_sort(mut arr: Option<&mut [i32]>, low: i32, high: i32) {
    if low < high {
        // Call partition exactly as in C, using the same parameters
        let i = partition(arr.as_deref_mut(), low, high);
        // Recurse on left part: low .. i-1
        quick_sort(arr.as_deref_mut(), low, i - 1);
        // Recurse on right part: i+1 .. high
        quick_sort(arr, i + 1, high);
    }
}
