# Sorting Algorithms Benchmark

A Rust project for implementing, testing, and benchmarking sorting algorithms. Measure comparisons, swaps, and execution time across different data distributions.

Inspired by reading through Programming Pearls on sorting and wanting to implement, and benchmark, the different sorts in Rust.

[Read Programming Pearls on Sorting!](https://dl.acm.org/doi/pdf/10.1145/358027.381121)

Programming Pearls on Sorting was cited in one of my favorite blog posts about bugs, testing, and philosophy of writing code by Joshua Bloch. [Extra, Extra - Read All About It: Nearly All Binary Searches and Mergesorts are Broken](https://research.google/blog/extra-extra-read-all-about-it-nearly-all-binary-searches-and-mergesorts-are-broken/ "Nearly All Binary Searches and Mergesorts are Broken")

In the spirit of Programming Pearls, here are some short, elegant versions of these sorts without benchmarking statistics added:

## Heap Sort (13 lines)


A lucid implementation that clearly separates heap construction from extraction. A tribute to W.D. Maurer's 19-line FORTRAN implementation of Heap Sort.

```rust
	fn heap_sort<T: Ord>(a: &mut [T]) {
	    let n = a.len();
	    for i in (0..n/2).rev() {
	        sift_down(a, i, n);
	    }
	    for i in (1..n).rev() {
	        a.swap(0, i);
	        sift_down(a, 0, i);
	    }
	}
	
	fn sift_down<T: Ord>(a: &mut [T], mut r: usize, e: usize) {
	    loop {
	        let c = r * 2 + 1;
	        if c >= e { break; }
	        let c = if c + 1 < e && a[c] < a[c + 1] { c + 1 } else { c };
	        if a[r] < a[c] { a.swap(r, c); r = c; } else { break; }
	    }
	}
```

## String Sort (8 lines)

A Rust tribute to McIlroy's 12-line constant time radix sort using bucket sort on each character position:

```rust
	fn string_sort(strings: &mut [&str]) {
	    let max = strings.iter().map(|s| s.len()).max().unwrap_or(0);
	    for i in (0..max).rev() {
	        let mut b = vec![Vec::new(); 257];
	        for s in strings.drain(..) {
	            b[s.as_bytes().get(i).map(|&c| c as usize + 1).unwrap_or(0)].push(s);
	        }
	        for mut v in b { strings.append(&mut v); }
	    }
	}
```

Do not use this .as_bytes on Unicode, it works on ASCII chars!

## Insertion Sort (7 lines)


The essence of simplicity; each element finds its proper place.

```rust
	fn insertion_sort<T: Ord>(a: &mut [T]) {
	    for i in 1..a.len() {
	        let mut j = i;
	        while j > 0 && a[j] < a[j - 1] {
	            a.swap(j, j - 1);
	            j -= 1;
	        }
	    }
	}
```

With that you can start to see why Bently was recommending writing your own insertion sort and ignoring the "System Sort!"

## Quicksort (14 lines)

Elegant recursion with Rust's split_at_mut for safe partitioning:

```rust
	fn quicksort<T: Ord>(a: &mut [T]) {
	    if a.len() < 2 { return; }
	    let p = partition(a);
	    let (l, r) = a.split_at_mut(p);
	    quicksort(l);
	    quicksort(&mut r[1..]);
	}
	
	fn partition<T: Ord>(a: &mut [T]) -> usize {
	    let (p, mut i) = (a.len() - 1, 0);
	    for j in 0..p {
	        if a[j] < a[p] {
	            a.swap(i, j);
	            i += 1;
	        }
	    }
	    a.swap(i, p);
	    i
	}
```

This 14 line quicksort is not tail call recursive optimized, and indeed could lead to a Stack Overflow.

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