use std::thread;

fn main() {
    static X: [i32; 3] = [1, 2, 3];

    let t1 = thread::spawn(|| dbg!(&X));
    let t2 = thread::spawn(|| dbg!(&X));

    t1.join().unwrap();
    t2.join().unwrap();
}
