# Using Message Passing to Transfer Data Between Threads

This part is going to explain how data transfer works between threads utilizing channels. If chat applications are familiar to you this concept should be easy to grasp, after all both use the same logic to transfer data.

*Check the code from the main rust file to understand how the concept works in action.*

This chapter is going to utilize `mpsc` library from the standard library, which stands for *"multiple producer, single consumer"*. **In short, the way Rust’s standard library implements channels means a channel can have multiple sending ends that produce values but only one receiving end that consumes those values.**

## Channels and Ownership Transference

*Check the rust file and check "example_one" function.*

## Sending Multiple Values and Seeing the Receiver Waiting

*Check the rust file and check "example_two" function.*

## Creating Multiple Producers by Cloning the Transmitter

*Check the rust file and check "example_three" function.*