use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, BatchSize};
use quickcheck::{Gen, Arbitrary};

use soa_bench::*;

fn gen_needles(gen: &mut Gen) -> ((u64, u32, u128, u64, bool), (u64, u32, u128, u64, bool)) {
    let needle1: (u64, u32, u128, u64, bool) = Arbitrary::arbitrary(gen);
    let needle2: (u64, u32, u128, u64, bool) = Arbitrary::arbitrary(gen);
    let lower_needle = std::cmp::min(needle1, needle2);
    let higher_needle = std::cmp::max(needle1, needle2);
    (lower_needle, higher_needle)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    for size in [2, 16, 256, 1024] {
        let mut gen = Gen::new(size);
        let aos: Vec<(u64, u32, u128, u64, bool)> = Arbitrary::arbitrary(&mut gen);

        let mut group = c.benchmark_group("search");
        group.bench_with_input("aos", &size, |b, _size| {
            b.iter_batched(||gen_needles(&mut gen),|(lower_needle, higher_needle)| {
                let lower_needle = (&lower_needle.0, &lower_needle.1, &lower_needle.2, &lower_needle.3, &lower_needle.4);
                let higher_needle = (&higher_needle.0, &higher_needle.1, &higher_needle.2, &higher_needle.3, &higher_needle.4);
                partition_range_vec(&aos, &lower_needle, &higher_needle)
            }, BatchSize::SmallInput);
        });

        group.bench_with_input("aos_segmented", &size, |b, _size| {
            b.iter_batched(||gen_needles(&mut gen), |(lower_needle, higher_needle)| {
                let lower_needle = (&lower_needle.0, &lower_needle.1, &lower_needle.2, &lower_needle.3, &lower_needle.4);
                let higher_needle = (&higher_needle.0, &higher_needle.1, &higher_needle.2, &higher_needle.3, &higher_needle.4);
                partition_range_vec_segmented(&aos, &lower_needle, &higher_needle)
            }, BatchSize::SmallInput);
        });


        let soa = aos_to_soa(aos);
        group.bench_with_input("soa", &size, |b, _size| {
            b.iter_batched(|| gen_needles(&mut gen), |(lower_needle, higher_needle)| {
                let lower_needle = (&lower_needle.0, &lower_needle.1, &lower_needle.2, &lower_needle.3, &lower_needle.4);
                let higher_needle = (&higher_needle.0, &higher_needle.1, &higher_needle.2, &higher_needle.3, &higher_needle.4);
                partition_range_soa(&soa, &lower_needle, &higher_needle)
            }, BatchSize::SmallInput);
        });
    }
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
