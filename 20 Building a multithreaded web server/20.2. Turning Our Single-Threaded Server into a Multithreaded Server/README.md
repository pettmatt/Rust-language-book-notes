# Turning Our Single-Threaded Server into a Multithreaded Server

If we want to optimise and enable multi-request processing we need to implement multithreading. Which if you don't remember means that we push some processes to another thread.

## Simulating a Slow Request in the Current Server Implementation

To be honest having an example where the `sleep` is used to simulate heavy traffic is quite cheap. I don't know, for some reason it rubs me the wrong way even if it works just fine to show how the system would act without multithreading.

## Improving Throughput with a Thread Pool

One way of introducing multi-threading is thread pools, which is a group of threads that is waiting for new processes if the main is busy. This method probably includes that the developer needs to specify the amount of threads in the pool. Seems like limiting the threads isn't required, but it gives the web application extra layer of protection against DoS attacks, which makes sense. In this kind of setup the pool is in managable size and won't expand to an amount that would crash the server if too many requests are coming in (or that's what I'm theorising). The server is just going to be slower, which is expected when there is too much traffic coming in.

Besides pooling there are also fork model which is also know as the join model, the single-threaded async I/O model and the multi-threaded async I/O model. In different approaches and projects a different model may be better than for example pooling, which may be more resource intensive (need to research more about the difference between these models).

### Spawning a Thread for Each Request

Because we're studying and implementing a new feature its better to start by spawning threads for each request and then later implement pooling to it, where each request is taking a new thread from the pool.

So, step one we need to implement thread spawning to the main function, which can be done with `std::thread` module. Quite simple and now we can implement it properly where we don't spawn a new thread, but create a pool, which is used to pick an available thread.

### Creating a Finite Number of Threads

First we need to declare a thread pool instance that takes a numeric value as an argument, which is the number of threads created in the pool. After that we replace `thread::spawn` with `pool.execute` and the closure within stays the same, quite simple isn't it?

### Building `ThreadPool` Using Compiler Driven Development

Now we need to build the `ThreadPool` so the code could be compiled and tested in action. Let's go with the instructions offered by the book, let's create new library named `hello` (`cargo new hello --lib`) and create simple `ThreadPool` struct in it. This won't compile yet, because... you know the answer... `ThreadPool` doesn't implement any methods which is why  the `new` method is not found. Implementing the `new` method is not enough, because we have more than one method used in the code, second one that needs to be implemented is `execute`, which probably needs some assistance from other module/trait to include wanted functionality. If we want to implement the closure the same way the `spawn` does, we can cheat a little and check how the function is implemented in the `thread` module.

```rs
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

And that's how spawn is implemented. `FnOnce` trait is used as the trait bound on the argument `F`. Additionally: **The F type parameter also has the trait bound Send and the lifetime bound 'static, which are useful in our situation: we need Send to transfer the closure from one thread to another and 'static because we don’t know how long the thread will take to execute. Let’s create an execute method on ThreadPool that will take a generic parameter of type F with these bounds.**

Great explanation from the book about the implemented execute function: **We still use the () after FnOnce because this FnOnce represents a closure that takes no parameters and returns the unit type (). Just like function definitions, the return type can be omitted from the signature, but even if we have no parameters, we still need the parentheses.**

And now the server should compile, but because the `execute` function is just a shell it does nothing at the moment. So let's implement a code that executes the closure passed as an argument.

### Validating the Number of Threads in `new`

Let's start from `new`, at the moment we're just returning an empty `ThreadPool` and not even checking if the argument is valid so let's validate it.

**We’ve also added some documentation for our ThreadPool with doc comments. Note that we followed good documentation practices by adding a section that calls out the situations in which our function can panic, as discussed in Chapter 14. Try running cargo doc --open and clicking the ThreadPool struct to see what the generated docs for new look like!** Seems like I need to edit my comments a little, but the book's comments are just too good.

And book threw a problem at us... I don't really feel like implementing the build right now so I leave for later.

```rs
pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError>
```

### Creating Space to Store the Threads

**We’ve changed the definition of ThreadPool to hold a vector of thread::JoinHandle<()> instances, initialized the vector with a capacity of size, set up a for loop that will run some code to create the threads, and returned a ThreadPool instance containing them.**

### A `Worker` Struct Responsible for Sending Code from the ThreadPool to a Thread

Because the standard library doesn't include a functionality that creates a pool of threads and waits for a request to come in and forward it to thread, we need to create the functionality through workers. What is a worker? **"The Worker picks up code that needs to be run and runs the code in the Worker’s thread".** Because `Worker` is used by the `ThreadPool` we have no reason to include public access to it, meaning it will stay as a private struct.

Nice little detail about how `thread::spawn` may function in real environment and why returning `Result` could be better: **Note: If the operating system can’t create a thread because there aren’t enough system resources, thread::spawn will panic. That will cause our whole server to panic, even though the creation of some threads might succeed. For simplicity’s sake, this behavior is fine, but in a production thread pool implementation, you’d likely want to use std::thread::Builder and its spawn method that returns Result instead.**

### Sending Requests to Threads via Channels

To be honest this part goes somewhat over my head, so I let the book to explain the important stuff.

**The next problem we’ll tackle is that the closures given to thread::spawn do absolutely nothing. Currently, we get the closure we want to execute in the execute method. But we need to give thread::spawn a closure to run when we create each Worker during the creation of the ThreadPool.**

1. **The ThreadPool will create a channel and hold on to the sender.**
2. **Each Worker will hold on to the receiver.**
3. **We’ll create a new Job struct that will hold the closures we want to send down the channel.**
4. **The execute method will send the job it wants to execute through the sender.**
5. **In its thread, the Worker will loop over its receiver and execute the closures of any jobs it receives.**

If I understood correctly we create a channel for every pool that manages the request. I'm not covering every single thing that is implemented in code so I focus on the important ones. It's possible that a receiver belongs to multiple workers, which may at this point create data race, which can be fixed by using `Arc<Mutex<T>>`, which in turn **will let multiple workers own the receiver, and Mutex will ensure that only one worker gets a job from the receiver at a time.**

### Implementing the `execute` Method

And now to the meat of the application, the execute method. We have set up quite a bit and atleast my head is little overwhelmed by it all. And here we make some of our code more readable by using type aliases. I'm quite done with the explaining the code in my notes after analysing or reading the explanation so check the `lib` file to get a grasp or just read the book I don't know.

Quite interesting chapter, but I need to work on my own code in order to be able to explain with confidence how this program works. Not that it's hard to grasp, but I wouldn't be able to come up with a program like this, maybe if I followed a blueprint that included the things I needed to include to make the program compile and work as designed.