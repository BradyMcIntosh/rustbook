fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    print_user(&user1);

    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("otherusername456"),
        active: true,
        sign_in_count: 1,
    };

    print_user(&user2);

    user2.active = false;
    user2.sign_in_count = 3;

    print_user(&user2);

    let user3 = build_user(String::from("thirdguy@example.com"), String::from("randomname789"));

    print_user(&user3);

    // struct update syntax
    let user4 = User {
        email: String::from("updated@example.com"),
        ..user1
    };

    print_user(&user4);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!("User: '{0}' at '{1}'; active: {2}; sign-in count: {3}", user.username, user.email, user.active, user.sign_in_count);
}
