use std::{collections::HashMap, time::Duration};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use indexmap::IndexMap;
use itertools::Itertools;
use rand::{rngs::StdRng, RngCore, SeedableRng};

fn vec(source: &[(u64, u64)]) -> Vec<(u64, Vec<u64>)> {
    source
        .iter()
        .group_by(|it| it.0)
        .into_iter()
        .map(|(key, group)| (key, group.into_iter().map(|it| it.1).collect()))
        .collect()
}

fn hashmap(source: &[(u64, u64)]) -> HashMap<u64, Vec<u64>> {
    let mut result: HashMap<u64, Vec<u64>> = HashMap::new();
    for (k, v) in source.iter() {
        result.entry(*k).or_default().push(*v);
    }
    result
}

fn indexmap(source: &[(u64, u64)]) -> IndexMap<u64, Vec<u64>> {
    let mut result: IndexMap<u64, Vec<u64>> = IndexMap::new();
    for (k, v) in source.iter() {
        result.entry(*k).or_default().push(*v);
    }
    result
}

fn bench_group_by_trivial(c: &mut Criterion) {
    let mut group = c.benchmark_group("group_by_trivial");
    group
        .sample_size(20)
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(1));
    let mut rng = StdRng::from_seed(*b"42424242424242424242424242424242");
    for key_count in [8, 32, 128, 1024].into_iter() {
        for value_key_ratio in [1, 2, 64] {
            let keys = (0u64..key_count).map(|_| rng.next_u64()).collect_vec();
            let values = (0..(key_count * value_key_ratio))
                .map(|_| rng.next_u64())
                .collect_vec();
            let mut source = Vec::new();
            for key in keys {
                for value in &values {
                    source.push((key, *value));
                }
            }
            group.bench_with_input(
                BenchmarkId::new(
                    "Vec+group_by",
                    format!("{},{}v/k", key_count, value_key_ratio),
                ),
                &source,
                |b, source| b.iter(|| vec(source)),
            );
            group.bench_with_input(
                BenchmarkId::new("Hashmap", format!("{},{}v/k", key_count, value_key_ratio)),
                &source,
                |b, source| b.iter(|| hashmap(source)),
            );
            group.bench_with_input(
                BenchmarkId::new("Indexmap", format!("{},{}v/k", key_count, value_key_ratio)),
                &source,
                |b, source| b.iter(|| indexmap(source)),
            );
        }
    }
    group.finish();
}

criterion_group!(benches, bench_group_by_trivial);
criterion_main!(benches);
