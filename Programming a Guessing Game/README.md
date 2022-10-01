# Notes

*Because this chapter is heavily about code and syntax this README file will include mostly my thoughts about Rust and its syntax.*

Note 1: On line 1 ordinary inclusion of a library/package. Line 7 is little bit more interesting without reading the answer from the book. In other language such as JavaScript `let` would indicate a declaration of a variable, but here it was said at the beginning that `You’ll learn about let, match, methods, associated functions...` so if I didn't understand incorrectly the `let` is used the same way as in JS, so what is `mut` used for? Is it used to get user's input. `String::new()` Creates new instance of string which is stored in the variable.

Answer: Yep. Seems like declaring variables is basicly the same as in JavaScript. Example `let apples = 5`. Only difference being that `let` variable's original value cannot be overwritten, which is called being *immutable*. Keyword `mut` enables variable to overwrite previous values, becoming *mutable*.

Note 2: According to the book `::` after the String means the code is accessing associated function which would be a `new` function which creates a new instance of a given type, which would be string in this case. My guess is that the `new` function works the same way as any other and it accepts arguments, which would mean I should be able to declare a value through `new`. But in this case it would make a mess of the code so let's keep unnecessary complexity out of this project.

Note 3: Just so I understood this right lets "talk" about `io::stdin()`. From io library we're calling an associated function called `stdin` which is then piped through `read_line` and `expect`. My guess being that `expect` is used as an error catcher.

Answer: Seems like I was little bit off by "piping". Io library allows the code get input from user through `read_line` method which gets `&mut guess` as an argument. `&` means the variable is reference. Which... here's full explanation from the book "*...which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.*"

Note 4: seems like `.expect` requires some notes. This part is more interesting than I thought. `read_line` return a *Result* which is an enumeration of the `read_line` which contains the "status" of how the operation went (so to say). In this case because its argument is *mutable* there isn't going to be much of errors so it's going to return variant of `Ok`. If the variant would be `Err` it would trigger `expect` which in turn would crash the program. Just to note the `Ok` variant also goes through the `expect` method, but it will just return the user input value (or bytes of the input value). Without `expect` method the program is going to display a warning how the *Result* should be handled. In a sense *Result* acts as *a promise* from JavaScript environment.

Note 5: Crashing the program isn't the best way of handling error states, but for this program it is effective way of solving this problem.

Note 6: `The {} set of curly brackets is a placeholder: think of {} as little crab pincers that hold a value in place.` who would have known a programming book could be this cute.

Note 7: To be honest this syntax is quite impressive `println!("x = {} and y = {}", x, y);` not that I haven't seen it used through other languages, but I think it's overall a neat trick.

Note 8: Update to previous semicolon note in chapter 1. They are mandatory.

Note 9: Rust doesn't include random number generation function (yet) so in this project this functionality will be added through crate. Please check `Cargo.toml` file for details about this specific crate.

Note 10: Hmm... you learn something every day. Didn't know the specifics of Semantic Versioning, but its summary looks like this *"Given a version number MAJOR.MINOR.PATCH..."* Check this website for more https://semver.org/.

Note 11: You indeed learn something new every day. Didn't really know the specific function of `x.lock` files, but I would guess they work the same way as in Rust (refering to npm lock files).

Note 12: Whenever you want to update package to another version which includes *MINOR* changes (0.8.4 -> 0.9.0) you need to manually change the version number of a dependency in `Cargo.toml` file. `cargo update` command is only able to download *PATCH* for a dependency.

Note 13: `use rand::Rng;` take a good look at this line and try to give reason for every part of it. `use` keyword is used to include a library in a project which in  this case is `rand` (check toml file for the name used here), after that the project is going to need access to random number generation which is associated function called `Rng`. Makes sense?

Question: Makes sense but why aren't we using `Rng::thread_rng` in code?

Answer: No idea (at this moment).

Book: *The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods. Chapter 10 will cover traits in detail.*

Note 14: `.gen_range(1..=100)` ... what am I seeing? Well book. Tell me the answer.

Book: *Next, we’re adding two lines in the middle. In the first line, we call the rand::thread_rng function that gives us the particular random number generator that we’re going to use: one that is local to the current thread of execution and seeded by the operating system. Then we call the gen_range method on the random number generator. This method is defined by the Rng trait that we brought into scope with the use rand::Rng statement. The gen_range method takes a range expression as an argument and generates a random number in the range. The kind of range expression we’re using here takes the form start..=end and is inclusive on the lower and upper bounds, so we need to specify 1..=100 to request a number between 1 and 100.*

Question: Makes sense but what does the `..` operator do specificly?

Note 15: `cargo doc --open` pretty neat command. Probably more useful tool in the future when someone (I) achieves better understanding of Rust.

Note 16: This part is for comparing numbers section. Another keyword `match` and `.cmp` is from the library included in this section which gets an argument as reference (good for memory).

Note 17: Answer for previous question after note 13. The reason why `Rng` wasn't used was probably because `Rng` is part of rand library which contains specific things and when we need access outside of that we have to use `Rand` or something like that. *Feeling stupid*.

Question: Why are we using library which includes comparison instead of using if? `Ordering::Less => println!("Too small!")`

Answer: It's probably just more optimal to use a method which is part of the standard library than to use if-statements for comparing.

Book: *First we add another use statement, bringing a type called std::cmp::Ordering into scope from the standard library. The Ordering type is another enum and has the variants Less, Greater, and Equal. These are the three outcomes that are possible when you compare two values. Then we add five new lines at the bottom that use the Ordering type. The cmp method compares two values and can be called on anything that can be compared. It takes a reference to whatever you want to compare with: here it’s comparing the guess to the secret_number. Then it returns a variant of the Ordering enum we brought into scope with the use statement. We use a match expression to decide what to do next based on which variant of Ordering was returned from the call to cmp with the values in guess and secret_number.*

*Let’s walk through an example with the match expression we use here. Say that the user has guessed 50 and the randomly generated secret number this time is 38. When the code compares 50 to 38, the cmp method will return Ordering::Greater, because 50 is greater than 38. The match expression gets the Ordering::Greater value and starts checking each arm’s pattern. It looks at the first arm’s pattern, Ordering::Less, and sees that the value Ordering::Greater does not match Ordering::Less, so it ignores the code in that arm and moves to the next arm. The next arm’s pattern is Ordering::Greater, which does match Ordering::Greater! The associated code in that arm will execute and print Too big! to the screen. The match expression ends after the first successful match, so it won’t look at the last arm in this scenario.*

Note 18: The code won't compile at this point so let's see what the book have in store.

Answer: My guess is after looking at the compiler error is that `guess` should be converted to an intiger.

Book: *```let guess: u32 = guess.trim().parse().expect("Please type a number!");```
We create a variable named guess. But wait, doesn’t the program already have a variable named guess? It does, but helpfully Rust allows us to shadow the previous value of guess with a new one. Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess for example. We’ll cover this in more detail in Chapter 3, but for now know that this feature is often used when you want to convert a value from one type to another type.*

Note 19: Remember that `match` arm body cannot contain semicolons. Use comma instead `Ordering::Less => println!("Too small"),`

Note 20: `Loops` and `break`ing out. Seem normal. Nothing to add.

Note 21: New `match` arm body seems interesting to say the least. Keyword `num` references to a number type, but does `_` reference to any other type?

Answer: Kind of.

Book *The underscore, _, is a catchall value; in this example, we’re saying we want to match all Err values, no matter what information they have inside them.*

Note 22: Well that took more time to finish this part than I was expecting. Well whatever atleast I understand Rust a little better.