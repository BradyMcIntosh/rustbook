fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("User1: '{0}' at '{1}'; active: {2}; sign-in count: {3}", user1.username, user1.email, user1.active, user1.sign_in_count);

    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("otherusername456"),
        active: true,
        sign_in_count: 1,
    };

    println!("User2: '{0}' at '{1}'; active: {2}; sign-in count: {3}", user2.username, user2.email, user2.active, user2.sign_in_count);

    user2.active = false;
    user2.sign_in_count = 3;

    println!("User2: '{0}' at '{1}'; active: {2}; sign-in count: {3}", user2.username, user2.email, user2.active, user2.sign_in_count);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
