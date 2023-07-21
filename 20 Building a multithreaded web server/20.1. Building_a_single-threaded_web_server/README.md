# Building a Single-Threaded Web Server

*Because of the nature of this chapter the majority of notes are included in the rust files.*

## Listening to the TCP Connection

After setting up the server and testing how it works I'm kinda surprised that one connection creates following terminal prompt. Just one request.

    Connection established!
    Connection established!
    Connection established!
    Connection established!
    Connection established!
    Connection established!
    Connection established!
    Connection established!
    Connection established!
    Connection established!

**We’ve chosen this port for two reasons: HTTP isn’t normally accepted on this port so our server is unlikely to conflict with any other web server you might have running on your machine, and 7878 is rust typed on a telephone.** I was kinda wondering the port choice, but **"HTTP isn't normally acceptec on this port"** doesn't that just imply that **"we're using this port because it's rarely used for this purpose"**? Yes, the answer is yes.

```rs
let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
```
Bceause the `bind` method return `Result<T, E>`, success or failure we need to handle the `Result` some how. In this case we take the easy route and use `unwrap` which doesn't handle failure state with grace and will just stop the program. Here's some reasons why the binding can fail:

-  Ports below 1023 need administrator privileges and can create an error if used
-  If two instances are running at the same time

**Sometimes, you’ll see multiple messages printed for one browser request; the reason might be that the browser is making a request for the page as well as a request for other resources, like the favicon.ico icon that appears in the browser tab.** Makes sense. Interesting that using Postman will result Rust prompting an empty line in console. Ah, seems like when accessing through browser the whole stream goes out of scope, which is why we don't get the "`Connection established!`" prompt when trying again after the browser connection fails to retrieve data from our TCP server or something else happens, but I'm too lazy at the moment to figure it out.

## Reading the Request

Reading a request requires some more imports from standard library which is why we're importing `io::{prelude::*, BufReader}` and `net::{TcpListener, TcpStream}`.

The handle connection is pretty interesting which is why I'm going to go line by line what happens in the http_request. So, we want a vector out of the buf_reader, which is why we go through the lines, which is why we use the `line` method, which returns an iterator of `Result`, which means we need to handle the possible error, which is why we're taking the easy route and just using the `map` method, which inturn will either take the success value of stop the program with `|result| result.unwrap()`. Finally we need to check if the line is empty, if it is why would we want to store that line, which is why we return the value that is not empty with `take_while(|line| !line.is_empty())` method and finally we're transforming the iterator into a collection with the `collect` method.