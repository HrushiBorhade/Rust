// In this section, we’ll talk about the operations on String that every collection type has, such as creating, updating, and reading. We’ll also discuss the ways in which String is different from the other collections, namely how indexing into a String is complicated by the differences between how people and computers interpret String data.
pub fn demonstrate_string_operations() {
    println!("inside demonstrate string operation function");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}

