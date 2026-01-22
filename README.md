# Sorting Algorithms Benchmark

A Rust project for implementing, testing, and benchmarking sorting algorithms. Measure comparisons, swaps, and execution time across different data distributions.

Inspired by reading through Programming Pearls on sorting and wanting to implement, and benchmark, the different sorts in Rust.

[Read Programming Pearls on Sorting!](https://dl.acm.org/doi/pdf/10.1145/358027.381121)

Programming Pearls on Sorting was cited in one of my favorite blog posts about bugs, testing, and philosophy of writing code by Joshua Bloch. [Extra, Extra - Read All About It: Nearly All Binary Searches and Mergesorts are Broken](https://research.google/blog/extra-extra-read-all-about-it-nearly-all-binary-searches-and-mergesorts-are-broken/ "Nearly All Binary Searches and Mergesorts are Broken")

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

Benchmarking reveals interesting behavior, like insertion sort being faster on arrays of length 100, and quicksort being basically n^(2) (quadratic) on reverse sorted lists.