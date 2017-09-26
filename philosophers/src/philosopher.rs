use std::thread;
use std::time::Duration;
use table::Table;

pub struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    pub fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    pub fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(400));
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} started eating.", self.name);

        thread::sleep(Duration::from_millis(2000));

        println!("{} ended eating.", self.name);
    }
}
