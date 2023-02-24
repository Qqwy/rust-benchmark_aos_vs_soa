use criterion::{criterion_group, criterion_main, Criterion, BatchSize, Throughput, PlotConfiguration, AxisScale, BenchmarkGroup, measurement::WallTime};
use std::time::Duration;
use quickcheck::{Gen, Arbitrary};

use soa_bench::*;

fn gen_needles(gen: &mut Gen) -> ((u64, u32, u128, u64, bool), (u64, u32, u128, u64, bool)) {
    let needle1: (u64, u32, u128, u64, bool) = Arbitrary::arbitrary(gen);
    let needle2: (u64, u32, u128, u64, bool) = Arbitrary::arbitrary(gen);
    let lower_needle = std::cmp::min(needle1, needle2);
    let higher_needle = std::cmp::max(needle1, needle2);
    (lower_needle, higher_needle)
}

pub fn benchme(group: &mut BenchmarkGroup<WallTime>, name: &str, gen: &mut Gen, size: usize, fun: impl Fn(&(&u64, &u32, &u128, &u64, &bool), &(&u64, &u32, &u128, &u64, &bool)) -> (usize, usize)) {
    group.bench_with_input(name, &size, |b, _size| {
        b.iter_batched(||gen_needles(gen),|(lower_needle, higher_needle)| {
            let lower_needle = (&lower_needle.0, &lower_needle.1, &lower_needle.2, &lower_needle.3, &lower_needle.4);
            let higher_needle = (&higher_needle.0, &higher_needle.1, &higher_needle.2, &higher_needle.3, &higher_needle.4);
            fun(&lower_needle, &higher_needle)
        }, BatchSize::SmallInput);
    });
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("search");
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(3));

    for size in [2, 4, 8, 16, 32, 64, 256, 1024, 2048, 4096, 8192, 16384, 32768, 65536] {
        group.throughput(Throughput::Elements(size as u64));

        let mut gen = Gen::new(size);
        let mut aos: Vec<(u64, u32, u128, u64, bool)> = Arbitrary::arbitrary(&mut gen);
        aos.sort_unstable();

        benchme(&mut group, "aos", &mut gen, size, |l, h| partition_range_vec(&aos, l, h));
        benchme(&mut group, "aos_segmented", &mut gen, size, |l, h| partition_range_vec_segmented(&aos, l, h));
        benchme(&mut group, "aos_segmented_alt", &mut gen, size, |l, h| partition_range_vec_segmented_alt(&aos, l, h));

        let soa = aos_to_soa(aos);
        benchme(&mut group, "soa", &mut gen, size, |l, h| partition_range_soa(&soa, l, h));
        benchme(&mut group, "soa_alt", &mut gen, size, |l, h| partition_range_soa_alt(&soa, l, h));
    }
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
