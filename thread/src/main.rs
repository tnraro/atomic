use std::{sync::Arc, thread};

fn main() {
    let a = Arc::new([1, 2, 3]);

    let t1 = thread::spawn({
        let a = a.clone();
        move || dbg!(a)
    });
    let t2 = thread::spawn({
        let a = a.clone();
        move || dbg!(a)
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
