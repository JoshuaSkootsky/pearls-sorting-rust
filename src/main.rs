use rand::Rng;
use std::time::Instant;

const SAMPLE_SIZES: &[usize] = &[100, 1000, 10000];

fn generate_random_data(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(0..100000)).collect()
}

fn generate_nearly_sorted_data(n: usize) -> Vec<i32> {
    let mut data: Vec<i32> = (0..n as i32).collect();
    let mut rng = rand::thread_rng();
    for _ in (n / 100)..(n / 10) {
        let i = rng.gen_range(0..n);
        let j = rng.gen_range(0..n);
        data.swap(i, j);
    }
    data
}

fn generate_reverse_sorted_data(n: usize) -> Vec<i32> {
    (0..n as i32).rev().collect()
}

fn generate_many_duplicates_data(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(0..10)).collect()
}

struct SortMetrics {
    compares: u64,
    swaps: u64,
}

impl SortMetrics {
    fn new() -> Self {
        Self {
            compares: 0,
            swaps: 0,
        }
    }
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

fn is_sorted<T: PartialOrd>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

fn run_benchmark(name: &str, data: &[i32], sort_fn: fn(&mut [i32]) -> SortMetrics) {
    let mut data = data.to_vec();
    let start = Instant::now();
    let metrics = sort_fn(&mut data);
    let elapsed = start.elapsed();

    println!(
        "  {:20} | elements: {:>7} | compares: {:>12} | swaps: {:>10} | time: {:>10?}",
        name,
        data.len(),
        metrics.compares,
        metrics.swaps,
        elapsed
    );
    if !is_sorted(&data) {
        println!("  WARNING: Data is NOT sorted!");
    }
}

fn quicksort_simple(arr: &mut [i32]) -> SortMetrics {
    let mut metrics = SortMetrics::new();
    quicksort(arr, &mut metrics);
    metrics
}

fn main() {
    println!("=== Sorting Algorithms Benchmark ===\n");

    for &size in SAMPLE_SIZES {
        println!("Sample size: {}", size);

        let random_data = generate_random_data(size);
        let nearly_sorted = generate_nearly_sorted_data(size);
        let reverse_sorted = generate_reverse_sorted_data(size);
        let duplicates = generate_many_duplicates_data(size);

        run_benchmark("Quicksort", &random_data, quicksort_simple);
        run_benchmark(
            "Quicksort (nearly sorted)",
            &nearly_sorted,
            quicksort_simple,
        );
        run_benchmark("Quicksort (reverse)", &reverse_sorted, quicksort_simple);
        run_benchmark("Quicksort (duplicates)", &duplicates, quicksort_simple);

        println!();
    }

    println!("=== How to Add Your Own Sort ===");
    println!("1. Create a new function with signature: fn my_sort(arr: &mut [i32]) -> SortMetrics");
    println!("2. Count comparisons with: metrics.compares += 1");
    println!("3. Count swaps with: metrics.swaps += 1");
    println!("4. Add to run_benchmark() call below:");
    println!("   run_benchmark(\"MySort\", &random_data, my_sort);");
    println!("5. Run: cargo run --release");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let metrics = quicksort_simple(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
        assert!(metrics.compares > 0);
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        quicksort_simple(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single() {
        let mut arr = vec![42];
        quicksort_simple(&mut arr);
        assert_eq!(arr, vec![42]);
    }
}
