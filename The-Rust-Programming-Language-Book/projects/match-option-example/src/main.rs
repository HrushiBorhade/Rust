fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    println!("value of plus_one on five:{:?} ",six);
    let none_value : Option<i32> = None;
    let none_value_plus_one = plus_one(none_value);
    println!("none plus one: {:?}", none_value_plus_one);

}
