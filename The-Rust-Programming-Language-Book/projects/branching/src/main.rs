fn main() {
    let num = 6;
    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest.
    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 4, 3, or 2");
    }
}
