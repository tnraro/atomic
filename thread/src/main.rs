use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    let t1 = thread::spawn(move || {
        for n in numbers {
            println!("{n}");
        }
    });

    println!("main thread");
    t1.join().unwrap();
}
