# Improving Our I/O Project

In this chapter the goal is to improve the application in chapter 12 by using replacing some functionality with iterators, which would make the code clearer and more readable. In this chapter the `Config::build` and `search` are going to get overhauled and majority of the project and possible comments will be found from the Rust files.

## Choosing Between Loops or Iterators

This is straight from the book, but I think it capsulates nicely why iterators can be better.

**The next logical question is which style you should choose in your own code and why: the original implementation in Listing 13-21 or the version using iterators in Listing 13-22. Most Rust programmers prefer to use the iterator style. It’s a bit tougher to get the hang of at first, but once you get a feel for the various iterator adaptors and what they do, iterators can be easier to understand. Instead of fiddling with the various bits of looping and building new vectors, the code focuses on the high-level objective of the loop. This abstracts away some of the commonplace code so it’s easier to see the concepts that are unique to this code, such as the filtering condition each element in the iterator must pass.**

**But are the two implementations truly equivalent? The intuitive assumption might be that the more low-level loop will be faster. Let’s talk about performance.**