use std::{
    collections::{BTreeSet, HashSet},
    time::Duration,
};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use indexmap::IndexSet;
use itertools::Itertools;
use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};

fn vec_sort(source: &[u64]) -> Vec<u64> {
    let mut result: Vec<_> = source.to_vec();
    result.sort();
    result.dedup();
    result
}

fn btreeset_auto(source: &[u64]) -> BTreeSet<u64> {
    source.iter().cloned().collect()
}

fn hashset_auto(source: &[u64]) -> HashSet<u64> {
    source.iter().cloned().collect()
}

fn indexset_auto(source: &[u64]) -> IndexSet<u64> {
    source.iter().cloned().collect()
}

fn bench_dedup_trivial(c: &mut Criterion) {
    let mut group = c.benchmark_group("dedup_trivial");
    group
        .sample_size(20)
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(1));
    let mut rng = StdRng::from_seed(*b"42424242424242424242424242424242");
    for item_count in [1, 32, 128, 2048].into_iter() {
        let items = (0..item_count).map(|_| rng.next_u64()).collect_vec();
        for dup_probability in [0, 10, 50, 90, 100, 200, 1000] {
            let mut source = Vec::new();
            for item in items.iter() {
                source.push(*item);
                if dup_probability < 100 {
                    if rng.gen_ratio(dup_probability, 100) {
                        source.push(*item);
                    }
                } else {
                    for _ in 0..(dup_probability / 100) {
                        source.push(*item);
                    }
                }
            }
            group.bench_with_input(
                BenchmarkId::new("Vec+sort", format!("{},{}%", item_count, dup_probability)),
                &source,
                |b, source| b.iter(|| vec_sort(source)),
            );
            group.bench_with_input(
                BenchmarkId::new("BTreeSet", format!("{},{}%", item_count, dup_probability)),
                &source,
                |b, source| b.iter(|| btreeset_auto(source)),
            );
            group.bench_with_input(
                BenchmarkId::new("HashSet", format!("{},{}%", item_count, dup_probability)),
                &source,
                |b, source| b.iter(|| hashset_auto(source)),
            );
            group.bench_with_input(
                BenchmarkId::new("IndexSet", format!("{},{}%", item_count, dup_probability)),
                &source,
                |b, source| b.iter(|| indexset_auto(source)),
            );
        }
    }
    group.finish();
}

criterion_group!(benches, bench_dedup_trivial);
criterion_main!(benches);
