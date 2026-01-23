pub use crate::SortMetrics;

pub fn sort<T: Ord>(array: &mut [T]) {
    let n = array.len();
    for i in (0..n/2).rev() {
        sift_down(array, i, n);
    }
    for i in (1..n).rev() {
        array.swap(0,i);
        sift_down(a, 0, i);
    }
}

fn sift_down<T: Ord>(array: &mut [T],  mut r: usize, e: usize) {
    loop {
        let c = r * 2 + 1;
        if c >= e { break };
        let c = if c + 1 < e && a[c] < a[c + 1] { c + 1 } else { c };
        if a[r] < a[c] { a.swap(r, c); r = c; } else { break; }
    }
}