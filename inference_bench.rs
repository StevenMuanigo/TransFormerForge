use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn benchmark_preprocessing(c: &mut Criterion) {
    c.bench_function("text_preprocessing", |b| {
        let text = "This is a sample text for benchmarking preprocessing performance!";
        b.iter(|| {
            black_box(text.to_lowercase())
        });
    });
}

fn benchmark_tokenization(c: &mut Criterion) {
    let mut group = c.benchmark_group("tokenization");
    
    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let text = "word ".repeat(size);
            b.iter(|| {
                black_box(text.split_whitespace().collect::<Vec<_>>())
            });
        });
    }
    
    group.finish();
}

criterion_group!(benches, benchmark_preprocessing, benchmark_tokenization);
criterion_main!(benches);
