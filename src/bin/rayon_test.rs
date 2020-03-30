extern crate rayon;

use rayon::{ThreadPool, ThreadPoolBuilder};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const RAYON_THREAD_NUM: usize = 32;
const ACCESS_THREAD_NUM: usize = 64;
const TEST_TIMES_PER_THREAD: usize = 10000;
const MAX_RAND_ITEMS: usize = 10;
const MAX_RAND_SLEEP_MS: u64 = 100;

fn rand_items() -> usize {
    let num = vec![1, 2];
    let address = &num as *const Vec<i32>;
    address as usize % MAX_RAND_ITEMS
}

fn rand_sleep_ms() -> u64 {
    let num = vec![1, 2];
    let address = &num as *const Vec<i32>;
    address as u64 % MAX_RAND_SLEEP_MS
}

fn main() {
    let pool = Arc::new(
        ThreadPoolBuilder::new()
            .num_threads(RAYON_THREAD_NUM)
            .thread_name(|index| format!("rayon-{}", index))
            .build()
            .unwrap(),
    );

    println!("test begin");

    let mut handles = Vec::with_capacity(ACCESS_THREAD_NUM);
    for i in 0..ACCESS_THREAD_NUM {
        let pool_clone = pool.clone();

        let handle = thread::spawn(move || {
            test_pool(i, pool_clone, TEST_TIMES_PER_THREAD);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("test done");
}

fn test_pool(id: usize, pool: Arc<ThreadPool>, test_times: usize) {
    let thread_name = format!("thread-{}", id);

    for i in 0..test_times {
        run_pool1(&pool, &thread_name, i);

        run_pool2(&pool, &thread_name, i);
    }
}

fn run_pool1(pool: &ThreadPool, thread_name: &str, i: usize) {
    let item_num = rand_items();
    let unused2: [u8; 100] = [19; 100];

    pool.scope(|s| {
        println!("run_pool1 in {} times {}", thread_name, i);

        let _unused3: [u8; 128] = [20; 128];
        let address = &unused2 as *const _;
        let num = address as usize % 60;

        // for sleep_ms in items {
        for _ in 0..(item_num + num) {
            s.spawn(move |_| {
                let du = Duration::from_millis(rand_sleep_ms());
                // let du = Duration::from_millis(sleep_ms);
                thread::sleep(du);
            })
        }
    });
}

fn run_pool2(pool: &ThreadPool, thread_name: &str, i: usize) {
    let item_num = rand_items();
    let _unused1: [u8; 100] = [18; 100];
    let unused2: [u8; 100] = [19; 100];

    pool.scope(|s| {
        println!("run_pool2 in {} times {}", thread_name, i);

        let _unused3: [u8; 128] = [20; 128];
        let address = &unused2 as *const _;
        let num = address as usize % 30;

        // for sleep_ms in items {
        for _ in 0..(item_num + num) {
            s.spawn(move |_| {
                let du = Duration::from_millis(rand_sleep_ms());
                // let du = Duration::from_millis(sleep_ms);
                thread::sleep(du);
            })
        }
    });
}
