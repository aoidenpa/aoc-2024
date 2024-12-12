use aoc_template::day_executer::execute_day;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn run_day(c: &mut Criterion, day: u32, part: u32) {
    c.bench_function(&format!("day{}-part{}", day, part), |b| {
        b.iter(|| execute_day(black_box(day), black_box(part), false))
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    // for day in 1..=aoc_client::last_unlocked_day(2024).expect("AoC 2024 is not unlocked yet") {
    //     for part in 1..=2 {
    //         run_day(c, day, part);
    //     }
    // }
    run_day(c, 6, 2);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
