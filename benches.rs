fn main() {
    divan::main();
}

#[divan::bench(args = [1_000_000, 10_000_000])]
fn bench_primes(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::core::primes::Primes::default().take(n));
}

#[divan::bench(args = [1_000, 10_000, 100_000])]
fn bench_00001(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00001::solve(n));
}

#[divan::bench(args = [1_000_000, 10_000_000, 100_000_000])]
fn bench_00002(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00002::solve(n));
}

#[divan::bench(args = [13_195, 600_851_475_143])]
fn bench_00003(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00003::solve(n));
}

#[divan::bench(args = [100, 1000])]
fn bench_00004(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00004::solve(n));
}

#[divan::bench(args = [10, 15, 20])]
fn bench_00005(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00005::solve(n));
}

#[divan::bench(args = [10, 100])]
fn bench_00006(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00006::solve(n));
}

#[divan::bench(args = [6, 10_001])]
fn bench_00007(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00007::solve(n));
}

#[divan::bench(args = [4, 13])]
fn bench_00008(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00008::solve(n));
}

#[divan::bench(args = [1_000])]
fn bench_00009(bencher: divan::Bencher, n: i64) {
    bencher.bench_local(move || euler::euler::e00009::solve(n));
}

#[divan::bench(args = [10, 200_000])]
fn bench_00010(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00010::solve(n));
}

#[divan::bench(args = [4])]
fn bench_00011(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00011::solve(n));
}

#[divan::bench(args = [5, 500])]
fn bench_00012(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00012::solve(n));
}

#[divan::bench(args = [10])]
fn bench_00013(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00013::solve(n));
}

#[divan::bench(args = [100_000])]
fn bench_00014(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00014::solve(n));
}

#[divan::bench(args = [20])]
fn bench_00015(bencher: divan::Bencher, n: u128) {
    bencher.bench_local(move || euler::euler::e00015::solve(n));
}

#[divan::bench(args = [1000])]
fn bench_00016(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00016::solve(n));
}
