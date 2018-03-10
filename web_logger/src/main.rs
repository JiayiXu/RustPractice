use std::collections::LinkedList;
use std::time::{SystemTime, UNIX_EPOCH};

struct Logger {
    nodes: LinkedList<u64>,
}

impl Logger {
    fn new() -> Logger {
        Logger {
            nodes: LinkedList::new()  
        }
    }

    fn get_current_time() -> u64 {
        let start = SystemTime::now();
        start.duration_since(UNIX_EPOCH).expect("Can't convert").as_secs()
    }

    fn log_hit(&mut self) {
        let now = Logger::get_current_time();
        self.prune(now);
        self.nodes.push_back(now);
    }

    fn prune(&mut self, now: u64) {
        loop {
            if self.nodes.len() == 0 || self.nodes.front().unwrap() < &(now - 300) {
                break;
            }

            self.nodes.pop_front();
        }
    }

    fn get_hit(&mut self) -> usize {
        self.prune(Logger::get_current_time());
        self.nodes.len()
    }
}


fn main() {
    println!("Hello, world!");
}
