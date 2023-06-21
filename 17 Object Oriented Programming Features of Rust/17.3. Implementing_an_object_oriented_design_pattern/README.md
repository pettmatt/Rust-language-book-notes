# Implementing an Object-Oriented Design Pattern

What does object-oriented design pattern mean? In short it's a way to split an object into different states. For example, one that holds the value, one that changes the value and one that changes the access to the value. So, what benefits does this way of programming bring? Well, let's take the book example of the process of posting a new post. It starts with a draft, which needs to be reviewed and after that approved, each of these parts of the process can be split into intependent functions, that have set amount of power over the post state. Another benefit of this implementation is that the programmer only needs to update the code inside one of the state, if you needed to change what happens on review only the `review` function would need changes. 

*Remember to check the rust file how this can be implemented in code. Example to this can be found in main rust file.*

## Defining Post and Creating a New Instance in the Draft State

*Defining library methods, check the state-lib rust file.*

## Storing the Text of the Post Content

Correction about what is considered to be part of the state pattern. `Add_text` doesn't depend on the state the post is in, so it's not part of the state pattern! But it's necessary part of the state pattern or atleast it is in this implementation.

*Defining library methods, check the state-lib rust file.*

## Ensuring the Content of a Draft Post Is Empty

Nothing interesting to add. This section didn't really contain enything other than "Yep, this is a method that needs to be included, but the logic is going to be added later. Oh, at this point we need just an empty response from the content method so let's return an empty string in it for now."

*Defining library method, check the state-lib rust file.*

## Requesting a Review of the Post Changes Its State

*Defining library methods, check the state-lib rust file.*

## Adding `approve` to Change the Behavior of `content`

Interesting stuff when you get the hang of the logic.

*Defining library methods, check the state-lib rust file.*

## Trade-offs of the State Pattern

This section highlights the ups and downs of the state pattern, which are a few, but may create larger problems when application becomes larger than was originally designed *(to be clear, problems in maintainability)*. Another thing that is somewhat of a problem in this approach method is the code is quite wet, meaning the states contain functions with similiar logic, which could be defined in trait State itself, but that would introduce compiler error, because Rust is unable to calculate what the `<Self>` is and how much resource it needs.

*Defining library methods, check the state-lib rust file.*

### Encoding States and Behavior as Types & Implementing Transitions as Transformations into Different Types

Here the book shows an example how the code can be optimized in a way that makes it impossible to show the contents of a post that is still in draft and at the same time making the code DRYer. 

The second implementation example doesn't really follow the rules of a state pattern, but in this case it makes the code more simplistic, easier to not write bad code that would break how certain state should function and easier to read.

*Defining library methods, check the lib rust file.*
