/// Swap two elements in a mutable slice at the given indices.
fn swap_elements(arr: &mut [i32], idx_a: usize, idx_b: usize) {
    arr.swap(idx_a, idx_b);
}

/// Partition function corresponding to the C `partition`.
///
/// - `arr`: full mutable slice being sorted
/// - `low`: starting index of the partition (inclusive)
/// - `high`: ending index of the partition (inclusive)
///
/// Returns the partition index.
fn partition_slice(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i: isize = low as isize - 1;

    for j in low..=high - 1 {
        if arr[j] <= pivot {
            i += 1;
            swap_elements(arr, i as usize, j);
        }
    }

    let pi = (i + 1) as usize;
    swap_elements(arr, pi, high);
    pi
}

/// Safe, idiomatic Rust version of the C `quickSort` operating on a slice.
///
/// Sorts the subrange `[low, high]` (inclusive) using in-place quicksort.
fn quick_sort_range(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let p = partition_slice(arr, low, high);
        // Recurse on the left part if it is non-empty
        if p > 0 { // avoid underflow when p == 0
            quick_sort_range(arr, low, p - 1);
        }
        // Recurse on the right part if it is non-empty
        if p + 1 <= high {
            quick_sort_range(arr, p + 1, high);
        }
    }
}

/// Public, safe entry point: sorts the entire slice using quicksort.
///
/// This mirrors calling `quickSort(arr, 0, n-1)` in the original C code.
pub fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let len = arr.len();
    quick_sort_range(arr, 0, len - 1);
}
