#!/bin/sh
set -eu  # Add -u for undefined variable check

echo "Building with aggressive optimizations..."
cargo build --release
echo ""
echo -e "\nRunning benchmarks..."
./target/release/sorting_pearls
