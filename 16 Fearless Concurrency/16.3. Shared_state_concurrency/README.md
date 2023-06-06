# Shared-State Concurrency

This chapter focuses on sharing data without giving ownership to one thread which would deny access from other threads. So, shared-state concurrency, in a nutshell means that a state can be shared to multiple threads/processes giving the state a ability to have multiple ownerships.

## Using Mutexes to Allow Access to Data from One Thread at a Time

Mutex acts as a manager of given data, what tracks how many variables/functions it has given access to the data. The receiver of the data is given a lock access on request. Mutexes are often seen as hard to use, but to me it seems quite simple considering there is only two things to remember in order to use them successfully.


1. **You must attempt to acquire the lock before using the data.**
2. **When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.**

*TBH the second point seems kinda weird. Why would I need to unlock the data so other threads can acquire the lock? Or does mutex consider the transaction to be completed when the data is unlocked...*

**For a real-world metaphor for a mutex, imagine a panel discussion at a conference with only one microphone. Before a panelist can speak, they have to ask or signal that they want to use the microphone. When they get the microphone, they can talk for as long as they want to and then hand the microphone to the next panelist who requests to speak. If a panelist forgets to hand the microphone off when they’re finished with it, no one else is able to speak. If management of the shared microphone goes wrong, the panel won’t work as planned!**

## The API of `Mutex<T>`

*Check the rust file and check "example_one" function.*

## Sharing a `Mutex<T>` Between Multiple Threads

*Check the rust file and check "example_two" function.*

## Multiple Ownership with Multiple Threads

*Check the rust file and check "example_two" function.*

## Atomic Reference Counting with `Arc<T>`

*Check the rust file and check "example_three" function.*

**You could also use this program’s structure to do more complicated operations than just incrementing a counter. Using this strategy, you can divide a calculation into independent parts, split those parts across threads, and then use a Mutex<T> to have each thread update the final result with its part.**

## Similarities Between `RefCell<T>`/`Rc<T>` and `Mutex<T>`/`Arc<T>`

**You might have noticed that counter is immutable but we could get a mutable reference to the value inside it; this means `Mutex<T>` provides interior mutability, as the Cell family does. In the same way we used `RefCell<T>` in Chapter 15 to allow us to mutate contents inside an `Rc<T>`, we use `Mutex<T>` to mutate contents inside an `Arc<T>`.**

*Didn't even think about that. Interesting, that could help to grasp Mutex better.*

Just like `Rc<T>`, `Mutex` can also create logic errors that can create memory leaks. Rust cannot identify these errors.

**If you’re interested in deadlocks, try creating a Rust program that has a deadlock; then research deadlock mitigation strategies for mutexes in any language and have a go at implementing them in Rust. The standard library API documentation for `Mutex<T>` and MutexGuard offers useful information.** Let's see if I have enough energy for that.