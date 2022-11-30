use std::{collections::BTreeSet, iter, time::Duration};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use itertools::Itertools;
use rand::{distributions::Alphanumeric, rngs::StdRng, Rng, RngCore, SeedableRng};

fn vec_sort(source: &[String]) -> Vec<String> {
    let mut result: Vec<_> = source.iter().cloned().collect();
    result.sort();
    result.dedup();
    result
}

fn btreeset_auto(source: &[String]) -> BTreeSet<String> {
    source.iter().cloned().collect()
}

fn bench_sort_with_dedup(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort_and_dedup_trivial");
    group
        .sample_size(20)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(1));
    let mut rng = StdRng::from_seed(b"42424242424242424242424242424242".clone());
    for item_count in [8, 32, 128, 2048].into_iter() {
        let items = (0..item_count)
                .map(|_| {
                    let len = rng.gen_range(2..32);
                    (&mut rng)
                        .sample_iter(&Alphanumeric)
                        .take(len)
                        .map(char::from)
                        .collect::<String>()
                })
                .collect_vec();
        for dup_probability in [0, 10, 50, 90, 100, 200, 1000] {
            let mut source = Vec::new();
            for item in items.iter() {
                source.push(item.clone());
                if dup_probability < 100 {
                    if rng.gen_ratio(dup_probability, 100) {
                        source.push(item.clone());
                    }
                } else {
                    for _ in 0..(dup_probability / 100) {
                        source.push(item.clone());
                    }
                }
            }
            group.bench_with_input(
                BenchmarkId::new("Vec+sort", format!("{},{}%", item_count, dup_probability)),
                &source,
                |b, source| b.iter(|| vec_sort(&source)),
            );
            group.bench_with_input(
                BenchmarkId::new("BTreeSet", format!("{},{}%", item_count, dup_probability)),
                &source,
                |b, source| b.iter(|| btreeset_auto(&source)),
            );
        }
    }
    group.finish();
}

criterion_group!(benches, bench_sort_with_dedup);
criterion_main!(benches);
