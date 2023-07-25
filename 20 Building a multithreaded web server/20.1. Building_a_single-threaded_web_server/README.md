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

## A Closer Look at an HTTP Request

In web development HTTP requests are one of the corner stones and my guess is that this sub chapter won't include something note worthy, but if the book has something note worthy I note it down.

To completly understand the previously explained code snippet we need to understand the structure of the request.

```
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

The request structure is as shown above, which can be observed in the console when a request comes in. First line includes the details of the request such as the type which could be for example GET, POST or PUT, the uri "/" and protocol details or how the book puts it the version "HTTP/1.1". The rest of the prompt is the header contents of the request. The message body is missing for now, because the server didn't send anything to the client.

```rs
Request: [
    "GET / HTTP/1.1",
    "User-Agent: PostmanRuntime/7.32.3",
    "Accept: */*",
    "Cache-Control: no-cache",
    "Postman-Token: 4a982697-5159-4db1-a838-b4d41de61f4e",
    "Host: 127.0.0.1:7878",
    "Accept-Encoding: gzip, deflate, br",
    "Connection: keep-alive",
]
```

**Try making a request from a different browser or asking for a different address, such as 127.0.0.1:7878/test, to see how the request data changes.** When changing the address the uri is more noticable in the prompt "`GET /test HTTP/1.1`".

## Writing a Response

Now, let's get our hands in to the meat of a server, how to send a response to the client. Skipping the details how success can be seen from the HTTP response, sending a response to the client is quite simple process. In Rust we just need to use the TCP stream to send a response using `write_all` method. More details in the main rust file. The response won't display anything interesting because we just confirmed to the client "Yep, it was a success" by responding with only the success code 200.

## Returning Real HTML

Now that we know how we can create a successful response we can send something more interesting, such as HTML file. We need to access another file, which is why we need another standard library's module, which is `fs` module short for file system. Through `format` macro we create our final response message that includes the status of the response, the length of the content and the content itself which is the `hello.html` file. Nothing complicated, but it's new way of creating a response compared to how you would do it in Node.js for example. Also this example uses the standard library, which is more or less barebones version what some other projects would use.

*Note: The file system library checks the file path from the root of the project, not from the src directory.*

## Validating the Request and Selectively Responding

Now, in real world projects we don't always send the same response, because some times the client wants to do something else than receive data. So we need to validate what the request wants to do, which can be put simply that we need to add an if-statement to check what kind of request the server is responding to. Reminder that the if statement `GET / HTTP/1.1` also states that the request needs to be made to the frontpage, which is clarified with the single `/` even if it looks like a separator.

## A Touch of Refactoring

At the moment of writing the code includes quite a bit of repetitive code, which could be put into its own function. But because I want to follow the book I may not include the fix I would make. I would separate the response to its own function, because at the moment our Rust file includes two responses, one for the frontpage and one for the 404 error message.

Seems like the book is going to do same kind of refactoring as I was thinking (not surprising). I probably need to think more with tuples, at least the book utilizes them quite a bit. Anyway the if statement is refactored to just check if the request is for the frontpage or not. After which the if statement returns a tuple that includes the response code and the file (`("HTTP/1.1 200 OK", "hello.html")`). The book example is a lot cleaner what my refactoring solution would have been, I would have taken the contents of the if statements and thrown them into their own function and kept the if statements as is and just including the function call. With the book's example there is no need to create a new function and no need to include the logic in the if statement scopes.