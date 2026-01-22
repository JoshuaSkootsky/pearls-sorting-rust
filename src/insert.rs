pub use crate::SortMetrics;

pub fn sort(arr: &mut [i32]) -> SortMetrics {
    let mut metrics = SortMetrics::new();
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 {
            metrics.compares += 1;
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                metrics.swaps += 1;
            } else {
                break;
            }
            j -= 1;
        }
    }
    metrics
}