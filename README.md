# Sorting Algorithms Benchmark

A Rust project for implementing, testing, and benchmarking sorting algorithms. Measure comparisons, swaps, and execution time across different data distributions.

## Quick Start

```bash
cargo run --release
```

## Project Structure

```
sorting_pearls/
├── src/main.rs          # All sorting code, benchmarks, and tests
└── Cargo.toml           # Dependencies (rand for random data)
```

## Data Types

The benchmark runs on 4 data distributions:
- **Random**: Uniform random integers
- **Nearly Sorted**: Sorted with ~1% elements swapped
- **Reverse Sorted**: Descending order
- **Duplicates**: Values in range 0-10

## Adding a New Sort

1. Create a function with this signature:
```rust
fn my_sort(arr: &mut [i32]) -> SortMetrics {
    let mut metrics = SortMetrics::new();
    // your sorting code here
    metrics
}
```

2. Count operations:
   - `metrics.compares += 1` - each time you compare two elements
   - `metrics.swaps += 1` - each time you swap two elements

3. Add to benchmark in `main()`:
```rust
run_benchmark("MySort", &random_data, my_sort);
```

4. Run:
```bash
cargo run --release
```

## Example: Insertion Sort

```rust
fn insertion_sort(arr: &mut [i32]) -> SortMetrics {
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
```

## Metrics Explained

| Metric | Description |
|--------|-------------|
| Compares | Number of element comparisons made |
| Swaps | Number of element exchanges |
| Time | Wall-clock execution time |

## Testing

```bash
cargo test
```

## Sample Output

```
=== Sorting Algorithms Benchmark ===

Sample size: 10000
  Quicksort            | elements:   10000 | compares:       149949 | swaps:      67644 | time:  470.117µs
  Quicksort (nearly sorted) | elements:   10000 | compares:       378491 | swaps:      41852 | time:  400.959µs
  Quicksort (reverse)  | elements:   10000 | compares:     49995000 | swaps:       5000 | time: 23.176896ms
  Quicksort (duplicates) | elements:   10000 | compares:      5034836 | swaps:      19016 | time:  2.52543ms
```

## Ideas to Implement

- Bubble Sort
- Selection Sort
- Insertion Sort
- Merge Sort
- Heap Sort
- Radix Sort
- Shell Sort
- Tim Sort

## Notes

- Always use `--release` for accurate timing
- Larger sample sizes (1000, 10000) show clearer algorithmic differences
- Compare metrics across data types to understand algorithm behavior
