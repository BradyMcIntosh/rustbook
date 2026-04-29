use traits::{Review, SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        repost: false,
    };

    let review = Review {
        subject: String::from("Atlas Shrugged"),
        rating: 3,
        author: String::from("That Guy With The Glasses"),
        text: String::from("Needs work."),
    };

    println!("1 new post: {}", post.summarize());
    println!("Review of {} by {}: {}", review.subject, review.author, review.summarize());
}
