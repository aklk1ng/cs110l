use crossbeam_channel;
use std::{thread, time};

fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let mut output_vec: Vec<U> = Vec::with_capacity(input_vec.len());
    output_vec.resize_with(input_vec.len(), Default::default);
    // TODO: implement parallel map!
    let (in_s, in_r) = crossbeam_channel::unbounded();
    let (out_s, out_r) = crossbeam_channel::unbounded();
    let mut threads = Vec::with_capacity(num_threads);

    for _ in 0..num_threads {
        let in_r = in_r.clone();
        let out_s = out_s.clone();
        threads.push(thread::spawn(move || {
            while let Ok(pair) = in_r.recv() {
                let (idx, val) = pair;
                out_s
                    .send((idx, f(val)))
                    .expect("there is no data in channel");
            }
        }));
    }

    let len = input_vec.len();
    for i in 0..len {
        in_s.send((len - i - 1, input_vec.pop().unwrap()))
            .expect("there is no receiver");
    }

    drop(in_s);
    drop(out_s);

    while let Ok(pair) = out_r.recv() {
        let (idx, val) = pair;
        output_vec[idx] = val;
    }

    for handle in threads {
        handle.join().expect("Panic occurred in thread!");
    }

    output_vec
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}
