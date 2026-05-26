use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn generate_history(size: usize) -> String {
    let commands = [
        "ls", "git", "cd", "cargo", "docker", "kubectl", "vim", "cat", "echo", "ssh",
    ];
    let mut lines = Vec::with_capacity(size);
    for i in 0..size {
        let cmd = commands[i % commands.len()];
        let suffix = if i % 3 == 0 {
            " --verbose --long-flag --another=value"
        } else {
            ""
        };
        lines.push(format!("{cmd}{suffix}"));
    }
    lines.join("\n")
}

fn bench_analyze(c: &mut Criterion) {
    let input_100 = generate_history(100);
    let input_1k = generate_history(1_000);
    let input_10k = generate_history(10_000);
    let input_100k = generate_history(100_000);

    let mut group = c.benchmark_group("analyze");
    group.bench_function("100 entries", |b| {
        b.iter(|| shellist::analyze(black_box(&input_100)))
    });
    group.bench_function("1k entries", |b| {
        b.iter(|| shellist::analyze(black_box(&input_1k)))
    });
    group.bench_function("10k entries", |b| {
        b.iter(|| shellist::analyze(black_box(&input_10k)))
    });
    group.bench_function("100k entries", |b| {
        b.iter(|| shellist::analyze(black_box(&input_100k)))
    });
    group.finish();
}

criterion_group!(benches, bench_analyze);
criterion_main!(benches);
