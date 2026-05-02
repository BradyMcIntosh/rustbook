use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) ->&str {
        println!("Hear ye, hear ye! {announcement}");
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where T: Display,
{
    println!("Announcement: {ann}");
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}!");

    let novel = String::from("It was the best of times, it was the worst of times. The end.");
    let first_sentence = novel.split(".").next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence
    };

    println!("First sentence of novel: '{0}', lvl {1}", i.part, i.level());

    println!("Anyway, {}", i.announce_and_return_part("I forgot what I was going to say..."));

    let s: &'static str = "I have a static lifetime.";
    println!("This string has a static lifetime: '{s}'");
    println!("I don't know what that's good for, but I like how it sounds.");

    println!("{}", longest_with_an_announcement("abcd", "lorem ipsum", "The longest is...!"));
}
