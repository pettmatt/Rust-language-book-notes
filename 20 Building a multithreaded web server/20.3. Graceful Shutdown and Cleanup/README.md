# Graceful Shutdown and Cleanup

When starting the application the console shows couple of warning messages about unused values and when we exit from the command with `ctrl+C` we will get an unpleasant error message. This chapter will cover how these issues can be solved with **grace**.

## Implementing the `Drop` Trait on `ThreadPool`

`Drop` trait is able to handle cleanup for us and that can be one piece of the puzzle why processes don't cleanup themselves as expected. On `Drop` we will just cicle through the workers and one by one shut them down. Unfortunatly this approach won't work right out of the box and the compiler suggests that we could implement `Copy` trait. But that isn't the underlying problem.

**The error tells us we can’t call join because we only have a mutable borrow of each worker and join takes ownership of its argument. To solve this issue, we need to move the thread out of the Worker instance that owns thread so join can consume the thread. We did this in Listing 17-15: if Worker holds an `Option<thread::JoinHandle<()>>` instead, we can call the take method on the Option to move the value out of the Some variant and leave a None variant in its place. In other words, a Worker that is running will have a Some variant in thread, and when we want to clean up a Worker, we’ll replace Some with None so the Worker doesn’t have a thread to run.**

After wrapping the thread with `Option` we need to adjust the code a little so the thread holds `Some` value. And finally we destructure the `Some` before we try to call the `join` method on a thread.

**The take method on Option takes the Some variant out and leaves None in its place. We’re using if let to destructure the Some and get the thread; then we call join on the thread. If a worker’s thread is already None, we know that worker has already had its thread cleaned up, so nothing happens in that case.**

And that should resolve the startup warnings.

## Signaling to the Threads to Stop Listening for Jobs

Now to the next problem, the problematic error message when exiting the program and the program logic. At the moment reason why we're unable to exit the program safely is the logic itself, because the threads are always looking for requests meaning that logic needs to be changed. Using the existing `Drop` isn't an option because it would create a situation where the main thread blocks requests, waiting for the first thread to finish. So, editing the `Drop` implementation is the next task.

Because we're going to need to `Drop` the sender we need to change the type to `Option` and fix parts of code that try to insert other values in the sender or expect other than `Option`. After changing sender type to `Option` we are able to `Drop` the sender in `ThreadPool` drop implementation.

**Dropping sender closes the channel, which indicates no more messages will be sent. When that happens, all the calls to recv that the workers do in the infinite loop will return an error.**

Now we need to change the worker loop so our application can exit gracefully. **Which means the threads will finish when the ThreadPool drop implementation calls join on them.**

Seems like I managed to do something odd when two requests result in this kind of prompt:

    Worker 0 got a job; executing
    thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }', src\main.rs:40:49
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    Shutting down.
    Worker 1 got a job; executing
    Shutting down worker 0
    Worker 2 disconnected; shutting down.
    thread 'Worker 2 disconnected; shutting down.
    mainWorker 2 disconnected; shutting down.
    ' panicked at 'Worker 2 disconnected; shutting down.
    thread 'called `Result::unwrap()` on an `Err` value: Any { .. }Worker 2 disconnected; shutting down.
    ', <unnamed>Worker 2 disconnected; shutting down.
    hello/src/lib.rsWorker 2 disconnected; shutting down.
    ' panicked at ':Worker 2 disconnected; shutting down.
    called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }56Worker 2 disconnected; shutting down.
    ', Worker 2 disconnected; shutting down.
    :Worker 2 disconnected; shutting down.
    src\main.rs31Worker 2 disconnected; shutting down.
    :Worker 2 disconnected; shutting down.

    40Worker 2 disconnected; shutting down.
    :Worker 2 disconnected; shutting down.
    49Worker 2 disconnected; shutting down.

    Worker 2 disconnected; shutting down.
    Worker 2 disconnected; shutting down.

Oh right, it would be good to remember to copy rest of the files to this directory from 20.2. I'm glad that it was just my own forgetfulness. I hope the book addresses the problem of prompting `Worker N disconnected; shutting down.` infinitely.

**When worker 0 finished, the main thread waited for the rest of the workers to finish. At that point, they had all exited their loops and stopped.**

Seems like that was it and because a normal server isn't designed to just stop working after X amount of requests this end result is expected and doesn't really affect the application at the end of the day. Oh... seems like I forgot to add `break` to the `match` in `lib` file, line 81. Now the application exits correctly after cleaning and shutting down the workers. Final prompt:

    Worker 0 got a job; executing
    Shutting down.
    Worker 1 got a job; executing
    Shutting down worker 0
    Worker 2 disconnected; shutting down.
    Worker 3 disconnected; shutting down.
    Worker 0 disconnected; shutting down.
    Worker 1 disconnected; shutting down.
    Shutting down worker 1
    Shutting down worker 2
    Shutting down worker 3

The book listed some good features that could be added in the project, but I feel like it's better if I start my own project and try to finish it.

**We could do more here! If you want to continue enhancing this project, here are some ideas:**

- **Add more documentation to ThreadPool and its public methods.**
- **Add tests of the library’s functionality.**
- **Change calls to unwrap to more robust error handling.**
- **Use ThreadPool to perform some task other than serving web requests.**
- **Find a thread pool crate on crates.io and implement a similar web server using the crate instead. Then compare its API and robustness to the thread pool we implemented.**