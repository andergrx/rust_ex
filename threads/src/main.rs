
use core::time;
use std::sync::{Arc, RwLock, Mutex};
use std::thread;

fn main() {
    let rdata = Arc::new(RwLock::new(1));

    let mut handles = vec![];

    for i in 0..=100 {
        let data = Arc::clone(&rdata);

        let t = thread::spawn(move || -> () {
            if i < 50 {
                std::thread::sleep(time::Duration::from_millis(3));
                let data = data.read().expect("Could not read RwLock");

                println!("Thread {i} reads {:?}", data);
            } else {
                let mut data = data.write().unwrap();

                *data += 1;

                println!("Thread {i} writes {:?}", data);
            }
        });

        handles.push(t);
    }

    for t in handles {
        t.join().unwrap();
    }

    println!("The final value of the locked data is {:?}", rdata);



    let a = Arc::new(Mutex::new(0));

    let mut handle = vec![];

    for i in 0..=100 {
        let c = Arc::clone(&a);

        let t = thread::spawn(move || {
            let mut value = c.lock().unwrap();

            *value += i;

            println!("Thread {i} writes {:?}", value);
        });

        handle.push(t);
    }

    for t in handle {
        t.join().unwrap();
    }

    println!("Final Value: {:?}", a);
}


