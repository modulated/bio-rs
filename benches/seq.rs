use bio::FASTA;
use criterion::*;

static ECOLI: &str = "benches/ecoli.fasta";
static SAL: &str = "benches/salmonella.fasta";

fn read_counts(name: &str) {
	let r = FASTA::from_file(name);

	let _ = r.seq.counts();
}

fn orf(fasta: &FASTA) {
	let _r = fasta.seq.orf(1);
}

fn len(seq: &FASTA) {
	let _r = seq.seq.len();
}

fn run(c: &mut Criterion) {
	c.bench_function("read_counts", |b| b.iter(|| read_counts(black_box(ECOLI))));

	let sal = FASTA::from_file(SAL);

	c.bench_with_input(BenchmarkId::new("ORF Salmonella", &sal), &sal, |b, s| {
		b.iter(|| orf(s));
	});

	let ecoli = FASTA::from_file(ECOLI);

	c.bench_with_input(BenchmarkId::new("Len E Coli", &ecoli), &ecoli, |b, s| {
		b.iter(|| len(s));
	});
}

criterion_group!(bench, run);
criterion_main!(bench);
