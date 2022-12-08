use std::{collections::HashSet, time::Duration};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use indexmap::IndexSet;
use itertools::Itertools;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

fn vec(source: &[u64], values: &[u64]) -> Vec<bool> {
    let vec = source.iter().cloned().collect_vec();
    values.iter().map(|it| vec.contains(it)).collect()
}

fn sorted_vec(source: &[u64], values: &[u64]) -> Vec<bool> {
    let mut vec = source.iter().cloned().collect_vec();
    vec.sort();
    values
        .iter()
        .map(|it| vec.binary_search(it).is_ok())
        .collect()
}

fn hashset(source: &[u64], values: &[u64]) -> Vec<bool> {
    let set: HashSet<_> = source.iter().cloned().collect();
    values.iter().map(|it| set.contains(it)).collect()
}

fn indexset(source: &[u64], values: &[u64]) -> Vec<bool> {
    let set: IndexSet<_> = source.iter().cloned().collect();
    values.iter().map(|it| set.contains(it)).collect()
}

fn bench_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains_trivial");
    group
        .sample_size(20)
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(1));
    let mut rng = StdRng::from_seed(*b"42424242424242424242424242424242");
    for item_count in [32, 128, 1024].into_iter() {
        let items = (0..item_count)
            .map(|_| rng.gen_range(0..0x80000000u64))
            .collect_vec();
        for search_times in [8, 32, 128] {
            for exist_all_ration in [0.1f64, 0.5, 0.9].into_iter() {
                let exist_count = ((search_times as f64) * exist_all_ration).round() as u64;
                if exist_count == 0 {
                    continue;
                }
                let mut find_items = items.iter().take(exist_count as _).cloned().collect_vec();
                while find_items.len() < search_times {
                    find_items.push(rng.gen_range(0x8000_0001_u64..0x0001_0000_0000_u64));
                }
                find_items.shuffle(&mut rng);
                group.bench_with_input(
                    BenchmarkId::new(
                        "Vec",
                        format!(
                            "{} times,{} items,{}% found",
                            search_times,
                            item_count,
                            exist_all_ration * 100f64
                        ),
                    ),
                    &(&items, &find_items),
                    |b, (items, find_items)| b.iter(|| vec(items, find_items)),
                );
                group.bench_with_input(
                    BenchmarkId::new(
                        "sorted vec",
                        format!(
                            "{} times,{} items,{}% found",
                            search_times,
                            item_count,
                            exist_all_ration * 100f64
                        ),
                    ),
                    &(&items, &find_items),
                    |b, (items, find_items)| b.iter(|| sorted_vec(items, find_items)),
                );
                group.bench_with_input(
                    BenchmarkId::new(
                        "HashSet",
                        format!(
                            "{} times,{} items,{}% found",
                            search_times,
                            item_count,
                            exist_all_ration * 100f64
                        ),
                    ),
                    &(&items, &find_items),
                    |b, (items, find_items)| b.iter(|| hashset(items, find_items)),
                );
                group.bench_with_input(
                    BenchmarkId::new(
                        "HashSet",
                        format!(
                            "{} times,{} items,{}% found",
                            search_times,
                            item_count,
                            exist_all_ration * 100f64
                        ),
                    ),
                    &(&items, &find_items),
                    |b, (items, find_items)| b.iter(|| indexset(items, find_items)),
                );
            }
        }
    }
    group.finish();
}

criterion_group!(benches, bench_contains);
criterion_main!(benches);
