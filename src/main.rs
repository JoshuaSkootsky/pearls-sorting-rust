use rand::Rng;
use std::time::Instant;

mod insert;
mod quick;

use insert::sort as insertion_sort;
use insert::sort_with_swap as insertion_sort_with_swap;
use quick::sort as quick_sort;

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

pub struct SortMetrics {
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

fn is_sorted<T: PartialOrd>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

fn run_benchmark(name: &str, data: &[i32], sort_fn: fn(&mut [i32]) -> SortMetrics) {
    // Makes a safe copy of the original data, preserving original data
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

fn compare_benchmarks(
    name: &str,
    data: &[i32],
    sort1_name: &str,
    sort1: fn(&mut [i32]) -> SortMetrics,
    sort2_name: &str,
    sort2: fn(&mut [i32]) -> SortMetrics,
) {
    // Makes a safe copy of the original data, preserving original data
    let mut data1 = data.to_vec();
    let start1 = Instant::now();
    let metrics1 = sort1(&mut data1);
    let elapsed1 = start1.elapsed();
    // Makes a safe copy of the original data, preserving original data
    let mut data2 = data.to_vec();
    let start2 = Instant::now();
    let metrics2 = sort2(&mut data2);
    let elapsed2 = start2.elapsed();

    if !is_sorted(&data1) {
        println!("  WARNING: {} did NOT sort the data!", sort1_name);
    }
    if !is_sorted(&data2) {
        println!("  WARNING: {} did NOT sort the data!", sort2_name);
    }
    println!("{}", name);
    println!(
        "  {:20} | compares: {:>12} | swaps: {:>10} | time: {:>10?}",
        sort1_name, metrics1.compares, metrics1.swaps, elapsed1
    );
    println!(
        "  {:20} | compares: {:>12} | swaps: {:>10} | time: {:>10?}",
        sort2_name, metrics2.compares, metrics2.swaps, elapsed2
    );
    println!("{}", "-".repeat(80));
}

fn main() {
    println!("=== Sorting Algorithms Benchmark ===\n");

    for &size in SAMPLE_SIZES {
        println!("Sample size: {}", size);

        let random_data = generate_random_data(size);
        let nearly_sorted = generate_nearly_sorted_data(size);
        let reverse_sorted = generate_reverse_sorted_data(size);
        let duplicates = generate_many_duplicates_data(size);

        // run_benchmark("Quicksort             ", &random_data, quick_sort);
        // run_benchmark("Quicksort (reverse)   ", &reverse_sorted, quick_sort);
        // run_benchmark("Quicksort (duplicates)", &duplicates, quick_sort);

        compare_benchmarks(
            "Nearly Sorted",
            &nearly_sorted,
            "Insertion Sort With Swap",
            insertion_sort_with_swap,
            "Insertion Sort",
            insertion_sort,
        );

        compare_benchmarks(
            "Duplicate Data",
            &duplicates,
            "Insertion Sort With Swap",
            insertion_sort_with_swap,
            "Insertion Sort",
            insertion_sort,
        );

        compare_benchmarks(
            "Reverse",
            &reverse_sorted,
            "Insertion Sort With Swap",
            insertion_sort_with_swap,
            "Insertion Sort",
            insertion_sort,
        );

        compare_benchmarks(
            "Duplicates",
            &duplicates,
            "Insertion Sort With Swap",
            insertion_sort_with_swap,
            "Insertion Sort",
            insertion_sort,
        );

        println!("");
        println!("{}", "-".repeat(80));
        println!("");

        // compare_benchmarks(
        //     "Nearly Sorted Comparison",
        //     &nearly_sorted,
        //     "Quicksort",
        //     quick_sort,
        //     "Insertion Sort",
        //     insertion_sort,
        // );

        // compare_benchmarks(
        //     "Duplicate Data Comparison",
        //     &duplicates,
        //     "Quicksort",
        //     quick_sort,
        //     "Insertion Sort",
        //     insertion_sort,
        // );

        println!();

        // run_benchmark("Insertion_sort (nearly sorted)", &nearly_sorted ,insertion_sort);
    }

    println!("=== How to Add Your Own Sort ===");
    println!("1. Create a new function with signature: fn my_sort(arr: &mut [i32]) -> SortMetrics");
    println!("2. Count comparisons with: metrics.compares += 1");
    println!("3. Count swaps with: metrics.swaps += 1");
    println!("4. Add to run_benchmark() call below:");
    println!("   run_benchmark(\"MySort\", &random_data, my_sort);");
    println!("5. Run: cargo run --release");
    // alternatively use the compare benchmarks functions
    // and run with ./benchmark.sh
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let metrics = quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
        assert!(metrics.compares > 0);
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single() {
        let mut arr = vec![42];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }
}
