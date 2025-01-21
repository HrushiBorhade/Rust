fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u32,
    }

    fn build_user(username: String, email: String) -> User {
        let user = User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        };
        user
    }
    let user1 = User {
        active: true,
        username: String::from("adam"),
        email: String::from("adam@gmail.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        active: true,
        username: String::from("eve"),
        email: String::from("eve@gmail.com"),
        sign_in_count: 1,
    };

    println!("user 1 username: {}", user1.username);
    println!("user 2 username: {}", user2.username);
    user2.username = String::from("ema");
    println!("user 2 username after renaming: {}", user2.username);

    let user3 = build_user(String::from("steve"), String::from("steve@gmail.com"));
    println!("user 3 from build_user username: {}", user3.username);

    //  In this example, we can no longer use user3 as a whole after creating user4
    // because the String in the username field of user3 was moved into user4.
    // If we had given user4 new String values for both email and username,
    // and thus only used the active and sign_in_count values from user3,
    // then user3 would still be valid after creating user4.
    // Both active and sign_in_count are types that implement the Copy trait,
    // so the behavior we discussed in the “Stack-Only Data: Copy” section would apply.
    // We can still use user3.email in this example, since its value was not moved out.

    let user4 = User {
        email: String::from("someother@gmail.com"),
        ..user3
    };

    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    
}
