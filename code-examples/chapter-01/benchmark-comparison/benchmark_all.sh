#!/bin/bash
# benchmark_all.sh

echo "=== Running All Benchmarks ==="
echo ""

# Rust
if command -v cargo &> /dev/null; then
    echo "=== Rust Benchmark ==="
    cd rust
    cargo build --release 2>/dev/null
    cargo run --release --bin benchmark
    cd ..
    echo ""
fi

# C++
if command -v g++ &> /dev/null; then
    echo "=== C++ Benchmark ==="
    g++ -O3 -std=c++17 cpp/cpp_benchmark.cpp -o cpp/cpp_benchmark
    ./cpp/cpp_benchmark
    echo ""
fi

# Java
if command -v javac &> /dev/null; then
    echo "=== Java Benchmark ==="
    javac java/JavaBenchmark.java
    java -cp java JavaBenchmark
    echo ""
fi

# Python
if command -v python3 &> /dev/null; then
    echo "=== Python Benchmark ==="
    python3 python/python_benchmark.py
    echo ""
fi

echo "=== Benchmark Complete ===="