use aoe2rec::Savegame;
use criterion::BenchmarkId;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("test_replays");
    group.sample_size(10);
    group.sampling_mode(criterion::SamplingMode::Flat);
    group.significance_level(0.1);

    println!("{:?}", std::env::current_dir());
    for entry in std::fs::read_dir("recs").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let extension = match path.extension() {
            Some(extension) => extension,
            None => {
                continue;
            }
        };
        if extension != "aoe2record" {
            continue;
        }

        let replay_data = match std::fs::read(&path) {
            Ok(data) => data,
            Err(err) => {
                continue;
            }
        };

        let name = match path.to_str() {
            Some(name) => name,
            None => {
                continue;
            }
        };

        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &replay_data,
            |b, replay_data| {
                b.iter(|| {
                    let savegame = Savegame::from_slice(&replay_data);
                    let _ = black_box(savegame);
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
