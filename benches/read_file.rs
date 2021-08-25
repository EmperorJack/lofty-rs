use criterion::{criterion_group, criterion_main, Criterion};
use lofty::Probe;

macro_rules! test_read {
	($function:ident, $path:expr) => {
		fn $function() {
			Probe::new().read_from_path($path).unwrap();
		}
	};
}

test_read!(read_aiff, "tests/assets/a_text.aiff");
test_read!(read_ape, "tests/assets/a.ape");
test_read!(read_flac, "tests/assets/a.flac");
// test_read!(read_m4a, "tests/assets/a.m4a"); TODO
test_read!(read_mp3, "tests/assets/a.mp3");
test_read!(read_vorbis, "tests/assets/a.ogg");
test_read!(read_opus, "tests/assets/a.opus");
test_read!(read_riff, "tests/assets/a.wav");

fn bench_sig(c: &mut Criterion) {
	let mut g = c.benchmark_group("File reading");
	g.bench_function("AIFF", |b| b.iter(read_aiff));
	g.bench_function("APE", |b| b.iter(read_ape));
	g.bench_function("FLAC", |b| b.iter(read_flac));
	// g.bench_function("MP4", |b| b.iter(read_m4a));
	g.bench_function("MP3", |b| b.iter(read_mp3));
	g.bench_function("VORBIS", |b| b.iter(read_vorbis));
	g.bench_function("OPUS", |b| b.iter(read_opus));
	g.bench_function("RIFF", |b| b.iter(read_riff));
}

criterion_group!(benches, bench_sig);
criterion_main!(benches);
