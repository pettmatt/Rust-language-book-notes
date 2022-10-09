# Control flow

*This topic could be interesting. In the sense of how are certain functionalities different compared to other languages.*

## If statements

Note 1: If-statement syntax isn't that different, which is just written without the `if ( condition )` parentheses.

Note 2: When it comes to the condition it **needs** to be boolean. For example in JavaScript this is acceptable condition `if(number)`, which means "as long as the variable has a value (greater than 0) this condition is true". But in Rust, because this value is considered as a number the program would prompt an error `expected 'bool', found integer.`. *Atleast the error messages are clear.*

Book: *Example from note 2 should be changed so the condition would be a boolean. Example `if number != 0`.*

Book: *Using too many else if expressions can clutter your code, so if you have more than one, you might want to refactor your code. Chapter 6 describes a powerful Rust branching construct called match for these cases.*

Book: *Because `if` is an expression, we can use it on the right side of a let statement to assign the outcome to a variable...*

Note 3: ^ And here is another reason why remembering certain terms is useful.

# Loops

Note 4: To be honest, loop seem some what primitive. But this is probably just the most basic one. Atleast when it comes to other languages there is so many ways to loop, for example in JavaScript there is; while, foreach, for, case and many more.

Note 5: Interesting, there is a way to label loops with `'counting_up: loop {}` syntax. The label is useful for example breaking out of a specific loop with `break 'counting_up;`.

Note 6: For whatever reason `while` loop breaks the loop before even printing out a single line. Weird. Hmm... and now it works, indeed weird. Probably just forgot to save the file. **Boys and girls, don't be like me and remember to save your files.**

Note 7: Seems like `for` loop uses my favorite for-syntax `for element in array`.