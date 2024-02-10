use rand::{thread_rng, Rng};

fn main() {
    let mut vec = vec![1, 2, 3, 4, 1, 2, 3, 4];
    let mut rng = thread_rng();
    println!("{:?} 初始状态", vec);

    swap(&mut vec, rng.gen_range(2..=7));
    println!("{:?} 拿走头名字长度(2-7)插入尾部 (怎么变n+4都会相同)", vec);

    let temp: Vec<_> = vec.drain(..3).collect();
    let index = rng.gen_range(1..vec.len() - 1);
    vec.splice(index..index, temp);
    println!("{vec:?} 拿走头3张插入中间 (插到4-8之间刚好首尾一样)");

    let target = vec.remove(0);
    println!("{vec:?} 拿走首牌 {target} (尾不变)");

    let temp: Vec<_> = vec.drain(..rng.gen_range(1..=3)).collect();
    let index = rng.gen_range(1..vec.len() - 1);
    vec.splice(index..index, temp);
    println!("{vec:?} 南1 北2 不清楚3 插入中间 (尾不变)");

    vec.drain(..rng.gen_range(1..=2));
    println!("{vec:?} 男1 女2 删除 (尾不变)");

    swap(&mut vec, 7);
    println!("{vec:?} 见证奇迹的时刻 (尾牌: 男倒2 女倒3)");

    while vec.len() > 1 {
        let temp = vec.remove(0);
        vec.push(temp);
        vec.remove(0);
        println!("{vec:?} 好运留下来烦恼丢出去")
    }
    assert_eq!(target, vec.remove(0))
}
fn swap<T>(vec: &mut Vec<T>, n: usize) {
    for _ in 0..n {
        let val = vec.remove(0);
        vec.push(val)
    }
}
