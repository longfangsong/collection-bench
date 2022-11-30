use std::{
    collections::{BTreeSet, HashMap, HashSet},
    iter, time::Duration,
};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use itertools::Itertools;
use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};

fn vec(source: &[(u64, u64)]) -> Vec<(u64, Vec<u64>)> {
    source
        .into_iter()
        .group_by(|it| it.0)
        .into_iter()
        .map(|(key, group)| (key, group.into_iter().map(|it| it.1).collect()))
        .collect()
}

fn hashmap(source: &[(u64, u64)]) -> HashMap<u64, Vec<u64>> {
    let mut result: HashMap<u64, Vec<u64>> = HashMap::new();
    for (k, v) in source.into_iter() {
        result.entry(*k).or_default().push(*v);
    }
    result
}

fn bench_group_by(c: &mut Criterion) {
    let mut group = c.benchmark_group("group_by");
    group
        .sample_size(20)
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(1));
    let mut rng = StdRng::from_seed(b"42424242424242424242424242424242".clone());
    for key_count in [1, 8, 32, 128, 1024].into_iter() {
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
                    "Vec + group_by",
                    format!("{},{}", key_count, value_key_ratio),
                ),
                &source,
                |b, source| b.iter(|| vec(source)),
            );
            group.bench_with_input(
                BenchmarkId::new("Hashmap", format!("{},{}", key_count, value_key_ratio)),
                &source,
                |b, source| b.iter(|| hashmap(source)),
            );
        }
    }
    group.finish();
}

criterion_group!(benches, bench_group_by);
criterion_main!(benches);
