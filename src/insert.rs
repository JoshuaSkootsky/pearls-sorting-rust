pub use crate::SortMetrics;

// sort with the "swap" method
pub fn sort_with_swap(arr: &mut [i32]) -> SortMetrics {
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

    return metrics
}

pub fn sort(arr: &mut [i32]) -> SortMetrics {
    let mut metrics = SortMetrics::new();
    for i in 1..arr.len() {
        let mut j = i;
        let key = arr[i]; // key is the value at the max index
        // When we are done, the value at arr[i] will be greater than arr[i -1]
        while j > 0 && { 
            metrics.compares += 1; // Count every check
            arr[j - 1] > key // this is logically evaluated Boolean
        }{
            arr[j] = arr[j - 1];
            metrics.swaps +=1; // record the move
            j -= 1; // increment counter down
            // keep decrementing until arr[j - 1] is not greater than key
        }

        arr[j] = key // insert left

    }

    return metrics
}