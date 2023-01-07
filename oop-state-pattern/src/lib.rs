pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})), // can only create posts in draft state
            content: String::new(),
        }
    }

    // adds text to the post
    // does not need to interact with the state
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // placeholder implementation for returning empty string when in draft
    pub fn content(&self) -> &str {
        // want to call the content method on state and not post
        // as_ref gets the reference to the value inside Option
        self.state.as_ref().unwrap().content(self)
    }

    // request review
    pub fn request_review(&mut self) {
        // take method moves the Some value and replace with None
            // this ensures that Post can't use the old state value
        // then re-assign new state
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    // self: Box<Self> means that self must be wrapped in Box
    // the method takes the ownership of Box<Self>
    // cannot use default implementation for request_review and approve
        // because trait doen't know what the concrete self will be exactly
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
