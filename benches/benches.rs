use pdf_bitmap::extract_jpgs;

use criterion::{criterion_group, criterion_main, Criterion};

fn open_and_extract(c: &mut Criterion) {
    c.bench_function("pdf_bitmap", |b| {
        b.iter(|| {
            let data = std::fs::read("test.pdf").unwrap();
            let imgs = pdf_bitmap::extract_jpgs(&data);

            assert_eq!(imgs.len(), 20);
        })
    });
}

fn extract(c: &mut Criterion) {
    let data = std::fs::read("test.pdf").unwrap();

    c.bench_function("extract", |b| {
        b.iter(|| {
            let imgs = extract_jpgs(&data);
            assert_eq!(imgs.len(), 20);
        })
    });
}

criterion_group!(benches, extract, open_and_extract);
criterion_main!(benches);
