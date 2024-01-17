use rayon::prelude::*;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

fn main() {
    let begin = Instant::now();
    let (size, count) = dir_size(r#"C:\Mll"#);
    let size = (size as f64) / (1 << 30) as f64;
    println!("数量:{count} 大小:{size:.2}G 耗时:{:.2?}", begin.elapsed())
}

pub fn dir_size<P: AsRef<Path>>(path: P) -> (u64, u64) {
    // for_each 并行执行会有数据竞态问题它所接受的闭包是 Fn 也就是不可变变量
    // 使用原子 U64 来计数, 内部有原子锁防止数据竞态
    let size = AtomicU64::new(0);
    let count = AtomicU64::new(0);
    fs::read_dir(path)
        .unwrap()
        .par_bridge()
        .filter_map(Result::ok)
        .for_each(|entry| {
            let meta = entry.metadata().unwrap();
            // 过滤 link 文件
            if meta.is_symlink() {
                return;
            }

            count.fetch_add(1, Ordering::SeqCst);
            if meta.is_dir() {
                let (s, c) = dir_size(entry.path()); // 递归调用进行深度计算
                size.fetch_add(s, Ordering::SeqCst);
                count.fetch_add(c, Ordering::SeqCst);
            } else {
                size.fetch_add(meta.len(), Ordering::SeqCst);
            }
        });
    (size.into_inner(), count.into_inner())
}
