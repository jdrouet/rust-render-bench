use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{black_box, criterion_group, criterion_main};

// This is a struct that tells Criterion.rs to use the "futures" crate's current-thread executor
use criterion::async_executor::FuturesExecutor;
use rust_render_bench::{
    sailfish::{render, render_async},
    TodoItem,
};

fn rendering(c: &mut Criterion) {
    for size in [10usize, 1000] {
        c.bench_with_input(BenchmarkId::new("sailfish async", size), &size, |b, &s| {
            let list = (0..s)
                .map(|idx| TodoItem {
                    done: idx % 2 == 0,
                    content: format!("Item {idx}"),
                })
                .collect::<Vec<_>>();
            b.to_async(FuturesExecutor)
                .iter(|| render_async(black_box(list.clone())));
        });
        c.bench_with_input(BenchmarkId::new("sailfish sync", size), &size, |b, &s| {
            let list = (0..s)
                .map(|idx| TodoItem {
                    done: idx % 2 == 0,
                    content: format!("Item {idx}"),
                })
                .collect::<Vec<_>>();
            b.iter(|| render(black_box(list.clone())));
        });
    }
}

criterion_group!(benches, rendering);
criterion_main!(benches);
