# Sorting Algorithms Benchmark

A Rust project for implementing, testing, and benchmarking sorting algorithms. Measure comparisons, swaps, and execution time across different data distributions.

[Read Programming Pearls on Sorting!](https://dl.acm.org/doi/pdf/10.1145/358027.381121)

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

## Compiling with Optimizations

Use `benchmark.sh` to run with compilter optimizations.

The `benchmark.sh` script uses the `--release` flag and is much faster than simply using and using the unoptimized Cargo run (which is actually debug mode!). 

For Let’s look at the 10,000 random elements with Quicksort:

| Mode | Time µs | Time ms |
| ------| ------- | ------- |
| Debug |  4,788.968 µs | ~4.78 ms |
| Release | 442.453 µs | ~0.44 ms |

Result: The optimized release version is 10x faster.

For the 10,000 reverse sorted elements (i.e. Quicksort kryptonite!) case:


| Mode |  Time ms |
| ------| ------- |
| Debug | 395.02 ms |
| Release | 23.12 ms |

Result: `--release` is 17x faster.

In Rust, `cargo run --release` enables LLVM optimizations, inlines functions, and removes many runtime checks (like integer overflow checks in some contexts), which explains it runs 10x faster.

2. Algorithmic Behavior (Quicksort Fixed Pivot)

- Algorithmic Behavior: The number of comparisons for Reverse data is exactly \(\frac{n(n-1)}{2}\). For \(n=10,000\), that is \(49,995,000\). This confirms your Quicksort is degrading from \(O(n \log n)\) to \(O(n^2)\) on sorted/reverse data.

- Duplicate Data: Your implementation also struggles with duplicates (likely due to how it handles elements equal to the pivot), taking 2.4 ms compared to the 0.4 ms for random data at the 10,000 scale.

3. Comparison Count Consistency


Notice that the compares and swaps stay almost identical between the two runs (e.g., exactly 49,995,000 compares for Reverse/10,000 in both runs).


- This is good! It means your logic is deterministic.

- The slight variations in "Random" compares (148,643 vs 152,573) are simply because the random seed changed between executions.
