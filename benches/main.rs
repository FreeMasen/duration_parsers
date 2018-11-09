#![feature(test)]

extern crate test;
extern crate random_durations;
#[cfg(feature = "nom")]
extern crate nom_duration_parser;
#[cfg(feature = "pest")]
extern crate pest_duration_parser;
#[cfg(feature = "combine")]
extern crate combine_duration_parser;
#[cfg(feature = "hand")]
extern crate hand_rolled_duration_parser;

use test::{Bencher, black_box};

#[cfg(feature = "nom")]
#[bench]
fn nom(b: &mut Bencher) {
    b.iter(|| {
        let duration = random_durations::gen_random_dur();
        black_box(nom_duration_parser::parse(&format!("{}", duration))).unwrap();
    });
}


#[cfg(feature = "pest")]
#[bench]
fn pest(b: &mut Bencher) {
    b.iter(|| {
        let duration = random_durations::gen_random_dur();
        black_box(pest_duration_parser::parse(&format!("{}", duration))).unwrap();
    });
}

#[cfg(feature = "combine")]
#[bench]
fn combine(b: &mut Bencher) {
    b.iter(|| {
        let duration = random_durations::gen_random_dur();
        black_box(combine_duration_parser::parse(&format!("{}", duration))).unwrap();
    });
}

#[cfg(feature = "hand")]
#[bench]
fn hand_rolled(b: &mut Bencher) {
    b.iter(|| {
        let duration = random_durations::gen_random_dur();
        black_box(hand_rolled_duration_parser::parse(&format!("{}", duration))).unwrap();
    })
}