// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.

use std::io;

fn main() {
    println!("Enter a string: ");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line.");

    let s = s.trim().to_string();

    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);

    // Mutable references have one big restriction: you can have only one mutable
    // reference to a particular piece of data in a particular scope
    let mut str = String::from("hello");

    {
        let r1 = &mut str;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut str;
    //  ----------------- ^^^^^^ mutable borrow occurs here

    let ref1 = &str; // no problem
    let ref2 = &str; // no problem
    println!("{ref1} and {ref2}");
    // variables r1 and r2 will not be used after this point

    let ref3 = &mut str; // no problem
    println!("{ref3}");
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
