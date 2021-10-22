use bio::Seq;
use criterion::*;
use std::fs::read_to_string;

static FILENAME: &str = "benches/ecoli.fasta";

fn read_counts(name: &str) {
	let r = read_to_string(name).unwrap();

	let s = Seq::new(&r);
	let _ = s.counts();
}

fn run(c: &mut Criterion) {
	c.bench_function("read_counts", |b| {
		b.iter(|| read_counts(black_box(FILENAME)))
	});
}

criterion_group!(bench, run);
criterion_main!(bench);
