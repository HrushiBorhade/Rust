// write a function that takes a string of words separated by spaces 
//and returns the first word it finds in that string. If the function 
//doesn’t find a space in the string, the whole string must be one word,
//so the entire string should be returned.

fn main() {
    
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("first word of hello world : {}", hello);
    println!("second word of hello world : {}", world );

    let start_slice = &s[..2];
    println!("start at index 0 slice: {}", start_slice);
   
    let end_slice = &s[2..];
    println!("end at length  slice: {}", end_slice);

    let first = first_word(&s);
    println!("first word is : {}" , first);

    let a = [1,2,3,4,5];
    let a_slice = &a[1..3];

    assert_eq!(a_slice, &[2,3]);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]

}

