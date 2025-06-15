use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Slow Recursive Fibonacci
fn fibonacci_recursive(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

/// Fast Iterative Fibonacci
fn fibonacci_iterative(n: u64) -> u64 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    a
}

/// Benchmark both functions
fn benchmark_fibonacci(c: &mut Criterion) {
    c.bench_function("recursive fibonacci 20", |b| {
        b.iter(|| fibonacci_recursive(black_box(20)))
    });
    c.bench_function("iterative fibonacci 20", |b| {
        b.iter(|| fibonacci_iterative(black_box(20)))
    });
}

/// Register and run benchmarks
criterion_group!(benches, benchmark_fibonacci);
criterion_main!(benches);
