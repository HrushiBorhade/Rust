mod function_example;
mod struct_example;
fn main() {
    let list = vec![10,20,30,40,50];
    let result = function_example::largest(&list);
    println!("The largest element in the list : {result}");

    let list = vec!['a', 'y', 'm'];
    let result = function_example::largest(&list);
    println!("The largest element in the list : {result}");

    let both_integer = struct_example::Point {x:4, y:5};
    let both_float = struct_example::Point {x:4.0, y:5.0};
    let mixed_type = struct_example::Point {x:4, y:5.0};

    println!("The value of x in both_integer: {}", both_integer.x);
    
    println!("The value of x in both_float: {}", both_float.x);
   
    println!("The value of x in mixed_type: {}", mixed_type.x);
}

