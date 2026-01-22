pub use crate::SortMetrics;

pub fn sort(arr: &mut [i32]) -> SortMetrics {
    let mut metrics = SortMetrics::new();
    quicksort(arr, &mut metrics);
    metrics
}

fn quicksort<T: PartialOrd>(arr: &mut [T], metrics: &mut SortMetrics) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr, metrics);
    let (left, right) = arr.split_at_mut(pivot_index);
    quicksort(left, metrics);
    quicksort(&mut right[1..], metrics);
}

fn partition<T: PartialOrd>(arr: &mut [T], metrics: &mut SortMetrics) -> usize {
    if arr.is_empty() {
        return 0;
    }
    let pivot_idx = arr.len() - 1;
    let mut store_idx = 0;

    for i in 0..pivot_idx {
        metrics.compares += 1;
        if arr[i] <= arr[pivot_idx] {
            if i != store_idx {
                arr.swap(i, store_idx);
                metrics.swaps += 1;
            }
            store_idx += 1;
        }
    }
    if store_idx != pivot_idx {
        arr.swap(store_idx, pivot_idx);
        metrics.swaps += 1;
    }
    store_idx
}