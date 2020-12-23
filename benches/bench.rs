use criterion::{criterion_group, criterion_main};
use criterion::{AxisScale, PlotConfiguration};
use criterion::{BatchSize, BenchmarkId, Criterion};

use sorting_explorer::algorithms::{BubbleSort, InsertionSort, SelectionSort, StdSort};
use sorting_explorer::SortingAlgorithm;

fn make_sorted(n: usize) -> Vec<usize> {
    (0..n).collect()
}

fn make_reversed(n: usize) -> Vec<usize> {
    (0..n).rev().collect()
}

fn make_shuffled(n: usize) -> Vec<usize> {
    use rand::prelude::*;
    let mut unsorted_vec: Vec<usize> = (0..n).collect();
    let mut rng = StdRng::seed_from_u64(1234567);
    unsorted_vec.shuffle(&mut rng);
    unsorted_vec
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let groups: [(&'static str, fn(usize) -> Vec<usize>); 3] = [
        ("Sorted", make_sorted),
        ("Reversed", make_reversed),
        ("Shuffled", make_shuffled),
    ];

    for (group_name, make) in groups.iter() {
        let mut group = c.benchmark_group(*group_name);
        let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
        group.plot_config(plot_config);

        for &n in [0, 1, 3, 10, 32, 100, 320, 1000, 3200].iter() {
            let vec = make(n);

            group.bench_function(BenchmarkId::new("StdSort", n), |b| {
                b.iter_batched(
                    || vec.clone(),
                    |mut vec| {
                        StdSort.sort(&mut vec[..]);
                    },
                    BatchSize::LargeInput,
                )
            });

            group.bench_function(BenchmarkId::new("BubbleSort", n), |b| {
                b.iter_batched(
                    || vec.clone(),
                    |mut vec| {
                        BubbleSort.sort(&mut vec[..]);
                    },
                    BatchSize::LargeInput,
                )
            });

            group.bench_function(BenchmarkId::new("InsertionSort", n), |b| {
                b.iter_batched(
                    || vec.clone(),
                    |mut vec| {
                        InsertionSort::default().sort(&mut vec[..]);
                    },
                    BatchSize::LargeInput,
                )
            });

            group.bench_function(BenchmarkId::new("InsertionSort (binary)", n), |b| {
                b.iter_batched(
                    || vec.clone(),
                    |mut vec| {
                        InsertionSort::binary_search().sort(&mut vec[..]);
                    },
                    BatchSize::LargeInput,
                )
            });

            group.bench_function(BenchmarkId::new("SelectionSort", n), |b| {
                b.iter_batched(
                    || vec.clone(),
                    |mut vec| {
                        SelectionSort.sort(&mut vec[..]);
                    },
                    BatchSize::LargeInput,
                )
            });
        }

        group.finish();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
