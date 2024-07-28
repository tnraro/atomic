use std::thread;

fn main() {
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));

    let t1 = thread::spawn(move || dbg!(x));
    let t2 = thread::spawn(move || dbg!(x));

    t1.join().unwrap();
    t2.join().unwrap();
}
