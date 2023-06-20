// Post and new method were created in "Defining Post and Creating a New Instance in the Draft State" section
pub struct Post {
    // These properties are private that can only be changed through methods.
    // Which means the properties cannot be edited accidentally by functions outside of this library.
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // Implement new method to Post, which creates starting point for the instance of a post.
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // Created in "Storing the Text of the Post Content" section
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // Created in "Requesting a Review of the Post Changes Its State" section
    // "State", "impl state for Draft" and "impl State for PendingReview" were also tweaked at this point.
    pub fn request_review(&mut self) {
        // Consume current state and return new
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    // Created in "Adding approve to Change the Behavior of content" section
    pub fn approve(&mut self) {
        // If state contains a value
        if let Some(s) = self.state.take() {
            // mutate self state value with approve
            self.state = Some(s.approve())
        }
    }

    pub fn content(&self) -> &str {
        // Just to add with the information given by the book.
        // This is one of the cases where programmer may have more information than the compiler,
        // which is why we need to include unwrap() which will never panic because state will always contain Some value.
        // But compiler doesn't know it so we need to add unwrap here.
        // Now we need to implement content to state trait
        self.state.as_ref().unwrap().content(self)
    }
}

trait State {
    // Notice that we're not dealing with a reference.
    // This trait contains the possible methods in state.
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        "" // By default returns nothing
    }
}

struct Draft {}

// When state is in draft state the post has access to these methods.
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // Don't change the state because draft shouldn't be able to change it, so return current value
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

// When pending for review the functionality is different. Meaning request_review is overwritten.
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // change the state
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// Note: These different implementations of request_review are implementations of the same function,
// but with different rules of how they will be returning the Box<dyn State> in different states.

struct Published {}

impl State for Published {
    // Post is already published. No point in approving or requesting a review.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Only once overriden. By default returns nothing expect here.
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}