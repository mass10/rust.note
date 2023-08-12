use std::ops;

/// 何らかのキュー
struct MyStringQueue {
    queue: std::vec::Vec<String>,
}

impl MyStringQueue {
    /// コンストラクター
    fn new() -> MyStringQueue {
        MyStringQueue {
            queue: std::vec::Vec::new(),
        }
    }

    /// オブジェクトをトレースする
    fn dump(&self) {
        println!("MyTaskmanager trace");
        for i in &self.queue {
            println!("> [{}]", i);
        }
    }
}

/// デストラクターの実装
impl Drop for MyStringQueue {
    fn drop(&mut self) {
        println!("Dropping MyTaskmanager");
    }
}

/// AddAssign(operator +=)の実装
impl ops::AddAssign<String> for MyStringQueue {
    fn add_assign(&mut self, rhs: String) {
        println!("AddAssign");
        self.queue.push(rhs);
    }
}

/// Add(operator +)の実装
impl ops::Add<String> for MyStringQueue {
    type Output = MyStringQueue;

    fn add(mut self, rhs: String) -> MyStringQueue {
        println!("Add");
        self.queue.push(rhs);
        self
    }
}

/// エントリーポイント
fn main() {
    let mut taskman = MyStringQueue::new();

    taskman += String::new();
    taskman += String::from("A");
    taskman += String::from("B");
    taskman += String::from("C");

    taskman.dump();
}
