# Error Handling 

Every developer's most beloved section of software development... Applying the bug spray, or in this case more fitting would be applying repellent. After all the most important part of error handling is to prevent software breaking bugs. How is it handled in Rust and what the community has come up with compared to other languages?

The book has great opening about this topic including the next paragraphs:

**Book: Rust groups errors into two major categories: recoverable and unrecoverable errors. For a recoverable error, such as a file not found error, we most likely just want to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array, and so we want to immediately stop the program...**

To someone coming from JavaScript environment (including me) this can sound kinda weird that errors are divided into these groups. Well, if it helps to communicate the cause then why not. In action these two error types differentiate how they present the error, recoverable errors throw `Result<T, E>` errors and unrecoverable errors panic (`panic!` macro), which crashes the application when it encounters a bug. Panic can be viewed as "a bug that breaks the way Rust is expecting to operate", which can be accessing an index of an array that is out of scope which is impossible to access.