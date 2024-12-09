use crossbeam::channel::Receiver;
use std::collections::VecDeque;
use std::fmt::Debug;
use crate::calctrait::CanCalculate;

pub struct SlidingAlgorithm<T>
where
    T: CanCalculate + Debug,
    T::DiffType: CanCalculate + Default + Debug,
{
    max_window_size: usize,
    threshold: T::DiffType,
    data_receiver: Receiver<T>, // 消费者获取数据
}

impl<T> SlidingAlgorithm<T>
where
    T: CanCalculate + Debug,
    T::DiffType: CanCalculate + Default + Debug,
{
    pub fn new(
        max_window_size: usize,
        threshold: T::DiffType,
        data_receiver: Receiver<T>, // 消费者获取数据
    ) -> Self {
        SlidingAlgorithm {
            max_window_size,
            threshold,
            data_receiver,
        }
    }

    pub fn run(&self) {
        let mut window = VecDeque::new(); // 滑动窗口

        loop {
            // 获取数据
            let data = self.get_data();
            if data.is_none() {
                continue; // 如果没有数据，等待
            }
            let data = data.unwrap();

            // 向窗口添加数据
            window.push_back(data);

            // 计算变化率
            let current_max_change_rate = self.calculate_max_change_rate(&window);

            println!("windows: {:?}", window);

            // 判断变化率是否满足阈值，若满足则重启窗口
            if current_max_change_rate {
                println!("Reach the threshold, next window will be open.");
                window.clear(); // 清空窗口
            }

            // 窗口控制，超过最大窗口大小时丢弃最旧的数据
            if window.len() > self.max_window_size {
                window.pop_front();
            }
        }
    }

    // 从数据源获取数据
    fn get_data(&self) -> Option<T> {
        self.data_receiver.recv().ok()
    }

    // 计算窗口内所有数据的最大变化率
    fn calculate_max_change_rate(&self, window: &VecDeque<T>) -> bool {
        // 获取最后一个元素
        if let Some(ref last) = window.back() {
            // 计算最后一个元素与窗口内元素的变化率
            for i in 0..(window.len() - 1) {
                let val1 = &window[i];
                let val2 = last;

                // 计算变化率
                let index_diff = (window.len() - i - 1) as i32;

                // 判断是否达到阈值
                if val2.check_threshold(val1, index_diff, &self.threshold) {
                    return true; // 达到阈值，返回 true
                }
            }
        } else {
            println!("Deque is empty.");
        }
        false // 默认返回 false
    }
}