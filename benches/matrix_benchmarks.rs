use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tensor_operations::manual::Matrix;
use tensor_operations::optimized::OptimizedMatrix;

fn create_manual_matrix(size: usize) -> Matrix {
    let mut data = Vec::new();
    for i in 0..size {
        let mut row = Vec::new();
        for j in 0..size {
            row.push((i * size + j) as f64);
        }
        data.push(row);
    }
    Matrix::new(data).unwrap()
}

fn create_optimized_matrix(size: usize) -> OptimizedMatrix {
    let mut data = Vec::new();
    for i in 0..size {
        let mut row = Vec::new();
        for j in 0..size {
            row.push((i * size + j) as f64);
        }
        data.push(row);
    }
    OptimizedMatrix::new(data).unwrap()
}

fn benchmark_addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("addition");

    for size in [10, 20, 50].iter() {
        let manual_a = create_manual_matrix(*size);
        let manual_b = create_manual_matrix(*size);

        group.bench_with_input(
            format!("manual_{}x{}", size, size),
            size,
            |b, _| b.iter(|| black_box(&manual_a).add(black_box(&manual_b))),
        );

        let opt_a = create_optimized_matrix(*size);
        let opt_b = create_optimized_matrix(*size);

        group.bench_with_input(
            format!("optimized_{}x{}", size, size),
            size,
            |b, _| b.iter(|| black_box(&opt_a).add(black_box(&opt_b))),
        );
    }

    group.finish();
}

fn benchmark_multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiplication");
    group.sample_size(10); // Reduce sample size for expensive operation

    for size in [5, 10, 20].iter() {
        let manual_a = create_manual_matrix(*size);
        let manual_b = create_manual_matrix(*size);

        group.bench_with_input(
            format!("manual_{}x{}", size, size),
            size,
            |b, _| b.iter(|| black_box(&manual_a).mul(black_box(&manual_b))),
        );

        let opt_a = create_optimized_matrix(*size);
        let opt_b = create_optimized_matrix(*size);

        group.bench_with_input(
            format!("optimized_{}x{}", size, size),
            size,
            |b, _| b.iter(|| black_box(&opt_a).mul(black_box(&opt_b))),
        );
    }

    group.finish();
}

fn benchmark_determinant(c: &mut Criterion) {
    let mut group = c.benchmark_group("determinant");

    for size in [2, 3, 4, 5].iter() {
        let manual_m = create_manual_matrix(*size);

        group.bench_with_input(
            format!("manual_{}x{}", size, size),
            size,
            |b, _| b.iter(|| black_box(&manual_m).determinant()),
        );

        let opt_m = create_optimized_matrix(*size);

        group.bench_with_input(
            format!("optimized_{}x{}", size, size),
            size,
            |b, _| b.iter(|| black_box(&opt_m).determinant()),
        );
    }

    group.finish();
}

fn benchmark_transpose(c: &mut Criterion) {
    let mut group = c.benchmark_group("transpose");

    for size in [10, 50, 100].iter() {
        let manual_m = create_manual_matrix(*size);

        group.bench_with_input(
            format!("manual_{}x{}", size, size),
            size,
            |b, _| b.iter(|| black_box(&manual_m).transpose()),
        );

        let opt_m = create_optimized_matrix(*size);

        group.bench_with_input(
            format!("optimized_{}x{}", size, size),
            size,
            |b, _| b.iter(|| black_box(&opt_m).transpose()),
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_addition,
    benchmark_multiplication,
    benchmark_determinant,
    benchmark_transpose
);
criterion_main!(benches);
