use std::thread;

fn main() {
    let numbers  = Vec::from_iter(0..=53);

    let t1 = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });

    let average = t1.join().unwrap();
    println!("average: {average}");
}
