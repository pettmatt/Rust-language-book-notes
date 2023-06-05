use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // example_one();
    // example_two();
    example_three();
}

fn example_one() {
    // Channel returns a tuple that contains the transmitter (tx) and 
    // the second element is the receiver (rx). 
    let (tx, rx) = mpsc::channel();

    // Create new thread and pass used variables to it (tx).
    thread::spawn(move || {
        let val = String::from("hi");
        // Will return a Result. It either sends response or returns an error (if receiver couldn't be found).
        // In this case unwrap would result into a panic if it receives an error.
        tx.send(val).unwrap();

        // This test visualises how tx and rx function in action.
        // Because the value "val" was send it cannot be accessed here by println macro (this also means ownership is moved to the receiver).
        // This is not allowed because Rust can't be sure that the macro has 
        // access to the variable when it needs it.
        // println!("Message is \"{}\"", val);
    });

    // recv will block the application from proceeding until a response.
    // try_recv won't block the progression, but it might not receive the message.
    let received = rx.recv().unwrap();
    println!("Message: {}", received);
}

fn example_two() {
    // Simple example how the threads can "talk" with each other.
    // The transmitter will send multiple messages to receiver which will display the message.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("the"),
            String::from("other"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Notice how receiver will be handled like an iterator.
    // Iteration will end when transmitter is done.
    for received in rx {
        println!("Got: {}", received);
    }
}

fn example_three() {
    let (tx, rx) = mpsc::channel();

    // Because of "multiple producer, single consumer" functionality we can
    // have multiple transmitters and single receiver.
    // Second transmitter that will transmit to the same receiver.
    let tx1 = tx.clone();

    // Transmitters will be put in their own threads,
    // while the receiver will stay in the main thread.
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Nothing fancy or new here, just visualising how easy it is to copy a transmitter.
    for received in rx {
        println!("Message: {}", received);
    }
}