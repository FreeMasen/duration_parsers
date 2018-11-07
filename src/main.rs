extern crate duration;
extern crate random_durations;
#[cfg(feature = "nom")]
extern crate nom_duration_parser;
#[cfg(feature = "combine")]
extern crate combine_duration_parser;
#[cfg(feature = "pest")]
extern crate pest_duration_parser;


fn main() {
    if cfg!(feature = "nom") {
        for d in random_durations::gen_random_durs(1000) {
            let s = format!("{}", d);
            let p = nom_duration_parser::parse(&s).unwrap();
            assert_eq!(d, p);
        }
    }
    if cfg!(feature = "combine") {
        for d in random_durations::gen_random_durs(1000) {
            let s = format!("{}", d);
            let p = combine_duration_parser::parse(&s).unwrap();
            assert_eq!(d, p);
        }
    }
    if cfg!(feature = "pest") {
        for d in random_durations::gen_random_durs(1000) {
            let s = format!("{}", d);
            let p = pest_duration_parser::parse(&s).unwrap();
            assert_eq!(d, p);
        }
    }
    if cfg!(not(any(feature = "nom", feature = "combine", feature = "pest"))) {
        println!("{}", random_durations::gen_random_durs_text(1000).join("\n"));
    }
}

