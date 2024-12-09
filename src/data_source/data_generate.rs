use crossbeam::channel::{unbounded, Receiver, Sender};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use std::sync::{Arc, Mutex};
use std::{thread, time};

pub struct DataSource {
    data: Vec<i32>,
    current_index: usize,
    data_sender: Sender<i32>, // 消费者获取数据
}

impl DataSource {
    pub fn new(data: Vec<i32>, data_sender: Sender<i32>) -> Self {
        DataSource {
            data,
            current_index: 0,
            data_sender,
        }
    }

    // 启动数据生产
    pub fn run(&self) {
        // 创建一个线程局部的随机数生成器
        let mut  rng = thread_rng();

        // 定义一个 `i32` 类型的范围，例如从 -50 到 50
        let range = Uniform::from(-1000i32..=1000i32);

        // 一次生成 10 个符合指定范围的 `i32` 类型随机数
        let random_numbers: Vec<i32> = (0..500)
            .map(|_| rng.sample(&range)) // 从范围中随机选择
            .collect();
        for data in random_numbers {
            self.data_sender.send(data).unwrap();
            // 模拟数据生产间隔
            thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}
