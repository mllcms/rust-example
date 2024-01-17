use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let begin = Instant::now();

    let n: usize = 1_000_000;
    let res = (2..n)
        .par_bridge()
        .filter(|&i| (2..i).find(|f| i % f == 0).is_none())
        .count();

    include_str!("dir_size.rs");
    println!("范围:2-{n} 数量:{res} 耗时:{:.2?}", begin.elapsed());
}
