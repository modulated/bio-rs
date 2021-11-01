use bio::FASTA;
use criterion::*;

static SAL: &str = "benches/salmonella.fasta";

fn read_counts(f: &FASTA) {
	let _r = f.seq.counts();
}

fn orf(f: &FASTA) {
	let _r = f.seq.orf(1);
}

fn len(f: &FASTA) {
	let _r = f.seq.len();
}

fn run(c: &mut Criterion) {
	let sal = FASTA::from_file(SAL);

	c.bench_function("LEN", |b| b.iter(|| len(&sal)));
	c.bench_function("COUNTS", |b| b.iter(|| read_counts(&sal)));
	c.bench_function("ORF", |b| b.iter(|| orf(&sal)));
}

criterion_group!(bench, run);
criterion_main!(bench);
