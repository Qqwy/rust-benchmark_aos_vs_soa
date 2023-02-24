use criterion::{
    criterion_group, criterion_main, measurement::WallTime, AxisScale, BatchSize, BenchmarkGroup,
    Criterion, PlotConfiguration, Throughput,
};
use quickcheck::{Arbitrary, Gen};
use std::time::Duration;

use soa_bench::*;

fn gen_needles<T0, T1, T2, T3, T4>(gen: &mut Gen) -> ((T0, T1, T2, T3, T4), (T0, T1, T2, T3, T4))
where
    T0: Ord + Arbitrary,
    T1: Ord + Arbitrary,
    T2: Ord + Arbitrary,
    T3: Ord + Arbitrary,
    T4: Ord + Arbitrary,
{
    let needle1: (T0, T1, T2, T3, T4) = Arbitrary::arbitrary(gen);
    let needle2: (T0, T1, T2, T3, T4) = Arbitrary::arbitrary(gen);
    let lower_needle = std::cmp::min(&needle1, &needle2);
    let higher_needle = std::cmp::max(&needle1, &needle2);
    (lower_needle.clone(), higher_needle.clone())
}

pub fn benchme<T0, T1, T2, T3, T4>(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    gen: &mut Gen,
    size: usize,
    fun: impl Fn(&(&T0, &T1, &T2, &T3, &T4), &(&T0, &T1, &T2, &T3, &T4)) -> (usize, usize),
) where
    T0: Ord + Arbitrary,
    T1: Ord + Arbitrary,
    T2: Ord + Arbitrary,
    T3: Ord + Arbitrary,
    T4: Ord + Arbitrary,
{
    group.bench_with_input(name, &size, |b, _size| {
        b.iter_batched(
            || gen_needles(gen),
            |(lower_needle, higher_needle)| {
                let lower_needle = (
                    &lower_needle.0,
                    &lower_needle.1,
                    &lower_needle.2,
                    &lower_needle.3,
                    &lower_needle.4,
                );
                let higher_needle = (
                    &higher_needle.0,
                    &higher_needle.1,
                    &higher_needle.2,
                    &higher_needle.3,
                    &higher_needle.4,
                );
                fun(&lower_needle, &higher_needle)
            },
            BatchSize::SmallInput,
        );
    });
}

pub fn criterion_benchmark<T0, T1, T2, T3, T4>(c: &mut Criterion)
where
    T0: Ord + Arbitrary,
    T1: Ord + Arbitrary,
    T2: Ord + Arbitrary,
    T3: Ord + Arbitrary,
    T4: Ord + Arbitrary,
{
    let mut group = c.benchmark_group(format!(
        "search {}",
        std::any::type_name::<(T0, T1, T2, T3, T4)>()
    ));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));

    for size in [
        1, 2, 4, 8, 16, 32, 64, 256, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
    ] {
        group.throughput(Throughput::Elements(size as u64));

        let mut gen = Gen::new(size);
        // Generate an array with exactly `size` random elements
        let mut aos: Vec<(T0, T1, T2, T3, T4)> =
            (0..size).map(|_| Arbitrary::arbitrary(&mut gen)).collect();
        aos.sort_unstable();

        benchme(&mut group, "aos", &mut gen, size, |l, h| {
            partition_range_aos(&aos, l, h)
        });
        benchme(&mut group, "aos_segmented", &mut gen, size, |l, h| {
            partition_range_aos_segmented(&aos, l, h)
        });
        benchme(&mut group, "aos_segmented_alt", &mut gen, size, |l, h| {
            partition_range_aos_segmented_alt(&aos, l, h)
        });

        // Make sure the vecs are allocated right next to each other,
        // faking a full-blown struct-of-arrays implementation
        let bump_arena = bumpalo::Bump::with_capacity(expected_soa_size(&aos));
        let soa = aos_to_soa(&bump_arena, aos);

        benchme(&mut group, "soa", &mut gen, size, |l, h| {
            partition_range_soa(&soa, l, h)
        });
        benchme(&mut group, "soa_alt", &mut gen, size, |l, h| {
            partition_range_soa_alt(&soa, l, h)
        });
    }
}

type E = u64;
type A = u32;
type V = u128;
type T = u64;
type R = bool;
criterion_group!(benches,
                 criterion_benchmark<E, A, V, T, R>,
                 criterion_benchmark<A, V, E, T, R>,
                 criterion_benchmark<V, A, E, T, R>,
                 criterion_benchmark<T, E, A, V, R>,

                 criterion_benchmark<E, A, String, T, R>,
                 criterion_benchmark<A, String, E, T, R>,
                 criterion_benchmark<String, A, E, T, R>,
                 criterion_benchmark<T, E, A, String, R>
);
criterion_main!(benches);
