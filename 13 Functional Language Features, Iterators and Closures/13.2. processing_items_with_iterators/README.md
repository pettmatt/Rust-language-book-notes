# Processing a Series of Items with Iterators

In a nutshell iterators allow developers to perform tasks on a sequence of items. Iterators are responsible for the logic of iterating over each item and determining when the sequence has finished. They are also lazy that have no effect unless a method is called that consumes the iterator to use it. Languages that don't include iterators in their standard library give developers view options to implement the functionality, but simplest way of the implementation is to create loop that possibly includes repetitive code to achieve the same functionality. 

## The Iterator Trait and the next Method

It's important to understand what trait(s) a functionality uses to understand what the functionality returns and expects. Traits also define which methods the functionalities that uses it has access to. *(Badly constructed sentence, but I hope it gives good enough picture).*

```rs
// The trait of Iterator
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

In the trait itself we can see what it's expecting. An item, which is then used in return and additionally the trait gives access to `next` method. Below is an example how iterators can call `next` method. Note that the iterator is mutable. When iterators are used in `for` loops the loop takes over the ownership and changes the variable into a mutable. All that is done in the background which is why it's easily over looked. Anyway, after the iterator is created the `next` method is called four times, each time it is called the iterator changes internal state (which keeps track of where it is in the sequence). This can be also called "consuming", which means the earlier state gets "used" and becomes unaccessable, and this continues as long as there is a state to progress to.

```rs
#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
}
```

**Also note that the values we get from the calls to next are immutable references to the values in the vector. The iter method produces an iterator over immutable references. If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter. Similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter.**

## Methods that Consume the Iterator

Reasons why `next` is implemented by default in Iterator trait is because some of the methods of the trait call the `next` method and without the implementation the method would cause an error.

**Methods that call next are called consuming adaptors, because calling them uses up the iterator. One example is the sum method, which takes ownership of the iterator and iterates through the items by repeatedly calling next, thus consuming the iterator. As it iterates through, it adds each item to a running total and returns the total when iteration is complete. Listing 13-13 has a test illustrating a use of the sum method:**

```rs
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
```

After the `sum` method call the ownership is transferred to `sum` which in turn consumes it.

## Methods that Produce Other Iterators

**Iterator adaptors are methods defined on the Iterator trait that don’t consume the iterator. Instead, they produce different iterators by changing some aspect of the original iterator.**

Hmm... so do  they consume the original and return a copy of the original iterator with some changes? Functionally I would believe the outcome is the same, so it doesn't really matter if they consume it if they return iterator at the end.

```rs
let v1: Vec<i32> = vec![1, 2, 3];

// Returns new iteration that contains the original values incremented by 1
v1.iter().map(|x| x + 1); // The return of closure!
```

The code above produces an error because the iterator never gets consumed, so because the nature of iterator adaptors is "lazy" the closure never gets called. We can collect the resulting values from iterator by consuming it with standard library method `collect`.

```rs
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

**Because map takes a closure, we can specify any operation we want to perform on each item. This is a great example of how closures let you customize some behavior while reusing the iteration behavior that the Iterator trait provides.**

**You can chain multiple calls to iterator adaptors to perform complex actions in a readable way. But because all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors.**

## Using Closures that Capture Their Environment

The example can be found in `lib.rs` file, but in a nutshell... **we use filter with a closure that captures the shoe_size variable from its environment to iterate over a collection of Shoe struct instances. It will return only shoes that are the specified size.** Sounds more complicated than what it is.