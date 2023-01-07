use oop_state_pattern::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    // content should return empty as it has not been approved yet
    assert_eq!("", post.content());

    post.request_review();
    // content should return empty as it has not been approved yet
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

