use rayon::iter::*;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::time::Instant;
use std::{fs, io, thread};

// 让 build 生成的文件携带这个 标志
const FLAG: &str = "FLAG-123456";

fn main() {
    let begin = Instant::now();

    let (tx, rx) = mpsc::channel();
    let res = thread::spawn(|| rx.into_iter().collect());
    file_content_search(".", FLAG.as_bytes(), tx);
    let res: Vec<PathBuf> = res.join().unwrap();

    println!("找到:{} 耗时:{:.2?}", res.len(), begin.elapsed())
}

fn file_content_search<P: AsRef<Path>>(path: P, search: &[u8], tx: Sender<PathBuf>) {
    fs::read_dir(path)
        .unwrap()
        .par_bridge()
        .filter_map(Result::ok)
        .for_each(|entry| {
            if entry.metadata().unwrap().is_dir() {
                return file_content_search(entry.path(), search, tx.clone());
            }
            let path = entry.path();
            if let Ok(true) = bytes_search(&path, search) {
                tx.send(path).unwrap()
            }
        });
}

fn bytes_search<P: AsRef<Path>>(path: P, search: &[u8]) -> io::Result<bool> {
    let mut file = fs::File::open(path)?;
    let mut buf = [0; 1 << 10];
    let mut offset = 0;

    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break Ok(false); // EOF
        }

        unsafe {
            for item in buf.get_unchecked(0..n).iter() {
                if offset == search.len() {
                    return Ok(true);
                }
                if item == search.get_unchecked(offset) {
                    offset += 1
                } else {
                    offset = 0
                }
            }
        }
    }
}
