# Reference Cycles Can Leak Memory

Even if Rust is designed and builded like a modern language to prevent bad coding habits it's not impossible to write a program that has memory leaking problems. What is *memory leak* anyways? Well, when a process requires memory system reserves the amount that is needed and sometimes for some reason, it may never be reclaimed by the system. That is called memory leak, and when the process is executed multiple times it creates more memory leaking, which in turn creates performance issues (most notable examples can be seen in video games).

Weeeell how does memory leaks happen in Rust even if it tries to prevent it? **Preventing memory leaks entirely is not one of Rust’s guarantees, meaning memory leaks are memory safe in Rust.** By using `Rc<T>` and `RefCell<T>`. **it’s possible to create references where items refer to each other in a cycle. This creates memory leaks because the reference count of each item in the cycle will never reach 0, and the values will never be dropped.**

## Creating a Reference Cycle

So, the reason for *memory leaks* is reference cycles, how does it look like in Rust?

```rs
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // In this case Option can create memory leaks.
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// To show how things are progressing
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
```

Memory leaks go somewhat over my head so I let the book to explain why the code will create a cycle that never collects the memory.

**The reference count of the `Rc<List>` instances in both a and b are 2 after we change the list in a to point to b. At the end of main, Rust drops the variable b, which decreases the reference count of the b `Rc<List>` instance from 2 to 1. The memory that `Rc<List>` has on the heap won’t be dropped at this point, because its reference count is 1, not 0. Then Rust drops a, which decreases the reference count of the a `Rc<List>` instance from 2 to 1 as well. This instance’s memory can’t be dropped either, because the other `Rc<List>` instance still refers to it. The memory allocated to the list will remain uncollected forever.**

**If you uncomment the last println! and run the program, Rust will try to print this cycle with a pointing to b pointing to a and so forth until it overflows the stack.**

In other words if the programs has `RefCell<T>` values that contain `Rc<T>` or other values that have the ability of interior mutability and reference counting. Rust can't catch memory leaks because they are logical mistakes, which can be caught with code reviews. The ability to notice and solve *memory leaking* problems come with experience.

## Preventing Reference Cycles: Turning an `Rc<T>` into a `Weak<T>`

To recap how `Rc<T>` works: **So far, we’ve demonstrated that calling Rc::clone increases the strong_count of an `Rc<T>` instance, and an `Rc<T>` instance is only cleaned up if its `strong_count` is 0.**

The rest of this part focuses on introducing `weak_count` and its features.

### Creating a Tree Data Structure: a Node with Child Nodes

In this part the book demonstrates how children behave with `ReCell` and `Rc`. Majority of the code can be found in the main rust file.

```rs
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
```

**We want a Node to own its children, and we want to share that ownership with variables so we can access each Node in the tree directly.** And what does this mean in action? That's the reasoning why we're using `Vec<T>` in side the children and because we want to modify those values later the `Vec<T>` is surrounded by `RefCell<T>`.

Next let's use this struct in the main function of a Rust file. First one will be simple instance of the struct with value 3 with no children and named as `leaf`, which indicates well that it shouldn't have children because leafs are usually children on a branch. Talking about branch let's then create a instance of a node named branch, which will contain value 5 and one child, the leaf that was created.

```rs
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
```

**We clone the Rc<Node> in leaf and store that in branch, meaning the Node in leaf now has two owners: leaf and branch.** And this is how we can share the ownership with children in its simplicity.

### Adding a Reference from a Child to Its Parent

If the title means that children could somehow interact with the parent by updating a part of a value in parent's value that references to the value of a child, then this could be interesting read.

**To make the child node aware of its parent, we need to add a parent field to our Node struct definition. The trouble is in deciding what the type of parent should be. We know it can’t contain an `Rc<T>`, because that would create a reference cycle with leaf.parent pointing to branch and branch.children pointing to leaf, which would cause their strong_count values to never be 0.** Infinity loop memory leak warning! What if the child has access only to some values so it would not have access to its own value through the parent?

**So instead of `Rc<T>`, we’ll make the type of parent use `Weak<T>`, specifically a `RefCell<Weak<Node>>`.** Oh, right `weak` was also a thing, well let's see how it can be utilized.

Ofcourse we need to update the struct to include the parent as a `Weak<T>`, which can be modified later.

```rs
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
```

So, let's update the main function code. The leaf will be created the same way, but now it creates an empty instance of `Weak<T>` for the parent, which is later updated to refer to the parent as a `Weak<T>`. Seems kinda funny to create the same `weak` instance for the parent variable for the branch, but eh.. it doesn't really matter.

```rs
let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
});

let branch = Rc::new(Node {
    value: 5,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![Rc::clone(&leaf)]),
});

*leaf.parent.borrow_mut() = Rc::downgrade(&branch);
```

The point of this story, `Weak<T>` can be utilize to get pass the infinity reference cycle.

### Visualizing Changes to strong_count and weak_count

This part is just for visualizing the end product and how the application ends up in that kind of situtation by printing the steps (How clean up is handled with strong_count and weak_count). Check the Rust file for full code. But in short the point of this is to visualize that `weak_count` doesn't affect when a variable is dropped compared to `strong_count`, which can create memory leaks when used incorrectly.

