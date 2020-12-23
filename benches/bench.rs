use criterion::{criterion_group, criterion_main};
use criterion::{AxisScale, PlotConfiguration};
use criterion::{BatchSize, BenchmarkId, Criterion};

use rand::prelude::*;

use sorting_explorer::algorithms::{BubbleSort, InsertionSort, SelectionSort, StdSort};
use sorting_explorer::SortingAlgorithm;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Reversed");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for n in &[0, 1, 5, 10, 50, 100, 500, 1000, 5000] {
        let unsorted_vec: Vec<i32> = (0..*n).rev().collect();

        group.bench_function(BenchmarkId::new("StdSort", n), |b| {
            b.iter_batched_ref(
                || unsorted_vec.clone(),
                |vec| {
                    StdSort.sort(&mut vec[..]);
                },
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("BubbleSort", n), |b| {
            b.iter_batched_ref(
                || unsorted_vec.clone(),
                |vec| {
                    BubbleSort.sort(&mut vec[..]);
                },
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("InsertionSort", n), |b| {
            b.iter_batched_ref(
                || unsorted_vec.clone(),
                |vec| {
                    InsertionSort::default().sort(&mut vec[..]);
                },
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("InsertionSort (binary)", n), |b| {
            b.iter_batched_ref(
                || unsorted_vec.clone(),
                |vec| {
                    InsertionSort::binary_search().sort(&mut vec[..]);
                },
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("SelectionSort", n), |b| {
            b.iter_batched_ref(
                || unsorted_vec.clone(),
                |vec| {
                    SelectionSort.sort(&mut vec[..]);
                },
                BatchSize::LargeInput,
            )
        });
    }
    group.finish();

    let mut group = c.benchmark_group("Shuffled");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for n in &[0, 1, 5, 10, 50, 100, 500, 1000, 5000] {
        let mut unsorted_vec: Vec<i32> = (0..*n).collect();
        let mut rng = StdRng::seed_from_u64(1234567);
        unsorted_vec.shuffle(&mut rng);

        group.bench_function(BenchmarkId::new("StdSort", n), |b| {
            b.iter_batched_ref(
                || unsorted_vec.clone(),
                |vec| {
                    StdSort.sort(&mut vec[..]);
                },
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("BubbleSort", n), |b| {
            b.iter_batched_ref(
                || unsorted_vec.clone(),
                |vec| {
                    BubbleSort.sort(&mut vec[..]);
                },
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("InsertionSort", n), |b| {
            b.iter_batched_ref(
                || unsorted_vec.clone(),
                |vec| {
                    InsertionSort::default().sort(&mut vec[..]);
                },
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("InsertionSort (binary)", n), |b| {
            b.iter_batched_ref(
                || unsorted_vec.clone(),
                |vec| {
                    InsertionSort::binary_search().sort(&mut vec[..]);
                },
                BatchSize::LargeInput,
            )
        });

        group.bench_function(BenchmarkId::new("SelectionSort", n), |b| {
            b.iter_batched_ref(
                || unsorted_vec.clone(),
                |vec| {
                    SelectionSort.sort(&mut vec[..]);
                },
                BatchSize::LargeInput,
            )
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
