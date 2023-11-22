use std::thread;
use std::sync::{Mutex, Arc, mpsc::channel};

fn scoped() {
    vec![1, 2, 3];
}

fn scoped2() -> Vec<u32> {
    vec![1, 2, 3]
}

fn scoped3() {
    let v1 = vec![1, 2, 3];
    let v2 = v1;
}

fn scoped4()
{
    vec![1, 2, 3].clone();
    "".to_string().clone();
}

fn scoped5()
{
    fn foo(v1: &Vec<u32>)
    {
        for v in v1 {
            println!("{}", v);
        }
    }
    let v1 = vec![1, 2, 3];
    foo(&v1);

    v1;
}

fn thread1()
{
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().ok();
}

fn thread2()
{
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn thread3()
{
    let (sender, receiver) = channel();
    let handle = thread::spawn(move || {
        let v = vec![1, 2, 3];
        sender.send(v).unwrap();
    });

    handle.join().ok();
    receiver.recv().unwrap();
}

fn main()
{
    scoped();
    scoped2();
    scoped3();
    scoped4();
    scoped5();

    thread1();
    thread2();
}