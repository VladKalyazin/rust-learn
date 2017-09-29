use std::thread;

#[no_mangle]
pub extern fn process(iterations: i64) {
    let handles: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(move || {
                let mut x = 0;
                for _ in 0..iterations {
                    x += 1
                }
                x
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}

#[no_mangle]
pub extern fn process_2(iterations: i64) -> i64 {
    let mut x = 0;

    for _ in 0..iterations {
        x += 1
    }

    x
}
