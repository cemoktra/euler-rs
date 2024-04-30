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

#[divan::bench(args = [1000])]
fn bench_00017(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00017::solve(n));
}

#[divan::bench]
fn bench_00018(bencher: divan::Bencher) {
    bencher.bench_local(move || euler::euler::e00018::solve(euler::euler::e00018::DATA));
}

#[divan::bench]
fn bench_00019(bencher: divan::Bencher) {
    bencher.bench_local(euler::euler::e00019::solve);
}

#[divan::bench(args = [100])]
fn bench_00020(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00020::solve(n));
}

#[divan::bench(args = [10000])]
fn bench_00021(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00021::solve(n));
}

#[divan::bench]
fn bench_00022(bencher: divan::Bencher) {
    let file_content = std::fs::read_to_string("data/e00022.txt").unwrap();
    let mut names = file_content
        .split(',')
        .map(|name| &name[1..name.len() - 1])
        .collect::<Vec<_>>();

    bencher.bench_local(move || euler::euler::e00022::solve(names.as_mut_slice()));
}

#[divan::bench]
fn bench_00023(bencher: divan::Bencher) {
    bencher.bench_local(euler::euler::e00023::solve);
}

#[divan::bench(args = [1_000_000])]
fn bench_00024(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00024::solve(n));
}

#[divan::bench(args = [1_000])]
fn bench_00025(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00025::solve(n));
}

#[divan::bench(args = [1_000])]
fn bench_00026(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00026::solve(n));
}

#[divan::bench(args = [1_000])]
fn bench_00027(bencher: divan::Bencher, n: i64) {
    bencher.bench_local(move || euler::euler::e00027::solve(n));
}

#[divan::bench(args = [1_001])]
fn bench_00028(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00028::solve(n));
}

#[divan::bench(args = [100])]
fn bench_00029(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00029::solve(n));
}

#[divan::bench(args = [5])]
fn bench_00030(bencher: divan::Bencher, n: u32) {
    bencher.bench_local(move || euler::euler::e00030::solve(n));
}

#[divan::bench(args = [200])]
fn bench_00031(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00031::solve(n));
}

#[divan::bench()]
fn bench_00032(bencher: divan::Bencher) {
    bencher.bench_local(euler::euler::e00032::solve);
}

#[divan::bench()]
fn bench_00033(bencher: divan::Bencher) {
    bencher.bench_local(euler::euler::e00033::solve);
}

#[divan::bench()]
fn bench_00034(bencher: divan::Bencher) {
    bencher.bench_local(euler::euler::e00034::solve);
}

#[divan::bench(args = [1_000_000])]
fn bench_00035(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00035::solve(n));
}

#[divan::bench(args = [1_000_000])]
fn bench_00036(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00036::solve(n));
}

#[divan::bench(args = [1_000_000])]
fn bench_00037(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00037::solve(n));
}

#[divan::bench()]
fn bench_00038(bencher: divan::Bencher) {
    bencher.bench_local(euler::euler::e00038::solve);
}

#[divan::bench()]
fn bench_00039(bencher: divan::Bencher) {
    bencher.bench_local(euler::euler::e00039::solve);
}

#[divan::bench()]
fn bench_00040(bencher: divan::Bencher) {
    bencher.bench_local(euler::euler::e00040::solve);
}

#[divan::bench()]
fn bench_00041(bencher: divan::Bencher) {
    bencher.bench_local(euler::euler::e00041::solve);
}

#[divan::bench]
fn bench_00042(bencher: divan::Bencher) {
    let file_content = std::fs::read_to_string("data/e00042.txt").unwrap();
    let words = file_content
        .split(',')
        .map(|word| word.trim_end_matches('\n').trim_matches('"'))
        .collect::<Vec<_>>();

    bencher.bench_local(move || euler::euler::e00042::solve(words.as_slice()));
}

#[divan::bench()]
fn bench_00043(bencher: divan::Bencher) {
    bencher.bench_local(euler::euler::e00043::solve);
}

#[divan::bench()]
fn bench_00044(bencher: divan::Bencher) {
    bencher.bench_local(euler::euler::e00044::solve);
}

#[divan::bench()]
fn bench_00045(bencher: divan::Bencher) {
    bencher.bench_local(euler::euler::e00045::solve);
}

#[divan::bench()]
fn bench_00046(bencher: divan::Bencher) {
    bencher.bench_local(euler::euler::e00046::solve);
}

#[divan::bench(args = [3, 4])]
fn bench_00047(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00047::solve(n));
}

#[divan::bench(args = [1_000])]
fn bench_00048(bencher: divan::Bencher, n: u128) {
    bencher.bench_local(move || euler::euler::e00048::solve(n));
}

#[divan::bench()]
fn bench_00049(bencher: divan::Bencher) {
    bencher.bench_local(euler::euler::e00049::solve);
}

#[divan::bench(args = [1_000, 1_000_000])]
fn bench_00050(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00050::solve(n));
}

#[divan::bench()]
fn bench_00051(bencher: divan::Bencher) {
    bencher.bench_local(euler::euler::e00051::solve);
}

#[divan::bench()]
fn bench_00052(bencher: divan::Bencher) {
    bencher.bench_local(euler::euler::e00052::solve);
}

#[divan::bench(args = [1_000_000])]
fn bench_00053(bencher: divan::Bencher, n: usize) {
    bencher.bench_local(move || euler::euler::e00053::solve(n));
}

#[divan::bench]
fn bench_00054(bencher: divan::Bencher) {
    let file_content = std::fs::read_to_string("data/e00054.txt").unwrap();
    let hand_pairs = file_content
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let hand1 = (&line[0..14]).into();
            let hand2 = (&line[15..]).into();
            (hand1, hand2)
        })
        .collect::<Vec<_>>();

    bencher.bench_local(move || euler::euler::e00054::solve(&hand_pairs));
}

#[divan::bench(args = [10_000])]
fn bench_00055(bencher: divan::Bencher, n: u128) {
    bencher.bench_local(move || euler::euler::e00055::solve(n));
}
