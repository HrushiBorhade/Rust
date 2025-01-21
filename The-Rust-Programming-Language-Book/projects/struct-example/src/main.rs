#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    println!("Calculate area of rectangle with width = 30 and height = 50");
    let rect1 = Rectangle {
        width:30,
        height:50,
    };
    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");
    println!("The area of rectangle : {}", area(&rect1));
}

fn area(rectangle :&Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

