# Using Threads to Run Code Simultaneously

Using multiple threads to process the different functionalities of a program introduces more complexity, but it can also improve the performance. Using threads is handled through API provided by the operating system. **The Rust standard library uses a 1:1 model of thread implementation, whereby a program uses one operating system thread per one language thread.**

## Creating a New Thread with spawn

By using `thread::spawn` the program can "push" processes to other threads. Even if the new thread process is created first before a `main` thread it may or may not finish before the `main` thread. For example the example code of the book will produce a following output.

```
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 4 from the main thread!
hi number 5 from the spawned thread!
```

The numbers keep going on logically, but some times the spawned thread will be finished faster than the main thread (*but it seems there is a pattern to it*). If this is not taken into account the end result may not be what is expected. Other thing to note, the `thread::spawn` will not keep running after the main `for` loop finishes, because the `thread::spawn` goes out of scope.

The `thread::sleep` method will stop a thread from running for a short while and allow different threads to run. This doesn't guarantee that the threads take turns, that depends on the operating system and how their API handles threads.

## Waiting for All Threads to Finish Using `join` Handles

Because we cannot gurantee the order of run time, when certain thread runs and if a thread is even able to run in the test code we can introduce a handle `join`, which is **an owned value that, when we call the join method on it, will wait for its thread to finish**.

After implementing `join` handle *(`handle.join().unwrap()`)* the code will run to the end, meaning the spawned thread will have time to process its `for` loop. To be more specific the handle will **block** the main thread from exiting and performing work, preventing the spawned thread from going to out of scope. The program will still follow the old logic even if the main thread is being blocked, when the `thread::sleep` is executed it will give room to the main thread to execute its code and vice versa. **The two threads continue alternating, but the main thread waits because of the call to handle.join() and does not end until the spawned thread is finished.**

Remember, if the threads are meant to run paralel to each other the `handle.join().unwrap();` should be put after those functionalities. If the `join` handle was to be put before the second `for` loop, it would block the main thread from executing its code before the spawned thread is done.

So, pay attention where the `join` handle is placed.

## Using move Closures with Threads

*This section includes a subject that was introduced in 13.1. Capturing environments and closures.*

If we wanted to use existing variables and values inside a new thread created with `spawn`, we could pass it over using `closures` (`|| { /* code goes here */ }`). Closures will check what variables are being used inside them and include them by borrowing them. Probably in majority of the cases this would introduce a possible problem when the value can be expire before the thread is finished. This can be fixed by using keyword `move`, which will pass over the ownership of the values used inside of the closure. And because of ownership rules the variable cannot be dropped before it has been consumed by the closure of the thread or the ownership is passed to the original thread. Neat, isn't it?

**By adding the move keyword before the closure, we force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values.**