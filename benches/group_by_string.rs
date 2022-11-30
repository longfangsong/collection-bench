use std::{
    collections::{BTreeSet, HashMap, HashSet},
    iter,
    time::Duration,
};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use itertools::Itertools;
use rand::{distributions::Alphanumeric, rngs::StdRng, Rng, RngCore, SeedableRng};

fn vec(source: &[(String, String)]) -> Vec<(String, Vec<String>)> {
    source
        .into_iter()
        .group_by(|(it, _)| it)
        .into_iter()
        .map(|(key, group)| {
            (
                key.clone(),
                group.into_iter().map(|it| it.1.clone()).collect(),
            )
        })
        .collect()
}

fn hashmap(source: &[(String, String)]) -> HashMap<String, Vec<String>> {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    for (k, v) in source.into_iter() {
        result.entry(k.clone()).or_default().push(v.clone());
    }
    result
}

fn bench_group_by_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("group_by_string");
    group
        .sample_size(20)
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(1));
    let mut rng = StdRng::from_seed(b"42424242424242424242424242424242".clone());
    for key_count in [8, 32, 128].into_iter() {
        let keys = (0u64..key_count)
            .map(|_| {
                let len = rng.gen_range(2..32);
                (&mut rng)
                    .sample_iter(&Alphanumeric)
                    .take(len)
                    .map(char::from)
                    .collect::<String>()
            })
            .collect_vec();
        for value_key_ratio in [1, 2, 32] {
            let values = (0..(key_count * value_key_ratio))
                .map(|_| {
                    let len = rng.gen_range(2..32);
                    (&mut rng)
                        .sample_iter(&Alphanumeric)
                        .take(len)
                        .map(char::from)
                        .collect::<String>()
                })
                .collect_vec();
            let mut source = Vec::new();
            for key in &keys {
                for value in &values {
                    source.push((key.clone(), value.clone()));
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
        }
    }
    group.finish();
}

criterion_group!(benches, bench_group_by_string);
criterion_main!(benches);
