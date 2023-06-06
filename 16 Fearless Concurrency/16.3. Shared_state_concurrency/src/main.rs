use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::thread;

fn main() {
    // example_one();
    // example_two();
    example_three();
}

fn example_one() {
    let m = Mutex::new(5);

    {
        // Acquiring the lock will lock the access to the resource.
        // Unwrap will panic if the lock() fails, locking the resource from everyone.
        // Remember, the value inside a mutex is not i32, but a Mutex<i32>,
        // which is why the method lock is necessary. Lock will return the final i32 value.
        let mut num = m.lock().unwrap();
        *num = 6;
        // If we want to be specific Mutex<T> is a smart pointer, which includes
        // a drop method which is triggered when the MutexGuard 
        // (which is returned by the lock) goes out of scope,
        // meaning now. The drop also releases the lock.
    }

    // Because the lock was released we can check its value by printing it.
    // Will print out: m = Mutex { data: 6, poisoned: false, .. }
    println!("m = {:?}", m);
}

// This will create an error which is why it's commented out.
// Uncomment to see what kind of error and why.
// fn example_two() {
//     // Rc is wrapped around Mutex in "Multiple ownership with multiple threads".
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];

//     // Without error handling Rust won't let us to use mutex like this,
//     // because the previous iteration has access to the lock.
//     for _ in 0..10 {
//         // Added in "Multiple ownership with multiple threads".
//         // Because Rc isn't safe way of giving access to multiple threads at the same time
//         // this won't work. Check the error message.
//         // let counter = Rc::clone(&counter);

//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         // Push the thread that has access to the lock to handles vector.
//         handles.push(handle);
//     }

//     // Make surer the threads have finished by calling the join method.
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }

fn example_three() {
    // Arc is safe way of resolving problems that come with concurrent situations,
    // but it will have effect on performance.
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

    // And now the result should be printed out without a problem.
    println!("Result: {}", *counter.lock().unwrap());
}