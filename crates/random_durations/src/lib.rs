extern crate rand;
extern crate duration;

use duration::Duration;
use rand::prelude::*;

pub fn gen_random_durs(num: usize) -> Vec<Duration> {
    let mut ret = Vec::with_capacity(num);
    for _ in 0..num {
        ret.push(gen_random_dur());
    }
    ret
}

pub fn gen_random_durs_text(num: usize) -> Vec<String> {
    gen_random_durs(num).iter().map(|d| format!("{}", d)).collect()
}

pub fn gen_random_dur() -> Duration {
    fn rand(rng: &mut ThreadRng) -> f32 {
        let ret = rng.gen_range(1.0, 100.0);
        let bigger: f32 = ret * 1000.0;
        bigger.floor() / 100.0
    }
    let mut rng = thread_rng();
    let mut ret = Duration::new();
    if rng.gen() {
        ret.set_years(rand(&mut rng));
    }
    if rng.gen() {
        ret.set_months(rand(&mut rng));
    }
    if rng.gen() {
        ret.set_weeks(rand(&mut rng));
    }
    if rng.gen() {
        ret.set_days(rand(&mut rng));
    }
    if rng.gen() {
        ret.set_hours(rand(&mut rng));
    }
    if rng.gen() {
        ret.set_minutes(rand(&mut rng));
    }
    if rng.gen() {
        ret.set_seconds(rand(&mut rng));
    }
    if ret.is_empty() {
        gen_random_dur()
    } else {
        ret
    }
}
