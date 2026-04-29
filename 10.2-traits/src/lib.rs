pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Review {
    pub subject: String,
    pub rating: u8,
    pub author: String,
    pub text: String,
}

impl Summary for Review {
    fn summarize_author(&self) -> String {
        String::from(&self.author)
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        String::from(&self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("[{}]", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        repost: false,
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

pub fn push<T: Summary>(item: &T) {
    println!("Here's an update: {}", item.summarize());
}

pub fn pair_up<T, U>(t: &T, u: U) -> String
where
    T: Summary,
    U: Clone + Summary,
{
    format!("{} | {}", t.summarize(), u.summarize())
}
