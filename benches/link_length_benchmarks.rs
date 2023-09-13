use criterion::{black_box, criterion_group, criterion_main, Criterion};
use network::Links;
use std::{path::PathBuf, mem};
use rand::{seq::IteratorRandom, rngs::StdRng, SeedableRng};

pub fn parse_link_lengths(c: &mut Criterion) {

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("fixtures/network_big.xml");
    let links = Links::from_xml(black_box(path), false).unwrap();
    println!("size:      {}", mem::size_of_val(&links));
    println!("size:      {}", mem::size_of_val(&links.lengths));


    c.bench_function("build link lengths from xml", |b| {
        b.iter(|| {
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.push("fixtures/network_big.xml");
            let _ = Links::from_xml(black_box(path), false).unwrap();
        })
    });
    

}

pub fn access_link_lengths(c: &mut Criterion) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("fixtures/network_big.xml");
    let links = Links::from_xml(path, false).unwrap();
    let mut rng = StdRng::seed_from_u64(1234);
    let mut link_keys: Vec<_> = (0..links.lengths.len()).collect::<Vec<_>>();
    link_keys.sort();
    let samples = black_box(link_keys.iter().choose_multiple(&mut rng, 1000));

    c.bench_function("query link lengths", |b| {
        b.iter(|| {
            for lid in &samples {
                links.get(**lid);
            }
        })
    });
}

criterion_group!(link_lengths_benchmarks, parse_link_lengths, access_link_lengths);
criterion_main!(link_lengths_benchmarks);