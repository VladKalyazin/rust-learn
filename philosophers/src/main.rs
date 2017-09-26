mod philosopher;
mod table;

use philosopher::Philosopher;
use std::sync::{Arc, Mutex};
use std::thread;
use table::Table;

fn main() {
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ]
    });

    let philosophers = vec![
        Philosopher::new("p1", 0, 1),
        Philosopher::new("p2", 1, 2),
        Philosopher::new("p3", 2, 3),
        Philosopher::new("p4", 3, 4),
        Philosopher::new("p5", 0, 4),
    ];

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let table = table.clone();

            thread::spawn(move || {
                p.eat(&table);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
