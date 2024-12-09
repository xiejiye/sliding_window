mod data_source;
mod sliding; // 顶级声明 data_source 模块
mod calctrait;
mod calc;

use crate::data_source::data_generate::DataSource;
use crate::sliding::SlidingAlgorithm;
use crossbeam::channel::unbounded;
use std::sync::{Arc, Mutex};
use std::thread;
// use crate::sliding;

fn main() {
    let data = vec![1, 3, 4, 6, 9, 12, 15, 17, 19];

    // 创建缓冲通道
    let (data_sender, data_receiver) = unbounded::<i32>();
    let data_source = Arc::new(Mutex::new(DataSource::new(data, data_sender)));

    // 启动线程并调用 run 方法
    thread::spawn({
        let data_source = Arc::clone(&data_source); // 克隆 Arc，跨线程共享
        move || {
            let data_source = data_source.lock().unwrap(); // 锁住数据源
            data_source.run(); // 调用 run 方法
        }
    });

    // 创建滑动算法实例
    let sliding_algorithm = SlidingAlgorithm::new(50, 500, data_receiver);

    // 启动滑动算法线程
    sliding_algorithm.run();
}
