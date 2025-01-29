mod vector;
mod string;
use std::collections::HashMap;

fn main() {
    vector::demonstrate_vector_operations();
    string::demonstrate_string_operations();
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

}
