use aoc2019::{self, day01, day02, day03};
use criterion::{criterion_group, criterion_main, Criterion};
use std::{fs, io, time::Duration};

fn target01(c: &mut Criterion) {
    let day01 = fs::read_to_string("data/01.txt").unwrap();
    c.bench_function("day_01", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day01.as_bytes());
            day01::run(reader)
        })
    });
}
fn target02(c: &mut Criterion) {
    let day02 = fs::read_to_string("data/02.txt").unwrap();
    c.bench_function("day_02", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day02.as_bytes());
            day02::run(reader)
        })
    });
}
fn target03(c: &mut Criterion) {
    let day03 = fs::read_to_string("data/03.txt").unwrap();
    c.bench_function("day_03", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day03.as_bytes());
            day03::run(reader)
        })
    });
}
criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().warm_up_time(Duration::from_secs(5));
    targets = target01, target02, target03
}

criterion_main!(benches);
