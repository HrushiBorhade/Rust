use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::mem::drop;

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    //associated type
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list : {:#?}", list);



    // Deref trait
    let x = 5;
    let y = &x;

    println!("y : {}", *y);

    let y  = MyBox::new(x);
    println!("y for MyBox: {}", *y);

    //Deref Coercion
    hello("Rust");
    hello(&MyBox("world"));

    // Drop trait - automatically calls after it is out of scope
    let c = CustomSmartPointer {
        data: String::from("random shit")
    };
    let d = CustomSmartPointer {
        data: String::from("random shit 2")
    };

    println!("CustomSmartPointers c and d created");
    drop(d);
    println!("d is now dropped before getting out of scope");
    println!("Out of scope")

}

fn hello(name: &str) {
    println!("hello {name}");
}
