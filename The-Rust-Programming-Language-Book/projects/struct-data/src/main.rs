fn main() {
    struct User {
        active: bool,
        username: String,
        email:String,
        sign_in_count: u32
    }


fn build_user(username:String, email:String) -> User {
    let user = User {
        active:true,
        username, 
        email,
        sign_in_count:1,
    };
    user
}
    let user1 = User {
        active:true,
        username:String::from("adam"),
        email:String::from("adam@gmail.com"),
        sign_in_count:1,
    }; 

    let mut user2 = User {
        active:true,
        username:String::from("eve"),
        email:String::from("eve@gmail.com"),
        sign_in_count:1,
    };

    println!("user 1 username: {}", user1.username);
    println!("user 2 username: {}", user2.username);
    user2.username = String::from("ema");
    println!("user 2 username after renaming: {}", user2.username);

    let user3 = build_user(String::from("steve"), String::from("steve@gmail.com"));
    println!("user 3 from build_user username: {}", user3.username);

}

