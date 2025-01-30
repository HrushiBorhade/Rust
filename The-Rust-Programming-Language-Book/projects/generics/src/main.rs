mod function_example;

fn main() {
    let list = vec![10,20,30,40,50];
    let result = function_example::largest(&list);
    println!("The largest element in the list : {result}");

    let list = vec!['a', 'y', 'm'];
    let result = function_example::largest(&list);
    println!("The largest element in the list : {result}");
}
