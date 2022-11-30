use std::{collections::BTreeSet, iter, time::Duration};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};

fn vec_sort(source: &[u64]) -> Vec<u64> {
    let mut result: Vec<_> = source.iter().cloned().collect();
    result.sort();
    result.dedup();
    result
}

fn btreeset_auto(source: &[u64]) -> BTreeSet<u64> {
    source.iter().cloned().collect()
}

fn bench_sort_with_dedup(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort_and_dedup");
    group
        .sample_size(20)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(1));
    let mut rng = StdRng::from_seed(b"42424242424242424242424242424242".clone());
    for item_count in [0u64, 1, 8, 32, 128, 2048, 0x80000].into_iter() {
        for ratio in [0u64, 1, 2, 10, 1000] {
            let source: Vec<_> = if ratio != 0 {
                (0..item_count)
                    .map(|_| rng.gen_range(0..(item_count * ratio)))
                    .collect()
            } else {
                let n = rng.next_u64();
                iter::once(n).cycle().take(item_count as _).collect()
            };
            group.bench_with_input(
                BenchmarkId::new("Vec + sort", format!("{},{}", item_count, ratio)),
                &source,
                |b, source| b.iter(|| vec_sort(&source)),
            );
            group.bench_with_input(
                BenchmarkId::new("BTreeSet", format!("{},{}", item_count, ratio)),
                &source,
                |b, source| b.iter(|| btreeset_auto(&source)),
            );
        }
    }
    group.finish();
}

criterion_group!(benches, bench_sort_with_dedup);
criterion_main!(benches);
