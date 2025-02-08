use crate::List::{Cons, Nil};
use std::mem::drop;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Condvar;

#[derive(Debug)]
enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
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
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // let b = Box::new(5);
    // println!("b = {b}");

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // println!("list : {:#?}", list);

    // Deref trait
    let x = 5;
    let y = &x;

    println!("y : {}", *y);

    let y = MyBox::new(x);
    println!("y for MyBox: {}", *y);

    //Deref Coercion
    hello("Rust");
    hello(&MyBox("world"));

    // Drop trait - automatically calls after it is out of scope
    let c = CustomSmartPointer {
        data: String::from("random shit"),
    };
    let d = CustomSmartPointer {
        data: String::from("random shit 2"),
    };

    println!("CustomSmartPointers c and d created");
    drop(d);
    println!("d is now dropped before getting out of scope");

    let a1 = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a1 = {}", Rc::strong_count(&a1));
    let a2 = Cons(3, Rc::clone(&a1));
    println!("count after creating a1 = {}", Rc::strong_count(&a1));
    {
        let a3 = Cons(4, Rc::clone(&a1));
        println!("count after creating a3 = {}", Rc::strong_count(&a1));
    }
    println!("count after a3 goes out of scope = {}", Rc::strong_count(&a1));
    println!("Out of scope");
}

fn hello(name: &str) {
    println!("hello {name}");
}
