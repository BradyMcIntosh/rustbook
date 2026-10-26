use oo_patterns::{Post, TypePost};

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today.");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today.", post.content());

    let mut post2 = Post::new();

    post2.add_text("I didn't eat a salad for lunch today.");
    post2.request_review();
    post2.reject();

    let mut typepost = TypePost::new();

    typepost.add_text("Pizza time :)");
}
