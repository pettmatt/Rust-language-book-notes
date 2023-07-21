# Final Project: Building a Multithreaded Web Server

By the looks of it this chapther will be more interesting than the previous one. The list contains basic web development topics such as HTTP, making HTTP requests and responding to requests, but we also get exposed to more advanced topics that I wouldn't have thought of going through in this book such as TCP connections and thread pools.

- Learn a bit about TCP and HTTP.
- Listen for TCP connections on a socket.
- Parse a small number of HTTP requests.
- Create a proper HTTP response.
- Improve the throughput of our server with a thread pool.

Naturally because Rust doesn't include these functionalities by itself we're going to utilize multiple crates or that's what I think the chapter is going to include. Seems like I'm a little bit off and we aren't going to necessary get to use a crate that handles these functionalities for us.