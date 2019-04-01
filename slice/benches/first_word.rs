#[macro_use]
extern crate bencher;

use bencher::Bencher;

#[path="../src/utils.rs"]
mod utils;
use utils::first_word;
use utils::first_word2;

const TEST_COUNT: u32 = 100;

fn bench_first_word(b: &mut Bencher) {
    let s = String::from("hello world");
    b.iter(|| {
        // Inner closure, the actual test
        for _i in 1..TEST_COUNT {
            let _hello = first_word(&s);
        }
    });
}

fn bench_first_word2(b: &mut Bencher) {
    let s = String::from("hello world");
    b.iter(|| {
        // Inner closure, the actual test
        for _i in 1..TEST_COUNT {
            let _hello = first_word2(&s);
        }
    });
}

benchmark_group!(benches, bench_first_word, bench_first_word2);
benchmark_main!(benches);
